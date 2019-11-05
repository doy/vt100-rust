#![allow(clippy::cognitive_complexity)]

#[test]
fn split_escape_sequences() {
    let mut parser = vt100::Parser::new(24, 80);
    let contents = parser.screen().contents();
    parser.process(b"abc");
    assert_ne!(parser.screen().contents(), contents);
    let contents = parser.screen().contents();
    parser.process(b"abc\x1b[12;24Hdef");
    assert_ne!(parser.screen().contents(), contents);
    let contents = parser.screen().contents();
    assert!(contents.contains("abc"));
    assert!(contents.contains("def"));
    assert_eq!(parser.screen().cursor_position(), (11, 26));

    parser.process(b"\x1b");
    assert_eq!(parser.screen().cursor_position(), (11, 26));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"[");
    assert_eq!(parser.screen().cursor_position(), (11, 26));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"1");
    assert_eq!(parser.screen().cursor_position(), (11, 26));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"2");
    assert_eq!(parser.screen().cursor_position(), (11, 26));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b";");
    assert_eq!(parser.screen().cursor_position(), (11, 26));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"2");
    assert_eq!(parser.screen().cursor_position(), (11, 26));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"4");
    assert_eq!(parser.screen().cursor_position(), (11, 26));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"H");
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);

    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    parser.process(b"\x1b");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"[");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"?");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"1");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"0");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"0");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"0");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"h");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);

    assert_eq!(parser.screen().title(), "");
    parser.process(b"\x1b");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"]");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"0");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b";");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"a");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b" ");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"'");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"[");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"]");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"_");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"\x07");
    assert_eq!(parser.screen().title(), "a '[]_");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(parser.screen().cursor_position(), (11, 23));
    assert_eq!(parser.screen().contents(), contents);
}

#[test]
fn split_utf8() {
    let mut parser = vt100::Parser::new(24, 80);
    let contents = parser.screen().contents();
    parser.process(b"a");
    assert_ne!(parser.screen().contents(), contents);
    let contents = parser.screen().contents();

    parser.process(b"\xc3");
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"\xa1");
    assert_ne!(parser.screen().contents(), contents);
    let contents = parser.screen().contents();

    parser.process(b"\xe3");
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"\x82");
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"\xad");
    assert_ne!(parser.screen().contents(), contents);
    let contents = parser.screen().contents();

    parser.process(b"\xf0");
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"\x9f");
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"\x92");
    assert_eq!(parser.screen().contents(), contents);
    parser.process(b"\xa9");
    assert_ne!(parser.screen().contents(), contents);
}
