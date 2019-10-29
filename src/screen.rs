struct State {
    size: crate::pos::Pos,
    cells: Vec<Vec<crate::cell::Cell>>,
    cursor_position: crate::pos::Pos,
    fgcolor: crate::color::Color,
    bgcolor: crate::color::Color,
    bold: bool,
    italic: bool,
    inverse: bool,
    underline: bool,
}

impl vte::Perform for State {
    fn print(&mut self, _c: char) {}

    fn execute(&mut self, _b: u8) {}

    fn hook(
        &mut self,
        _params: &[i64],
        _intermediates: &[u8],
        _ignore: bool,
    ) {
    }

    fn put(&mut self, _b: u8) {}

    fn unhook(&mut self) {}

    fn osc_dispatch(&mut self, _params: &[&[u8]]) {}

    fn csi_dispatch(
        &mut self,
        _params: &[i64],
        _intermediates: &[u8],
        _ignore: bool,
        _c: char,
    ) {
    }

    fn esc_dispatch(
        &mut self,
        _params: &[i64],
        _intermediates: &[u8],
        _ignore: bool,
        _b: u8,
    ) {
    }
}

pub struct Screen {
    parser: vte::Parser,
    state: State,
}

impl Screen {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            parser: vte::Parser::new(),
            state: State {
                size: crate::pos::Pos {
                    row: rows,
                    col: cols,
                },
                cells: vec![
                    vec![crate::cell::Cell::default(); cols as usize];
                    rows as usize
                ],
                cursor_position: crate::pos::Pos::default(),
                fgcolor: crate::color::Color::default(),
                bgcolor: crate::color::Color::default(),
                bold: false,
                italic: false,
                inverse: false,
                underline: false,
            },
        }
    }

    pub fn rows(&self) -> u16 {
        self.state.size.row
    }

    pub fn cols(&self) -> u16 {
        self.state.size.col
    }

    pub fn set_window_size(&mut self, rows: u16, cols: u16) {
        self.state.size = crate::pos::Pos {
            row: rows,
            col: cols,
        };
    }

    pub fn process(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.parser.advance(&mut self.state, *byte);
        }
    }

    pub fn cell(&self, row: u16, col: u16) -> Option<&crate::cell::Cell> {
        self.state
            .cells
            .get(row as usize)
            .and_then(|v| v.get(col as usize))
    }

    pub fn window_contents(
        &self,
        row_start: u16,
        col_start: u16,
        row_end: u16,
        col_end: u16,
    ) -> String {
        unimplemented!()
    }

    pub fn window_contents_formatted(
        &self,
        row_start: u16,
        col_start: u16,
        row_end: u16,
        col_end: u16,
    ) -> String {
        unimplemented!()
    }

    pub fn cursor_position(&self) -> (u16, u16) {
        (
            self.state.cursor_position.row,
            self.state.cursor_position.col,
        )
    }

    pub fn fgcolor(&self) -> crate::color::Color {
        self.state.fgcolor
    }

    pub fn bgcolor(&self) -> crate::color::Color {
        self.state.bgcolor
    }

    pub fn bold(&self) -> bool {
        self.state.bold
    }

    pub fn italic(&self) -> bool {
        self.state.italic
    }

    pub fn inverse(&self) -> bool {
        self.state.inverse
    }

    pub fn underline(&self) -> bool {
        self.state.underline
    }

    pub fn title(&self) -> Option<&str> {
        unimplemented!()
    }

    pub fn icon_name(&self) -> Option<&str> {
        unimplemented!()
    }

    pub fn hide_cursor(&self) -> bool {
        unimplemented!()
    }

    pub fn alternate_buffer_active(&self) -> bool {
        unimplemented!()
    }

    pub fn application_cursor(&self) -> bool {
        unimplemented!()
    }

    pub fn application_keypad(&self) -> bool {
        unimplemented!()
    }

    pub fn bracketed_paste(&self) -> bool {
        unimplemented!()
    }

    pub fn mouse_reporting_button_motion(&self) -> bool {
        unimplemented!()
    }

    pub fn mouse_reporting_sgr_mode(&self) -> bool {
        unimplemented!()
    }

    pub fn mouse_reporting_press(&self) -> bool {
        unimplemented!()
    }

    pub fn mouse_reporting_press_release(&self) -> bool {
        unimplemented!()
    }

    pub fn check_audible_bell(&mut self) -> bool {
        unimplemented!()
    }

    pub fn check_visual_bell(&mut self) -> bool {
        unimplemented!()
    }
}
