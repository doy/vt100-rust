use crate::term::BufWrite as _;
use std::convert::TryInto as _;
use unicode_width::UnicodeWidthChar as _;

const DEFAULT_MULTI_PARAMS: &[i64] = &[0];

#[derive(enumset::EnumSetType, Debug)]
enum Mode {
    ApplicationKeypad,
    ApplicationCursor,
    HideCursor,
    AlternateScreen,
    BracketedPaste,
}

/// The xterm mouse handling mode currently in use.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MouseProtocolMode {
    /// Mouse handling is disabled.
    None,

    /// Mouse button events should be reported on button press. Also known as
    /// X10 mouse mode.
    Press,

    /// Mouse button events should be reported on button press and release.
    /// Also known as VT200 mouse mode.
    PressRelease,

    // Highlight,
    /// Mouse button events should be reported on button press and release, as
    /// well as when the mouse moves between cells while a button is held
    /// down.
    ButtonMotion,

    /// Mouse button events should be reported on button press and release,
    /// and mouse motion events should be reported when the mouse moves
    /// between cells regardless of whether a button is held down or not.
    AnyMotion,
    // DecLocator,
}

impl Default for MouseProtocolMode {
    fn default() -> Self {
        Self::None
    }
}

/// The encoding to use for the enabled `MouseProtocolMode`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MouseProtocolEncoding {
    /// Default single-printable-byte encoding.
    Default,

    /// UTF-8-based encoding.
    Utf8,

    /// SGR-like encoding.
    Sgr,
    // Urxvt,
}

impl Default for MouseProtocolEncoding {
    fn default() -> Self {
        Self::Default
    }
}

/// Represents the overall terminal state.
#[derive(Clone, Debug)]
pub struct Screen {
    grid: crate::grid::Grid,
    alternate_grid: crate::grid::Grid,

    attrs: crate::attrs::Attrs,
    saved_attrs: crate::attrs::Attrs,

    title: String,
    icon_name: String,

    modes: enumset::EnumSet<Mode>,
    mouse_protocol_mode: MouseProtocolMode,
    mouse_protocol_encoding: MouseProtocolEncoding,

    audible_bell_count: usize,
    visual_bell_count: usize,
}

impl Screen {
    pub(crate) fn new(
        size: crate::grid::Size,
        scrollback_len: usize,
    ) -> Self {
        Self {
            grid: crate::grid::Grid::new(size, scrollback_len),
            alternate_grid: crate::grid::Grid::new(size, 0),

            attrs: crate::attrs::Attrs::default(),
            saved_attrs: crate::attrs::Attrs::default(),

            title: String::default(),
            icon_name: String::default(),

            modes: enumset::EnumSet::default(),
            mouse_protocol_mode: MouseProtocolMode::default(),
            mouse_protocol_encoding: MouseProtocolEncoding::default(),

            audible_bell_count: 0,
            visual_bell_count: 0,
        }
    }

    pub(crate) fn set_size(&mut self, rows: u16, cols: u16) {
        self.grid.set_size(crate::grid::Size { rows, cols });
        self.alternate_grid
            .set_size(crate::grid::Size { rows, cols });
    }

    /// Returns the current size of the terminal.
    ///
    /// The return value will be (rows, cols).
    pub fn size(&self) -> (u16, u16) {
        let size = self.grid().size();
        (size.rows, size.cols)
    }

    /// Returns the current position in the scrollback.
    ///
    /// This position indicates the offset from the top of the screen, and is
    /// `0` when the normal screen is in view.
    pub fn scrollback(&self) -> usize {
        self.grid().scrollback()
    }

    pub(crate) fn set_scrollback(&mut self, rows: usize) {
        self.grid_mut().set_scrollback(rows);
    }

    /// Returns the text contents of the terminal.
    ///
    /// This will not include any formatting information, and will be in plain
    /// text format.
    pub fn contents(&self) -> String {
        let mut contents = String::new();
        self.write_contents(&mut contents);
        contents
    }

    fn write_contents(&self, contents: &mut String) {
        self.grid().write_contents(contents);
    }

