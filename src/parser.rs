/// A parser for terminal output which produces an in-memory representation of
/// the terminal contents.
pub struct Parser<CB: crate::Callbacks = ()> {
    parser: vte::Parser,
    screen: crate::screen::WrappedScreen<CB>,
}

impl Parser {
    /// The default number of rows used when creating a parser with
    /// [`Parser::default`].
    ///
    /// The value of this constant is 24.
    // `unwrap` is not `const` in this crate's MSRV.
    pub const DEFAULT_ROWS: core::num::NonZeroU16 =
        match core::num::NonZeroU16::new(24) {
            Some(n) => n,
            None => unreachable!(),
        };

    /// The default number of columns used when creating a parser with
    /// [`Parser::default`].
    ///
    /// The value of this constant is 80.
    pub const DEFAULT_COLS: core::num::NonZeroU16 =
        match core::num::NonZeroU16::new(80) {
            Some(n) => n,
            None => unreachable!(),
        };

    /// Creates a new terminal parser of the given size and with the given
    /// amount of scrollback.
    #[must_use]
    pub fn new(
        rows: core::num::NonZeroU16,
        cols: core::num::NonZeroU16,
        scrollback_len: usize,
    ) -> Self {
        Self::new_with_callbacks(rows, cols, scrollback_len, ())
    }
}

impl<CB: crate::Callbacks> Parser<CB> {
    /// Creates a new terminal parser of the given size and with the given
    /// amount of scrollback. Terminal events will be reported via method
    /// calls on the provided [`Callbacks`](crate::Callbacks) implementation.
    pub fn new_with_callbacks(
        rows: core::num::NonZeroU16,
        cols: core::num::NonZeroU16,
        scrollback_len: usize,
        callbacks: CB,
    ) -> Self {
        let screen = crate::screen::Screen::new(
            crate::grid::Size { rows, cols },
            scrollback_len,
        );
        Self {
            parser: vte::Parser::new(),
            screen: crate::screen::WrappedScreen { screen, callbacks },
        }
    }

    /// Processes the contents of the given byte string, and updates the
    /// in-memory terminal state.
    pub fn process(&mut self, bytes: &[u8]) {
        self.parser.advance(&mut self.screen, bytes);
    }

    /// Resizes the terminal.
    ///
    /// When the screen height is decreased, this function does the following:
    ///
    /// * If the cursor is not at the bottom of the screen, rows from the
    ///   bottom are discarded until the cursor is at the bottom (or until the
    ///   desired height is reached, whichever is earlier).
    /// * If the screen is still too tall after that, rows from the top of the
    ///   screen are pushed into scrollback. If this would cause scrollback to
    ///   exceed the maximum size, rows from the top of scrollback are
    ///   discarded, and the [`on_scroll`] callback is called for each of them.
    ///
    /// When the screen height is increased, this function does the following:
    ///
    /// * Rows from the bottom of scrollback are pushed to the top of the
    ///   screen and removed from scrollback, until the desired height is
    ///   reached.
    /// * If scrollback is emptied but the screen is still too short, blank
    ///   rows are added to the bottom of the screen until the desired height
    ///   is reached.
    ///
    /// When the screen width is changed, each row is naively truncated or
    /// extended with blank cells as necessary. This does not match the
    /// behavior of most terminals and may be changed in a future
    /// SemVer-incompatible version.
    ///
    /// [`on_scroll`]: crate::Callbacks::on_scroll
    pub fn set_size(
        &mut self,
        rows: core::num::NonZeroU16,
        cols: core::num::NonZeroU16,
    ) {
        self.screen.set_size_with_cb(rows, cols);
    }

    /// Returns a reference to a [`Screen`](crate::Screen) object containing
    /// the terminal state.
    #[must_use]
    pub fn screen(&self) -> &crate::Screen {
        &self.screen.screen
    }

    /// Returns a mutable reference to a [`Screen`](crate::Screen) object
    /// containing the terminal state.
    #[must_use]
    pub fn screen_mut(&mut self) -> &mut crate::Screen {
        &mut self.screen.screen
    }

    /// Returns a reference to the [`Callbacks`](crate::Callbacks) state object
    /// passed into the constructor.
    pub fn callbacks(&self) -> &CB {
        &self.screen.callbacks
    }

    /// Returns a mutable reference to the [`Callbacks`](crate::Callbacks)
    /// state object passed into the constructor.
    pub fn callbacks_mut(&mut self) -> &mut CB {
        &mut self.screen.callbacks
    }

    /// Returns mutable references to both the [screen](Self::screen_mut) and
    /// [callbacks](Self::callbacks_mut).
    pub fn screen_and_callbacks_mut(
        &mut self,
    ) -> (&mut crate::Screen, &mut CB) {
        (&mut self.screen.screen, &mut self.screen.callbacks)
    }
}

impl Default for Parser {
    /// Returns a parser with dimensions 80x24 and no scrollback.
    fn default() -> Self {
        Self::new(Self::DEFAULT_ROWS, Self::DEFAULT_COLS, 0)
    }
}

#[cfg(feature = "std")]
impl std::io::Write for Parser {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.process(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "embedded-io")]
impl embedded_io::ErrorType for Parser {
    type Error = core::convert::Infallible;
}

#[cfg(feature = "embedded-io")]
impl embedded_io::Write for Parser {
    fn write(
        &mut self,
        buf: &[u8],
    ) -> core::result::Result<usize, Self::Error> {
        self.process(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> core::result::Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(feature = "embedded-io")]
impl embedded_io::WriteReady for Parser {
    fn write_ready(&mut self) -> core::result::Result<bool, Self::Error> {
        Ok(true)
    }
}
