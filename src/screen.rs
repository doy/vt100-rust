struct State {
    grid: crate::grid::Grid,
    cursor_position: crate::grid::Pos,
    attrs: crate::attrs::Attrs,
    got_audible_bell: bool,
    got_visual_bell: bool,
}

impl State {
    fn new(rows: u16, cols: u16) -> Self {
        let size = crate::grid::Size::new(rows, cols);
        Self {
            grid: crate::grid::Grid::new(size),
            cursor_position: crate::grid::Pos::new(0, 0, size),
            attrs: crate::attrs::Attrs::default(),
            got_audible_bell: false,
            got_visual_bell: false,
        }
    }

    fn cell(&self, pos: crate::grid::Pos) -> Option<&crate::cell::Cell> {
        self.grid.cell(pos)
    }

    fn cell_mut(
        &mut self,
        pos: crate::grid::Pos,
    ) -> Option<&mut crate::cell::Cell> {
        self.grid.cell_mut(pos)
    }

    fn current_cell(&self) -> Option<&crate::cell::Cell> {
        self.cell(self.cursor_position)
    }

    fn current_cell_mut(&mut self) -> Option<&mut crate::cell::Cell> {
        self.cell_mut(self.cursor_position)
    }

    fn pos(&self, row: u16, col: u16) -> crate::grid::Pos {
        crate::grid::Pos::new(row, col, *self.grid.size())
    }
}

impl State {
    const DEFAULT_SGR_PARAMS: &'static [i64] = &[0];

    // control codes

    fn text(&mut self, c: char) {
        let attrs = self.attrs;
        if let Some(cell) = self.current_cell_mut() {
            cell.set(c.to_string(), attrs);
            self.cursor_position.col_inc(1);
        } else {
            panic!("couldn't find current cell")
        }
    }

    fn bel(&mut self) {
        self.got_audible_bell = true;
    }

    fn bs(&mut self) {
        // XXX is this correct? is backwards wrapping a thing?
        self.cursor_position.col_dec(1);
    }

    fn tab(&mut self) {
        self.cursor_position.next_tabstop();
    }

    fn lf(&mut self) {
        self.cursor_position.row_inc(1);
    }

    fn cr(&mut self) {
        self.cursor_position.col_set(0);
    }

    // escape codes

    // csi codes

    // CSI D
    fn cub(&mut self, params: &[i64]) {
        let offset = params.get(0).copied().unwrap_or(1);
        self.cursor_position.col_dec(offset as u16);
    }

    // CSI H
    fn cup(&mut self, params: &[i64]) {
        // XXX need to handle value overflow
        self.cursor_position = self.pos(
            normalize_absolute_position(params.get(0).map(|i| *i as u16)),
            normalize_absolute_position(params.get(1).map(|i| *i as u16)),
        );
    }

    // CSI J
    fn ed(&mut self, params: &[i64]) {
        match params.get(0).copied().unwrap_or(0) {
            0 => unimplemented!(),
            1 => unimplemented!(),
            2 => {
                self.grid = crate::grid::Grid::new(*self.grid.size());
            }
            _ => {}
        }
    }

    // CSI m
    fn sgr(&mut self, params: &[i64]) {
        // XXX need to handle value overflow
        // XXX need to handle incorrect numbers of parameters for some of the
        // fancier options
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
                            self.attrs.fgcolor = crate::color::Color::Rgb(
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
                                crate::color::Color::Idx(params[i] as u8);
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
                            self.attrs.bgcolor = crate::color::Color::Rgb(
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
                                crate::color::Color::Idx(params[i] as u8);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }

    // osc codes
}

impl vte::Perform for State {
    fn print(&mut self, c: char) {
        self.text(c)
    }

    fn execute(&mut self, b: u8) {
        match b {
            7 => self.bel(),
            8 => self.bs(),
            9 => self.tab(),
            10 => self.lf(),
            13 => self.cr(),
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

    fn csi_dispatch(
        &mut self,
        params: &[i64],
        _intermediates: &[u8],
        _ignore: bool,
        c: char,
    ) {
        match c {
            'D' => self.cub(params),
            'H' => self.cup(params),
            'J' => self.ed(params),
            'm' => self.sgr(params),
            _ => {}
        }
    }

    fn osc_dispatch(&mut self, _params: &[&[u8]]) {}

    // don't care
    fn hook(&mut self, _: &[i64], _: &[u8], _: bool) {}
    fn put(&mut self, _b: u8) {}
    fn unhook(&mut self) {}
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
        self.state.grid.size().rows()
    }

    pub fn cols(&self) -> u16 {
        self.state.grid.size().cols()
    }

    pub fn set_window_size(&mut self, rows: u16, cols: u16) {
        self.state.grid.set_size(crate::grid::Size::new(rows, cols));
        self.state
            .cursor_position
            .set_size(crate::grid::Size::new(rows, cols));
    }

    pub fn process(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.parser.advance(&mut self.state, *byte);
        }
    }

    pub fn cell(&self, row: u16, col: u16) -> Option<&crate::cell::Cell> {
        self.state.cell(self.state.pos(row, col))
    }

    pub fn window_contents(
        &self,
        row_start: u16,
        col_start: u16,
        row_end: u16,
        col_end: u16,
    ) -> String {
        self.state
            .grid
            .window_contents(row_start, col_start, row_end, col_end)
    }

    pub fn window_contents_formatted(
        &self,
        row_start: u16,
        col_start: u16,
        row_end: u16,
        col_end: u16,
    ) -> String {
        self.state
            .grid
            .window_contents_formatted(row_start, col_start, row_end, col_end)
    }

    pub fn cursor_position(&self) -> (u16, u16) {
        (
            self.state.cursor_position.row(),
            self.state.cursor_position.col(),
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
        let ret = self.state.got_audible_bell;
        self.state.got_audible_bell = false;
        ret
    }

    pub fn check_visual_bell(&mut self) -> bool {
        let ret = self.state.got_visual_bell;
        self.state.got_visual_bell = false;
        ret
    }
}

fn normalize_absolute_position(i: Option<u16>) -> u16 {
    let i = if let Some(i) = i {
        if i == 0 {
            1
        } else {
            i
        }
    } else {
        1
    };
    i - 1
}
