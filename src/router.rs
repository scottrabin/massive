use crate::graphql;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use hyper::{Body, Method, Request, Response, StatusCode};
use std::time::Duration;

pub async fn route(
    req: Request<Body>,
    database: Pool<ConnectionManager<PgConnection>>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let context = graphql::context::Context {
        db: database.get_timeout(Duration::new(1, 0))?,
    };

    Ok(match (req.method(), req.uri().path()) {
        (&Method::POST, "/graphql") => {
            graphql::juniper_hyper::execute_graphql(req, context).await?
        }
        (&Method::GET, "/graphiql") => {
            Response::new(juniper::http::graphiql::graphiql_source("/graphql").into())
        }
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())?,
    })
}
