# `info` - nil, table
Provides wavewall high-level information about the tileset. At the moment, this isn't necessary, but stay tuned!

# Fields
## `tile_size` - nil, positive number

Sidelength of the tiles, in pixels.

::: details Nil
If nil, wavewall will get the dimensions from a random tile.
:::

::: details Example
```lua
info = {
    tile_size = 32,
}
```
:::
