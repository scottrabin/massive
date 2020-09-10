-- Your SQL goes here
CREATE table recipe_ingredient_assoc
(
    recipe_id uuid references recipes(id),
    ingredient_id uuid references ingredients(id),

    PRIMARY KEY (recipe_id, ingredient_id)
)