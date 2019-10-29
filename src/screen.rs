struct State {
    size: crate::pos::Pos,
    rows: Vec<crate::row::Row>,
    cursor_position: crate::pos::Pos,
    attrs: crate::attrs::Attrs,
}

impl State {
    const DEFAULT_SGR_PARAMS: &'static [i64] = &[0];

    fn new(rows: u16, cols: u16) -> Self {
        let size = crate::pos::Pos {
            row: rows,
            col: cols,
        };
        Self {
            size,
            rows: Self::new_rows(size),
            cursor_position: crate::pos::Pos::default(),
            attrs: crate::attrs::Attrs::default(),
        }
    }

    fn new_rows(size: crate::pos::Pos) -> Vec<crate::row::Row> {
        vec![crate::row::Row::new(size.col); size.row as usize]
    }

    fn cell(&self, pos: crate::pos::Pos) -> Option<&crate::cell::Cell> {
        self.rows.get(pos.row as usize).and_then(|r| r.get(pos.col))
    }

    fn cell_mut(
        &mut self,
        pos: crate::pos::Pos,
    ) -> Option<&mut crate::cell::Cell> {
        self.rows
            .get_mut(pos.row as usize)
            .and_then(|v| v.get_mut(pos.col))
    }

    fn current_cell(&self) -> Option<&crate::cell::Cell> {
        self.cell(self.cursor_position)
    }

    fn current_cell_mut(&mut self) -> Option<&mut crate::cell::Cell> {
        self.cell_mut(self.cursor_position)
    }
}

impl vte::Perform for State {
    fn print(&mut self, c: char) {
        let attrs = self.attrs;
        if let Some(cell) = self.current_cell_mut() {
            cell.set(c.to_string(), attrs);
            self.cursor_position.col += 1;
            if self.cursor_position.col > self.size.col {
                self.cursor_position.col = 0;
                self.cursor_position.row += 1;
            }
        } else {
            panic!("couldn't find current cell")
        }
    }

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
        params: &[i64],
        _intermediates: &[u8],
        _ignore: bool,
        c: char,
    ) {
        match c {
            'D' => {
                let offset = params.get(0).copied().unwrap_or(1);
                if self.cursor_position.col >= offset as u16 {
                    self.cursor_position.col -= offset as u16;
                }
            }
            'H' => {
                let row = params.get(0).copied().unwrap_or(1);
                let row = if row == 0 { 1 } else { row };
                let col = params.get(1).copied().unwrap_or(1);
                let col = if col == 0 { 1 } else { col };
                self.cursor_position = crate::pos::Pos {
                    row: row as u16 - 1,
                    col: col as u16 - 1,
                };
            }
            'J' => match params.get(0).copied().unwrap_or(0) {
                0 => {}
                1 => {}
                2 => {
                    self.rows = Self::new_rows(self.size);
                }
                _ => {}
            },
            'm' => {
                let params = if params.is_empty() {
                    Self::DEFAULT_SGR_PARAMS
                } else {
                    params
                };
                let mut i = 0;
                while i < params.len() {
                    match params[i] {
                        0 => self.attrs = crate::attrs::Attrs::default(),
                        1 => self.attrs.bold = true,
                        3 => self.attrs.italic = true,
                        4 => self.attrs.underline = true,
                        7 => self.attrs.inverse = true,
                        22 => self.attrs.bold = false,
                        23 => self.attrs.italic = false,
                        24 => self.attrs.underline = false,
                        27 => self.attrs.inverse = false,
                        n if n >= 30 && n <= 37 => {
                            self.attrs.fgcolor =
                                crate::color::Color::Idx((n as u8) - 30);
                        }
                        38 => {
                            i += 1;
                            if i >= params.len() {
                                unimplemented!()
                            }
                            match params[i] {
                                2 => {
                                    i += 3;
                                    if i >= params.len() {
                                        unimplemented!()
                                    }
                                    self.attrs.fgcolor =
                                        crate::color::Color::Rgb(
                                            params[i - 2] as u8,
                                            params[i - 1] as u8,
                                            params[i] as u8,
                                        );
                                }
                                5 => {
                                    i += 1;
                                    if i >= params.len() {
                                        unimplemented!()
                                    }
                                    self.attrs.fgcolor =
                                        crate::color::Color::Idx(
                                            params[i] as u8,
                                        );
                                }
                                _ => {}
                            }
                        }
                        n if n >= 40 && n <= 47 => {
                            self.attrs.bgcolor =
                                crate::color::Color::Idx((n as u8) - 40);
                        }
                        48 => {
                            i += 1;
                            if i >= params.len() {
                                unimplemented!()
                            }
                            match params[i] {
                                2 => {
                                    i += 3;
                                    if i >= params.len() {
                                        unimplemented!()
                                    }
                                    self.attrs.bgcolor =
                                        crate::color::Color::Rgb(
                                            params[i - 2] as u8,
                                            params[i - 1] as u8,
                                            params[i] as u8,
                                        );
                                }
                                5 => {
                                    i += 1;
                                    if i >= params.len() {
                                        unimplemented!()
                                    }
                                    self.attrs.bgcolor =
                                        crate::color::Color::Idx(
                                            params[i] as u8,
                                        );
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }
            _ => {}
        }
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
            state: State::new(rows, cols),
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
        self.state.cell(crate::pos::Pos { row, col })
    }

    pub fn window_contents(
        &self,
        row_start: u16,
        col_start: u16,
        row_end: u16,
        col_end: u16,
    ) -> String {
        let mut contents = String::new();
        for row in row_start..=(row_end.min(self.state.size.row)) {
            contents +=
                &self.state.rows[row as usize].contents(col_start, col_end);
        }
        contents
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
        self.state.attrs.fgcolor
    }

    pub fn bgcolor(&self) -> crate::color::Color {
        self.state.attrs.bgcolor
    }

    pub fn bold(&self) -> bool {
        self.state.attrs.bold
    }

    pub fn italic(&self) -> bool {
        self.state.attrs.italic
    }

    pub fn underline(&self) -> bool {
        self.state.attrs.underline
    }

    pub fn inverse(&self) -> bool {
        self.state.attrs.inverse
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
