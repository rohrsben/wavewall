# `pseudotiles` - nil, table
Pseudotiles allow you to add tiles to your tileset without creating a whole new `.png` file. They are transformations of "original" tiles, or tiles that do have a corresponding `.png`.

Each entry in the table has a left-hand side of an original, and a right-hand side of another table. This table describes what transformations to apply, and what the new pseudotile should be called.

::: details The transformations
Valid transformations are a string from the following 7 options:
- Rotations: `"90"`, `"180"`, and `"270"`
- Reflections: `"horizontal"`, `"vertical"`, `"diagonal"`, and `"antidiagonal"`
:::

For example, say you've got two `.pngs` in your tileset: `tile_a` and `tile_b`, and you want to create some pseudotiles from them. It would look like this:

```lua
pseudotiles = {
    tile_a = {
        pseudo_a = "90",
        pseudo_b = "diagonal",
    },
    tile_b = {
        pseudo_c = "90",
        pseudo_d = "horizontal",
    },
}
```

This would bring the number of "usable" tiles in your tileset from 2 to 6.

Pseudotiles are usable in every place that a regular tile is, except for inside `pseudotiles`.

::: info
Wavewall will let you know if what you're trying to do is impossible, but at the moment it doesn't check for anything else. For instance, you could define 100 pseudotiles of the exact same transform and wavewall wouldn't complain! 
:::