    /// Returns the text contents of the terminal by row, restricted to the
    /// given subset of columns.
    ///
    /// This will not include any formatting information, and will be in plain
    /// text format.
    ///
    /// Newlines will not be included.
    pub fn rows(
        &self,
        start: u16,
        width: u16,
    ) -> impl Iterator<Item = String> + '_ {
        self.grid().visible_rows().map(move |row| {
            let mut contents = String::new();
            row.write_contents(&mut contents, start, width);
            contents
        })
    }

    /// Returns the formatted visible contents of the terminal.
    ///
    /// Formatting information will be included inline as terminal escape
    /// codes. The result will be suitable for feeding directly to a raw
    /// terminal parser, and will result in the same visual output.
    pub fn contents_formatted(&self) -> Vec<u8> {
        let mut contents = vec![];
        self.write_contents_formatted(&mut contents);
        contents
    }

    fn write_contents_formatted(&self, contents: &mut Vec<u8>) {
        crate::term::HideCursor::new(self.hide_cursor()).write_buf(contents);
        let prev_attrs = self.grid().write_contents_formatted(contents);
        self.attrs.write_escape_code_diff(contents, &prev_attrs);
    }

    /// Returns the formatted visible contents of the terminal by row,
    /// restricted to the given subset of columns.
    ///
    /// Formatting information will be included inline as terminal escape
    /// codes. The result will be suitable for feeding directly to a raw
    /// terminal parser, and will result in the same visual output.
    ///
    /// You are responsible for positioning the cursor before printing each
    /// row, and the final cursor position after displaying each row is
    /// unspecified.
    pub fn rows_formatted(
        &self,
        start: u16,
        width: u16,
    ) -> impl Iterator<Item = Vec<u8>> + '_ {
        self.grid().visible_rows().enumerate().map(move |(i, row)| {
            let i = i.try_into().unwrap();
            let mut contents = vec![];
            row.write_contents_formatted(
                &mut contents,
                start,
                width,
                i,
                false,
                crate::grid::Pos { row: i, col: start },
                crate::attrs::Attrs::default(),
            );
            contents
        })
    }

    /// Returns a terminal byte stream sufficient to turn the visible contents
    /// of the screen described by `prev` into the visible contents of the
    /// screen described by `self`.
    ///
    /// The result of rendering `prev.contents_formatted()` followed by
    /// `self.contents_diff(prev)` should be equivalent to the result of
    /// rendering `self.contents_formatted()`. This is primarily useful when
    /// you already have a terminal parser whose state is described by `prev`,
    /// since the diff will likely require less memory and cause less
    /// flickering than redrawing the entire screen contents.
    pub fn contents_diff(&self, prev: &Self) -> Vec<u8> {
        let mut contents = vec![];
        self.write_contents_diff(&mut contents, prev);
        contents
    }

    fn write_contents_diff(&self, contents: &mut Vec<u8>, prev: &Self) {
        if self.hide_cursor() != prev.hide_cursor() {
            crate::term::HideCursor::new(self.hide_cursor())
                .write_buf(contents);
        }
        let prev_attrs = self.grid().write_contents_diff(
            contents,
            prev.grid(),
            prev.attrs,
        );
        self.attrs.write_escape_code_diff(contents, &prev_attrs);
    }

    /// Returns a sequence of terminal byte streams sufficient to turn the
    /// visible contents of the subset of each row from `prev` (as described
    /// by `start` and `width`) into the visible contents of the corresponding
    /// row subset in `self`.
    ///
    /// You are responsible for positioning the cursor before printing each
    /// row, and the final cursor position after displaying each row is
    /// unspecified.
    pub fn rows_diff<'a>(
        &'a self,
        prev: &'a Self,
        start: u16,
        width: u16,
    ) -> impl Iterator<Item = Vec<u8>> + 'a {
        self.grid()
            .visible_rows()
            .zip(prev.grid().visible_rows())
            .enumerate()
            .map(move |(i, (row, prev_row))| {
                let i = i.try_into().unwrap();
                let mut contents = vec![];
                row.write_contents_diff(
                    &mut contents,
                    prev_row,
                    start,
                    width,
                    i,
                    false,
                    crate::grid::Pos { row: i, col: start },
                    crate::attrs::Attrs::default(),
                );
                contents
            })
    }

    /// Returns terminal escape sequences sufficient to set the current
    /// terminal's input modes.
    ///
    /// Supported modes are:
    /// * application keypad
    /// * application cursor
    /// * bracketed paste
    /// * xterm mouse support
    pub fn input_mode_formatted(&self) -> Vec<u8> {
        let mut contents = vec![];
        self.write_input_mode_formatted(&mut contents);
        contents
    }

    fn write_input_mode_formatted(&self, contents: &mut Vec<u8>) {
        crate::term::ApplicationKeypad::new(
            self.mode(Mode::ApplicationKeypad),
        )
        .write_buf(contents);
        crate::term::ApplicationCursor::new(
            self.mode(Mode::ApplicationCursor),
        )
        .write_buf(contents);
        crate::term::BracketedPaste::new(self.mode(Mode::BracketedPaste))
            .write_buf(contents);
        crate::term::MouseProtocolMode::new(
            self.mouse_protocol_mode,
            MouseProtocolMode::None,
        )
        .write_buf(contents);
        crate::term::MouseProtocolEncoding::new(
            self.mouse_protocol_encoding,
            MouseProtocolEncoding::Default,
        )
        .write_buf(contents);
    }

    /// Returns terminal escape sequences sufficient to change the previous
    /// terminal's input modes to the input modes enabled in the current
    /// terminal.
    pub fn input_mode_diff(&self, prev: &Self) -> Vec<u8> {
        let mut contents = vec![];
        self.write_input_mode_diff(&mut contents, prev);
        contents
    }

    fn write_input_mode_diff(&self, contents: &mut Vec<u8>, prev: &Self) {
        if self.mode(Mode::ApplicationKeypad)
            != prev.mode(Mode::ApplicationKeypad)
        {
            crate::term::ApplicationKeypad::new(
                self.mode(Mode::ApplicationKeypad),
            )
            .write_buf(contents);
        }
        if self.mode(Mode::ApplicationCursor)
            != prev.mode(Mode::ApplicationCursor)
        {
            crate::term::ApplicationCursor::new(
                self.mode(Mode::ApplicationCursor),
            )
            .write_buf(contents);
        }
        if self.mode(Mode::BracketedPaste) != prev.mode(Mode::BracketedPaste)
        {
            crate::term::BracketedPaste::new(self.mode(Mode::BracketedPaste))
                .write_buf(contents);
        }
        crate::term::MouseProtocolMode::new(
            self.mouse_protocol_mode,
            prev.mouse_protocol_mode,
        )
        .write_buf(contents);
        crate::term::MouseProtocolEncoding::new(
            self.mouse_protocol_encoding,
            prev.mouse_protocol_encoding,
        )
        .write_buf(contents);
    }

    /// Returns terminal escape sequences sufficient to set the current
    /// terminal's window title.
    pub fn title_formatted(&self) -> Vec<u8> {
        let mut contents = vec![];
        self.write_title_formatted(&mut contents);
        contents
    }

    fn write_title_formatted(&self, contents: &mut Vec<u8>) {
        crate::term::ChangeTitle::new(&self.icon_name, &self.title, "", "")
            .write_buf(contents);
    }

    /// Returns terminal escape sequences sufficient to change the previous
    /// terminal's window title to the window title set in the current
    /// terminal.
    pub fn title_diff(&self, prev: &Self) -> Vec<u8> {
        let mut contents = vec![];
        self.write_title_diff(&mut contents, prev);
        contents
    }

    fn write_title_diff(&self, contents: &mut Vec<u8>, prev: &Self) {
        crate::term::ChangeTitle::new(
            &self.icon_name,
            &self.title,
            &prev.icon_name,
            &prev.title,
        )
        .write_buf(contents);
    }

    /// Returns terminal escape sequences sufficient to cause audible and
    /// visual bells to occur if they have been received since the terminal
    /// described by `prev`.
    pub fn bells_diff(&self, prev: &Self) -> Vec<u8> {
        let mut contents = vec![];
        self.write_bells_diff(&mut contents, prev);
        contents
    }

    fn write_bells_diff(&self, contents: &mut Vec<u8>, prev: &Self) {
        if self.audible_bell_count != prev.audible_bell_count {
            crate::term::AudibleBell::default().write_buf(contents);
        }
        if self.visual_bell_count != prev.visual_bell_count {
            crate::term::VisualBell::default().write_buf(contents);
        }
    }

    /// Returns the `Cell` object at the given location in the terminal, if it
    /// exists.
    pub fn cell(&self, row: u16, col: u16) -> Option<&crate::cell::Cell> {
        self.grid().visible_cell(crate::grid::Pos { row, col })
    }

    /// Returns the current cursor position of the terminal.
    ///
    /// The return value will be (row, col).
    pub fn cursor_position(&self) -> (u16, u16) {
        let pos = self.grid().pos();
        (pos.row, pos.col)
    }

    /// Returns the terminal's window title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the terminal's icon name.
    pub fn icon_name(&self) -> &str {
        &self.icon_name
    }

    /// Returns a value which changes every time an audible bell is received.
    ///
    /// Typically you would store this number after each call to `process`,
    /// and trigger an audible bell whenever it changes.
    ///
    /// You shouldn't rely on the exact value returned here, since the exact
    /// value will not be maintained by `contents_formatted` or
    /// `contents_diff`.
    pub fn audible_bell_count(&self) -> usize {
        self.audible_bell_count
    }

    /// Returns a value which changes every time an visual bell is received.
    ///
    /// Typically you would store this number after each call to `process`,
    /// and trigger an visual bell whenever it changes.
    ///
    /// You shouldn't rely on the exact value returned here, since the exact
    /// value will not be maintained by `contents_formatted` or
    /// `contents_diff`.
    pub fn visual_bell_count(&self) -> usize {
        self.visual_bell_count
    }

    /// Returns whether the terminal should be in application keypad mode.
    pub fn application_keypad(&self) -> bool {
        self.mode(Mode::ApplicationKeypad)
    }

    /// Returns whether the terminal should be in application cursor mode.
    pub fn application_cursor(&self) -> bool {
        self.mode(Mode::ApplicationCursor)
    }

    /// Returns whether the terminal should be in hide cursor mode.
    pub fn hide_cursor(&self) -> bool {
        self.mode(Mode::HideCursor)
    }

    /// Returns whether the terminal should be in bracketed paste mode.
    pub fn bracketed_paste(&self) -> bool {
        self.mode(Mode::BracketedPaste)
    }

    /// Returns the currently active `MouseProtocolMode`
    pub fn mouse_protocol_mode(&self) -> MouseProtocolMode {
        self.mouse_protocol_mode
    }

    /// Returns the currently active `MouseProtocolEncoding`
    pub fn mouse_protocol_encoding(&self) -> MouseProtocolEncoding {
        self.mouse_protocol_encoding
    }

    fn grid(&self) -> &crate::grid::Grid {
        if self.mode(Mode::AlternateScreen) {
            &self.alternate_grid
        } else {
            &self.grid
        }
    }

    fn grid_mut(&mut self) -> &mut crate::grid::Grid {
        if self.mode(Mode::AlternateScreen) {
            &mut self.alternate_grid
        } else {
            &mut self.grid
        }
    }

    fn drawing_row(&self, pos: crate::grid::Pos) -> Option<&crate::row::Row> {
        self.grid().drawing_row(pos)
    }

    fn drawing_cell_mut(
        &mut self,
        pos: crate::grid::Pos,
    ) -> Option<&mut crate::cell::Cell> {
        self.grid_mut().drawing_cell_mut(pos)
    }

    fn current_cell_mut(&mut self) -> &mut crate::cell::Cell {
        self.grid_mut().current_cell_mut()
    }

    fn enter_alternate_grid(&mut self) {
        self.grid_mut().set_scrollback(0);
        self.set_mode(Mode::AlternateScreen);
    }

    fn exit_alternate_grid(&mut self) {
        self.clear_mode(Mode::AlternateScreen);
    }

    fn save_cursor(&mut self) {
        self.grid_mut().save_cursor();
        self.saved_attrs = self.attrs;
    }

    fn restore_cursor(&mut self) {
        self.grid_mut().restore_cursor();
        self.attrs = self.saved_attrs;
    }

    fn set_mode(&mut self, mode: Mode) {
        self.modes.insert(mode);
    }

    fn clear_mode(&mut self, mode: Mode) {
        self.modes.remove(mode);
    }

    fn mode(&self, mode: Mode) -> bool {
        self.modes.contains(mode)
    }

    fn set_mouse_mode(&mut self, mode: MouseProtocolMode) {
        self.mouse_protocol_mode = mode;
    }

    fn clear_mouse_mode(&mut self, mode: MouseProtocolMode) {
        if self.mouse_protocol_mode == mode {
            self.mouse_protocol_mode = MouseProtocolMode::default();
        }
    }

    fn set_mouse_encoding(&mut self, encoding: MouseProtocolEncoding) {
        self.mouse_protocol_encoding = encoding;
    }

    fn clear_mouse_encoding(&mut self, encoding: MouseProtocolEncoding) {
        if self.mouse_protocol_encoding == encoding {
            self.mouse_protocol_encoding = MouseProtocolEncoding::default();
        }
    }
}

