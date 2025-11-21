---
title: Pseudotiles
---

# About
Pseudotiles are transformations of "real" tiles. A real tile is one which is created from a `.png` in the tileset's directory.

These real tiles are then transformed in some way, either a rotation or a mirroring across an axis. 

After creation, these pseudotiles are usable anywhere a real tile is (except for inside `pseudotiles`!)

### The transformations
- Rotations: `"90"`, `"180"`, `"270"`
- Mirrorings: `"horizontal"`, `"vertical"`, `"diagonal"`, `"antidiagonal"`

::: details Examples
TODO
:::

### Setting the option
`pseudotiles` is a table of tables. The keys are the names of real tiles, and the corresponding tables are which pseudos to create. The generalized form is this:

```
real tile name = {
    pseudo tile name = transformation to apply
}
```

And in reality, like this:

```lua
pseudotiles = {
    tile_a = {
        pseudo_a = "90",
        pseudo_b = "horizontal"
    },
    tile_b = {
        pseudo_c = "180",
        pseudo_d = "horizontal"
    }
}
```
