#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

mod cell;
pub use cell::Cell;
mod color;
pub use color::Color;
mod parser;
mod pos;
mod screen;
pub use screen::Screen;
