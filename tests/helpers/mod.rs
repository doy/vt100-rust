mod fixtures;
pub use fixtures::fixture;
pub use fixtures::FixtureScreen;

macro_rules! is {
    ($got:expr, $expected:expr) => {
        if ($got) != ($expected) {
            eprintln!("{} != {}:", stringify!($got), stringify!($expected));
            eprintln!("     got: {:?}", $got);
            eprintln!("expected: {:?}", $expected);
            return false;
        }
    };
}
macro_rules! ok {
    ($e:expr) => {
        if !($e) {
            eprintln!("!{}", stringify!($e));
            return false;
        }
    };
}

pub fn compare_screens(
    got: &vt100::Screen,
    expected: &vt100::Screen,
) -> bool {
    is!(got.contents(), expected.contents());
    is!(got.contents_formatted(), expected.contents_formatted());
    is!(
        got.contents_diff(vt100::Parser::default().screen()),
        expected.contents_diff(vt100::Parser::default().screen())
    );

    let (rows, cols) = got.size();

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

    is!(got.title(), expected.title());
    is!(got.icon_name(), expected.icon_name());

    is!(
        got.audible_bell_count() > 0,
        expected.audible_bell_count() > 0
    );
    is!(
        got.visual_bell_count() > 0,
        expected.visual_bell_count() > 0
    );

    is!(got.application_keypad(), expected.application_keypad());
    is!(got.application_cursor(), expected.application_cursor());
    is!(got.hide_cursor(), expected.hide_cursor());
    is!(got.bracketed_paste(), expected.bracketed_paste());
    is!(got.mouse_protocol_mode(), expected.mouse_protocol_mode());
    is!(
        got.mouse_protocol_encoding(),
        expected.mouse_protocol_encoding()
    );

    true
}

#[allow(dead_code)]
pub fn contents_formatted_reproduces_state(input: &[u8]) -> bool {
    let mut parser = vt100::Parser::default();
    parser.process(input);
    contents_formatted_reproduces_screen(parser.screen())
}

pub fn contents_formatted_reproduces_screen(screen: &vt100::Screen) -> bool {
    let empty_screen = vt100::Parser::default().screen().clone();

    let mut new_input = screen.contents_formatted();
    new_input.extend(screen.input_mode_formatted());
    new_input.extend(screen.title_formatted());
    new_input.extend(screen.bells_diff(&empty_screen));
    let mut new_parser = vt100::Parser::default();
    new_parser.process(&new_input);
    let got_screen = new_parser.screen().clone();

    compare_screens(&got_screen, &screen)
}

fn assert_contents_formatted_reproduces_state(input: &[u8]) {
    assert!(contents_formatted_reproduces_state(input));
}

#[allow(dead_code)]
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
    let mut diff_input = screen.contents_diff(&prev_screen);
    diff_input.extend(screen.input_mode_diff(&prev_screen));
    diff_input.extend(screen.title_diff(&prev_screen));
    diff_input.extend(screen.bells_diff(&prev_screen));

    let mut diff_prev_input = prev_screen.contents_formatted();
    diff_prev_input.extend(screen.input_mode_formatted());
    diff_prev_input.extend(screen.title_formatted());
    diff_prev_input
        .extend(screen.bells_diff(vt100::Parser::default().screen()));

    let mut new_parser = vt100::Parser::default();
    new_parser.process(&diff_prev_input);
    new_parser.process(&diff_input);
    let got_screen = new_parser.screen().clone();

    compare_screens(&got_screen, &screen)
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn assert_reproduces_state(input: &[u8]) {
    assert_reproduces_state_from(input, &[]);
}

pub fn assert_reproduces_state_from(input: &[u8], prev_input: &[u8]) {
    let full_input: Vec<_> =
        prev_input.iter().chain(input.iter()).copied().collect();
    assert_contents_formatted_reproduces_state(&full_input);
    assert_contents_diff_reproduces_state_from(input, prev_input);
}

#[allow(dead_code)]
pub fn format_bytes(bytes: &[u8]) -> String {
    let mut v = vec![];
    for b in bytes {
        match *b {
            10 => v.extend(b"\\n"),
            13 => v.extend(b"\\r"),
            27 => v.extend(b"\\e"),
            c if c < 32 => v.extend(format!("\\x{:02x}", c).as_bytes()),
            b => v.push(b),
        }
    }
    String::from_utf8_lossy(&v).to_string()
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
