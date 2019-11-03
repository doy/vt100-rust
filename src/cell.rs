use unicode_normalization::UnicodeNormalization as _;

#[derive(Clone, Debug, Default)]
pub struct Cell {
    contents: String,
    attrs: crate::attrs::Attrs,
}

impl Cell {
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

    pub(crate) fn reset(&mut self) {
        self.contents = String::new();
        self.attrs = crate::attrs::Attrs::default();
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn has_contents(&self) -> bool {
        self.contents != ""
    }

    pub fn is_wide(&self) -> bool {
        crate::unicode::str_width(&self.contents) > 1
    }

    pub(crate) fn attrs(&self) -> &crate::attrs::Attrs {
        &self.attrs
    }

    pub fn fgcolor(&self) -> crate::color::Color {
        self.attrs.fgcolor
    }

    pub fn bgcolor(&self) -> crate::color::Color {
        self.attrs.bgcolor
    }

    pub fn bold(&self) -> bool {
        self.attrs.bold()
    }

    pub fn italic(&self) -> bool {
        self.attrs.italic()
    }

    pub fn inverse(&self) -> bool {
        self.attrs.inverse()
    }

    pub fn underline(&self) -> bool {
        self.attrs.underline()
    }
}
