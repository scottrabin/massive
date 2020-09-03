use hyper::{header, Body, Method, Request, Response, StatusCode};
use juniper::{
    http::{GraphQLRequest, GraphQLResponse},
    RootNode,
};

use super::context::Context;
use super::mutation::Mutation;
use super::query::Query;

type Schema = RootNode<'static, Query, Mutation>;

/// Parses a request into the relevant GraphQL parts
async fn parse_request(req: Request<Body>) -> Result<GraphQLRequest, Box<dyn std::error::Error>> {
    match req.method() {
        &Method::POST => {
            let chunk = hyper::body::to_bytes(req.into_body()).await?;

            let input = String::from_utf8(chunk.into_iter().collect())?;

            Ok(serde_json::from_str::<GraphQLRequest>(&input)?)
        }
        _ => unimplemented!("{:?}", req),
    }
}

/// Converts a GraphQL result into an HTTP response containing the data
fn into_response(
    gql_result: GraphQLResponse,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string_pretty(&gql_result)?.into())
        .map_err(Into::into)
}

/// Converts a request sent to the GraphQL endpoint into the resulting data,
/// using the root node and context given as the basis.
pub async fn execute_graphql(
    req: Request<Body>,
    context: &Context,
) -> Result<Response<Body>, hyper::Error> {
    let root_node: Schema = Schema::new(Query, Mutation);

    let batch_request = parse_request(req).await.unwrap();

    let graphql_result = batch_request.execute(&root_node, context);

    into_response(graphql_result).map_err(|_| panic!())
}
