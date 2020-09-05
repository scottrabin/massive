use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub instructions: String,
}
