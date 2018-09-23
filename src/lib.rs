#![cfg_attr(feature = "cargo-clippy", feature(tool_lints))]
// we use empty enums to represent opaque c pointers, but we don't have a way
// to indicate that those pointers do actually have additional alignment
// restrictions, so casting them to their prefixes is actually safe
#![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]

extern crate libc;

mod cell;
mod color;
mod ffi;
mod screen;
mod types;

pub use screen::Screen;
pub use cell::Cell;
pub use color::Color;
