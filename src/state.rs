pub struct State<'a, T: crate::callbacks::Callbacks> {
    screen: &'a mut crate::perform::WrappedScreen,
    callbacks: &'a mut T,
}

impl<'a, T: crate::callbacks::Callbacks> State<'a, T> {
    pub fn new(
        screen: &'a mut crate::perform::WrappedScreen,
        callbacks: &'a mut T,
    ) -> Self {
        Self { screen, callbacks }
    }
}

impl<'a, T: crate::callbacks::Callbacks> vte::Perform for State<'a, T> {
    fn print(&mut self, c: char) {
        if c == '\u{fffd}' || ('\u{80}'..'\u{a0}').contains(&c) {
            self.callbacks.error(&mut self.screen.0);
        }
        self.screen.print(c);
    }

    fn execute(&mut self, b: u8) {
        match b {
            7 => self.callbacks.audible_bell(&mut self.screen.0),
            8..=15 => {}
            _ => {
                self.callbacks.error(&mut self.screen.0);
            }
        }
        self.screen.execute(b);
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, b: u8) {
        if intermediates.is_empty() && b == b'g' {
            self.callbacks.visual_bell(&mut self.screen.0);
        }
        self.screen.esc_dispatch(intermediates, ignore, b);
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        ignore: bool,
        c: char,
    ) {
        if intermediates.first().is_none() && c == 't' {
            let mut iter = params.iter();
            let op = iter.next().and_then(|x| x.first().copied());
            if op == Some(8) {
                let (screen_rows, screen_cols) = self.screen.0.size();
                let rows = iter.next().map_or(screen_rows, |x| {
                    *x.first().unwrap_or(&screen_rows)
                });
                let cols = iter.next().map_or(screen_cols, |x| {
                    *x.first().unwrap_or(&screen_cols)
                });
                self.callbacks.resize(&mut self.screen.0, (rows, cols));
            }
        }
        self.screen.csi_dispatch(params, intermediates, ignore, c);
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bel_terminated: bool) {
        self.screen.osc_dispatch(params, bel_terminated);
    }

    fn hook(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        ignore: bool,
        action: char,
    ) {
        self.screen.hook(params, intermediates, ignore, action);
    }
}
