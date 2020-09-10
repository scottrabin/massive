table! {
    ingredients (id) {
        id -> Uuid,
        name -> Varchar,
        density -> Float8,
        grams_per_unit -> Nullable<Float8>,
    }
}

table! {
    recipe_ingredient_assoc (recipe_id, ingredient_id) {
        recipe_id -> Uuid,
        ingredient_id -> Uuid,
    }
}

table! {
    recipes (id) {
        id -> Uuid,
        name -> Varchar,
        instructions -> Text,
    }
}

joinable!(recipe_ingredient_assoc -> ingredients (ingredient_id));
joinable!(recipe_ingredient_assoc -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipe_ingredient_assoc,
    recipes,
);
