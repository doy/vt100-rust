pub struct Parser {
    parser: vte::Parser,
    screen: crate::screen::Screen,
}

impl Parser {
    /// Creates a new terminal parser of the given size.
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            parser: vte::Parser::new(),
            screen: crate::screen::Screen::new(crate::grid::Size {
                rows,
                cols,
            }),
        }
    }

    /// Processes the contents of the given byte string, and updates the
    /// in-memory terminal state.
    pub fn process(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.parser.advance(&mut self.screen, *byte);
        }
    }

    pub fn screen(&self) -> &crate::screen::Screen {
        &self.screen
    }

    pub fn screen_mut(&mut self) -> &mut crate::screen::Screen {
        &mut self.screen
    }
}
