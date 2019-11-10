#[test]
fn object_creation() {
    let parser = vt100::Parser::default();
    assert_eq!(parser.screen().size(), (24, 80));
}

#[test]
fn process_text() {
    let mut parser = vt100::Parser::default();
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    parser.process(input);
    assert_eq!(parser.screen().contents(), "foobar");
}

#[test]
fn set_size() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().size(), (24, 80));
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.screen_mut().set_size(34, 8);
    assert_eq!(parser.screen().size(), (34, 8));
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[30;5H");
    assert_eq!(parser.screen().cursor_position(), (29, 4));

    parser.screen_mut().set_size(24, 80);
    assert_eq!(parser.screen().size(), (24, 80));
    assert_eq!(parser.screen().cursor_position(), (23, 4));

    parser.screen_mut().set_size(34, 8);
    assert_eq!(parser.screen().size(), (34, 8));
    assert_eq!(parser.screen().cursor_position(), (23, 4));

    parser.process(b"\x1b[?1049h");
    assert_eq!(parser.screen().size(), (34, 8));
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.screen_mut().set_size(24, 80);
    assert_eq!(parser.screen().size(), (24, 80));
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[?1049l");
    assert_eq!(parser.screen().size(), (24, 80));
    assert_eq!(parser.screen().cursor_position(), (23, 4));

    parser.screen_mut().set_size(34, 8);
    parser.process(b"\x1bc01234567890123456789");
    assert_eq!(parser.screen().contents(), "01234567890123456789");

    parser.screen_mut().set_size(24, 80);
    assert_eq!(parser.screen().contents(), "01234567\n89012345\n6789");

    parser.screen_mut().set_size(34, 8);
    assert_eq!(parser.screen().contents(), "01234567\n89012345\n6789");
}

#[test]
fn cell_contents() {
    let mut parser = vt100::Parser::default();
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    parser.process(input);
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "f");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "b");
    assert_eq!(parser.screen().cell(0, 4).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(0, 5).unwrap().contents(), "r");
    assert_eq!(parser.screen().cell(0, 6).unwrap().contents(), "");
}

#[test]
fn cell_colors() {
    let mut parser = vt100::Parser::default();
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    parser.process(input);

    assert_eq!(
        parser.screen().cell(0, 0).unwrap().fgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(0, 3).unwrap().fgcolor(),
        vt100::Color::Idx(2)
    );
    assert_eq!(
        parser.screen().cell(0, 4).unwrap().fgcolor(),
        vt100::Color::Idx(2)
    );
    assert_eq!(
        parser.screen().cell(0, 4).unwrap().bgcolor(),
        vt100::Color::Idx(2)
    );
}

#[test]
fn cell_attrs() {
    let mut parser = vt100::Parser::default();
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    parser.process(input);

    assert!(parser.screen().cell(0, 4).unwrap().italic());
}
