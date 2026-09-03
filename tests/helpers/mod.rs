use std::num::NonZeroU16;

mod fixtures;
#[allow(unused_imports)]
pub use fixtures::FixtureScreen;
#[allow(unused_imports)]
pub use fixtures::fixture;

pub static mut QUIET: bool = false;

macro_rules! is {
    ($got:expr, $expected:expr) => {
        if ($got) != ($expected) {
            if !unsafe { QUIET } {
                eprintln!(
                    "{} != {}:",
                    stringify!($got),
                    stringify!($expected)
                );
                eprintln!("     got: {:?}", $got);
                eprintln!("expected: {:?}", $expected);
            }
            return false;
        }
    };
}
macro_rules! ok {
    ($e:expr) => {
        if !($e) {
            if !unsafe { QUIET } {
                eprintln!("!{}", stringify!($e));
            }
            return false;
        }
    };
}

/// Like [`vt100::Parser::new`], but takes [`u16`]s instead of [`NonZeroU16`]s.
///
/// Panics if `rows` or `cols` is 0.
pub fn new(rows: u16, cols: u16, scrollback_len: usize) -> vt100::Parser {
    vt100::Parser::new(
        NonZeroU16::new(rows).unwrap(),
        NonZeroU16::new(cols).unwrap(),
        scrollback_len,
    )
}

/// Like [`vt100::Parser::new_with_callbacks`], but takes [`u16`]s instead of
/// [`NonZeroU16`]s.
///
/// Panics if `rows` or `cols` is 0.
pub fn new_with_callbacks<CB: vt100::Callbacks>(
    rows: u16,
    cols: u16,
    scrollback_len: usize,
    callbacks: CB,
) -> vt100::Parser<CB> {
    vt100::Parser::new_with_callbacks(
        NonZeroU16::new(rows).unwrap(),
        NonZeroU16::new(cols).unwrap(),
        scrollback_len,
        callbacks,
    )
}

/// Like [`vt100::Screen::set_size`], but takes [`u16`]s instead of
/// [`NonZeroU16`]s.
///
/// Panics if `rows` or `cols` is 0.
pub fn set_size(screen: &mut vt100::Screen, rows: u16, cols: u16) {
    screen.set_size(
        NonZeroU16::new(rows).unwrap(),
        NonZeroU16::new(cols).unwrap(),
    );
}

/// Like [`vt100::Parser::set_size`], but takes [`u16`]s instead of
/// [`NonZeroU16`]s.
///
/// Panics if `rows` or `cols` is 0.
pub fn set_parser_size<CB: vt100::Callbacks>(
    parser: &mut vt100::Parser<CB>,
    rows: u16,
    cols: u16,
) {
    parser.set_size(
        NonZeroU16::new(rows).unwrap(),
        NonZeroU16::new(cols).unwrap(),
    );
}

/// Like [`vt100::Screen::size`], but returns plain [`u16`]s.
pub fn size(screen: &vt100::Screen) -> (u16, u16) {
    let (rows, cols) = screen.size();
    (rows.get(), cols.get())
}

#[derive(Eq, PartialEq)]
struct Str<'a>(&'a str);

impl std::fmt::Debug for Str<'_> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        f.write_str("\"")?;
        for c in self.0.chars() {
            match c {
                '\n' => f.write_str("\\n")?,
                '\r' => f.write_str("\\r")?,
                '\\' => f.write_str("\\\\")?,
                ' '..='~' => f.write_str(&c.to_string())?,
                _ => {
                    f.write_fmt(format_args!("\\u{{{:x}}}", u32::from(c)))?
                }
            }
        }
        f.write_str("\"")?;
        Ok(())
    }
}

pub fn compare_screens(
    got: &vt100::Screen,
    expected: &vt100::Screen,
) -> bool {
    let (rows, cols) = size(got);

    is!(got.contents(), expected.contents());
    is!(
        Str(&got.contents_formatted()),
        Str(&expected.contents_formatted())
    );
    for (got_row, expected_row) in
        got.rows(0, cols).zip(expected.rows(0, cols))
    {
        is!(got_row, expected_row);
    }
    for (got_row, expected_row) in got
        .rows_formatted(0, cols)
        .zip(expected.rows_formatted(0, cols))
    {
        is!(Str(&got_row), Str(&expected_row));
    }
    for i in 0..rows {
        is!(got.row_wrapped(i), expected.row_wrapped(i));
    }
    is!(
        Str(&got.contents_diff(vt100::Parser::default().screen())),
        Str(&expected.contents_diff(vt100::Parser::default().screen()))
    );

    is!(Str(&got.contents_diff(got)), Str(""));

    for row in 0..rows {
        for col in 0..cols {
            let expected_cell = expected.cell(row, col);
            let got_cell = got.cell(row, col);
            is!(got_cell, expected_cell);
        }
    }

    is!(got.cursor_position(), expected.cursor_position());
    ok!(got.cursor_position().0 <= rows);
    ok!(expected.cursor_position().0 <= rows);
    ok!(got.cursor_position().1 <= cols);
    ok!(expected.cursor_position().1 <= cols);

    is!(got.application_keypad(), expected.application_keypad());
    is!(got.application_cursor(), expected.application_cursor());
    is!(got.hide_cursor(), expected.hide_cursor());
    is!(got.bracketed_paste(), expected.bracketed_paste());
    is!(got.mouse_protocol_mode(), expected.mouse_protocol_mode());
    is!(
        got.mouse_protocol_encoding(),
        expected.mouse_protocol_encoding()
    );
    is!(got.cursor_style(), expected.cursor_style());

    true
}

