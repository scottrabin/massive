use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use std::net::SocketAddr;

mod graphql;

async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let context = graphql::context::Context {};

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/graphql") => graphql::juniper_hyper::execute_graphql(req, &context).await,
        (&Method::GET, "/graphiql") => Ok(Response::new(
            juniper::http::graphiql::graphiql_source("/graphql").into(),
        )),
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())
            .unwrap()),
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(router))
    }));

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
