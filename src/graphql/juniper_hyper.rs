use hyper::{header, Body, Method, Request, Response, StatusCode};
use juniper::{
    http::{GraphQLRequest, GraphQLResponse},
    RootNode,
};

use super::{context::Context, error::GQLError, mutation::Mutation, query::Query};

/// Converts a GraphQL result into an HTTP response containing the data
fn into_response(gql_result: GraphQLResponse) -> Result<Response<Body>, super::GQLError> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string_pretty(&gql_result)?.into())
        .map_err(Into::into)
}

/// Converts a request sent to the GraphQL endpoint into the resulting data,
/// using the root node and context given as the basis.
pub async fn execute_graphql<'a, 'b>(
    req: Request<Body>,
    context: Context,
) -> Result<Response<Body>, GQLError> {
    let root_node: RootNode<'b, Query, Mutation> = RootNode::new(Query, Mutation);

    let graphql_request = match req.method() {
        &Method::POST => parse_post_request(req).await,
        &Method::GET => unimplemented!("GET /graphql is not yet implemented"),
        method => Err(GQLError::UnrecognizedMethod(method.clone())),
    }?;

    let graphql_result = graphql_request.execute(&root_node, &context);

    into_response(graphql_result)
}

/// Parses a POST request into a GraphQLRequest
async fn parse_post_request(req: Request<Body>) -> Result<GraphQLRequest, GQLError> {
    let chunk = hyper::body::to_bytes(req.into_body()).await?;

    let input = String::from_utf8(chunk.into_iter().collect())?;

    Ok(serde_json::from_str::<GraphQLRequest>(&input)?)
}
