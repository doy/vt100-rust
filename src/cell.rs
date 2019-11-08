use unicode_normalization::UnicodeNormalization as _;
use unicode_width::UnicodeWidthChar as _;

/// Represents a single terminal cell.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Cell {
    contents: String,
    attrs: crate::attrs::Attrs,
}

impl Cell {
    pub(crate) fn set(&mut self, c: String, a: crate::attrs::Attrs) {
        self.contents = c;
        self.attrs = a;
    }

    pub(crate) fn append(&mut self, c: char) {
        self.contents.push(c);
        // some fonts have combined characters but can't render combining
        // characters correctly, so try to prefer precombined characters when
        // possible
        if !unicode_normalization::is_nfc(&self.contents) {
            self.contents = self.contents.nfc().collect();
        }
    }

    pub(crate) fn clear(&mut self, bgcolor: crate::attrs::Color) {
        self.contents.clear();
        self.attrs.clear();
        self.attrs.bgcolor = bgcolor;
    }

    /// Returns the text contents of the cell.
    ///
    /// Can include multiple unicode characters if combining characters are
    /// used, but will contain at most one character with a non-zero character
    /// width.
    pub fn contents(&self) -> &str {
        &self.contents
    }

    /// Returns whether the cell contains any text data.
    pub fn has_contents(&self) -> bool {
        self.contents != ""
    }

    /// Returns whether the text data in the cell represents a wide character.
    pub fn is_wide(&self) -> bool {
        // strings in this context should always be an arbitrary character
        // followed by zero or more zero-width characters, so we should only
        // have to look at the first character
        let width = self
            .contents
            .chars()
            .next()
            .map_or(0, |c| c.width().unwrap_or(0));
        width > 1
    }

    pub(crate) fn attrs(&self) -> &crate::attrs::Attrs {
        &self.attrs
    }

    /// Returns the foreground color of the cell.
    pub fn fgcolor(&self) -> crate::attrs::Color {
        self.attrs.fgcolor
    }

    /// Returns the background color of the cell.
    pub fn bgcolor(&self) -> crate::attrs::Color {
        self.attrs.bgcolor
    }

    /// Returns whether the cell should be rendered with the bold text
    /// attribute.
    pub fn bold(&self) -> bool {
        self.attrs.bold()
    }

    /// Returns whether the cell should be rendered with the italic text
    /// attribute.
    pub fn italic(&self) -> bool {
        self.attrs.italic()
    }

    /// Returns whether the cell should be rendered with the underlined text
    /// attribute.
    pub fn underline(&self) -> bool {
        self.attrs.underline()
    }

    /// Returns whether the cell should be rendered with the inverse text
    /// attribute.
    pub fn inverse(&self) -> bool {
        self.attrs.inverse()
    }
}
