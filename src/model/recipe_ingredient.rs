use super::Ingredient;
use super::Recipe;
use crate::schema::recipe_ingredient_assoc;
use uuid::Uuid;

#[derive(Identifiable, Debug, Queryable, Associations)]
#[belongs_to(Recipe, foreign_key = "recipe_id")]
#[belongs_to(Ingredient, foreign_key = "ingredient_id")]
#[primary_key(recipe_id, ingredient_id)]
#[table_name = "recipe_ingredient_assoc"]
pub struct RecipeIngredient {
    pub recipe_id: Uuid,
    pub ingredient_id: Uuid,
}
