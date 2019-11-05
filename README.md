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
let mut parser = vt100::Parser::new(24, 80);
parser.process(b"this text is \x1b[31mRED\x1b[m");
assert_eq!(
    parser.screen().cell(0, 13).unwrap().fgcolor(),
    vt100::Color::Idx(1),
);
```
