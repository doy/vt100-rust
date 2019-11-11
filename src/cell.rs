use unicode_normalization::UnicodeNormalization as _;
use unicode_width::UnicodeWidthChar as _;

const CODEPOINTS_IN_CELL: usize = 6;

/// Represents a single terminal cell.
#[derive(Clone, Debug, Default, Eq)]
pub struct Cell {
    contents: [char; CODEPOINTS_IN_CELL],
    len: u8,
    attrs: crate::attrs::Attrs,
}

#[allow(clippy::collapsible_if)]
impl PartialEq<Cell> for Cell {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        if self.attrs != other.attrs {
            return false;
        }
        let len = self.len();
        self.contents[..len] == other.contents[..len]
    }
}

impl Cell {
    #[inline]
    fn len(&self) -> usize {
        (self.len & 0x0f) as usize
    }

    pub(crate) fn set(&mut self, c: char, a: crate::attrs::Attrs) {
        self.contents[0] = c;
        self.len = 1;
        // strings in this context should always be an arbitrary character
        // followed by zero or more zero-width characters, so we should only
        // have to look at the first character
        self.set_wide(c.width().unwrap_or(0) > 1);
        self.attrs = a;
    }

    pub(crate) fn append(&mut self, c: char) {
        if self.len() >= CODEPOINTS_IN_CELL {
            return;
        }

        self.contents[self.len()] = c;
        self.len += 1;

        // some fonts have combined characters but can't render combining
        // characters correctly, so try to prefer precombined characters when
        // possible
        if unicode_normalization::is_nfc_quick(
            self.contents.iter().copied().take(CODEPOINTS_IN_CELL),
        ) == unicode_normalization::IsNormalized::Yes
        {
            return;
        }

        let mut new_contents = ['\x00'; CODEPOINTS_IN_CELL];
        let mut new_len = 0;
        for c in self
            .contents
            .iter()
            .copied()
            .take(self.len())
            .nfc()
            .take(CODEPOINTS_IN_CELL)
        {
            new_contents[new_len as usize] = c;
            new_len += 1;
        }
        self.contents = new_contents;
        self.len = new_len;
        self.set_wide(new_contents[0].width().unwrap_or(0) > 1);
    }

    pub(crate) fn clear(&mut self, attrs: crate::attrs::Attrs) {
        self.len = 0;
        self.attrs = attrs;
    }

    /// Returns the text contents of the cell.
    ///
    /// Can include multiple unicode characters if combining characters are
    /// used, but will contain at most one character with a non-zero character
    /// width.
    pub fn contents(&self) -> String {
        self.contents.iter().take(self.len()).collect::<String>()
    }

    /// Returns whether the cell contains any text data.
    pub fn has_contents(&self) -> bool {
        self.len > 0
    }

    /// Returns whether the text data in the cell represents a wide character.
    pub fn is_wide(&self) -> bool {
        self.len & 0x80 == 0x80
    }

    fn set_wide(&mut self, wide: bool) {
        if wide {
            self.len |= 0x80;
        } else {
            self.len &= 0x7f;
        }
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
