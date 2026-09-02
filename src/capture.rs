//! Types related to in-progress capturing of terminal data.
//!
//! See [`crate::Callbacks::on_scroll`].

#[cfg(doc)]
use crate::Screen;

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
    /// format as [`Screen::contents_formatted_basic`].
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

/// Iterator returned by [`basic_formatted_to_plain`].
pub struct BasicFormattedToPlain<'a> {
    capture: &'a str,
}

impl<'a> Iterator for BasicFormattedToPlain<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.capture.is_empty() {
            let Some((before, after)) = self.capture.split_once('\x1b')
            else {
                return Some(core::mem::take(&mut self.capture));
            };
            self.capture = match after.split_once('m') {
                Some((_esc_body, after_esc)) => after_esc,
                None => "",
            };
            if !before.is_empty() {
                return Some(before);
            }
        }
        None
    }
}

/// Converts a "basic formatted" capture into a plain text one.
///
/// This function splits a "basic formatted" screen capture, as returned by
/// [`Screen::contents_formatted_basic`], into a plain text one, by stripping
/// out the SGR escape sequences.
///
/// This function returns an iterator that yields pieces of the plain text
/// plain text capture, each containing no SGR escape sequences. To obtain the
/// plain text capture as a single string, use [`Iterator::collect::<String>`].
///
/// Every part of `capture` must have come from
/// [`Screen::contents_formatted_basic`] or
/// [`RowContents::write_formatted_basic`] (from the [`on_scroll`] callback).
/// If this requirement is not upheld, the results of this function are
/// undefined, other than that it will not panic.
///
/// [`on_scroll`]: crate::Callbacks::on_scroll
#[must_use]
pub fn basic_formatted_to_plain(capture: &str) -> BasicFormattedToPlain<'_> {
    BasicFormattedToPlain { capture }
}

/// Iterator returned by [`basic_formatted_rows`].
pub struct BasicFormattedRows<'a> {
    capture: Option<&'a str>,
    width: u16,
}

impl<'a> Iterator for BasicFormattedRows<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        use unicode_width::UnicodeWidthChar as _;

        let capture = self.capture?;
        let mut width = 0_u16;
        let mut chars = capture.char_indices();

        while let Some((i, c)) = chars.next() {
            if c == '\x1b' {
                while chars.next().is_some_and(|(_, c)| c != 'm') {}
                continue;
            }
            if c == '\n' {
                let (row, rest) = capture.split_at(i);
                self.capture = Some(&rest[c.len_utf8()..]);
                return Some(row);
            }

            // This crate renders every character that has a width greater than
            // 1 in exactly 2 columns (see `Screen::prepare_text`), so clamp
            // the width to 2 to be consistent with how we render.
            let char_width = c.width().map_or(1, |w| {
                u16::try_from(w).unwrap_or(u16::MAX).clamp(0, 2)
            });

            // Always append zero-width characters, even if we're over the
            // width limit due to the case described below where a single
            // character exceeds the limit.
            if char_width == 0 {
                continue;
            }

            let old_width = width;
            if let Some(new_width) = width.checked_add(char_width) {
                width = new_width;
                if new_width <= self.width {
                    // We haven't reached the screen width yet; continue adding
                    // characters. Use `<=` instead of `<` because we may be
                    // able to add more zero-width characters even if we've hit
                    // the limit exactly.
                    continue;
                }
            }
            if old_width == 0 {
                // Appending `c` would cause the row width to exceed the screen
                // width, but the row is currently empty. This case is unlikely
                // -- a one-column-wide screen will discard wide characters
                // written to it. However, we could reach this branch if the
                // terminal was resized after we had already captured
                // scrollback.
                //
                // Yielding an empty row would cause this iterator never to
                // terminate, so `continue` here so that we yield the single
                // char, even though its width exceeds the screen width.
                continue;
            }

            let (before, after) = capture.split_at(i);
            self.capture = Some(after);
            return Some(before);
        }
        // The remaining data in the capture fits in a single row.
        self.capture.take()
    }
}

/// Splits a "basic formatted" capture into individual rows.
///
/// This function splits a "basic formatted" screen capture, as returned by
/// [`Screen::contents_formatted_basic`] into individual rows. It returns an
/// iterator that yields each row in order. The rows will not end with
/// newlines. SGR parameters will not be reset at the start/end of each row;
/// the iterator simply splits `capture` into rows as-is, except for the
/// stripping out of newlines.
///
/// `screen_width` is the width of the terminal in columns.
///
/// Every part of `capture` must have come from
/// [`Screen::contents_formatted_basic`] or
/// [`RowContents::write_formatted_basic`] (from the [`on_scroll`] callback).
/// If this requirement is not upheld, the results of this function are
/// undefined, other than that it will not panic.
///
/// [`on_scroll`]: crate::Callbacks::on_scroll
#[must_use]
pub fn basic_formatted_rows(
    capture: &str,
    screen_width: u16,
) -> BasicFormattedRows<'_> {
    BasicFormattedRows {
        capture: Some(capture),
        width: screen_width,
    }
}
