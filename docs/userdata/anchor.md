---
title: Anchor
---
# About
`Anchor` is a table provided to a `placer` function. It has the following fields:

- `x`: the x coordinate of the anchor
- `y`: the y coordinate of the anchor

The coordinates are not in pixels; rather, they are in tile lengths. If we imagine an output image that can fit four tiles inside it, the anchors would look like this:

```
┌───────┬───────┐
│ (0,0) │ (1,0) │ (2,0)
├───────┼───────┤
│ (0,1) │ (1,1) │ (2,1)
└───────┴───────┘
  (0,2)   (1,2)   (2,2)
```
in `(x, y)` format.

Note that there will be an extra anchor both to the right and down for every image. This is to make up for shifting caused by the option `output.offset`. These are the anchors "outside" the box above (though they will likely still be visible, at least a bit!)
