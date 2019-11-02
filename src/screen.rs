use std::convert::TryInto as _;

const DEFAULT_MULTI_PARAMS: &[i64] = &[0];

#[derive(enumset::EnumSetType, Debug)]
enum Output {
    AudibleBell,
    VisualBell,
}

#[derive(enumset::EnumSetType, Debug)]
enum Mode {
    KeypadApplication,
    ApplicationCursor,
    HideCursor,
    BracketedPaste,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MouseProtocolMode {
    None,
    Press,
    PressRelease,
    // Highlight,
    ButtonMotion,
    AnyMotion,
    // DecLocator,
}

impl Default for MouseProtocolMode {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MouseProtocolEncoding {
    Default,
    Utf8,
    Sgr,
    // Urxvt,
}

impl Default for MouseProtocolEncoding {
    fn default() -> Self {
        Self::Default
    }
}

struct State {
    grid: crate::grid::Grid,
    alternate_grid: crate::grid::Grid,
    use_alternate_grid: bool,

    attrs: crate::attrs::Attrs,

    title: String,
    icon_name: String,

    outputs: enumset::EnumSet<Output>,
    modes: enumset::EnumSet<Mode>,
    mouse_protocol_mode: MouseProtocolMode,
    mouse_protocol_encoding: MouseProtocolEncoding,
}

impl State {
    fn new(rows: u16, cols: u16) -> Self {
        let size = crate::grid::Size { rows, cols };
        Self {
            grid: crate::grid::Grid::new(size),
            alternate_grid: crate::grid::Grid::new(size),
            use_alternate_grid: false,

            attrs: crate::attrs::Attrs::default(),

            title: String::default(),
            icon_name: String::default(),

            outputs: enumset::EnumSet::default(),
            modes: enumset::EnumSet::default(),
            mouse_protocol_mode: MouseProtocolMode::default(),
            mouse_protocol_encoding: MouseProtocolEncoding::default(),
        }
    }

    fn new_grid(&self) -> crate::grid::Grid {
        crate::grid::Grid::new(*self.grid().size())
    }

    fn grid(&self) -> &crate::grid::Grid {
        if self.use_alternate_grid {
            &self.alternate_grid
        } else {
            &self.grid
        }
    }

    fn grid_mut(&mut self) -> &mut crate::grid::Grid {
        if self.use_alternate_grid {
            &mut self.alternate_grid
        } else {
            &mut self.grid
        }
    }

    fn row(&self, pos: crate::grid::Pos) -> Option<&crate::row::Row> {
        self.grid().row(pos)
    }

    fn cell(&self, pos: crate::grid::Pos) -> Option<&crate::cell::Cell> {
        self.grid().cell(pos)
    }

    fn cell_mut(
        &mut self,
        pos: crate::grid::Pos,
    ) -> Option<&mut crate::cell::Cell> {
        self.grid_mut().cell_mut(pos)
    }

    fn current_cell_mut(&mut self) -> &mut crate::cell::Cell {
        self.grid_mut()
            .current_cell_mut()
            .expect("cursor not pointing to a cell")
    }

    fn enter_alternate_grid(&mut self) {
        self.use_alternate_grid = true;
    }

    fn exit_alternate_grid(&mut self) {
        self.use_alternate_grid = false;
    }

    fn set_output(&mut self, output: Output) {
        self.outputs.insert(output);
    }

    fn clear_output(&mut self, output: Output) {
        self.outputs.remove(output);
    }

