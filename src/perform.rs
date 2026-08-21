use alloc::vec::Vec;

const BASE64: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
const CLIPBOARD_SELECTOR: &[u8] = b"cpqs01234567";

impl<CB: crate::Callbacks> vte::Perform for crate::screen::WrappedScreen<CB> {
    fn print(&mut self, c: char) {
        if c == '\u{fffd}' || ('\u{80}'..'\u{a0}').contains(&c) {
            self.callbacks.unhandled_char(&mut self.screen, c);
        } else {
            self.text(c);
        }
    }

    fn execute(&mut self, b: u8) {
        match b {
            7 => self.callbacks.audible_bell(&mut self.screen),
            8 => self.bs(),
            9 => self.tab(),
            10 => self.lf(),
            11 => self.vt(),
            12 => self.ff(),
            13 => self.cr(),
            // we don't implement shift in/out alternate character sets, but
            // it shouldn't count as an "error"
            14 | 15 => {}
            _ => self.callbacks.unhandled_control(&mut self.screen, b),
        }
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], _ignore: bool, b: u8) {
        if let Some(i) = intermediates.first() {
            self.callbacks.unhandled_escape(
                &mut self.screen,
                Some(*i),
                intermediates.get(1).copied(),
                b,
            );
        } else {
            match b {
                b'7' => self.decsc(),
                b'8' => self.decrc(),
                b'=' => self.deckpam(),
                b'>' => self.deckpnm(),
                b'M' => self.ri(),
                b'c' => self.ris(),
                b'g' => self.callbacks.visual_bell(&mut self.screen),
                _ => {
                    self.callbacks.unhandled_escape(
                        &mut self.screen,
                        None,
                        None,
                        b,
                    );
                }
            }
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        _ignore: bool,
        c: char,
    ) {
        let unhandled = |screen: &mut Self| {
            screen.callbacks.unhandled_csi(
                &mut screen.screen,
                intermediates.first().copied(),
                intermediates.get(1).copied(),
                &params.iter().collect::<Vec<_>>(),
                c,
            );
        };
        match intermediates.first() {
            None => match c {
                '@' => self.ich(canonicalize_params_1(params, 1)),
                'A' => self.cuu(canonicalize_params_1(params, 1)),
                'B' => self.cud(canonicalize_params_1(params, 1)),
                'C' => self.cuf(canonicalize_params_1(params, 1)),
                'D' => self.cub(canonicalize_params_1(params, 1)),
                'E' => self.cnl(canonicalize_params_1(params, 1)),
                'F' => self.cpl(canonicalize_params_1(params, 1)),
                'G' => self.cha(canonicalize_params_1(params, 1)),
                'H' | 'f' => {
                    self.cup(canonicalize_params_2(params, 1, 1));
                }
                'J' => self.ed(canonicalize_params_1(params, 0), unhandled),
                'K' => self.el(canonicalize_params_1(params, 0), unhandled),
                'L' => self.il(canonicalize_params_1(params, 1)),
                'M' => self.dl(canonicalize_params_1(params, 1)),
                'P' => self.dch(canonicalize_params_1(params, 1)),
                'S' => self.su(canonicalize_params_1(params, 1)),
                'T' => self.sd(canonicalize_params_1(params, 1)),
                'X' => self.ech(canonicalize_params_1(params, 1)),
                '`' => self.hpa(canonicalize_params_1(params, 1)),
                'b' => self.rep(canonicalize_params_1(params, 1)),
                'd' => self.vpa(canonicalize_params_1(params, 1)),
                'm' => self.sgr(params, unhandled),
                'r' => self.decstbm(canonicalize_params_decstbm(
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
                        self.callbacks.unhandled_csi(
                            &mut self.screen,
                            None,
                            None,
                            &params.iter().collect::<Vec<_>>(),
                            c,
                        );
                    }
                }
                _ => {
                    self.callbacks.unhandled_csi(
                        &mut self.screen,
                        None,
                        None,
                        &params.iter().collect::<Vec<_>>(),
                        c,
                    );
                }
            },
            Some(b' ') => match c {
                'q' => {
                    self.decscusr(
                        canonicalize_params_1(params, 0),
                        unhandled,
                    );
                }
                _ => {
                    self.callbacks.unhandled_csi(
                        &mut self.screen,
                        None,
                        None,
                        &params.iter().collect::<Vec<_>>(),
                        c,
                    );
                }
            },
            Some(b'?') => {
                match c {
                    'J' => self
                        .decsed(canonicalize_params_1(params, 0), unhandled),
                    'K' => self
                        .decsel(canonicalize_params_1(params, 0), unhandled),
                    'h' => self.decset(params, unhandled),
                    'l' => self.decrst(params, unhandled),
                    _ => {
                        self.callbacks.unhandled_csi(
                            &mut self.screen,
                            Some(b'?'),
                            intermediates.get(1).copied(),
                            &params.iter().collect::<Vec<_>>(),
                            c,
                        );
                    }
                }
            }
            Some(i) => {
                self.callbacks.unhandled_csi(
                    &mut self.screen,
                    Some(*i),
                    intermediates.get(1).copied(),
                    &params.iter().collect::<Vec<_>>(),
                    c,
                );
            }
        }
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], _bel_terminated: bool) {
        match params {
            [b"0", s] => {
                self.callbacks.set_window_icon_name(&mut self.screen, s);
                self.callbacks.set_window_title(&mut self.screen, s);
            }
            [b"1", s] => {
                self.callbacks.set_window_icon_name(&mut self.screen, s);
            }
            [b"2", s] => {
                self.callbacks.set_window_title(&mut self.screen, s);
            }
            [b"52", ty, data] => {
                match (
                    ty.iter().all(|c| CLIPBOARD_SELECTOR.contains(c)),
                    *data,
                ) {
                    (true, b"?") => {
                        self.callbacks
                            .paste_from_clipboard(&mut self.screen, ty);
                    }
                    (true, data)
                        if data.iter().all(|c| BASE64.contains(c)) =>
                    {
                        self.callbacks.copy_to_clipboard(
                            &mut self.screen,
                            ty,
                            data,
                        );
                    }
                    _ => {
                        self.callbacks
                            .unhandled_osc(&mut self.screen, params);
                    }
                }
            }
            _ => {
                self.callbacks.unhandled_osc(&mut self.screen, params);
            }
        }
    }
}

fn canonicalize_params_1(params: &vte::Params, default: u16) -> u16 {
    let first = params.iter().next().map_or(0, |x| *x.first().unwrap_or(&0));
    if first == 0 { default } else { first }
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
