use juniper::object;

use crate::schema::ingredients;
use crate::schema::recipe_ingredient_assoc;
use crate::{graphql::context::Context, model};

use super::ingredient::Ingredient;

#[derive(Clone, Debug)]
pub struct Recipe {
    pub id: uuid::Uuid,
    pub name: String,
    pub instructions: String,
}

#[object(Context=Context)]
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

    fn ingredients(&self, context: &Context) -> Vec<Ingredient> {
        use diesel::pg::expression::dsl::any;
        use diesel::query_dsl::*;

        let ingredient_ids =
            model::RecipeIngredient::belonging_to(Into::<model::Recipe>::into(self))
                .select(recipe_ingredient_assoc::ingredient_id);

        ingredients::table
            .filter(ingredients::id.eq(any(ingredient_ids)))
            .load::<model::Ingredient>(&context.db)
            .expect("could not load ingredients")
            .into_iter()
            .map(Into::into)
            .collect()
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

impl From<&Recipe> for model::Recipe {
    fn from(recipe: &Recipe) -> Self {
        model::Recipe {
            id: recipe.id,
            name: recipe.name,
            instructions: recipe.instructions,
        }
    }
}