    fn check_output(&mut self, output: Output) -> bool {
        let ret = self.outputs.contains(output);
        self.clear_output(output);
        ret
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

impl State {
    fn text(&mut self, c: char) {
        let pos = *self.grid().pos();
        if pos.col > 0 {
            let prev_cell = self
                .cell_mut(crate::grid::Pos {
                    row: pos.row,
                    col: pos.col - 1,
                })
                .unwrap();
            if prev_cell.is_wide() {
                prev_cell.reset();
            }
        }

        let width = crate::unicode::char_width(c);
        let attrs = self.attrs;

        self.grid_mut().col_wrap(width as u16);
        let cell = self.current_cell_mut();

        if width == 0 {
            if pos.col > 0 {
                let prev_cell = self
                    .cell_mut(crate::grid::Pos {
                        row: pos.row,
                        col: pos.col - 1,
                    })
                    .unwrap();
                prev_cell.append(c);
            } else if pos.row > 0 {
                let prev_row = self
                    .row(crate::grid::Pos {
                        row: pos.row - 1,
                        col: 0,
                    })
                    .unwrap();
                if prev_row.wrapped() {
                    let prev_cell = self
                        .cell_mut(crate::grid::Pos {
                            row: pos.row - 1,
                            col: self.grid().size().cols - 1,
                        })
                        .unwrap();
                    prev_cell.append(c);
                }
            }
        } else {
            cell.set(c.to_string(), attrs);
            self.grid_mut().col_inc(width as u16);
        }
    }

    // control codes

    fn bel(&mut self) {
        self.set_output(Output::AudibleBell);
    }

    fn bs(&mut self) {
        // XXX is this correct? is backwards wrapping a thing?
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
        self.grid_mut().save_pos();
    }

    // ESC 8
    fn decrc(&mut self) {
        self.grid_mut().restore_pos();
    }

    // ESC =
    fn deckpam(&mut self) {
        self.set_mode(Mode::KeypadApplication);
    }

    // ESC >
    fn deckpnm(&mut self) {
        self.clear_mode(Mode::KeypadApplication);
    }

    // ESC M
    fn ri(&mut self) {
        self.grid_mut().row_dec_scroll(1);
    }

    // ESC c
    fn ris(&mut self) {
        self.grid = self.new_grid();
        self.alternate_grid = self.new_grid();
        self.use_alternate_grid = false;
        self.attrs = crate::attrs::Attrs::default();
        self.modes = enumset::EnumSet::default();
        self.mouse_protocol_mode = MouseProtocolMode::default();
        self.mouse_protocol_encoding = MouseProtocolEncoding::default();
    }

    // ESC g
    fn vb(&mut self) {
        self.set_output(Output::VisualBell);
    }

    // csi codes

    // CSI @
    fn ich(&mut self, count: u16) {
        let pos = *self.grid().pos();
        self.grid_mut().insert_cells(pos, count);
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
        let pos = *self.grid().pos();
        match mode {
            0 => self.grid_mut().erase_all_forward(pos),
            1 => self.grid_mut().erase_all_backward(pos),
            2 => self.grid_mut().erase_all(),
            _ => {}
        }
    }

    // CSI ? J
    fn decsed(&mut self, mode: u16) {
        self.ed(mode);
    }

    // CSI K
    fn el(&mut self, mode: u16) {
        let pos = *self.grid().pos();
        match mode {
            0 => self.grid_mut().erase_row_forward(pos),
            1 => self.grid_mut().erase_row_backward(pos),
            2 => self.grid_mut().erase_row(pos),
            _ => {}
        }
    }

    // CSI ? K
    fn decsel(&mut self, mode: u16) {
        self.el(mode);
    }

    // CSI L
    fn il(&mut self, count: u16) {
        let pos = *self.grid().pos();
        self.grid_mut().insert_lines(pos, count);
    }

    // CSI M
    fn dl(&mut self, count: u16) {
        let pos = *self.grid().pos();
        self.grid_mut().delete_lines(pos, count);
    }

    // CSI P
    fn dch(&mut self, count: u16) {
        let pos = *self.grid().pos();
        self.grid_mut().delete_cells(pos, count);
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
        let pos = *self.grid().pos();
        self.grid_mut().erase_cells(pos, count);
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
                    self.alternate_grid = self.new_grid();
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
        // XXX need to handle incorrect numbers of parameters for some of the
        // fancier options
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
                        return;
                    }
                    match params[i] {
                        2 => {
                            i += 3;
                            if i >= params.len() {
                                return;
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
                                return;
                            }
                            self.attrs.fgcolor =
                                crate::color::Color::Idx(params[i] as u8);
                        }
                        _ => {}
                    }
                }
                39 => {
                    self.attrs.fgcolor = crate::color::Color::Default;
                }
                n if n >= 40 && n <= 47 => {
                    self.attrs.bgcolor =
                        crate::color::Color::Idx((n as u8) - 40);
                }
                48 => {
                    i += 1;
                    if i >= params.len() {
                        return;
                    }
                    match params[i] {
                        2 => {
                            i += 3;
                            if i >= params.len() {
                                return;
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
                                return;
                            }
                            self.attrs.bgcolor =
                                crate::color::Color::Idx(params[i] as u8);
                        }
                        _ => {}
                    }
                }
                49 => {
                    self.attrs.bgcolor = crate::color::Color::Default;
                }
                n if n >= 90 && n <= 97 => {
                    self.attrs.fgcolor =
                        crate::color::Color::Idx(n as u8 - 82);
                }
                n if n >= 100 && n <= 107 => {
                    self.attrs.bgcolor =
                        crate::color::Color::Idx(n as u8 - 92);
                }
                _ => {}
            }
            i += 1;
        }
    }

