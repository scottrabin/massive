use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
    pub density: f32,
    pub grams_per_unit: Option<f32>,
}
