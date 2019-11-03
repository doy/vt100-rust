#![allow(clippy::cognitive_complexity)]

#[test]
fn split_escape_sequences() {
    let mut screen = vt100::Screen::new(24, 80);
    let contents = screen.contents(0, 0, 23, 79);
    screen.process(b"abc");
    assert_ne!(screen.contents(0, 0, 23, 79), contents);
    let contents = screen.contents(0, 0, 23, 79);
    screen.process(b"abc\x1b[12;24Hdef");
    assert_ne!(screen.contents(0, 0, 23, 79), contents);
    let contents = screen.contents(0, 0, 23, 79);
    assert!(contents.contains("abc"));
    assert!(contents.contains("def"));
    assert_eq!(screen.cursor_position(), (11, 26));

    screen.process(b"\x1b");
    assert_eq!(screen.cursor_position(), (11, 26));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"[");
    assert_eq!(screen.cursor_position(), (11, 26));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"1");
    assert_eq!(screen.cursor_position(), (11, 26));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"2");
    assert_eq!(screen.cursor_position(), (11, 26));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b";");
    assert_eq!(screen.cursor_position(), (11, 26));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"2");
    assert_eq!(screen.cursor_position(), (11, 26));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"4");
    assert_eq!(screen.cursor_position(), (11, 26));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"H");
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);

    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    screen.process(b"\x1b");
    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"[");
    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"?");
    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"1");
    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"0");
    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"0");
    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"0");
    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"h");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);

    assert_eq!(screen.title(), "");
    screen.process(b"\x1b");
    assert_eq!(screen.title(), "");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"]");
    assert_eq!(screen.title(), "");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"0");
    assert_eq!(screen.title(), "");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b";");
    assert_eq!(screen.title(), "");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"a");
    assert_eq!(screen.title(), "");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b" ");
    assert_eq!(screen.title(), "");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"'");
    assert_eq!(screen.title(), "");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"[");
    assert_eq!(screen.title(), "");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"]");
    assert_eq!(screen.title(), "");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"_");
    assert_eq!(screen.title(), "");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"\x07");
    assert_eq!(screen.title(), "a '[]_");
    assert_eq!(
        screen.mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(screen.cursor_position(), (11, 23));
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
}

#[test]
fn split_utf8() {
    let mut screen = vt100::Screen::new(24, 80);
    let contents = screen.contents(0, 0, 23, 79);
    screen.process(b"a");
    assert_ne!(screen.contents(0, 0, 23, 79), contents);
    let contents = screen.contents(0, 0, 23, 79);

    screen.process(b"\xc3");
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"\xa1");
    assert_ne!(screen.contents(0, 0, 23, 79), contents);
    let contents = screen.contents(0, 0, 23, 79);

    screen.process(b"\xe3");
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"\x82");
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"\xad");
    assert_ne!(screen.contents(0, 0, 23, 79), contents);
    let contents = screen.contents(0, 0, 23, 79);

    screen.process(b"\xf0");
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"\x9f");
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"\x92");
    assert_eq!(screen.contents(0, 0, 23, 79), contents);
    screen.process(b"\xa9");
    assert_ne!(screen.contents(0, 0, 23, 79), contents);
}
