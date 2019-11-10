/// A parser for terminal output which produces an in-memory representation of
/// the terminal contents.
pub struct Parser {
    parser: vte::Parser,
    screen: crate::screen::Screen,
}

impl Parser {
    /// Creates a new terminal parser of the given size and with the given
    /// amount of scrollback.
    pub fn new(rows: u16, cols: u16, scrollback_len: usize) -> Self {
        Self {
            parser: vte::Parser::new(),
            screen: crate::screen::Screen::new(
                crate::grid::Size { rows, cols },
                scrollback_len,
            ),
        }
    }

    /// Processes the contents of the given byte string, and updates the
    /// in-memory terminal state.
    pub fn process(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.parser.advance(&mut self.screen, *byte);
        }
    }

    /// Returns a reference to a `Screen` object containing the terminal
    /// state.
    pub fn screen(&self) -> &crate::screen::Screen {
        &self.screen
    }

    /// Returns a mutable reference to a `Screen` object containing the
    /// terminal state.
    pub fn screen_mut(&mut self) -> &mut crate::screen::Screen {
        &mut self.screen
    }
}

impl Default for Parser {
    /// Returns a parser with dimensions 80x24 and no scrollback.
    fn default() -> Self {
        Self::new(24, 80, 0)
    }
}
