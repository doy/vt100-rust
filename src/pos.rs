#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pos {
    pub row: u16,
    pub col: u16,
}

impl Default for Pos {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
}
