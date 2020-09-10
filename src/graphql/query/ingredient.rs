use juniper::object;

use crate::model;
#[derive(Clone, Debug)]
pub struct Ingredient {
    pub id: uuid::Uuid,
    pub name: String,
    pub density: f64,
}

#[object(Context=crate::graphql::context::Context)]
impl Ingredient {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn density(&self) -> f64 {
        self.density.into()
    }
}

impl From<model::Ingredient> for Ingredient {
    fn from(ingredient: model::Ingredient) -> Self {
        Ingredient {
            id: ingredient.id,
            name: ingredient.name,
            density: ingredient.density,
        }
    }
}
