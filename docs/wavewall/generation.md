# `generation` (optional) - table

Controls some specifics about the image generation.

# Fields

## `tileset` (optional) - string, list of string
Which tileset to use when generating the image.

::: details String
Specifies the exact tileset to use.

Example:
```lua
generation = {
    tileset = "my_tileset",
},
```
:::

::: details List of string
Here the strings are the names of tilesets. Wavewall will randomly select an entry from the list.

Example:
```lua
generation = {
    tileset = {
        "tileset_a",
        "tileset_b"
    },
}
```
:::

## `offset` (optional) - boolean
By default, wavewall slightly shifts the output so that tiles don't end up in the same place across multiple images. This option allows you to disable that behaviour

Example:
```lua
generation = {
    offset = false
}
```
