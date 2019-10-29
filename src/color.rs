#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Color {
    Default,
    Idx(u8),
    Rgb(u8, u8, u8),
}

impl Default for Color {
    fn default() -> Self {
        Self::Default
    }
}