pub fn contents_formatted_reproduces_state(input: &[u8]) -> bool {
    let mut parser = vt100::Parser::default();
    parser.process(input);
    contents_formatted_reproduces_screen(parser.screen())
}

pub fn rows_formatted_reproduces_state(input: &[u8]) -> bool {
    let mut parser = vt100::Parser::default();
    parser.process(input);
    rows_formatted_reproduces_screen(parser.screen())
}

pub fn contents_formatted_reproduces_screen(screen: &vt100::Screen) -> bool {
    let mut new_input = screen.contents_formatted();
    new_input.push_str(&screen.input_mode_formatted());
    assert_eq!(new_input, screen.state_formatted());
    let mut new_parser = vt100::Parser::default();
    new_parser.process(new_input.as_bytes());
    let got_screen = new_parser.screen().clone();

    compare_screens(&got_screen, screen)
}

/// Like [`contents_formatted_reproduces_screen`], but reproduces the screen
/// with a parser of the same size rather than an 80x24 one.
pub fn contents_formatted_reproduces_sized_screen(
    screen: &vt100::Screen,
) -> bool {
    let mut new_input = screen.contents_formatted();
    new_input.push_str(&screen.input_mode_formatted());
    is!(Str(&new_input), Str(&screen.state_formatted()));
    let (rows, cols) = screen.size();
    let mut new_parser = vt100::Parser::new(rows, cols, 0);
    new_parser.process(new_input.as_bytes());
    compare_screens(new_parser.screen(), screen)
}

/// Checks the invariants that should hold for any screen, whatever has been
/// done to it.
pub fn screen_is_consistent(screen: &vt100::Screen) -> bool {
    let (rows, cols) = size(screen);
    let (cursor_row, cursor_col) = screen.cursor_position();
    ok!(cursor_row < rows);
    // The cursor is allowed to sit one column past the end of the screen when
    // a wrap is pending.
    ok!(cursor_col <= cols);
    is!(screen.rows(0, cols).count(), usize::from(rows));
    // Rows on the screen are exactly as wide as the screen. Rows in the
    // scrollback aren't resized until they're pulled back onto the screen, so
    // the rows currently scrolled into view are skipped here.
    let scrolled_back =
        u16::try_from(screen.scrollback()).unwrap_or(u16::MAX);
    for row in scrolled_back.min(rows)..rows {
        ok!(screen.cell(row, cols).is_none());
    }
    ok!(screen.cell(rows, 0).is_none());
    true
}

pub fn rows_formatted_reproduces_screen(screen: &vt100::Screen) -> bool {
    let mut new_input = String::new();
    let mut wrapped = false;
    for (idx, row) in screen.rows_formatted(0, 80).enumerate() {
        new_input.push_str("\x1b[m");
        if !wrapped {
            new_input.push_str(&format!("\x1b[{}H", idx + 1));
        }
        new_input.push_str(&row);
        wrapped = screen.row_wrapped(idx.try_into().unwrap());
    }
    new_input.push_str("\x1b[m");
    new_input.push_str(&screen.cursor_state_formatted());
    new_input.push_str(&screen.attributes_formatted());
    new_input.push_str(&screen.input_mode_formatted());
    let mut new_parser = vt100::Parser::default();
    new_parser.process(new_input.as_bytes());
    let got_screen = new_parser.screen().clone();

    compare_screens(&got_screen, screen)
}

fn assert_contents_formatted_reproduces_state(input: &[u8]) {
    assert!(contents_formatted_reproduces_state(input));
}

fn assert_rows_formatted_reproduces_state(input: &[u8]) {
    assert!(rows_formatted_reproduces_state(input));
}

pub fn contents_diff_reproduces_state(input: &[u8]) -> bool {
    contents_diff_reproduces_state_from(input, &[])
}

pub fn contents_diff_reproduces_state_from(
    input: &[u8],
    prev_input: &[u8],
) -> bool {
    let mut parser = vt100::Parser::default();
    parser.process(prev_input);
    let prev_screen = parser.screen().clone();
    parser.process(input);

    contents_diff_reproduces_state_from_screens(&prev_screen, parser.screen())
}