impl Screen {
    fn text(&mut self, c: char) {
        let pos = self.grid().pos();
        if pos.col > 0 {
            let attrs = self.attrs;
            let prev_cell = self
                .drawing_cell_mut(crate::grid::Pos {
                    row: pos.row,
                    col: pos.col - 1,
                })
                .unwrap();
            if prev_cell.is_wide() {
                prev_cell.clear(attrs);
            }
        }

        let width = c.width().unwrap_or(0).try_into().unwrap();
        let attrs = self.attrs;

        self.grid_mut().col_wrap(width);
        let cell = self.current_cell_mut();

        if width == 0 {
            if pos.col > 0 {
                let prev_cell = self
                    .drawing_cell_mut(crate::grid::Pos {
                        row: pos.row,
                        col: pos.col - 1,
                    })
                    .unwrap();
                prev_cell.append(c);
            } else if pos.row > 0 {
                let prev_row = self
                    .drawing_row(crate::grid::Pos {
                        row: pos.row - 1,
                        col: 0,
                    })
                    .unwrap();
                if prev_row.wrapped() {
                    let prev_cell = self
                        .drawing_cell_mut(crate::grid::Pos {
                            row: pos.row - 1,
                            col: self.grid().size().cols - 1,
                        })
                        .unwrap();
                    prev_cell.append(c);
                }
            }
        } else {
            cell.set(c, attrs);
            self.grid_mut().col_inc(1);
            if width > 1 {
                let attrs = self.attrs;
                let next_cell = self.current_cell_mut();
                next_cell.clear(attrs);
                self.grid_mut().col_inc(1);
            }
        }
    }

