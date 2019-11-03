#[derive(enumset::EnumSetType, Debug)]
pub enum TextMode {
    Bold,
    Italic,
    Underline,
    Inverse,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Attrs {
    pub fgcolor: crate::color::Color,
    pub bgcolor: crate::color::Color,
    pub mode: enumset::EnumSet<TextMode>,
}

impl Attrs {
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

    pub fn escape_code_diff(&self, other: &Self) -> String {
        let mut opts = vec![];

        if self.fgcolor != other.fgcolor {
            match self.fgcolor {
                crate::color::Color::Default => {
                    opts.push(39);
                }
                crate::color::Color::Idx(i) => {
                    if i < 8 {
                        opts.push(i + 30);
                    } else if i < 16 {
                        opts.push(i + 82);
                    } else {
                        opts.push(38);
                        opts.push(5);
                        opts.push(i);
                    }
                }
                crate::color::Color::Rgb(r, g, b) => {
                    opts.push(38);
                    opts.push(2);
                    opts.push(r);
                    opts.push(g);
                    opts.push(b);
                }
            }
        }

        if self.bgcolor != other.bgcolor {
            match self.bgcolor {
                crate::color::Color::Default => {
                    opts.push(49);
                }
                crate::color::Color::Idx(i) => {
                    if i < 8 {
                        opts.push(i + 40);
                    } else if i < 16 {
                        opts.push(i + 92);
                    } else {
                        opts.push(48);
                        opts.push(5);
                        opts.push(i);
                    }
                }
                crate::color::Color::Rgb(r, g, b) => {
                    opts.push(48);
                    opts.push(2);
                    opts.push(r);
                    opts.push(g);
                    opts.push(b);
                }
            }
        }

        if self.bold() != other.bold() {
            opts.push(if self.bold() { 1 } else { 21 });
        }
        if self.italic() != other.italic() {
            opts.push(if self.italic() { 3 } else { 23 });
        }
        if self.underline() != other.underline() {
            opts.push(if self.underline() { 4 } else { 24 });
        }
        if self.inverse() != other.inverse() {
            opts.push(if self.inverse() { 7 } else { 27 });
        }

        let strs: Vec<_> =
            opts.iter().map(std::string::ToString::to_string).collect();
        format!("\x1b[{}m", strs.join(";"))
    }
}
