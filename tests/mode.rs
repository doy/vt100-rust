#![allow(clippy::cognitive_complexity)]

#[test]
fn modes() {
    let mut parser = vt100::Parser::default();
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
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1l\x1b[?2004l"
    );

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1h");

    assert!(!parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
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
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1h\x1b[?2004l"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?1h");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?9h");

    assert!(!parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
    assert!(!parser.screen().hide_cursor());
    assert!(!parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::Press
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1h\x1b[?2004l\x1b[?9h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?9h");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?25l");

    assert!(!parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(!parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::Press
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25l\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"\x1b[?25l");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1h\x1b[?2004l\x1b[?9h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1000h");

    assert!(!parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(!parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::PressRelease
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25l\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1h\x1b[?2004l\x1b[?1000h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?1000h");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1002h");

    assert!(!parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(!parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::ButtonMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25l\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1h\x1b[?2004l\x1b[?1002h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?1002h");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1003h");

    assert!(!parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(!parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::AnyMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25l\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1h\x1b[?2004l\x1b[?1003h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?1003h");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1005h");

    assert!(!parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(!parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::AnyMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Utf8
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25l\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1h\x1b[?2004l\x1b[?1003h\x1b[?1005h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?1005h");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1006h");

    assert!(!parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(!parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::AnyMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25l\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1h\x1b[?2004l\x1b[?1003h\x1b[?1006h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?1006h");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?2004h");

    assert!(!parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::AnyMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25l\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1h\x1b[?2004h\x1b[?1003h\x1b[?1006h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?2004h");

    let screen = parser.screen().clone();
    parser.process(b"\x1b=");

    assert!(parser.screen().application_keypad());
    assert!(parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::AnyMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25l\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b=\x1b[?1h\x1b[?2004h\x1b[?1003h\x1b[?1006h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b=");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1l");

    assert!(parser.screen().application_keypad());
    assert!(!parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::AnyMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25l\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b=\x1b[?1l\x1b[?2004h\x1b[?1003h\x1b[?1006h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?1l");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?9l");

    assert!(parser.screen().application_keypad());
    assert!(!parser.screen().application_cursor());
    assert!(parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::AnyMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25l\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b=\x1b[?1l\x1b[?2004h\x1b[?1003h\x1b[?1006h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?25h");

    assert!(parser.screen().application_keypad());
    assert!(!parser.screen().application_cursor());
    assert!(!parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::AnyMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"\x1b[?25h");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b=\x1b[?1l\x1b[?2004h\x1b[?1003h\x1b[?1006h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1000l");

    assert!(parser.screen().application_keypad());
    assert!(!parser.screen().application_cursor());
    assert!(!parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::AnyMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b=\x1b[?1l\x1b[?2004h\x1b[?1003h\x1b[?1006h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1002l");

    assert!(parser.screen().application_keypad());
    assert!(!parser.screen().application_cursor());
    assert!(!parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::AnyMotion
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b=\x1b[?1l\x1b[?2004h\x1b[?1003h\x1b[?1006h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1003l");

    assert!(parser.screen().application_keypad());
    assert!(!parser.screen().application_cursor());
    assert!(!parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b=\x1b[?1l\x1b[?2004h\x1b[?1006h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?1003l");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1005l");

    assert!(parser.screen().application_keypad());
    assert!(!parser.screen().application_cursor());
    assert!(!parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Sgr
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b=\x1b[?1l\x1b[?2004h\x1b[?1006h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?1006l");

    assert!(parser.screen().application_keypad());
    assert!(!parser.screen().application_cursor());
    assert!(!parser.screen().hide_cursor());
    assert!(parser.screen().bracketed_paste());
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        vt100::MouseProtocolMode::None
    );
    assert_eq!(
        parser.screen().mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b=\x1b[?1l\x1b[?2004h"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?1006l");

    let screen = parser.screen().clone();
    parser.process(b"\x1b[?2004l");

    assert!(parser.screen().application_keypad());
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
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b=\x1b[?1l\x1b[?2004l"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b[?2004l");

    let screen = parser.screen().clone();
    parser.process(b"\x1b>");

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
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J"
    );
    assert_eq!(parser.screen().contents_diff(&screen), b"");
    assert_eq!(
        parser.screen().input_mode_formatted(),
        b"\x1b>\x1b[?1l\x1b[?2004l"
    );
    assert_eq!(parser.screen().input_mode_diff(&screen), b"\x1b>");
}

#[test]
fn alternate_buffer() {
    let mut parser = vt100::Parser::default();

    // 47

    parser.process(b"\x1bc");
    assert_eq!(parser.screen().contents(), "");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[m\x1b[2J\x1b[H1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(parser.screen().cursor_position(), (23, 2));

    parser.process(b"\x1b[?47h");
    assert_eq!(parser.screen().contents(), "");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"foobar");
    assert_eq!(parser.screen().contents(), "foobar");
    assert_eq!(parser.screen().cursor_position(), (0, 6));

    parser.process(b"\x1b[?47l");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(parser.screen().cursor_position(), (23, 2));

    parser.process(b"\x1b[?47h");
    assert_eq!(parser.screen().contents(), "foobar");
    assert_eq!(parser.screen().cursor_position(), (0, 6));

    parser.process(b"\x1b[?47l");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(parser.screen().cursor_position(), (23, 2));

    // 1049

    parser.process(b"\x1bc");
    assert_eq!(parser.screen().contents(), "");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[m\x1b[2J\x1b[H1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(parser.screen().cursor_position(), (23, 2));

    parser.process(b"\x1b[?1049h");
    assert_eq!(parser.screen().contents(), "");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"foobar");
    assert_eq!(parser.screen().contents(), "foobar");
    assert_eq!(parser.screen().cursor_position(), (0, 6));

    parser.process(b"\x1b[?1049l");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(parser.screen().cursor_position(), (23, 2));

    parser.process(b"\x1b[?1049h");
    assert_eq!(parser.screen().contents(), "");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[?1049l");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(parser.screen().cursor_position(), (23, 2));
}
