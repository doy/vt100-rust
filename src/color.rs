use std;

use types;

#[derive(Eq, PartialEq, Debug)]
pub enum Color {
    ColorDefault,
    ColorIdx(u8),
    ColorRgb(u8, u8, u8),
}

impl Color {
    pub fn new(color_impl: &types::ColorImpl) -> Color {
        let &types::ColorImpl(color_repr) = color_impl;
        let bytes: [u8; 4] = unsafe { std::mem::transmute(color_repr) };
        match bytes[3] {
            0 => Color::ColorDefault,
            1 => Color::ColorIdx(bytes[0]),
            2 => Color::ColorRgb(bytes[0], bytes[1], bytes[2]),
            _ => panic!("invalid color type"),
        }
    }
}
