---
title: Tileset
---

# About
The `tileset` section allows you to set which and how tiles should be stitched together.

I recommend defining this in a `tileset.lua` file which is contained in the tileset directory, and then importing it into `wavewall.lua`. This allows tilesets to be easily shareable, and I think that would be really cool to see! But more importantly, the settings probably can't be reused across tilesets, so it makes sense to keep them self-contained.

## Fields
### `info` - table
`info` is basically metadata for the tileset.

**Fields**:
- `name` - string

This has to be the same as the tileset's folder
- `tile_size` - nil, positive number

For now, this is automatically inferred and doesn't need to be set. Stay tuned though!

::: details Example
```lua
tileset = {
    info = {
        name = "my_tileset"
    }
}
```
:::

### `pseudotiles` - nil, table
See [Pseudotiles](/other/pseudotiles)

::: details Example
```lua
tileset = {
    pseudotiles = {
        tile_a = {
            pseudo_a = "90",
            pseudo_b = "horizontal"
        }
    }
}
```
:::

### `recipe` - nil, string
Allows you to specify a recipe to use.

::: details Example
```lua
tileset = {
    recipe = "some_recipe"
}
```
:::

### `recipes` - table
See [Recipes](/other/recipes)

::: details Example
```lua
tileset = {
    recipes = {
        recipe_a = {
            tiles = {"tile_a", "tile_b"},
        }
        recipe_b = {
            tiles = {
                tile_a = 95,
                pseudo_a = 5
            }
        }
    }
}
```
:::
