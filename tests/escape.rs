#![allow(clippy::cognitive_complexity)]

#[test]
fn deckpam() {
    let mut screen = vt100::Screen::new(24, 80);
    assert!(!screen.application_keypad());
    screen.process(b"\x1b=");
    assert!(screen.application_keypad());
    screen.process(b"\x1b>");
    assert!(!screen.application_keypad());
}

#[test]
fn ri() {
    let mut screen = vt100::Screen::new(24, 80);
    screen.process(b"foo\nbar\x1bMbaz");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "foo   baz\n   bar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
}

#[test]
fn ris() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.cursor_position(), (0, 0));

    let cell = screen.cell(0, 0).unwrap();
    assert_eq!(cell.contents(), "");

    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.contents_formatted(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );

    assert_eq!(screen.title(), "");
    assert_eq!(screen.icon_name(), "");

    assert_eq!(screen.fgcolor(), vt100::Color::Default);
    assert_eq!(screen.bgcolor(), vt100::Color::Default);

    assert!(!screen.bold());
    assert!(!screen.italic());
    assert!(!screen.underline());
    assert!(!screen.inverse());

    assert!(!screen.check_visual_bell());
    assert!(!screen.check_audible_bell());
    assert!(!screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.hide_cursor());
    assert!(!screen.bracketed_paste());
    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    assert_eq!(
        screen.mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );

    screen.process(b"f\x1b[31m\x1b[47;1;3;4moo\x1b[7m\x1b[21;21H\x1b]2;window title\x07\x1b]1;window icon name\x07\x1b[?25l\x1b[?1h\x1b=\x1b[?9h\x1b[?1000h\x1b[?1006h\x1b[?2004h\x07\x1bg");

    assert_eq!(screen.cursor_position(), (20, 20));

    let cell = screen.cell(0, 0).unwrap();
    assert_eq!(cell.contents(), "f");

    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "foo\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(screen.contents_formatted(0, 0, 23, 79), "f\x1b[31;47;1;3;4moo\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    assert_eq!(screen.title(), "window title");
    assert_eq!(screen.icon_name(), "window icon name");

    assert_eq!(screen.fgcolor(), vt100::Color::Idx(1));
    assert_eq!(screen.bgcolor(), vt100::Color::Idx(7));

    assert!(screen.bold());
    assert!(screen.italic());
    assert!(screen.underline());
    assert!(screen.inverse());

    assert!(screen.check_visual_bell());
    assert!(screen.check_audible_bell());
    assert!(screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(screen.hide_cursor());
    assert!(screen.bracketed_paste());
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(
        screen.mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );

    screen.process(b"\x1bc");
    assert_eq!(screen.cursor_position(), (0, 0));

    let cell = screen.cell(0, 0).unwrap();
    assert_eq!(cell.contents(), "");

    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.contents_formatted(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );

    // title and icon name don't change with reset
    assert_eq!(screen.title(), "window title");
    assert_eq!(screen.icon_name(), "window icon name");

    assert_eq!(screen.fgcolor(), vt100::Color::Default);
    assert_eq!(screen.bgcolor(), vt100::Color::Default);

    assert!(!screen.bold());
    assert!(!screen.italic());
    assert!(!screen.underline());
    assert!(!screen.inverse());

    assert!(!screen.check_visual_bell());
    assert!(!screen.check_audible_bell());
    assert!(!screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.hide_cursor());
    assert!(!screen.bracketed_paste());
    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    assert_eq!(
        screen.mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );
}

#[test]
fn vb() {
    let mut screen = vt100::Screen::new(24, 80);
    assert!(!screen.check_visual_bell());
    screen.process(b"\x1bg");
    assert!(screen.check_visual_bell());
    assert!(!screen.check_visual_bell());
}

#[test]
fn decsc() {
    let mut screen = vt100::Screen::new(24, 80);
    screen.process(b"foo\x1b7\r\n\r\n\r\n         bar\x1b8baz");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "foobaz\n\n\n         bar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
}
