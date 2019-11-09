use std::io::Write as _;

/// Represents a foreground or background color for cells.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Color {
    /// The default terminal color.
    Default,

    /// An indexed terminal color.
    Idx(u8),

    /// An RGB terminal color. The parameters are (red, green, blue).
    Rgb(u8, u8, u8),
}

impl Default for Color {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(enumset::EnumSetType, Debug)]
pub enum TextMode {
    Bold,
    Italic,
    Underline,
    Inverse,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Attrs {
    pub fgcolor: Color,
    pub bgcolor: Color,
    pub mode: enumset::EnumSet<TextMode>,
}

impl Attrs {
    pub fn clear(&mut self) {
        self.fgcolor = Color::default();
        self.bgcolor = Color::default();
        self.mode = enumset::EnumSet::default();
    }

    pub fn bold(&self) -> bool {
        self.mode.contains(TextMode::Bold)
    }

    pub fn set_bold(&mut self, bold: bool) {
        if bold {
            self.mode.insert(TextMode::Bold);
        } else {
            self.mode.remove(TextMode::Bold);
        }
    }

    pub fn italic(&self) -> bool {
        self.mode.contains(TextMode::Italic)
    }

    pub fn set_italic(&mut self, italic: bool) {
        if italic {
            self.mode.insert(TextMode::Italic);
        } else {
            self.mode.remove(TextMode::Italic);
        }
    }

    pub fn underline(&self) -> bool {
        self.mode.contains(TextMode::Underline)
    }

    pub fn set_underline(&mut self, underline: bool) {
        if underline {
            self.mode.insert(TextMode::Underline);
        } else {
            self.mode.remove(TextMode::Underline);
        }
    }

    pub fn inverse(&self) -> bool {
        self.mode.contains(TextMode::Inverse)
    }

    pub fn set_inverse(&mut self, inverse: bool) {
        if inverse {
            self.mode.insert(TextMode::Inverse);
        } else {
            self.mode.remove(TextMode::Inverse);
        }
    }

    pub fn write_escape_code_diff(
        &self,
        contents: &mut Vec<u8>,
        other: &Self,
    ) {
        let attrs = crate::term::Attrs::default();

        if self != other && self == &Self::default() {
            write!(contents, "{}", attrs).unwrap();
            return;
        }

        let attrs = if self.fgcolor == other.fgcolor {
            attrs
        } else {
            attrs.fgcolor(self.fgcolor)
        };
        let attrs = if self.bgcolor == other.bgcolor {
            attrs
        } else {
            attrs.bgcolor(self.bgcolor)
        };
        let attrs = if self.bold() == other.bold() {
            attrs
        } else {
            attrs.bold(self.bold())
        };
        let attrs = if self.italic() == other.italic() {
            attrs
        } else {
            attrs.italic(self.italic())
        };
        let attrs = if self.underline() == other.underline() {
            attrs
        } else {
            attrs.underline(self.underline())
        };
        let attrs = if self.inverse() == other.inverse() {
            attrs
        } else {
            attrs.inverse(self.inverse())
        };

        write!(contents, "{}", attrs).unwrap();
    }
}
