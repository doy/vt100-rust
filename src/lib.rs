//! This crate parses a terminal byte stream and provides an in-memory
//! representation of the rendered contents.
//!
//! # Overview
//!
//! This is essentially the terminal parser component of a graphical terminal
//! emulator pulled out into a separate crate. This can be used to not only
//! build graphical terminal emulators, but also many other types of
//! applications that need to interact with a terminal data stream directly,
//! such as terminal multiplexers or terminal recording applications.
//!
//! # Synopsis
//!
//! ```
//! let mut screen = vt100::Screen::new(24, 80);
//! screen.process(b"this text is \x1b[31mRED\x1b[m");
//! ```

// XXX this is broken with ale
// #![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::single_match)]

mod attrs;
mod cell;
mod grid;
mod row;
mod screen;
mod unicode;

pub use attrs::Color;
pub use cell::Cell;
pub use screen::{MouseProtocolEncoding, MouseProtocolMode, Screen};