    // CSI r
    fn csr(&mut self, (top, bottom, left, right): (u16, u16, u16, u16)) {
        self.grid_mut().set_scroll_region(
            top - 1,
            bottom - 1,
            left - 1,
            right - 1,
        );
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
                'r' => self.csr(canonicalize_params_csr(
                    params,
                    *self.grid().size(),
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
        self.state.grid().size().rows
    }

    pub fn cols(&self) -> u16 {
        self.state.grid().size().cols
    }

    pub fn set_window_size(&mut self, rows: u16, cols: u16) {
        self.state.grid.set_size(crate::grid::Size { rows, cols });
        self.state
            .alternate_grid
            .set_size(crate::grid::Size { rows, cols });
    }

    pub fn process(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.parser.advance(&mut self.state, *byte);
        }
    }

    pub fn cell(&self, row: u16, col: u16) -> Option<&crate::cell::Cell> {
        self.state.cell(crate::grid::Pos { row, col })
    }

    pub fn window_contents(
        &self,
        row_start: u16,
        col_start: u16,
        row_end: u16,
        col_end: u16,
    ) -> String {
        self.state
            .grid()
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
            .grid()
            .window_contents_formatted(row_start, col_start, row_end, col_end)
    }

    pub fn cursor_position(&self) -> (u16, u16) {
        (self.state.grid().pos().row, self.state.grid().pos().col)
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

    pub fn title(&self) -> &str {
        &self.state.title
    }

    pub fn icon_name(&self) -> &str {
        &self.state.icon_name
    }

    pub fn hide_cursor(&self) -> bool {
        self.state.mode(Mode::HideCursor)
    }

    pub fn alternate_buffer_active(&self) -> bool {
        self.state.use_alternate_grid
    }

    pub fn application_cursor(&self) -> bool {
        self.state.mode(Mode::ApplicationCursor)
    }

    pub fn application_keypad(&self) -> bool {
        self.state.mode(Mode::KeypadApplication)
    }

    pub fn bracketed_paste(&self) -> bool {
        self.state.mode(Mode::BracketedPaste)
    }

    pub fn mouse_reporting_press(&self) -> bool {
        self.state.mouse_protocol_mode == MouseProtocolMode::Press
    }

    pub fn mouse_reporting_press_release(&self) -> bool {
        self.state.mouse_protocol_mode == MouseProtocolMode::PressRelease
    }

    pub fn mouse_reporting_button_motion(&self) -> bool {
        self.state.mouse_protocol_mode == MouseProtocolMode::ButtonMotion
    }

    pub fn mouse_reporting_any_motion(&self) -> bool {
        self.state.mouse_protocol_mode == MouseProtocolMode::AnyMotion
    }

    pub fn mouse_reporting_utf8_mode(&self) -> bool {
        self.state.mouse_protocol_encoding == MouseProtocolEncoding::Utf8
    }

    pub fn mouse_reporting_sgr_mode(&self) -> bool {
        self.state.mouse_protocol_encoding == MouseProtocolEncoding::Sgr
    }

    pub fn check_audible_bell(&mut self) -> bool {
        self.state.check_output(Output::AudibleBell)
    }

    pub fn check_visual_bell(&mut self) -> bool {
        self.state.check_output(Output::VisualBell)
    }
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

fn canonicalize_params_csr(
    params: &[i64],
    size: crate::grid::Size,
) -> (u16, u16, u16, u16) {
    let top = params.get(0).copied().unwrap_or(0);
    let top = if top == 0 { 1 } else { i64_to_u16(top) };

    let bottom = params.get(1).copied().unwrap_or(0);
    let bottom = if bottom == 0 {
        size.rows
    } else {
        i64_to_u16(bottom)
    };

    let left = params.get(2).copied().unwrap_or(0);
    let left = if left == 0 { 1 } else { i64_to_u16(left) };

    let right = params.get(3).copied().unwrap_or(0);
    let right = if right == 0 {
        size.cols
    } else {
        i64_to_u16(right)
    };

    (top, bottom, left, right)
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
