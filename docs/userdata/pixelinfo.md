# About
`PixelInfo` is provided to functions in a `colorizer`. It has the following fields:

- `r`: the r value of the color (number from 0-255)
- `g`: the g value of the color (number from 0-255)
- `b`: the b value of the color (number from 0-255)
- `a`: the a value of the color (number from 0-255)
- `hex`: a string hex representation of the color (example: `"#00FF00FF"`)
- `x`: the x index of the pixel in the output image
- `y`: the y index of the pixel in the output image

`(x, y)` follows standard image indexing: `(0, 0)` in the top left, `(width - 1, height - 1)` in the bottom right.
