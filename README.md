# Ray
Simple backlight management tool.

Get more info about command by running `ray --help`.

# Install
- Install rust (by rustup recommended)
- `cargo install --git https://github.com/Amatrelan/ray`

Why no cargo crates?, maybe someday when I have energy to put CI to work for it.

# State of ray?
I have now used this about 2 months, and seems to work just fine, but there might always to be edge cases and cause some problems. So check if this works for you.

# Configurations
Currently there is no configuration available at all but functionality is quite self explanatory and all info is at `--help`.
One thing that's different from `light` is that you cannot decrease brightness with `decrease` below `1`, but you can always set it below `1`.

# Why `ray`
I used to use backlight control tool named [light](https://github.com/perkele1989/light) from `perkele1989`, but it seems to be so that he has dropped now support for it and is archived. I wanted to find similar simple tool, but there wasn't anything that ticked to marks for me. So I started writing my own.

Also `ray` isn't as powerful as light, and don't take care of many things. For example this wont help to setup udev rules.

# Troubleshooting
If this doesn't work, requires sudo, you need to add yourself to `video` group to access.
