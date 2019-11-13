// TODO: read all of this from terminfo

pub trait WriteTo<W: std::io::Write> {
    fn write_to(&self, w: &mut W) -> std::io::Result<()>;
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_to"]
pub struct ClearScreen;

impl<W: std::io::Write> WriteTo<W> for ClearScreen {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        w.write_all(b"\x1b[H\x1b[J")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct ClearRowForward;

impl<W: std::io::Write> WriteTo<W> for ClearRowForward {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        w.write_all(b"\x1b[K")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct CRLF;

impl<W: std::io::Write> WriteTo<W> for CRLF {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        w.write_all(b"\r\n")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
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

impl<W: std::io::Write> WriteTo<W> for MoveTo {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        if self.row == 0 && self.col == 0 {
            w.write_all(b"\x1b[H")?;
        } else {
            w.write_all(b"\x1b[")?;
            itoa::write(&mut *w, self.row + 1)?;
            w.write_all(b";")?;
            itoa::write(&mut *w, self.col + 1)?;
            w.write_all(b"H")?;
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct ClearAttrs;

impl<W: std::io::Write> WriteTo<W> for ClearAttrs {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        w.write_all(b"\x1b[m")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct Attrs {
    fgcolor: Option<crate::attrs::Color>,
    bgcolor: Option<crate::attrs::Color>,
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
    inverse: Option<bool>,
}

impl Attrs {
    pub fn fgcolor(mut self, fgcolor: crate::attrs::Color) -> Self {
        self.fgcolor = Some(fgcolor);
        self
    }

    pub fn bgcolor(mut self, bgcolor: crate::attrs::Color) -> Self {
        self.bgcolor = Some(bgcolor);
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
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

impl<W: std::io::Write> WriteTo<W> for Attrs {
    #[allow(unused_assignments)]
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        if self.fgcolor.is_none()
            && self.bgcolor.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underline.is_none()
            && self.inverse.is_none()
        {
            return Ok(());
        }

        w.write_all(b"\x1b[")?;
        let mut first = true;

        macro_rules! write_param {
            ($i:expr) => {
                if first {
                    first = false;
                } else {
                    w.write_all(b";")?;
                }
                itoa::write(&mut *w, $i).unwrap();
            };
        }

        if let Some(fgcolor) = self.fgcolor {
            match fgcolor {
                crate::attrs::Color::Default => {
                    write_param!(39);
                }
                crate::attrs::Color::Idx(i) => {
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
                crate::attrs::Color::Rgb(r, g, b) => {
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
                crate::attrs::Color::Default => {
                    write_param!(49);
                }
                crate::attrs::Color::Idx(i) => {
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
                crate::attrs::Color::Rgb(r, g, b) => {
                    write_param!(48);
                    write_param!(2);
                    write_param!(r);
                    write_param!(g);
                    write_param!(b);
                }
            }
        }

        if let Some(bold) = self.bold {
            if bold {
                write_param!(1);
            } else {
                write_param!(22);
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

        w.write_all(b"m")
    }
}

#[derive(Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
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

impl<W: std::io::Write> WriteTo<W> for MoveRight {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        match self.count {
            0 => {}
            1 => w.write_all(b"\x1b[C")?,
            n => {
                w.write_all(b"\x1b[")?;
                itoa::write(&mut *w, n)?;
                w.write_all(b"C")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
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

impl<W: std::io::Write> WriteTo<W> for EraseChar {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        match self.count {
            0 => {}
            1 => w.write_all(b"\x1b[X")?,
            n => {
                w.write_all(b"\x1b[")?;
                itoa::write(&mut *w, n)?;
                w.write_all(b"X")?;
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct HideCursor {
    state: bool,
}

impl HideCursor {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

impl<W: std::io::Write> WriteTo<W> for HideCursor {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        if self.state {
            w.write_all(b"\x1b[?25l")
        } else {
            w.write_all(b"\x1b[?25h")
        }
    }
}

#[derive(Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct MoveFromTo {
    from: crate::grid::Pos,
    to: crate::grid::Pos,
}

impl MoveFromTo {
    pub fn new(from: crate::grid::Pos, to: crate::grid::Pos) -> Self {
        Self { from, to }
    }
}

impl<W: std::io::Write> WriteTo<W> for MoveFromTo {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        if self.to == self.from {
            Ok(())
        } else if self.to.row == self.from.row + 1 && self.to.col == 0 {
            crate::term::CRLF::default().write_to(w)
        } else if self.to.row == self.from.row && self.to.col > self.from.col
        {
            crate::term::MoveRight::new(self.to.col - self.from.col)
                .write_to(w)
        } else {
            crate::term::MoveTo::new(self.to).write_to(w)
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct AudibleBell;

impl<W: std::io::Write> WriteTo<W> for AudibleBell {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        w.write_all(b"\x07")
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct VisualBell;

impl<W: std::io::Write> WriteTo<W> for VisualBell {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        w.write_all(b"\x1bg")
    }
}

#[must_use = "this struct does nothing unless you call write_buf"]
pub struct ChangeTitle<'a> {
    icon_name: &'a str,
    title: &'a str,
    prev_icon_name: &'a str,
    prev_title: &'a str,
}

impl<'a> ChangeTitle<'a> {
    pub fn new(
        icon_name: &'a str,
        title: &'a str,
        prev_icon_name: &'a str,
        prev_title: &'a str,
    ) -> Self {
        Self {
            icon_name,
            title,
            prev_icon_name,
            prev_title,
        }
    }
}

impl<'a, W: std::io::Write> WriteTo<W> for ChangeTitle<'a> {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        if self.icon_name == self.title
            && (self.icon_name != self.prev_icon_name
                || self.title != self.prev_title)
        {
            w.write_all(b"\x1b]0;")?;
            w.write_all(self.icon_name.as_bytes())?;
            w.write_all(b"\x07")?;
        } else {
            if self.icon_name != self.prev_icon_name {
                w.write_all(b"\x1b]1;")?;
                w.write_all(self.icon_name.as_bytes())?;
                w.write_all(b"\x07")?;
            }
            if self.title != self.prev_title {
                w.write_all(b"\x1b]2;")?;
                w.write_all(self.icon_name.as_bytes())?;
                w.write_all(b"\x07")?;
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct ApplicationKeypad {
    state: bool,
}

impl ApplicationKeypad {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

impl<W: std::io::Write> WriteTo<W> for ApplicationKeypad {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        if self.state {
            w.write_all(b"\x1b=")
        } else {
            w.write_all(b"\x1b>")
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct ApplicationCursor {
    state: bool,
}

impl ApplicationCursor {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

impl<W: std::io::Write> WriteTo<W> for ApplicationCursor {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        if self.state {
            w.write_all(b"\x1b[?1h")
        } else {
            w.write_all(b"\x1b[?1l")
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct BracketedPaste {
    state: bool,
}

impl BracketedPaste {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

impl<W: std::io::Write> WriteTo<W> for BracketedPaste {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        if self.state {
            w.write_all(b"\x1b[?2004h")
        } else {
            w.write_all(b"\x1b[?2004l")
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct MouseProtocolMode {
    mode: crate::screen::MouseProtocolMode,
    prev: crate::screen::MouseProtocolMode,
}

impl MouseProtocolMode {
    pub fn new(
        mode: crate::screen::MouseProtocolMode,
        prev: crate::screen::MouseProtocolMode,
    ) -> Self {
        Self { mode, prev }
    }
}

impl<W: std::io::Write> WriteTo<W> for MouseProtocolMode {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        if self.mode == self.prev {
            return Ok(());
        }

        match self.mode {
            crate::screen::MouseProtocolMode::None => match self.prev {
                crate::screen::MouseProtocolMode::None => Ok(()),
                crate::screen::MouseProtocolMode::Press => {
                    w.write_all(b"\x1b[?9l")
                }
                crate::screen::MouseProtocolMode::PressRelease => {
                    w.write_all(b"\x1b[?1000l")
                }
                crate::screen::MouseProtocolMode::ButtonMotion => {
                    w.write_all(b"\x1b[?1002l")
                }
                crate::screen::MouseProtocolMode::AnyMotion => {
                    w.write_all(b"\x1b[?1003l")
                }
            },
            crate::screen::MouseProtocolMode::Press => {
                w.write_all(b"\x1b[?9h")
            }
            crate::screen::MouseProtocolMode::PressRelease => {
                w.write_all(b"\x1b[?1000h")
            }
            crate::screen::MouseProtocolMode::ButtonMotion => {
                w.write_all(b"\x1b[?1002h")
            }
            crate::screen::MouseProtocolMode::AnyMotion => {
                w.write_all(b"\x1b[?1003h")
            }
        }
    }
}

#[derive(Default, Debug)]
#[must_use = "this struct does nothing unless you call write_buf"]
pub struct MouseProtocolEncoding {
    encoding: crate::screen::MouseProtocolEncoding,
    prev: crate::screen::MouseProtocolEncoding,
}

impl MouseProtocolEncoding {
    pub fn new(
        encoding: crate::screen::MouseProtocolEncoding,
        prev: crate::screen::MouseProtocolEncoding,
    ) -> Self {
        Self { encoding, prev }
    }
}

impl<W: std::io::Write> WriteTo<W> for MouseProtocolEncoding {
    fn write_to(&self, w: &mut W) -> std::io::Result<()> {
        if self.encoding == self.prev {
            return Ok(());
        }

        match self.encoding {
            crate::screen::MouseProtocolEncoding::Default => {
                match self.prev {
                    crate::screen::MouseProtocolEncoding::Default => Ok(()),
                    crate::screen::MouseProtocolEncoding::Utf8 => {
                        w.write_all(b"\x1b[?1005l")
                    }
                    crate::screen::MouseProtocolEncoding::Sgr => {
                        w.write_all(b"\x1b[?1006l")
                    }
                }
            }
            crate::screen::MouseProtocolEncoding::Utf8 => {
                w.write_all(b"\x1b[?1005h")
            }
            crate::screen::MouseProtocolEncoding::Sgr => {
                w.write_all(b"\x1b[?1006h")
            }
        }
    }
}
