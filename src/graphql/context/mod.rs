use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

pub struct Context {
    pub db: PooledConnection<ConnectionManager<PgConnection>>,
}

impl juniper::Context for Context {}
