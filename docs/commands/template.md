# About
`template` creates a template tile to make tileset creation easier.

## Usage
`wavewall template <size>`

- `size`: positive number

## Output
A black `.png`, with red markings at the center of the sides. The file will be named `template-<size>.png`, and be created in the configuration directory. Save it to a different location with the `--path` option for wavewall, like this:

```
wavewall --path /your/path/here.png template 256
```
