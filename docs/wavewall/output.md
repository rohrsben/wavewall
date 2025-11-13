# `output` - table

Output controls where and how wavewall creates the final result image.

# Fields
## `size` - table

The size of the output image, measured in pixels.

### Fields
- `height`: number
- `width`: number

## `filename` (optional) - string

The name of the saved image.

### Default
`"result.png"`

## `directory` (optional) - string
The directory where the image is saved.

### Default
The configuration directory, typically `$XDG_CONFIG_HOME/wavewall`
