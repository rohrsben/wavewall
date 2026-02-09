---
title: PixelInfo
---

# About
`PixelInfo` is a table provided to functions in a `colorizer`. It has the following fields:

- `r`: the r value of the color (number from 0-255)
- `g`: the g value of the color (number from 0-255)
- `b`: the b value of the color (number from 0-255)
- `a`: the a value of the color (number from 0-255)
- `hex`: a string hex representation of the color (example: `"#00FF00FF"`)
- `x`: the x index of the pixel in the output image (number)
- `y`: the y index of the pixel in the output image (number)
- `anchor_x`: the x index of the tile's [anchor](/userdata/placerinfo) (number)
- `anchor_y`: the y index of the tile's [anchor](/userdata/placerinfo) (number)
- `tile_x`: the x index of the pixel in its tile (number)
- `tile_y`: the y index of the pixel in its tile (number)
- `tile_name`: the name of the tile the pixel is inside of (string)

`(x, y)` follows standard image indexing: `(0, 0)` in the top left, `(width - 1, height - 1)` in the bottom right.
