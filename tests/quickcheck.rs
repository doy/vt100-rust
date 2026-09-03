use quickcheck::Arbitrary as _;

mod helpers;

#[derive(Clone, Debug)]
struct TerminalInput(Vec<u8>);

fn gen_range<T>(g: &mut quickcheck::Gen, range: std::ops::Range<T>) -> T
where
    T: Copy,
    T: quickcheck::Arbitrary,
    T: std::ops::Add<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::Sub<Output = T>,
{
    T::arbitrary(g) % (range.end - range.start) + range.start
}

impl quickcheck::Arbitrary for TerminalInput {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let size = {
            let s = g.size();
            gen_range(g, 0..s)
        };
        TerminalInput(
            (0..size)
                .flat_map(|_| choose_terminal_input_fragment(g))
                .collect(),
        )
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().map(TerminalInput))
    }
}

fn choose_terminal_input_fragment(g: &mut quickcheck::Gen) -> Vec<u8> {
    #[derive(Clone)]
    enum Fragment {
        Text,
        Control,
        Escape,
        Csi,
        #[allow(dead_code)]
        Osc,
        #[allow(dead_code)]
        Dcs,
    }

    impl quickcheck::Arbitrary for Fragment {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            match u8::arbitrary(g) {
                0u8..=231 => Fragment::Text,
                232..=239 => Fragment::Control,
                240..=247 => Fragment::Escape,
                248..=255 => Fragment::Csi,
            }
        }
    }

    match Fragment::arbitrary(g) {
        Fragment::Text => {
            let mut u: u32 = gen_range(g, 32..(2u32.pow(20) - 2048));
            // surrogates aren't valid codepoints on their own
            if u >= 0xD800 {
                u += 2048;
            }
            let c: Result<char, _> = std::convert::TryFrom::try_from(u);
            let c = match c {
                Ok(c) => c,
                Err(e) => panic!("failed to create char from {u}: {e}"),
            };
            let mut b = [0; 4];
            let s = c.encode_utf8(&mut b);
            (*s).to_string().into_bytes()
        }
        Fragment::Control => vec![gen_range(g, 7..14)],
        Fragment::Escape => {
            let mut v = vec![0x1b];
            let c = gen_range(g, b'0'..b'~');
            v.push(c);
            v
        }
        Fragment::Csi => {
            let mut v = vec![0x1b, b'['];
            // TODO: params
            let c = gen_range(g, b'@'..b'~');
            v.push(c);
            v
        }
        Fragment::Osc => {
            // TODO
            unimplemented!()
        }
        Fragment::Dcs => {
            // TODO
            unimplemented!()
        }
    }
    // TODO: sometimes add garbage in random places
}

/// A sequence of terminal input interleaved with resizes and scrollback
/// movement.
#[derive(Clone, Debug)]
struct TerminalSession {
    rows: u16,
    cols: u16,
    scrollback_len: usize,
    ops: Vec<Op>,
}

#[derive(Clone, Debug)]
enum Op {
    Input(Vec<u8>),
    /// A resize through [`vt100::Screen::set_size`].
    Resize(u16, u16),
    /// A resize through [`vt100::Parser::set_size`].
    ResizeWithCallbacks(u16, u16),
    Scrollback(usize),
}

impl quickcheck::Arbitrary for TerminalSession {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let len = gen_range(g, 1..g.size().max(2));
        Self {
            rows: gen_range(g, 1..12u16),
            cols: gen_range(g, 1..12u16),
            scrollback_len: gen_range(g, 0..8usize),
            ops: (0..len).map(|_| Op::arbitrary(g)).collect(),
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let this = self.clone();
        Box::new(self.ops.shrink().map(move |ops| Self {
            ops,
            ..this.clone()
        }))
    }
}

