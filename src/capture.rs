//! Types related to in-progress capturing of terminal data.
//!
//! See [`crate::Callbacks::on_scroll`].

/// Represents the state of an in-progress capture of "basic formatted"
/// terminal data.
///
/// This type is needed if you're capturing terminal data in a streaming
/// fashion using [`RowContents::write_formatted_basic`] in the [`on_scroll`]
/// callback.
///
/// [`on_scroll`]: crate::Callbacks::on_scroll
#[derive(Debug, Default, Clone)]
pub struct BasicFormattedCaptureState {
    attrs: crate::attrs::Attrs,
    newline_pending: bool,
}

impl BasicFormattedCaptureState {
    /// Creates a new [`BasicFormattedCaptureState`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Represents the contents of a row in the terminal.
pub struct RowContents<'a>(pub(crate) &'a crate::row::Row);

impl RowContents<'_> {
    /// Writes the contents of the row to the provided buffer, in the same
    /// format as [`crate::Screen::contents_formatted_basic`].
    ///
    /// # Errors
    ///
    /// If the writer returns an error, this method will forward that error.
    /// Otherwise, this method will not return any errors of its own.
    pub fn write_formatted_basic(
        &self,
        writer: &mut impl alloc::fmt::Write,
        state: &mut BasicFormattedCaptureState,
    ) -> alloc::fmt::Result {
        if state.newline_pending {
            writer.write_char('\n')?;
        }
        self.0
            .write_contents_formatted_basic(writer, &mut state.attrs)?;
        state.newline_pending = !self.0.wrapped();
        Ok(())
    }
}
