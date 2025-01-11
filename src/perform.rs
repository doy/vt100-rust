pub struct WrappedScreen<Callbacks: crate::callbacks::Callbacks = ()> {
    pub screen: crate::screen::Screen,
    pub callbacks: Callbacks,
}

impl WrappedScreen<()> {
    pub fn new(rows: u16, cols: u16, scrollback_len: usize) -> Self {
        Self::new_with_callbacks(rows, cols, scrollback_len, ())
    }
}

impl<Callbacks: crate::callbacks::Callbacks> WrappedScreen<Callbacks> {
    pub fn new_with_callbacks(
        rows: u16,
        cols: u16,
        scrollback_len: usize,
        callbacks: Callbacks,
    ) -> Self {
        Self {
            screen: crate::screen::Screen::new(
                crate::grid::Size { rows, cols },
                scrollback_len,
            ),
            callbacks,
        }
    }
}

impl<Callbacks: crate::callbacks::Callbacks> vte::Perform
    for WrappedScreen<Callbacks>
{
    fn print(&mut self, c: char) {
        if c == '\u{fffd}' || ('\u{80}'..'\u{a0}').contains(&c) {
            self.callbacks.error(&mut self.screen);
            log::debug!("unhandled text character: {c}");
        }
        self.screen.text(c);
    }

    fn execute(&mut self, b: u8) {
        match b {
            7 => self.callbacks.audible_bell(&mut self.screen),
            8 => self.screen.bs(),
            9 => self.screen.tab(),
            10 => self.screen.lf(),
            11 => self.screen.vt(),
            12 => self.screen.ff(),
            13 => self.screen.cr(),
            // we don't implement shift in/out alternate character sets, but
            // it shouldn't count as an "error"
            14 | 15 => {}
            _ => {
                self.callbacks.error(&mut self.screen);
                log::debug!("unhandled control character: {b}");
            }
        }
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], _ignore: bool, b: u8) {
        intermediates.first().map_or_else(
            || match b {
                b'7' => self.screen.decsc(),
                b'8' => self.screen.decrc(),
                b'=' => self.screen.deckpam(),
                b'>' => self.screen.deckpnm(),
                b'M' => self.screen.ri(),
                b'c' => self.screen.ris(),
                b'g' => self.callbacks.visual_bell(&mut self.screen),
                _ => {
                    log::debug!("unhandled escape code: ESC {b}");
                }
            },
            |i| {
                log::debug!("unhandled escape code: ESC {i} {b}");
            },
        );
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        _ignore: bool,
        c: char,
    ) {
        match intermediates.first() {
            None => match c {
                '@' => self.screen.ich(canonicalize_params_1(params, 1)),
                'A' => self.screen.cuu(canonicalize_params_1(params, 1)),
                'B' => self.screen.cud(canonicalize_params_1(params, 1)),
                'C' => self.screen.cuf(canonicalize_params_1(params, 1)),
                'D' => self.screen.cub(canonicalize_params_1(params, 1)),
                'E' => self.screen.cnl(canonicalize_params_1(params, 1)),
                'F' => self.screen.cpl(canonicalize_params_1(params, 1)),
                'G' => self.screen.cha(canonicalize_params_1(params, 1)),
                'H' => self.screen.cup(canonicalize_params_2(params, 1, 1)),
                'J' => self.screen.ed(canonicalize_params_1(params, 0)),
                'K' => self.screen.el(canonicalize_params_1(params, 0)),
                'L' => self.screen.il(canonicalize_params_1(params, 1)),
                'M' => self.screen.dl(canonicalize_params_1(params, 1)),
                'P' => self.screen.dch(canonicalize_params_1(params, 1)),
                'S' => self.screen.su(canonicalize_params_1(params, 1)),
                'T' => self.screen.sd(canonicalize_params_1(params, 1)),
                'X' => self.screen.ech(canonicalize_params_1(params, 1)),
                'd' => self.screen.vpa(canonicalize_params_1(params, 1)),
                'h' => self.screen.sm(params),
                'l' => self.screen.rm(params),
                'm' => self.screen.sgr(params),
                'r' => self.screen.decstbm(canonicalize_params_decstbm(
                    params,
                    self.screen.grid().size(),
                )),
                't' => {
                    let mut params_iter = params.iter();
                    let op =
                        params_iter.next().and_then(|x| x.first().copied());
                    if op == Some(8) {
                        let (screen_rows, screen_cols) = self.screen.size();
                        let rows =
                            params_iter.next().map_or(screen_rows, |x| {
                                *x.first().unwrap_or(&screen_rows)
                            });
                        let cols =
                            params_iter.next().map_or(screen_cols, |x| {
                                *x.first().unwrap_or(&screen_cols)
                            });
                        self.callbacks.resize(&mut self.screen, (rows, cols));
                    } else {
                        log::debug!(
                            "unhandled XTWINOPS: {}",
                            param_str(params)
                        );
                    }
                }
                _ => {
                    if log::log_enabled!(log::Level::Debug) {
                        log::debug!(
                            "unhandled csi sequence: CSI {} {}",
                            param_str(params),
                            c
                        );
                    }
                }
            },
            Some(b'?') => match c {
                'J' => self.screen.decsed(canonicalize_params_1(params, 0)),
                'K' => self.screen.decsel(canonicalize_params_1(params, 0)),
                'h' => self.screen.decset(params),
                'l' => self.screen.decrst(params),
                _ => {
                    if log::log_enabled!(log::Level::Debug) {
                        log::debug!(
                            "unhandled csi sequence: CSI ? {} {}",
                            param_str(params),
                            c
                        );
                    }
                }
            },
            Some(i) => {
                if log::log_enabled!(log::Level::Debug) {
                    log::debug!(
                        "unhandled csi sequence: CSI {} {} {}",
                        i,
                        param_str(params),
                        c
                    );
                }
            }
        }
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], _bel_terminated: bool) {
        match params {
            [b"0", s, ..] => self.screen.osc0(s),
            [b"1", s, ..] => self.screen.osc1(s),
            [b"2", s, ..] => self.screen.osc2(s),
            _ => {
                if log::log_enabled!(log::Level::Debug) {
                    log::debug!(
                        "unhandled osc sequence: OSC {}",
                        osc_param_str(params),
                    );
                }
            }
        }
    }

    fn hook(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        if log::log_enabled!(log::Level::Debug) {
            intermediates.first().map_or_else(
                || {
                    log::debug!(
                        "unhandled dcs sequence: DCS {} {}",
                        param_str(params),
                        action,
                    );
                },
                |i| {
                    log::debug!(
                        "unhandled dcs sequence: DCS {} {} {}",
                        i,
                        param_str(params),
                        action,
                    );
                },
            );
        }
    }
}

