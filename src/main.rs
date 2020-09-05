use diesel::PgConnection;
use dotenv::dotenv;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use std::{convert::Infallible, env, net::SocketAddr};

use massive::router;

#[derive(Debug)]
enum BootstrapError {
    DatabaseConnectionError(String),
}

#[tokio::main]
async fn main() -> Result<(), BootstrapError> {
    dotenv().ok();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let pool = database_pool(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))?;

    let make_svc = make_service_fn(|_| {
        // TODO: cloning a pool isn't expensive, but is there any way to avoid this seemingly-unnecessary clone?
        let pool = pool.clone();

        async move {
            Ok::<_, Infallible>(service_fn(move |request: Request<Body>| {
                router::route(request, pool.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}

type ConnectionPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

fn database_pool(database_url: &str) -> Result<ConnectionPool, BootstrapError> {
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);

    diesel::r2d2::Pool::builder()
        .max_size(2)
        .build(manager)
        .map_err(|err| BootstrapError::DatabaseConnectionError(err.to_string()))
}