    // control codes

    fn bel(&mut self) {
        self.audible_bell_count += 1;
    }

    fn bs(&mut self) {
        self.grid_mut().col_dec(1);
    }

    fn tab(&mut self) {
        self.grid_mut().col_tab();
    }

    fn lf(&mut self) {
        self.grid_mut().row_inc_scroll(1);
    }

    fn vt(&mut self) {
        self.lf();
    }

    fn ff(&mut self) {
        self.lf();
    }

    fn cr(&mut self) {
        self.grid_mut().col_set(0);
    }

    // escape codes

    // ESC 7
    fn decsc(&mut self) {
        self.save_cursor();
    }

    // ESC 8
    fn decrc(&mut self) {
        self.restore_cursor();
    }

    // ESC =
    fn deckpam(&mut self) {
        self.set_mode(Mode::ApplicationKeypad);
    }

    // ESC >
    fn deckpnm(&mut self) {
        self.clear_mode(Mode::ApplicationKeypad);
    }

    // ESC M
    fn ri(&mut self) {
        self.grid_mut().row_dec_scroll(1);
    }

    // ESC c
    fn ris(&mut self) {
        let title = self.title.clone();
        let icon_name = self.icon_name.clone();
        let audible_bell_count = self.audible_bell_count;
        let visual_bell_count = self.visual_bell_count;

        *self = Self::new(self.grid.size(), self.grid.scrollback_len());

        self.title = title;
        self.icon_name = icon_name;
        self.audible_bell_count = audible_bell_count;
        self.visual_bell_count = visual_bell_count;
    }

