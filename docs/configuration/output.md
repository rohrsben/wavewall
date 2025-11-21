---
title: Output
---

# About
The `output` section contains settings related to the output image, as well as some non-tileset-specific generation settings.

## Fields

### `directory` - nil, string
The directory in which to save the generated image.
::: details Nil
If `directory` is nil, it defaults to the configuration directory.
:::
::: details String
A fully expanded path.

::: details Example
`directory = "/home/<user>/directory"` (no trailing slash)
:::

### `filename` - nil, string
What name to give the generated `.png` file.
::: details Nil
If `filename` is nil, it defaults to `result.png`.
:::
::: details String
Any string that can be a valid filename on your operating system.
::: details Example
`filename = "save_file.png"`
:::

### `offset` - nil, bool
Wavewall adds a small amount of offset to where it places the tiles, to create a bit of variance between generations. This setting allows you to disable that behaviour.

::: details Example
```lua
offset = false
```
:::

### `size` - table
Specifies the dimensions of the output image.

**Fields**:
- `height` - positive number
- `width` - positive number

::: details Example
```lua
size = {
    width = 1920,
    height = 1080
}
```
:::

## Example
```lua
output = {
    filename = "not_result.png",
    directory = "/home/<user>/some/dir",
    size = {
        height = 50,
        width = 1000
    }
}
