---
title: Lua Guide
---

# About
This is not intended to be a full lua tutorial. This just helps differentiate and explain the more common ways that wavewall settings are created for people who have not programmed before.

# What are 'Types'?
Wavewall expects settings to be created in a specific way. If you set them wrong, nothing bad will happen: I've done my best to output useful error messages whenever something goes wrong. The error messages will look something like this:
```
Incorrect type (wavewall.output.offset)
  Expected: nil, bool
  Got: string
```

Here's what it means:
- `wavewall.output.offset` is the option that was incorrectly set
- `Expected: ...` is a list of the acceptable ways to set it
- `Got: ...` informs you of what was actually provided

`nil` and `bool` are called "types" in programming. There's a few more types besides, but the point is that wavewall expects option declarations to look a certain way. `Expected` has more than one type in the list because depending on how you set the option, wavewall will behave a different way. Now let's go over what types you'll need to understand.

# Types in wavewall

## Nil
If an option can be `nil`, that means it doesn't need to be set at all. This would provide optional functionality that you may or may not want to enable.

## Table
Tables are the meat and potatoes of a `wavewall` or `tileset` configuration file. Tables are kind of like containers of other types. Here's an example:

```lua
example_table = {
    this_is_a_string = "hello!",
    this_is_a_number = 5,
}
```
::: tip
Note the commas after every line! Those are required
:::

The table is everything inside the curly braces `{` and `}`, and it is assigned the name `example_table`. Tables are a lot like `JSON` if you're familiar with that. The names of the values (for instance, `this_is_a_string`) can be upper and lower case letters, and underscores. If we wanted to set `this_is_a_number` to `nil`, it would look like this - aka simply removing it:

```lua
example_table = {
    this_is_a_string = "hello!",
}
```

When there's a table nested inside a table, the items can be referred to with a short hand. For example:
```lua
option = {
    suboption = {
        setting = "example"
    },
    other_suboption = 5
}
```
To refer to `setting` you could say `option.suboption.setting`. This is also what that part means in the error message shown above.

## List
A list looks a lot like a table, but there are no equals signs. It would look like this:
```lua
example_list = {
    "first_item",
    "second_item",
    "third_item"
}
```
That's a list of strings, but wavewall will tell you what the list should be made of. A la `Expected: list of number`

## String
A string is some characters enclosed within quotation marks. They look like this:

```lua
all_one_string = "asd09qvw()&^FV(D*&6"
```

::: warning
The quotation marks are required! `"1"` and `1` might seem similar, but they mean very different things to wavewall
:::

Some options expect specific strings. Something like `Expected: string from ("a", "b", ...)`. This means what you might expect, and wavewall will tell you if you feed that option an invalid string.

## Color
This isn't a real type, but you'll see it in error messages. In reality, its anything that wavewall can accept to form a color. There's three options:
- string: a hex string, like `'#ff0000'` or (with alpha) `'#ff0000ff'`. Note the hashtag!
- table: a table with `r`, `g`, `b`, and optionally `a` values defined as numbers from 0 to 255
- a [ColorInfo object](/userdata/colorinfo)

::: details Examples
```lua
color1 = '#ff0000' -- red
color2 = '#00000080' -- a black thats kind of transparent
color3 = { r = 255, g = 255, b = 255} -- white
color4 = convert_hex('#ff0000') -- also red
  -- or, convert_hex(color1)
```
:::


## Number
A whole number, like `5` or `-100`. In almost every case these need to be positive; wavewall will tell you, but you'll probably also be able to figure it out yourself (what's a negative width?).

::: info
If you're particularly unlucky, you might see an error like this:

```
Incorrect type for '...'
  Expected: number
  Got: number
```

Make sure that you're not using a decimal. E.g. instead of `10.5`, use `10`
:::

## Function
Some options can be passed a lua function to enable more powerful features. I aim for users to never *need* to write a function, but in case you want to use one, check the documentation for that option to see what arguments the function requires and what return types it expects.

## Boolean
This refers to a value that is either true or false. It would look like this:
```lua
example_boolean = false
example_other = true
```
Note the lack of quotes!
