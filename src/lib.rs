#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

mod attrs;
mod cell;
mod color;
mod pos;
mod screen;

pub use cell::Cell;
pub use color::Color;
pub use screen::Screen;
