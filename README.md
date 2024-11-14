# Fir

Fir is a simple image viewer for Wayland. (It may or may not work on other platforms, but Wayland is the only one with first-class support.)


## Installation

Run the `install.sh` script. This will install fir under `/usr/local` by default, but this can be changed by setting the `PREFIX` environment variable.


## Configuration

Fir is configured by `$XDG_CONFIG_HOME/fir/config.toml` (`XDG_CONFIG_HOME` defaults to `$HOME/.config`).

Default settings:

```toml
# Distance in pixels that the Shift actions move the image.
shift_amount = 16

# The [grid] section holds settings relating to the background transparency grid.
[grid]
# Size in pixels of the background grid squares.
size = 16
# Color of the darker grid squares.
color_dark = "#404040"
# Color of the lighter grid squares.
color_light = "#808080"

# This table defines the keybinds. As of now, the default binds are always loaded first, so you can only overwrite them, not get rid of them entirely. This will be fixed in the future. Also, not all keys are bindable, since the SDL key names are not all valid as toml identifiers. This will be fixed in a future version. I've added a hack to somewhat mitigate this issue by allowing underscores to replace spaces in key names. A list of key names can be found [here](https://github.com/libsdl-org/SDL/blob/SDL2/src/events/SDL_keyboard.c#L350).
[binds]
Up = "ShiftUp"
Down = "ShiftDown"
Left = "ShiftLeft"
Right = "ShiftRight"
Keypad_0 = "ResetTransform"
F = "ToggleFullscreen"
Q = "Quit"
# These two are some of the keys whose names aren't valid toml, so their bindings can't actually be changed from the defaults. Sorry.
#Keypad_+ = "ZoomIn"
#Keypad_- = "ZoomOut"
```
