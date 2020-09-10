use crate::schema::ingredients;
use uuid::Uuid;

#[derive(Identifiable, Debug, Queryable)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
    pub density: f64,
    pub grams_per_unit: Option<f64>,
}
