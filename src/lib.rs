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
//! let mut parser = vt100::Parser::new(24, 80, 0);
//! parser.process(b"this text is \x1b[31mRED\x1b[m");
//! assert_eq!(
//!     parser.screen().cell(0, 13).unwrap().fgcolor(),
//!     vt100::Color::Idx(1),
//! );
//! ```

// XXX this is broken with ale
// #![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::single_match)]
#![allow(clippy::too_many_arguments)]

mod attrs;
mod cell;
mod grid;
mod parser;
mod row;
mod screen;
mod term;

pub use attrs::Color;
pub use cell::Cell;
pub use parser::Parser;
pub use screen::{MouseProtocolEncoding, MouseProtocolMode, Screen};
