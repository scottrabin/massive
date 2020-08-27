use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{convert::Infallible, net::SocketAddr};

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    return Ok(Response::new("Hello, World".into()));
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