    // ESC g
    fn vb(&mut self) {
        self.visual_bell_count += 1;
    }

    // csi codes

    // CSI @
    fn ich(&mut self, count: u16) {
        self.grid_mut().insert_cells(count);
    }

    // CSI A
    fn cuu(&mut self, offset: u16) {
        self.grid_mut().row_dec_clamp(offset);
    }

    // CSI B
    fn cud(&mut self, offset: u16) {
        self.grid_mut().row_inc_clamp(offset);
    }

    // CSI C
    fn cuf(&mut self, offset: u16) {
        self.grid_mut().col_inc_clamp(offset);
    }

    // CSI D
    fn cub(&mut self, offset: u16) {
        self.grid_mut().col_dec(offset);
    }

    // CSI G
    fn cha(&mut self, col: u16) {
        self.grid_mut().col_set(col - 1);
    }

    // CSI H
    fn cup(&mut self, (row, col): (u16, u16)) {
        self.grid_mut().set_pos(crate::grid::Pos {
            row: row - 1,
            col: col - 1,
        });
    }

    // CSI J
    fn ed(&mut self, mode: u16) {
        let attrs = self.attrs;
        match mode {
            0 => self.grid_mut().erase_all_forward(attrs),
            1 => self.grid_mut().erase_all_backward(attrs),
            2 => self.grid_mut().erase_all(attrs),
            _ => {}
        }
    }

    // CSI ? J
    fn decsed(&mut self, mode: u16) {
        self.ed(mode);
    }

