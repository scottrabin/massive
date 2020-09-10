use crate::schema::recipes;
use uuid::Uuid;

#[derive(Identifiable, Debug, Queryable)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub instructions: String,
}
