#![allow(clippy::cognitive_complexity)]

#[test]
fn deckpam() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    assert!(!parser.screen().application_keypad());
    parser.process(b"\x1b=");
    assert!(parser.screen().application_keypad());
    parser.process(b"\x1b>");
    assert!(!parser.screen().application_keypad());
}

#[test]
fn ri() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    parser.process(b"foo\nbar\x1bMbaz");
    assert_eq!(parser.screen().contents(), "foo   baz\n   bar");

    parser.process(b"\x1bc1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");

    parser.process(b"\x1b[H\x1bM");
    assert_eq!(parser.screen().contents(), "\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23");
}

#[test]
fn ris() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    let cell = parser.screen().cell(0, 0).unwrap();
    assert_eq!(cell.contents(), "");

    assert_eq!(parser.screen().contents(), "");
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );

    assert_eq!(parser.screen().title(), "");
    assert_eq!(parser.screen().icon_name(), "");

    assert!(!parser.screen_mut().check_visual_bell());
    assert!(!parser.screen_mut().check_audible_bell());
    assert!(!parser.screen().application_keypad());
    assert!(!parser.screen().application_cursor());
    assert!(!parser.screen().hide_cursor());
    assert!(!parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );

    parser.process(b"f\x1b[31m\x1b[47;1;3;4moo\x1b[7m\x1b[21;21H\x1b]2;window title\x07\x1b]1;window icon name\x07\x1b[?25l\x1b[?1h\x1b=\x1b[?9h\x1b[?1000h\x1b[?1006h\x1b[?2004h\x07\x1bg");

    assert_eq!(parser.screen().cursor_position(), (20, 20));

    let cell = parser.screen().cell(0, 0).unwrap();
    assert_eq!(cell.contents(), "f");

    assert_eq!(parser.screen().contents(), "foo");
    assert_eq!(
        parser.screen().contents_formatted(),
        &b"\x1b[?25l\x1b[m\x1b[H\x1b[Jf\x1b[31;47;1;3;4moo\x1b[21;21H"[..]
    );

    assert_eq!(parser.screen().title(), "window title");
    assert_eq!(parser.screen().icon_name(), "window icon name");

    assert!(parser.screen_mut().check_visual_bell());
    assert!(parser.screen_mut().check_audible_bell());
    assert!(parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );

    parser.process(b"\x07\x1bg\x1bc");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    let cell = parser.screen().cell(0, 0).unwrap();
    assert_eq!(cell.contents(), "");

    assert_eq!(parser.screen().contents(), "");
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );

    // title and icon name don't change with reset
    assert_eq!(parser.screen().title(), "window title");
    assert_eq!(parser.screen().icon_name(), "window icon name");

    // bell states don't change with reset
    assert!(parser.screen_mut().check_visual_bell());
    assert!(parser.screen_mut().check_audible_bell());

    assert!(!parser.screen().application_keypad());
    assert!(!parser.screen().application_cursor());
    assert!(!parser.screen().hide_cursor());
    assert!(!parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );
}

#[test]
fn vb() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    assert!(!parser.screen_mut().check_visual_bell());
    parser.process(b"\x1bg");
    assert!(parser.screen_mut().check_visual_bell());
    assert!(!parser.screen_mut().check_visual_bell());
}

#[test]
fn decsc() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    parser.process(b"foo\x1b7\r\n\r\n\r\n         bar\x1b8baz");
    assert_eq!(parser.screen().contents(), "foobaz\n\n\n         bar");
    assert_eq!(parser.screen().cursor_position(), (0, 6));

    parser.process(b"\x1b[?47h\x1b[20;20H");
    assert_eq!(parser.screen().contents(), "");
    assert_eq!(parser.screen().cursor_position(), (19, 19));

    parser.process(b"\x1b8");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[?47l\x1b[20;20H");
    assert_eq!(parser.screen().cursor_position(), (19, 19));

    parser.process(b"\x1b8");
    assert_eq!(parser.screen().cursor_position(), (0, 3));

    parser.process(b"\x1bc\x1b[31m\x1b[5;15r\x1b[?6hfoo\x1b7");
    assert_eq!(parser.screen().cursor_position(), (4, 3));
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;1H\x1b[31mfoo"
    );

    parser.process(b"\x1b[32m\x1b[?6lbar");
    assert_eq!(parser.screen().cursor_position(), (0, 3));
    assert_eq!(
        parser.screen().contents_formatted(),
        &b"\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[32mbar\x1b[5;1H\x1b[31mfoo\x1b[1;4H"[..]
    );

    parser.process(b"\x1b8\x1b[Hz");
    assert_eq!(parser.screen().cursor_position(), (4, 1));
    assert_eq!(
        parser.screen().contents_formatted(),
        &b"\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[32mbar\x1b[5;1H\x1b[31mzoo\x1b[5;2H"[..]
    );
}
