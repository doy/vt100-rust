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

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn has_contents(&self) -> bool {
        self.contents != ""
    }

    pub fn fgcolor(&self) -> crate::color::Color {
        self.attrs.fgcolor
    }

    pub fn bgcolor(&self) -> crate::color::Color {
        self.attrs.bgcolor
    }

    pub fn bold(&self) -> bool {
        self.attrs.bold
    }

    pub fn italic(&self) -> bool {
        self.attrs.italic
    }

    pub fn inverse(&self) -> bool {
        self.attrs.inverse
    }

    pub fn underline(&self) -> bool {
        self.attrs.underline
    }
}
