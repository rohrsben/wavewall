# `colorizer` (optional) - table, function
::: details Function Spec
- *Input*: a [`PixelInfo` object](/userdata/pixelinfo)
- *Output*: one of
    - a string hex code, like `"#303030"` or `"#303030FF"`
    - an rgb table with values 0-255, such as
```lua
{
    r = 255,
    g = 100,
    b = 30,
    a = 255 -- optional
}
```
:::
Setting the `colorizer` allows you to re-color the pixels in the output image. This is done one of two ways:
- provide a direct mapping from color-code to color-code, or
- provide a function which follows the spec above

These two methods can be mixed in different ways, so use whichever makes the most sense. That said, the direct mapping is *much* faster.

::: details Table
A `colorizer` table requires two fields:
- `conversions` - table
- `default` (optional) - function

`conversions` is a mapping from color-code to either another color-code, or a function. This is best understood with an example:
```lua
colorizer = {
    conversions = {
        ["#000000"] = "#FF0000",
        ["#00FF00"] = function (pixel_info)
            -- function logic omitted
            return color_code
        end
    }
}
```
In this example, each black pixel (`"#000000"`) gets mapped to red (`"#FF0000"`), while green pixels (`"#00FF00"`) get mapped to the result of a function which follows the above spec.

Alpha values are allowed as well, such as `"#000000FF"`, on both sides.

`default` is a function which is run for any pixel that does not have a match in the `conversions` table. If one is not provided (leaving it nil) then pixels that would hit the default are left unchanged.
:::

::: details Function
Setting `colorizer` to a single function causes wavewall to call that function for every single pixel. That might be what you want! But keep in mind that it may take a while depending on the output resolution
:::