    // CSI K
    fn el(&mut self, mode: u16) {
        let attrs = self.attrs;
        match mode {
            0 => self.grid_mut().erase_row_forward(attrs),
            1 => self.grid_mut().erase_row_backward(attrs),
            2 => self.grid_mut().erase_row(attrs),
            _ => {}
        }
    }

    // CSI ? K
    fn decsel(&mut self, mode: u16) {
        self.el(mode);
    }

    // CSI L
    fn il(&mut self, count: u16) {
        self.grid_mut().insert_lines(count);
    }

    // CSI M
    fn dl(&mut self, count: u16) {
        self.grid_mut().delete_lines(count);
    }

    // CSI P
    fn dch(&mut self, count: u16) {
        self.grid_mut().delete_cells(count);
    }

    // CSI S
    fn su(&mut self, count: u16) {
        self.grid_mut().scroll_up(count);
    }

    // CSI T
    fn sd(&mut self, count: u16) {
        self.grid_mut().scroll_down(count);
    }

    // CSI X
    fn ech(&mut self, count: u16) {
        let attrs = self.attrs;
        self.grid_mut().erase_cells(count, attrs);
    }

    // CSI d
    fn vpa(&mut self, row: u16) {
        self.grid_mut().row_set(row - 1);
    }

    // CSI h
    fn sm(&mut self, _params: &[i64]) {
        // nothing, i think?
    }

    // CSI ? h
    fn decset(&mut self, params: &[i64]) {
        for param in params {
            match param {
                1 => self.set_mode(Mode::ApplicationCursor),
                6 => self.grid_mut().set_origin_mode(true),
                9 => self.set_mouse_mode(MouseProtocolMode::Press),
                25 => self.clear_mode(Mode::HideCursor),
                47 => self.enter_alternate_grid(),
                1000 => self.set_mouse_mode(MouseProtocolMode::PressRelease),
                1002 => self.set_mouse_mode(MouseProtocolMode::ButtonMotion),
                1003 => self.set_mouse_mode(MouseProtocolMode::AnyMotion),
                1005 => self.set_mouse_encoding(MouseProtocolEncoding::Utf8),
                1006 => self.set_mouse_encoding(MouseProtocolEncoding::Sgr),
                1049 => {
                    self.decsc();
                    self.alternate_grid.clear();
                    self.enter_alternate_grid();
                }
                2004 => self.set_mode(Mode::BracketedPaste),
                _ => {}
            }
        }
    }

    // CSI l
    fn rm(&mut self, _params: &[i64]) {
        // nothing, i think?
    }

    // CSI ? l
    fn decrst(&mut self, params: &[i64]) {
        for param in params {
            match param {
                1 => self.clear_mode(Mode::ApplicationCursor),
                6 => self.grid_mut().set_origin_mode(false),
                9 => self.clear_mouse_mode(MouseProtocolMode::Press),
                25 => self.set_mode(Mode::HideCursor),
                47 => {
                    self.exit_alternate_grid();
                }
                1000 => {
                    self.clear_mouse_mode(MouseProtocolMode::PressRelease)
                }
                1002 => {
                    self.clear_mouse_mode(MouseProtocolMode::ButtonMotion)
                }
                1003 => self.clear_mouse_mode(MouseProtocolMode::AnyMotion),
                1005 => {
                    self.clear_mouse_encoding(MouseProtocolEncoding::Utf8)
                }
                1006 => self.clear_mouse_encoding(MouseProtocolEncoding::Sgr),
                1049 => {
                    self.exit_alternate_grid();
                    self.decrc();
                }
                2004 => self.clear_mode(Mode::BracketedPaste),
                _ => {}
            }
        }
    }

