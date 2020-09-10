table! {
    ingredients (id) {
        id -> Uuid,
        name -> Varchar,
        density -> Float8,
        grams_per_unit -> Nullable<Float8>,
    }
}

table! {
    recipes (id) {
        id -> Uuid,
        name -> Varchar,
        instructions -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipes,
);
