# Wayclicker
Rust based (soon to be GUI) autoclicker for wayland.

# Building
## Nix (recommended)
The resulting binary can be found in `./result/bin/wayclicker`.
```
nix build
```

# Usage
Make sure to run the following commands with sudo (recommended), or to add your user to the `input` group.
First, list device names connected to your system.
```
wayclicker list
```
Once you find your keyboard, start the server with the command below. You may need to try multiple devices.
```
wayclicker server --device /path/to/device
```
Now you should be set up. The default hotkey to toggle the autoclicker is F5, and the default clickspeed is 20hz. You can find more information with `wayclicker --help`.