    // CSI m
    fn sgr(&mut self, params: &[i64]) {
        let mut i = 0;

        macro_rules! next_param {
            () => {
                if i >= params.len() {
                    return;
                } else if let Some(n) = i64_to_u8(params[i]) {
                    i += 1;
                    n
                } else {
                    return;
                }
            };
        }

        loop {
            match next_param!() {
                0 => self.attrs = crate::attrs::Attrs::default(),
                1 => self.attrs.set_bold(true),
                3 => self.attrs.set_italic(true),
                4 => self.attrs.set_underline(true),
                7 => self.attrs.set_inverse(true),
                22 => self.attrs.set_bold(false),
                23 => self.attrs.set_italic(false),
                24 => self.attrs.set_underline(false),
                27 => self.attrs.set_inverse(false),
                n if n >= 30 && n <= 37 => {
                    self.attrs.fgcolor = crate::attrs::Color::Idx(n - 30);
                }
                38 => match next_param!() {
                    2 => {
                        let r = next_param!();
                        let g = next_param!();
                        let b = next_param!();
                        self.attrs.fgcolor =
                            crate::attrs::Color::Rgb(r, g, b);
                    }
                    5 => {
                        self.attrs.fgcolor =
                            crate::attrs::Color::Idx(next_param!());
                    }
                    _ => {}
                },
                39 => {
                    self.attrs.fgcolor = crate::attrs::Color::Default;
                }
                n if n >= 40 && n <= 47 => {
                    self.attrs.bgcolor = crate::attrs::Color::Idx(n - 40);
                }
                48 => match next_param!() {
                    2 => {
                        let r = next_param!();
                        let g = next_param!();
                        let b = next_param!();
                        self.attrs.bgcolor =
                            crate::attrs::Color::Rgb(r, g, b);
                    }
                    5 => {
                        self.attrs.bgcolor =
                            crate::attrs::Color::Idx(next_param!());
                    }
                    _ => {}
                },
                49 => {
                    self.attrs.bgcolor = crate::attrs::Color::Default;
                }
                n if n >= 90 && n <= 97 => {
                    self.attrs.fgcolor = crate::attrs::Color::Idx(n - 82);
                }
                n if n >= 100 && n <= 107 => {
                    self.attrs.bgcolor = crate::attrs::Color::Idx(n - 92);
                }
                _ => {}
            }
        }
    }

    // CSI r
    fn decstbm(&mut self, (top, bottom): (u16, u16)) {
        self.grid_mut().set_scroll_region(top - 1, bottom - 1);
    }

    // osc codes

    fn osc0(&mut self, s: &[u8]) {
        self.osc1(s);
        self.osc2(s);
    }

    fn osc1(&mut self, s: &[u8]) {
        if let Ok(s) = std::str::from_utf8(s) {
            self.icon_name = s.to_string();
        }
    }

    fn osc2(&mut self, s: &[u8]) {
        if let Ok(s) = std::str::from_utf8(s) {
            self.title = s.to_string();
        }
    }
}

impl vte::Perform for Screen {
    fn print(&mut self, c: char) {
        self.text(c)
    }

    fn execute(&mut self, b: u8) {
        match b {
            7 => self.bel(),
            8 => self.bs(),
            9 => self.tab(),
            10 => self.lf(),
            11 => self.vt(),
            12 => self.ff(),
            13 => self.cr(),
            _ => {
                log::warn!("unhandled control character: {}", b);
            }
        }
    }

