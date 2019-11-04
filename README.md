# vt100

This crate parses a terminal byte stream and provides an in-memory
representation of the rendered contents.

## Overview

This is essentially the terminal parser component of a graphical terminal
emulator pulled out into a separate crate. This can be used to not only
build graphical terminal emulators, but also many other types of
applications that need to interact with a terminal data stream directly,
such as terminal multiplexers or terminal recording applications.

## Synopsis

```rust
let mut screen = vt100::Screen::new(24, 80);
screen.process(b"this text is \x1b[31mRED\x1b[m");
```
