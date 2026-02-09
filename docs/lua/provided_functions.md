---
title: Provided Functions
--- 

# About

Wavewall provides some functions to use inside of its lua environment. These are intended to simplify some of the more annoying or repetitive parts of creating a tileset.

## `convert_rgb`
- *Input*: `r`, `g`, and `b` values: number from 0-255
- *Output*: a [`ColorInfo` object](/userdata/colorinfo)

::: details Example
```lua
color_info = convert_rgb(0, 0, 0)
print(color_info.hex) -- prints "#000000FF"
```
:::

## `convert_rgba`
- *Input*: `r`, `g`, `b`, and `a` values: number from 0-255
- *Output*: a [`ColorInfo` object](/userdata/colorinfo)

::: details Example
```lua
color_info = convert_rgba(0, 0, 0, 255)
print(color_info.hex) -- prints "#000000FF"
```
:::

## `convert_hex`
- *Input*: a hex color string
- *Output*: a [`ColorInfo` object](/userdata/colorinfo)

::: details Example
```lua
color_info = convert_hex("#000000")
print(color_info.r) -- prints "0"
```
:::

## `gradient`
Interpolates a gradient. For now, alpha values are lost when creating the gradient. This function is most useful when `at` is set to something dynamic, like `at = info.x/image_width` in a colorizer.

- Input: a table with these fields
    - `stops`: a list of [color](/lua/guide#color) values
    - `at`: a decimal value, from [0, 1.0]
- Output: a [`ColorInfo` object](/userdata/colorinfo)

::: details Examples
```lua
local args = {
    stops = {'#000000', '#ffffff'} -- black to white
    at = 0.25 -- 25% of the way through, so a dark grey
}
local dark_grey = gradient(args)

local args = {
    stops = { -- red -> green -> blue
        '#ff0000',
        { r = 0, g = 255, b = 0 },
        convert_rgb(0, 0, 255)
    },
    at = 0.9
}
local blueish =  gradient(args)
```
:::

## `rand_range`
Provides a random number in the given range.

- Input: two parameters:
    - `low`: positive number
    - `high`: positive number
- Output: a number within [high, low], aka inclusive.

## `create_all_pseudos`
Eases the creation of pseudotiles. Given the name of an original, creates all 7 transforms with simple names.

- *Input*: string of valid tile name
- *Output*: table for use in `pseudotiles`

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