    fn esc_dispatch(
        &mut self,
        _params: &[i64],
        intermediates: &[u8],
        _ignore: bool,
        b: u8,
    ) {
        match intermediates.get(0) {
            None => match b {
                b'7' => self.decsc(),
                b'8' => self.decrc(),
                b'=' => self.deckpam(),
                b'>' => self.deckpnm(),
                b'M' => self.ri(),
                b'c' => self.ris(),
                b'g' => self.vb(),
                _ => {
                    log::warn!("unhandled escape code: ESC {}", b);
                }
            },
            Some(i) => {
                log::warn!("unhandled escape code: ESC {} {}", i, b);
            }
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &[i64],
        intermediates: &[u8],
        _ignore: bool,
        c: char,
    ) {
        match intermediates.get(0) {
            None => match c {
                '@' => self.ich(canonicalize_params_1(params, 1)),
                'A' => self.cuu(canonicalize_params_1(params, 1)),
                'B' => self.cud(canonicalize_params_1(params, 1)),
                'C' => self.cuf(canonicalize_params_1(params, 1)),
                'D' => self.cub(canonicalize_params_1(params, 1)),
                'G' => self.cha(canonicalize_params_1(params, 1)),
                'H' => self.cup(canonicalize_params_2(params, 1, 1)),
                'J' => self.ed(canonicalize_params_1(params, 0)),
                'K' => self.el(canonicalize_params_1(params, 0)),
                'L' => self.il(canonicalize_params_1(params, 1)),
                'M' => self.dl(canonicalize_params_1(params, 1)),
                'P' => self.dch(canonicalize_params_1(params, 1)),
                'S' => self.su(canonicalize_params_1(params, 1)),
                'T' => self.sd(canonicalize_params_1(params, 1)),
                'X' => self.ech(canonicalize_params_1(params, 1)),
                'd' => self.vpa(canonicalize_params_1(params, 1)),
                'h' => self.sm(canonicalize_params_multi(params)),
                'l' => self.rm(canonicalize_params_multi(params)),
                'm' => self.sgr(canonicalize_params_multi(params)),
                'r' => self.decstbm(canonicalize_params_decstbm(
                    params,
                    self.grid().size(),
                )),
                _ => {
                    if log::log_enabled!(log::Level::Warn) {
                        log::warn!(
                            "unhandled csi sequence: CSI {} {}",
                            param_str(params),
                            c
                        )
                    }
                }
            },
            Some(b'?') => match c {
                'J' => self.decsed(canonicalize_params_1(params, 0)),
                'K' => self.decsel(canonicalize_params_1(params, 0)),
                'h' => self.decset(canonicalize_params_multi(params)),
                'l' => self.decrst(canonicalize_params_multi(params)),
                _ => {
                    if log::log_enabled!(log::Level::Warn) {
                        log::warn!(
                            "unhandled csi sequence: CSI ? {} {}",
                            param_str(params),
                            c
                        )
                    }
                }
            },
            Some(i) => {
                if log::log_enabled!(log::Level::Warn) {
                    log::warn!(
                        "unhandled csi sequence: CSI {} {} {}",
                        i,
                        param_str(params),
                        c
                    )
                }
            }
        }
    }

    fn osc_dispatch(&mut self, params: &[&[u8]]) {
        match (params.get(0), params.get(1)) {
            (Some(&b"0"), Some(s)) => self.osc0(s),
            (Some(&b"1"), Some(s)) => self.osc1(s),
            (Some(&b"2"), Some(s)) => self.osc2(s),
            _ => {
                if log::log_enabled!(log::Level::Warn) {
                    log::warn!(
                        "unhandled osc sequence: OSC {}",
                        osc_param_str(params),
                    )
                }
            }
        }
    }

    fn hook(&mut self, params: &[i64], intermediates: &[u8], _ignore: bool) {
        if log::log_enabled!(log::Level::Warn) {
            // TODO: include the final byte here (it seems to be a bug that
            // the vte parser doesn't currently pass it to this method)
            match intermediates.get(0) {
                None => log::warn!(
                    "unhandled dcs sequence: DCS {}",
                    param_str(params),
                ),
                Some(i) => log::warn!(
                    "unhandled dcs sequence: DCS {} {}",
                    i,
                    param_str(params),
                ),
            }
        }
    }
    fn put(&mut self, _: u8) {}
    fn unhook(&mut self) {}
}

fn canonicalize_params_1(params: &[i64], default: u16) -> u16 {
    let first = params.get(0).copied().unwrap_or(0);
    if first == 0 {
        default
    } else {
        i64_to_u16(first)
    }
}

fn canonicalize_params_2(
    params: &[i64],
    default1: u16,
    default2: u16,
) -> (u16, u16) {
    let first = params.get(0).copied().unwrap_or(0);
    let first = if first == 0 {
        default1
    } else {
        i64_to_u16(first)
    };

    let second = params.get(1).copied().unwrap_or(0);
    let second = if second == 0 {
        default2
    } else {
        i64_to_u16(second)
    };

    (first, second)
}

fn canonicalize_params_multi(params: &[i64]) -> &[i64] {
    if params.is_empty() {
        DEFAULT_MULTI_PARAMS
    } else {
        params
    }
}

fn canonicalize_params_decstbm(
    params: &[i64],
    size: crate::grid::Size,
) -> (u16, u16) {
    let top = params.get(0).copied().unwrap_or(0);
    let top = if top == 0 { 1 } else { i64_to_u16(top) };

    let bottom = params.get(1).copied().unwrap_or(0);
    let bottom = if bottom == 0 {
        size.rows
    } else {
        i64_to_u16(bottom)
    };

    (top, bottom)
}

fn i64_to_u16(i: i64) -> u16 {
    if i < 0 {
        0
    } else if i > i64::from(u16::max_value()) {
        u16::max_value()
    } else {
        i.try_into().unwrap()
    }
}

fn i64_to_u8(i: i64) -> Option<u8> {
    if i < 0 || i > i64::from(u8::max_value()) {
        None
    } else {
        Some(i.try_into().unwrap())
    }
}

fn param_str(params: &[i64]) -> String {
    let strs: Vec<_> = params
        .iter()
        .map(std::string::ToString::to_string)
        .collect();
    strs.join(" ; ")
}

fn osc_param_str(params: &[&[u8]]) -> String {
    let strs: Vec<_> = params
        .iter()
        .map(|b| format!("\"{}\"", std::string::String::from_utf8_lossy(*b)))
        .collect();
    strs.join(" ; ")
}
