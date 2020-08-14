## Quickstart

```bash
$ docker-compose up
```

Logging in to the `database` container:

```bash
$ docker-compose run database bash
```

## GraphQL Schema

```gql
type Recipe {
    id: ID!
    name: String!
    ingredients: IngredientConnection!
    steps: [String!]!
}

type IngredientConnection {
    edges: [IngredientEdge!]!
}

union IngredientEdge = MassIngredientEdge | CountIngredientEdge

type MassIngredientEdge {
    node: Ingredient!
    quantity: Float!
    unit: IngredientUnit!
}

type CountIngredientEdge {
    node: CountIngredient!
    quantity: Float!
}

union IngredientUnit = Gram | Milliliter | Quantity

interface Ingredient {
    id: ID!
    name: String!
    # grams per milliliter
    # converting mass to volume: (mass / density) = volume
    # converting volume to mass: (volume * density) = mass
    density: Float!
}

# Ingredients which come in discrete quantities, e.g. eggs
type CountIngredient implements Ingredient {
    gramsPerUnit: Float!
}

# Ingredients which come in separable quantities, e.g. flour
type MassIngredient implements Ingredient {
}

type RecipeBook {
    id: ID!
    recipes: RecipeConnection!
}

type RecipeConnection {
    edges: [RecipeEdge]
    pageInfo: PageInfo!
}

type PageInfo {
    # TODO
}

mutation CreateRecipe {

}

mutation UpdateRecipe {

}

mutation DeleteRecipe {

}

mutation CreateIngredient {

}

mutation UpdateIngredient {

}

mutation DeleteIngredient {
    // Not implemented
}
```

## Database Schema

```
ingredients
===========
id: UUID NOT NULL
name: string NOT NULL
density: float NOT NULL
grams_per_unit: float

recipes
=======
id: UUID NOT NULL
name: string NOT NULL
instructions: text NOT NULL

ingredients_in_recipe
=====================
recipe_id: UUID NOT NULL FKEY (recipes.id)
ingredient_id: UUID NOT NULL FKEY (ingredients.id)
quantity: float NOT NULL
unit: enum ("gram", "milliliter", "quantity")
```
