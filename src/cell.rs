#[derive(Clone, Debug)]
pub struct Cell {
    contents: String,
    fgcolor: crate::color::Color,
    bgcolor: crate::color::Color,
    bold: bool,
    italic: bool,
    inverse: bool,
    underline: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn fgcolor(&self) -> crate::color::Color {
        self.fgcolor
    }

    pub fn bgcolor(&self) -> crate::color::Color {
        self.bgcolor
    }

    pub fn bold(&self) -> bool {
        self.bold
    }

    pub fn italic(&self) -> bool {
        self.italic
    }

    pub fn inverse(&self) -> bool {
        self.inverse
    }

    pub fn underline(&self) -> bool {
        self.underline
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            contents: String::new(),
            fgcolor: crate::color::Color::Default,
            bgcolor: crate::color::Color::Default,
            bold: false,
            italic: false,
            inverse: false,
            underline: false,
        }
    }
}
