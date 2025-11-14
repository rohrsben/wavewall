# `recipes` - table
A recipe is where you define the different ways to combine the tiles in your tileset. `recipes` itself is a simple table, where every pair is just a name and a recipe. It looks like this:
```lua
recipes = {
    recipe_a = { ... },
    recipe_b = { ... },
}
```
At least one recipe needs to be defined in order to use the tileset. The simplest recipe doesn't actually require any options to be set, so the mininum configuration would look like this:
```lua
recipes = {
    default = {} -- "default" has no special meaning here
}
```


## `recipe` - table
At the moment a `recipe` is just one setting, though this is likely to change in the future (wavewall is in beta after all!).

### Fields
#### `tiles` - nil, list of string, table
`tiles` describes which tiles (pseudo or otherwise) to use when generating the image. 

::: details Nil
Leaving `tiles` nil will cause the recipe to use every tile.
:::

::: details List of string
If tiles is set as a list of strings, here being tile names, then for each spot in the result image where a tile should go, it will be randomly chosen from this list.

::: details Example
```lua
recipe_name = {
    tiles = {
        "tile_a",
        "tile_b",
        "another_tile"
    },
}
```
:::

::: details Table
This is a lot like the list of string version, except it allows you to pick how often each tile is chosen. It looks like this:

```lua
recipe_name = {
    tiles = {
        tile_a = 100,
        tile_b = 10,
        another_tile = 1
    }
}
```

The numbers here are weights, which means what matters is how large they are compared to one another, not what they add up to. That said, an easy way to get the likelihood you want is to make sure everything adds to 100.
