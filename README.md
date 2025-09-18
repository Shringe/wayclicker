# Wayclicker
Rust based (soon to be GUI) autoclicker for wayland. This was created because I have found no good autoclickers for wayland yet. This autoclicker uses uinput for sending clicks and evdev for listening for global hotkeys, bypassing the wayland compositor, so it should work for any system.

# Building
## Nix (recommended)
The resulting binary can be found in `./result/bin/wayclicker`.
```
nix build
```

# Usage
## Using the server directly
Make sure to run the following commands with sudo (recommended), or to add your user to the `input` group.
Adding your user to the `input` group is not recommended, as it allows any app to read your keystrokes similar to X11, diminishing many of the security enhancehments that wayland offers.
First, list device names connected to your system.
```
wayclicker list
```
Once you find your keyboard, start the server with the command below. You may need to try multiple devices.
```
wayclicker server --device /path/to/device
```
Now you should be set up. The default hotkey to toggle the autoclicker is F5, and the default clickspeed is 20hz. You can find more information with `wayclicker --help`.

## GUI (experimental)
Run the command below as a normal user. Wayclicker will use `pkexec` to prompt for superuser when necessary.
```
wayclicker client
```
