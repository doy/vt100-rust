#[test]
fn object_creation() {
    let screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.size(), (24, 80));
}

#[test]
fn process_text() {
    let mut screen = vt100::Screen::new(24, 80);
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    screen.process(input);
    assert_eq!(screen.contents(0, 0, 0, 50), "foobar");
}

#[test]
fn set_size() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.size(), (24, 80));
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.set_size(34, 8);
    assert_eq!(screen.size(), (34, 8));
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"\x1b[30;5H");
    assert_eq!(screen.cursor_position(), (29, 4));

    screen.set_size(24, 80);
    assert_eq!(screen.size(), (24, 80));
    assert_eq!(screen.cursor_position(), (23, 4));

    screen.set_size(34, 8);
    assert_eq!(screen.size(), (34, 8));
    assert_eq!(screen.cursor_position(), (23, 4));

    screen.process(b"\x1b[?1049h");
    assert_eq!(screen.size(), (34, 8));
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.set_size(24, 80);
    assert_eq!(screen.size(), (24, 80));
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"\x1b[?1049l");
    assert_eq!(screen.size(), (24, 80));
    assert_eq!(screen.cursor_position(), (23, 4));
}

#[test]
fn cell_contents() {
    let mut screen = vt100::Screen::new(24, 80);
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    screen.process(input);
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "f");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 2).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "b");
    assert_eq!(screen.cell(0, 4).unwrap().contents(), "a");
    assert_eq!(screen.cell(0, 5).unwrap().contents(), "r");
    assert_eq!(screen.cell(0, 6).unwrap().contents(), "");
}

#[test]
fn cell_colors() {
    let mut screen = vt100::Screen::new(24, 80);
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    screen.process(input);

    assert_eq!(screen.cell(0, 0).unwrap().fgcolor(), vt100::Color::Default);
    assert_eq!(screen.cell(0, 3).unwrap().fgcolor(), vt100::Color::Idx(2));
    assert_eq!(screen.cell(0, 4).unwrap().fgcolor(), vt100::Color::Idx(2));
    assert_eq!(screen.cell(0, 4).unwrap().bgcolor(), vt100::Color::Idx(2));
}

#[test]
fn cell_attrs() {
    let mut screen = vt100::Screen::new(24, 80);
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    screen.process(input);

    assert!(screen.cell(0, 4).unwrap().italic());
}
