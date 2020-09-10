use juniper::object;

use crate::model;

#[derive(Clone, Debug)]
pub struct Recipe {
    pub id: uuid::Uuid,
    pub name: String,
    pub instructions: String,
}

#[object(Context=crate::graphql::context::Context)]
impl Recipe {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn instructions(&self) -> &str {
        &self.instructions
    }
}

impl From<model::Recipe> for Recipe {
    fn from(recipe: model::Recipe) -> Self {
        Recipe {
            id: recipe.id,
            name: recipe.name,
            instructions: recipe.instructions,
        }
    }
}
