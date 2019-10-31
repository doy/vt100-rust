#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::single_match)]

mod attrs;
mod cell;
mod color;
mod grid;
mod row;
mod screen;
mod unicode;

pub use cell::Cell;
pub use color::Color;
pub use screen::Screen;
