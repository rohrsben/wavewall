---
title: Provided Functions
--- 

# About

Wavewall provides some functions to use inside of its lua environment. These are intended to simplify some of the more annoying or repetitive parts of creating a tileset.

## `convert_rgb`
::: details Function Spec
- *Input*: `r`, `g`, and `b` values: number from 0-255
- *Output*: a [`ColorInfo` object](/userdata/colorinfo)
:::

::: details Example
```lua
color_info = convert_rgb(0, 0, 0)
print(color_info.hex) -- prints "#000000FF"
```
:::

## `convert_rgba`
::: details Function Spec
- *Input*: `r`, `g`, `b`, and `a` values: number from 0-255
- *Output*: a [`ColorInfo` object](/userdata/colorinfo)
:::

::: details Example
```lua
color_info = convert_rgba(0, 0, 0, 255)
print(color_info.hex) -- prints "#000000FF"
```
:::

## `convert_hex`
::: details Function Spec
- *Input*: a hex color string
- *Output*: a [`ColorInfo` object](/userdata/colorinfo)
:::

::: details Example
```lua
color_info = convert_hex("#000000")
print(color_info.r) -- prints "0"
```
:::

## `create_all_pseudos`
Eases the creation of pseudotiles. Given the name of an original, creates all 7 transforms with simple names.

::: details Function Spec
- *Input*: string of valid tile name
- *Output*: table for use in `pseudotiles`
:::

::: details Example
```lua
pseudos = create_all_pseudos("original")

-- pseudos = {
--     original_90 = "90",
--     original_180 = "180",
--     original_270 = "270",
--     original_horizontal = "horizontal",
--     original_vertical = "vertical",
--     original_diagonal = "diagonal",
--     original_antidiagonal = "antidiagonal"
-- }

return {
    -- snip
    pseudotiles = {
        original = pseudos, -- or just 'original = create_all_pseudos("original")'
    }
}
```
:::
