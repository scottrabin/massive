use super::context::Context;
use crate::model;
use diesel::prelude::*;
use juniper::FieldResult;

mod ingredient;
mod recipe;

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    async fn recipes(&self, context: &Context) -> FieldResult<Vec<recipe::Recipe>> {
        use crate::schema::recipes::dsl::*;

        let result = recipes.limit(5).load::<model::Recipe>(&context.db)?;

        Ok(result.into_iter().map(Into::into).collect())
    }
}
