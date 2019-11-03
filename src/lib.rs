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
pub use screen::{MouseProtocolMode, MouseProtocolEncoding, Screen};
