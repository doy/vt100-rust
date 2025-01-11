/// A parser for terminal output which produces an in-memory representation of
/// the terminal contents.
pub struct Parser<Callbacks: crate::callbacks::Callbacks = ()> {
    parser: vte::Parser,
    screen: crate::perform::WrappedScreen<Callbacks>,
}

impl Parser {
    /// Creates a new terminal parser of the given size and with the given
    /// amount of scrollback.
    #[must_use]
    pub fn new(rows: u16, cols: u16, scrollback_len: usize) -> Self {
        Self {
            parser: vte::Parser::new(),
            screen: crate::perform::WrappedScreen::new(
                rows,
                cols,
                scrollback_len,
            ),
        }
    }
}

impl<Callbacks: crate::callbacks::Callbacks> Parser<Callbacks> {
    pub fn new_with_callbacks(
        rows: u16,
        cols: u16,
        scrollback_len: usize,
        callbacks: Callbacks,
    ) -> Self {
        Self {
            parser: vte::Parser::new(),
            screen: crate::perform::WrappedScreen::new_with_callbacks(
                rows,
                cols,
                scrollback_len,
                callbacks,
            ),
        }
    }

    /// Processes the contents of the given byte string, and updates the
    /// in-memory terminal state.
    pub fn process(&mut self, bytes: &[u8]) {
        self.parser.advance(&mut self.screen, bytes);
    }

    /// Returns a reference to a `Screen` object containing the terminal
    /// state.
    #[must_use]
    pub fn screen(&self) -> &crate::Screen {
        &self.screen.screen
    }

    /// Returns a mutable reference to a `Screen` object containing the
    /// terminal state.
    #[must_use]
    pub fn screen_mut(&mut self) -> &mut crate::Screen {
        &mut self.screen.screen
    }

    /// Returns a reference to the `Callbacks` state object passed into the
    /// constructor.
    pub fn callbacks(&self) -> &Callbacks {
        &self.screen.callbacks
    }

    /// Returns a mutable reference to the `Callbacks` state object passed
    /// into the constructor.
    pub fn callbacks_mut(&mut self) -> &mut Callbacks {
        &mut self.screen.callbacks
    }
}

impl Default for Parser {
    /// Returns a parser with dimensions 80x24 and no scrollback.
    fn default() -> Self {
        Self::new(24, 80, 0)
    }
}

impl std::io::Write for Parser {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.process(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
