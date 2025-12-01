---
title: Recipes
---

# About
A recipe lets you tell Wavewall how to combine the tiles you've created to form an output image. `recipes` is where you... well, define the recipes! It's a table with this shape:

```
recipes = {
    recipe name = recipe definition
}
```

Below are the fields you can set in each recipe definition

## Fields
- `tiles` - nil, list of string, table

What tiles should be used in the recipe.

::: details Nil
If `tiles` is nil, Wavewall will use any and all tiles in the tileset.
:::

::: details List of string
If `tiles` is a list of strings (here tile names), Wavewall will only select from tiles in the list.

::: details Example
```lua
tileset = {
    recipes = {
        recipe_list = {
            tiles = {
                "tile_a",
                "tile_b",
                "tile_c"
            }
        }
    }
}
```
:::

::: details Table
Setting `tiles` as a table allows you to control the likelihood with which each tile is chosen.

The numbers here are weights, which means that what matters is how large they are compared to each other, rather than how big the number itself is. That said, an easy way to reason about it is to make sure all the numbers add to 100 as if they were percentages.

::: details Example
```lua
tileset = {
    recipes = {
        recipe_weights = {
            tiles = {
                tile_a = 80,
                tile_b = 15,
                tile_c = 5
            }
        }
    }
}
```
:::

- `placer` - nil, function

Allows the user to choose where to place tiles.

Function spec:
- *Input*: one parameter, an [Anchor](/userdata/anchor)
- *Output*: one of:
    - `string`, here a tile name which is included in the recipe's `tiles`
    - `nil`, to fall back to random selection according to `tiles`

::: details Example
```lua
placer = function (anchor)
    if anchor.y == 3 then
        return "tile_a"
    end

    return nil
end
```

This function sets all tiles in the 4th row to `tile_a`, and falls back to random selection otherwise
:::