impl quickcheck::Arbitrary for Op {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        match gen_range(g, 0..10u8) {
            0 | 1 => Self::Resize(gen_range(g, 1..12), gen_range(g, 1..12)),
            2 | 3 => Self::ResizeWithCallbacks(
                gen_range(g, 1..12),
                gen_range(g, 1..12),
            ),
            4 => Self::Scrollback(gen_range(g, 0..10usize)),
            _ => Self::Input(choose_terminal_input_fragment(g)),
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            Self::Input(bytes) => Box::new(bytes.shrink().map(Self::Input)),
            _ => quickcheck::empty_shrinker(),
        }
    }
}

/// Counts the rows reported by `on_scroll`, to make sure the callback doesn't
/// panic and that resizes go through the callback path.
#[derive(Debug, Default)]
struct ScrollCounter(usize);

impl vt100::Callbacks for ScrollCounter {
    fn on_scroll(
        &mut self,
        contents: vt100::capture::RowContents<'_>,
        _alternate_screen: bool,
    ) {
        contents
            .write_formatted_basic(
                &mut String::new(),
                &mut Default::default(),
            )
            .unwrap();
        self.0 += 1;
    }
}

/// Resizing a screen in arbitrary ways, at arbitrary points in a stream of
/// terminal input, always leaves it in a consistent state.
fn resizes_keep_the_screen_consistent(session: TerminalSession) -> bool {
    let mut parser = helpers::new_with_callbacks(
        session.rows,
        session.cols,
        session.scrollback_len,
        ScrollCounter::default(),
    );
    for op in &session.ops {
        match op {
            Op::Input(bytes) => parser.process(bytes),
            Op::Resize(rows, cols) => {
                helpers::set_size(parser.screen_mut(), *rows, *cols);
            }
            Op::ResizeWithCallbacks(rows, cols) => {
                helpers::set_parser_size(&mut parser, *rows, *cols);
            }
            Op::Scrollback(n) => parser.screen_mut().set_scrollback(*n),
        }
        if !helpers::screen_is_consistent(parser.screen()) {
            return false;
        }
    }
    parser.screen_mut().set_scrollback(0);
    helpers::contents_formatted_reproduces_sized_screen(parser.screen())
}

#[test]
fn qc_resizes_short() {
    let mut qc = quickcheck::QuickCheck::new().tests(1_000).max_tests(1_000);
    qc.quickcheck(
        resizes_keep_the_screen_consistent as fn(TerminalSession) -> bool,
    );
}

#[test]
#[ignore]
fn qc_resizes_long() {
    let mut qc = quickcheck::QuickCheck::new()
        .tests(1_000_000)
        .max_tests(1_000_000);
    qc.quickcheck(
        resizes_keep_the_screen_consistent as fn(TerminalSession) -> bool,
    );
}

fn contents_formatted_reproduces_state_random(input: Vec<u8>) -> bool {
    helpers::contents_formatted_reproduces_state(&input)
}

fn contents_formatted_reproduces_state_structured(
    input: TerminalInput,
) -> bool {
    helpers::contents_formatted_reproduces_state(&input.0)
}

#[test]
#[ignore]
fn qc_structured_long() {
    let mut qc = quickcheck::QuickCheck::new()
        .tests(1_000_000)
        .max_tests(1_000_000);
    qc.quickcheck(
        contents_formatted_reproduces_state_structured
            as fn(TerminalInput) -> bool,
    );
}

#[test]
fn qc_structured_short() {
    let mut qc = quickcheck::QuickCheck::new().tests(1_000).max_tests(1_000);
    qc.quickcheck(
        contents_formatted_reproduces_state_structured
            as fn(TerminalInput) -> bool,
    );
}

#[test]
#[ignore]
fn qc_random_long() {
    let mut qc = quickcheck::QuickCheck::new()
        .tests(10_000_000)
        .max_tests(10_000_000);
    qc.quickcheck(
        contents_formatted_reproduces_state_random as fn(Vec<u8>) -> bool,
    );
}

#[test]
fn qc_random_short() {
    let mut qc = quickcheck::QuickCheck::new()
        .tests(10_000)
        .max_tests(10_000);
    qc.quickcheck(
        contents_formatted_reproduces_state_random as fn(Vec<u8>) -> bool,
    );
}
