use unicode_normalization::UnicodeNormalization as _;

/// Represents a single terminal cell.
#[derive(Clone, Debug, Default)]
pub struct Cell {
    contents: String,
    attrs: crate::attrs::Attrs,
}

impl Cell {
    /// Creates a new cell.
    pub fn new() -> Self {
        Self::default()
    }

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

    pub(crate) fn clear(&mut self) {
        self.contents.clear();
        self.attrs.clear();
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
        crate::unicode::str_width(&self.contents) > 1
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