pub fn contents_diff_reproduces_state_from_screens(
    prev_screen: &vt100::Screen,
    screen: &vt100::Screen,
) -> bool {
    let mut diff_input = screen.contents_diff(prev_screen);
    diff_input.push_str(&screen.input_mode_diff(prev_screen));
    assert_eq!(diff_input, screen.state_diff(prev_screen));

    let mut diff_prev_input = prev_screen.contents_formatted();
    diff_prev_input.push_str(&screen.input_mode_formatted());

    let mut new_parser = vt100::Parser::default();
    new_parser.process(diff_prev_input.as_bytes());
    new_parser.process(diff_input.as_bytes());
    let got_screen = new_parser.screen().clone();

    compare_screens(&got_screen, screen)
}

pub fn assert_contents_diff_reproduces_state_from_screens(
    prev_screen: &vt100::Screen,
    screen: &vt100::Screen,
) {
    assert!(contents_diff_reproduces_state_from_screens(
        prev_screen,
        screen,
    ));
}

fn assert_contents_diff_reproduces_state_from(
    input: &[u8],
    prev_input: &[u8],
) {
    assert!(contents_diff_reproduces_state_from(input, prev_input));
}

pub fn assert_reproduces_state(input: &[u8]) {
    assert_reproduces_state_from(input, &[]);
}

pub fn assert_reproduces_state_from(input: &[u8], prev_input: &[u8]) {
    let full_input: Vec<_> =
        prev_input.iter().chain(input.iter()).copied().collect();
    assert_contents_formatted_reproduces_state(&full_input);
    assert_rows_formatted_reproduces_state(&full_input);
    assert_contents_diff_reproduces_state_from(input, prev_input);
}

pub fn format_bytes(bytes: impl AsRef<[u8]>) -> String {
    let mut v = vec![];
    for b in bytes.as_ref() {
        match *b {
            10 => v.extend(b"\\n"),
            13 => v.extend(b"\\r"),
            27 => v.extend(b"\\e"),
            c if c < 32 || c == 127 => {
                v.extend(format!("\\x{c:02x}").as_bytes())
            }
            b => v.push(b),
        }
    }
    String::from_utf8_lossy(&v).into_owned()
}

fn hex_char(c: u8) -> Result<u8, String> {
    match c {
        b'0' => Ok(0),
        b'1' => Ok(1),
        b'2' => Ok(2),
        b'3' => Ok(3),
        b'4' => Ok(4),
        b'5' => Ok(5),
        b'6' => Ok(6),
        b'7' => Ok(7),
        b'8' => Ok(8),
        b'9' => Ok(9),
        b'a' => Ok(10),
        b'b' => Ok(11),
        b'c' => Ok(12),
        b'd' => Ok(13),
        b'e' => Ok(14),
        b'f' => Ok(15),
        b'A' => Ok(10),
        b'B' => Ok(11),
        b'C' => Ok(12),
        b'D' => Ok(13),
        b'E' => Ok(14),
        b'F' => Ok(15),
        _ => Err("invalid hex char".to_string()),
    }
}

pub fn hex(upper: u8, lower: u8) -> Result<u8, String> {
    Ok(hex_char(upper)? * 16 + hex_char(lower)?)
}

pub fn unhex(s: &[u8]) -> Vec<u8> {
    let mut ret = vec![];
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'\\' {
            match s[i + 1] {
                b'\\' => {
                    ret.push(b'\\');
                    i += 2;
                }
                b'x' => {
                    let upper = s[i + 2];
                    let lower = s[i + 3];
                    ret.push(hex(upper, lower).unwrap());
                    i += 4;
                }
                b'u' => {
                    assert_eq!(s[i + 2], b'{');
                    let mut digits = vec![];
                    let mut j = i + 3;
                    while s[j] != b'}' {
                        digits.push(s[j]);
                        j += 1;
                    }
                    let digits: Vec<_> = digits
                        .iter()
                        .copied()
                        .skip_while(|x| x == &b'0')
                        .collect();
                    let digits = String::from_utf8(digits).unwrap();
                    let codepoint = u32::from_str_radix(&digits, 16).unwrap();
                    let c = char::try_from(codepoint).unwrap();
                    let mut bytes = [0; 4];
                    ret.extend(c.encode_utf8(&mut bytes).bytes());
                    i = j + 1;
                }
                b'r' => {
                    ret.push(0x0d);
                    i += 2;
                }
                b'n' => {
                    ret.push(0x0a);
                    i += 2;
                }
                b't' => {
                    ret.push(0x09);
                    i += 2;
                }
                _ => panic!("invalid escape"),
            }
        } else {
            ret.push(s[i]);
            i += 1;
        }
    }
    ret
}

// Silence unused function warnings. This approach is better than annotating
// the functions with `#[allow(dead_code)]`, because that will silence dead
// code warnings within the bodies of the functions too.
#[allow(dead_code, unused_imports)]
fn allow_unused() {
    use assert_contents_diff_reproduces_state_from_screens;
    use assert_reproduces_state;
    use contents_diff_reproduces_state;
    use contents_formatted_reproduces_sized_screen;
    use format_bytes;
    use new;
    use new_with_callbacks;
    use screen_is_consistent;
    use set_parser_size;
    use set_size;
    use unhex;
}
