---
title: Colorizer
---

# About
`colorizer` is a setting which allows you to recolor the pixels in the output image after it has been generated.

You can do this by setting `colorizer` to either a function or a table.
If you choose a function, that function will be run for every pixel in the output image. This might be what you want! But it can also be a bit slow. Choosing table allows you to create direct color-to-color conversions, which are much faster to execute. You are still able to choose if a color should be recolored using a function, so unless necessary, I would go for the table version.

Whether you write one main function, or a function that only operates on one color, it will need to follow this spec:
- *Input*: one parameter, a [`PixelInfo` object](/userdata/pixelinfo)
- *Output*: one of:
    - a hex code, like `"#FF0000"` or `"#FF0000FF"`
    - a table with `r`, `g`, `b`, and `a` values. `a` is optional, and will default to `255`
::: details Example
```lua
function (info)
    -- interchanges the r, g, and b values
    return {
        r = info.g,
        g = info.b,
        b = info.r,
        a = info.a
    }
end
```
:::

## Ways to set
### Table
A `colorizer` table is a mapping from input color codes -- the left side of the equals sign -- to either another color code, or a function which follows the spec above. `colorizer.default` is treated specially: if this is set, any pixel which doesn't match a color code is fed to the `default`. This is probably most useful as a function, but you can set it to a color code as well if you like.

::: details Example
```lua
colorizer = {
    ["#000000"] = "#00FF00",
    ["#FF0000"] = function (info)
        -- function logic omitted
        return hex_code
    end,

    default = function (info)
        -- function logic omitted
        return hex_code
    end
}
```

Here black pixels (`"#000000"`) will get mapped to green (`"#00FF00"`), red pixels (`"#FF0000"`) will be set to the result of a function, and everything else will be set to the result of a second function.
:::
::: details Tips and Info
- The fields have to be set using the weird-looking `["#000000"]` because `#` means something in actual lua code. Doing it this way wraps it in a string.
- If you have a color-code variable, perhaps imported from something like a `colorscheme.lua`, and you want to use it on the left side of the `=`, make sure to wrap it in brackets as well. This is not necessary on the right side. A la
```lua
local my_variable = "#00FF00"
local other_var = "#0000FF"

colorizer = {
    [my_variable] = other_var
}
```
Otherwise Wavewall will attempt to convert `"my_variable"` to a color and throw an error
- You can include alpha values on both sides as well, for instance `"#FF0000FF"`

:::
### Function
See the example used in the function spec above. Setting `colorizer` this way is essentially the same as setting `colorizer` to a table with only `default` defined.

