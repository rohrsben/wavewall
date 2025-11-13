# Basics
Wavewall works using tilesets. A tileset is a configuration file, `tileset.lua`, and a collection of `.png` files (called tiles). The core idea is inside the tileset configuration you create recipes, which tell wavewall how to stitch together your tiles to create a final image. See the Tileset section in the sidebar for more!

Wavewall itself also has a configuration file, `wavewall.lua`, though this mostly deals with stuff like how big of an image to create and where to save it. Like tilesets, see the Wavewall section in the sidebar for the full story.

***

# What's a `.lua` file?
Wavewall is configured using the programming language [lua](https://www.lua.org). If you know it already, great! You'll be excited about the cool stuff this enables wavewall to do. If you don't though, don't worry! Wavewall's configuration is designed to be easy to use even if you've never programmed before. First, check out the [guide](/lua/guide) for the basics, and then get started!

***

# Getting started
First, create the configuration directory. This is either `$XDG_CONFIG_HOME/wavewall`, or `/home/<user>/.config/wavewall`. Then, inside the directory, create the file `wavewall.lua` and a directory for your tileset, along with a `tileset.lua`. It should be like this afterwards:
```
wavewall
├── my_tileset
│   └── tileset.lua
└── wavewall.lua
```

Now we need to add some tiles. Running `wavewall template <size>` will create a template `.png` of that size, which you can then draw on to your desire (I found [PIXILART](https://www.pixilart.com) worked pretty well). Put them in your tileset directory like so:

```
wavewall
├── my_tileset
│   ├── tile_a.png
│   ├── tile_b.png
│   └── tileset.lua
└── wavewall.lua
```

Once you've got some tiles, you just need a configuration. Inside your `.lua` files insert this text:
::: code-group
```lua [wavewall.lua]
return {
    output = {
        size = {
            height = 500, -- or, your monitors resolution here!
            width = 500
        },
    },
}
```

```lua [tileset.lua]
return {
    info = {
        size = {
            width = <your tile width here>,
            height = <your tile height here>
            -- right now, tiles have to be square. If you're seeing this,
            -- I'm still getting around to either changing the config
            -- or changing that requirement
        }
    },
    recipes = {
        first_recipe = {

        }
    }
}
```
:::

`first_recipe` has no options, but wavewall will still be able to run. From here, adjust how wavewall generates your image using the docs!
***

# The Documentation

The sidebar has a few sections. Here's what they mean:
- **Wavewall**: this section details what options you can put into `wavewall.lua` and what they do
- **Tileset**: much like above, this goes over the `tileset.lua` options
- **Commands**: `wavewall` has a few useful secondary commands. This details how to use them and what they do
- **Lua**: `wavewall` adds a few functions to the lua environment to make writing tileset configurations easier. This tells you what's been added and how to use them, as well as a simple lua guide.