fn canonicalize_params_1(params: &vte::Params, default: u16) -> u16 {
    let first = params.iter().next().map_or(0, |x| *x.first().unwrap_or(&0));
    if first == 0 {
        default
    } else {
        first
    }
}

fn canonicalize_params_2(
    params: &vte::Params,
    default1: u16,
    default2: u16,
) -> (u16, u16) {
    let mut iter = params.iter();
    let first = iter.next().map_or(0, |x| *x.first().unwrap_or(&0));
    let first = if first == 0 { default1 } else { first };

    let second = iter.next().map_or(0, |x| *x.first().unwrap_or(&0));
    let second = if second == 0 { default2 } else { second };

    (first, second)
}

fn canonicalize_params_decstbm(
    params: &vte::Params,
    size: crate::grid::Size,
) -> (u16, u16) {
    let mut iter = params.iter();
    let top = iter.next().map_or(0, |x| *x.first().unwrap_or(&0));
    let top = if top == 0 { 1 } else { top };

    let bottom = iter.next().map_or(0, |x| *x.first().unwrap_or(&0));
    let bottom = if bottom == 0 { size.rows } else { bottom };

    (top, bottom)
}

pub fn param_str(params: &vte::Params) -> String {
    let strs: Vec<_> = params
        .iter()
        .map(|subparams| {
            let subparam_strs: Vec<_> = subparams
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
            subparam_strs.join(" : ")
        })
        .collect();
    strs.join(" ; ")
}

fn osc_param_str(params: &[&[u8]]) -> String {
    let strs: Vec<_> = params
        .iter()
        .map(|b| format!("\"{}\"", std::string::String::from_utf8_lossy(b)))
        .collect();
    strs.join(" ; ")
}
