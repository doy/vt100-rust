// TODO: read all of this from terminfo

pub trait BufWrite {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result;

    fn write_string(&self, buf: &mut String) {
        self.write_fmt(buf)
            .expect("writing to a String cannot fail");
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct ClearScreen;

impl BufWrite for ClearScreen {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        writer.write_str("\x1b[H\x1b[J")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct ClearRowForward;

impl BufWrite for ClearRowForward {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        writer.write_str("\x1b[K")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct Crlf;

impl BufWrite for Crlf {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        writer.write_str("\r\n")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct Backspace;

impl BufWrite for Backspace {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        writer.write_str("\x08")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct SaveCursor;

impl BufWrite for SaveCursor {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        writer.write_str("\x1b7")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct RestoreCursor;

impl BufWrite for RestoreCursor {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        writer.write_str("\x1b8")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct MoveTo {
    row: u16,
    col: u16,
}

impl MoveTo {
    pub fn new(pos: crate::grid::Pos) -> Self {
        Self {
            row: pos.row,
            col: pos.col,
        }
    }
}

impl BufWrite for MoveTo {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        if self.row == 0 && self.col == 0 {
            writer.write_str("\x1b[H")
        } else {
            writer.write_str("\x1b[")?;
            write_itoa(writer, self.row + 1)?;
            writer.write_char(';')?;
            write_itoa(writer, self.col + 1)?;
            writer.write_char('H')
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct ClearAttrs;

impl BufWrite for ClearAttrs {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        writer.write_str("\x1b[m")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Intensity {
    Normal,
    Bold,
    Dim,
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct Attrs {
    fgcolor: Option<crate::Color>,
    bgcolor: Option<crate::Color>,
    intensity: Option<Intensity>,
    italic: Option<bool>,
    underline: Option<bool>,
    inverse: Option<bool>,
}

impl Attrs {
    pub fn fgcolor(mut self, fgcolor: crate::Color) -> Self {
        self.fgcolor = Some(fgcolor);
        self
    }

    pub fn bgcolor(mut self, bgcolor: crate::Color) -> Self {
        self.bgcolor = Some(bgcolor);
        self
    }

    pub fn intensity(mut self, intensity: Intensity) -> Self {
        self.intensity = Some(intensity);
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = Some(underline);
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = Some(inverse);
        self
    }
}

impl BufWrite for Attrs {
    #[allow(unused_assignments)]
    #[allow(clippy::branches_sharing_code)]
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        if self.fgcolor.is_none()
            && self.bgcolor.is_none()
            && self.intensity.is_none()
            && self.italic.is_none()
            && self.underline.is_none()
            && self.inverse.is_none()
        {
            return Ok(());
        }

        writer.write_str("\x1b[")?;
        let mut first = true;

        macro_rules! write_param {
            ($i:expr) => {{
                if first {
                    first = false;
                } else {
                    writer.write_char(';')?;
                }
                write_itoa(writer, $i)?;
            }};
        }

        if let Some(fgcolor) = self.fgcolor {
            match fgcolor {
                crate::Color::Default => {
                    write_param!(39);
                }
                crate::Color::Idx(i) => {
                    if i < 8 {
                        write_param!(i + 30);
                    } else if i < 16 {
                        write_param!(i + 82);
                    } else {
                        write_param!(38);
                        write_param!(5);
                        write_param!(i);
                    }
                }
                crate::Color::Rgb(r, g, b) => {
                    write_param!(38);
                    write_param!(2);
                    write_param!(r);
                    write_param!(g);
                    write_param!(b);
                }
            }
        }

        if let Some(bgcolor) = self.bgcolor {
            match bgcolor {
                crate::Color::Default => {
                    write_param!(49);
                }
                crate::Color::Idx(i) => {
                    if i < 8 {
                        write_param!(i + 40);
                    } else if i < 16 {
                        write_param!(i + 92);
                    } else {
                        write_param!(48);
                        write_param!(5);
                        write_param!(i);
                    }
                }
                crate::Color::Rgb(r, g, b) => {
                    write_param!(48);
                    write_param!(2);
                    write_param!(r);
                    write_param!(g);
                    write_param!(b);
                }
            }
        }

        if let Some(intensity) = self.intensity {
            match intensity {
                Intensity::Normal => write_param!(22),
                Intensity::Bold => write_param!(1),
                Intensity::Dim => write_param!(2),
            }
        }

        if let Some(italic) = self.italic {
            if italic {
                write_param!(3);
            } else {
                write_param!(23);
            }
        }

        if let Some(underline) = self.underline {
            if underline {
                write_param!(4);
            } else {
                write_param!(24);
            }
        }

        if let Some(inverse) = self.inverse {
            if inverse {
                write_param!(7);
            } else {
                write_param!(27);
            }
        }

        writer.write_char('m')
    }
}

#[derive(Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct MoveRight {
    count: u16,
}

impl MoveRight {
    pub fn new(count: u16) -> Self {
        Self { count }
    }
}

impl Default for MoveRight {
    fn default() -> Self {
        Self { count: 1 }
    }
}

impl BufWrite for MoveRight {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        match self.count {
            0 => Ok(()),
            1 => writer.write_str("\x1b[C"),
            n => {
                writer.write_str("\x1b[")?;
                write_itoa(writer, n)?;
                writer.write_char('C')
            }
        }
    }
}

#[derive(Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct EraseChar {
    count: u16,
}

impl EraseChar {
    pub fn new(count: u16) -> Self {
        Self { count }
    }
}

impl Default for EraseChar {
    fn default() -> Self {
        Self { count: 1 }
    }
}

impl BufWrite for EraseChar {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        match self.count {
            0 => Ok(()),
            1 => writer.write_str("\x1b[X"),
            n => {
                writer.write_str("\x1b[")?;
                write_itoa(writer, n)?;
                writer.write_char('X')
            }
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct HideCursor {
    state: bool,
}

impl HideCursor {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

impl BufWrite for HideCursor {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        if self.state {
            writer.write_str("\x1b[?25l")
        } else {
            writer.write_str("\x1b[?25h")
        }
    }
}

#[derive(Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct MoveFromTo {
    from: crate::grid::Pos,
    to: crate::grid::Pos,
}

impl MoveFromTo {
    pub fn new(from: crate::grid::Pos, to: crate::grid::Pos) -> Self {
        Self { from, to }
    }
}

impl BufWrite for MoveFromTo {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        if self.to.row == self.from.row + 1 && self.to.col == 0 {
            crate::term::Crlf.write_fmt(writer)
        } else if self.from.row == self.to.row && self.from.col < self.to.col
        {
            crate::term::MoveRight::new(self.to.col - self.from.col)
                .write_fmt(writer)
        } else if self.to != self.from {
            crate::term::MoveTo::new(self.to).write_fmt(writer)
        } else {
            Ok(())
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct ApplicationKeypad {
    state: bool,
}

impl ApplicationKeypad {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

impl BufWrite for ApplicationKeypad {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        if self.state {
            writer.write_str("\x1b=")
        } else {
            writer.write_str("\x1b>")
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct ApplicationCursor {
    state: bool,
}

impl ApplicationCursor {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

impl BufWrite for ApplicationCursor {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        if self.state {
            writer.write_str("\x1b[?1h")
        } else {
            writer.write_str("\x1b[?1l")
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct BracketedPaste {
    state: bool,
}

impl BracketedPaste {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

impl BufWrite for BracketedPaste {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        if self.state {
            writer.write_str("\x1b[?2004h")
        } else {
            writer.write_str("\x1b[?2004l")
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct MouseProtocolMode {
    mode: crate::MouseProtocolMode,
    prev: crate::MouseProtocolMode,
}

impl MouseProtocolMode {
    pub fn new(
        mode: crate::MouseProtocolMode,
        prev: crate::MouseProtocolMode,
    ) -> Self {
        Self { mode, prev }
    }
}

impl BufWrite for MouseProtocolMode {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        if self.mode == self.prev {
            return Ok(());
        }

        match self.mode {
            crate::MouseProtocolMode::None => match self.prev {
                crate::MouseProtocolMode::None => Ok(()),
                crate::MouseProtocolMode::Press => {
                    writer.write_str("\x1b[?9l")
                }
                crate::MouseProtocolMode::PressRelease => {
                    writer.write_str("\x1b[?1000l")
                }
                crate::MouseProtocolMode::ButtonMotion => {
                    writer.write_str("\x1b[?1002l")
                }
                crate::MouseProtocolMode::AnyMotion => {
                    writer.write_str("\x1b[?1003l")
                }
            },
            crate::MouseProtocolMode::Press => writer.write_str("\x1b[?9h"),
            crate::MouseProtocolMode::PressRelease => {
                writer.write_str("\x1b[?1000h")
            }
            crate::MouseProtocolMode::ButtonMotion => {
                writer.write_str("\x1b[?1002h")
            }
            crate::MouseProtocolMode::AnyMotion => {
                writer.write_str("\x1b[?1003h")
            }
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_string"]
pub struct MouseProtocolEncoding {
    encoding: crate::MouseProtocolEncoding,
    prev: crate::MouseProtocolEncoding,
}

impl MouseProtocolEncoding {
    pub fn new(
        encoding: crate::MouseProtocolEncoding,
        prev: crate::MouseProtocolEncoding,
    ) -> Self {
        Self { encoding, prev }
    }
}

impl BufWrite for MouseProtocolEncoding {
    fn write_fmt(
        &self,
        writer: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        if self.encoding == self.prev {
            return Ok(());
        }

        match self.encoding {
            crate::MouseProtocolEncoding::Default => match self.prev {
                crate::MouseProtocolEncoding::Default => Ok(()),
                crate::MouseProtocolEncoding::Utf8 => {
                    writer.write_str("\x1b[?1005l")
                }
                crate::MouseProtocolEncoding::Sgr => {
                    writer.write_str("\x1b[?1006l")
                }
            },
            crate::MouseProtocolEncoding::Utf8 => {
                writer.write_str("\x1b[?1005h")
            }
            crate::MouseProtocolEncoding::Sgr => {
                writer.write_str("\x1b[?1006h")
            }
        }
    }
}

fn write_itoa<I: itoa::Integer>(
    writer: &mut impl std::fmt::Write,
    i: I,
) -> std::fmt::Result {
    let mut itoa_buf = itoa::Buffer::new();
    writer.write_str(itoa_buf.format(i))
}
