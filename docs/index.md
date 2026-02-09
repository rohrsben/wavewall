# About
Wavewall is a tiling wallpaper generator. It takes small images (referred to as tiles) and stitches them together to form a larger, yet still cohesive image.

Wavewall is designed to provide very high control over the output image. You can pick which tiles to use and how often they should appear, and you're even able to recolor every pixel in the output programmatically to fit an existing aesthetic.

***

# How to use Wavewall
Wavewall is configured using the programming language [lua](https://www.lua.org). If you know it already, great! You'll be excited about the cool stuff this enables wavewall to do. If you don't though, don't worry! Wavewall's configuration is designed to be easy to use even if you've never programmed before. First, check out the [guide](/lua/guide) for the basics, and then get started!

::: info
Requires are a bit wonky at the moment. If you have the files `tilesetdir/{tileset.lua, stuff.lua}`, and want to import `stuff` from `tileset`, you will still need to fully qualify it as `require "tilesetdir.stuff"`. Basically, everything is from the perspective of `wavewall.lua`.
:::

***

# Getting started
First, create the configuration directory. This is either `$XDG_CONFIG_HOME/wavewall`, or `/home/<user>/.config/wavewall`. Then, inside the directory, create the file `wavewall.lua` and a directory for your tileset, along with a `tileset.lua`. It should be like this afterwards:
```
wavewall
├── wavewall.lua
└── my_tileset
    └── tileset.lua
```

Now we need to add some tiles. At the moment these are required to be `.png` files, and they all need to be the same size. Running `wavewall template <size>` will create a template `.png` of that size, which you can then draw on to your desire (I found [PIXILART](https://www.pixilart.com) worked pretty well). To speed things up, you can alsy try out one of the tilesets I provide on the [github repo](https://github.com/rohrsben/wavewall/tree/main/tilesets). Put the tiles in your tileset directory like so:

```
wavewall
├── wavewall.lua
└── my_tileset
    ├── tile_a.png
    ├── tile_b.png
    └── tileset.lua
```

Once you've got some tiles, you just need a configuration. Inside your `.lua` files insert this text:
::: code-group
```lua [wavewall.lua]
local tileset_config = require("my_tileset.tileset")

return {
    output = {
        height = 500, -- or, your monitors resolution here!
        width = 500
    },

    tileset = tileset_config
}
```

```lua [tileset.lua]
return {
    info = {
        name = "my_tileset"
    },
    recipes = {
        first_recipe = { }
    }
}
```
:::

`first_recipe` has no options, but wavewall will still be able to run. From here, adjust how Wavewall generates your image using the docs!
***

# The Documentation

The sidebar has a few sections. Here's what they mean:
- **Configuration**: this section details which options Wavewall allows you to set, how to set them, and what they do
- **Commands**: Wavewall has a few useful secondary commands, like `wavewall template` from above. This details how to use them and what they do
- **Lua**: Wavewall adds a few functions to the lua environment to make writing tileset configurations easier. This tells you what's been added and how to use them, as well as a simple lua guide.
- **UserData**: explains what the functions that are provided return, and how to use them
- **Other**: some settings require more in-depth explanation. This is their home
