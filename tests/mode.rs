#![allow(clippy::cognitive_complexity)]

#[test]
fn modes() {
    let mut screen = vt100::Screen::new(24, 80);
    assert!(!screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());

    screen.process(b"\x1b[?1h");

    assert!(!screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());

    screen.process(b"\x1b[?9h");

    assert!(!screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());

    screen.process(b"\x1b[?25l");

    assert!(screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());

    screen.process(b"\x1b[?1000h");

    assert!(screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());

    screen.process(b"\x1b[?1002h");

    assert!(screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());

    screen.process(b"\x1b[?1003h");

    assert!(screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());

    screen.process(b"\x1b[?1005h");

    assert!(screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(screen.mouse_reporting_any_motion());
    assert!(screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());

    screen.process(b"\x1b[?1006h");

    assert!(screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());

    screen.process(b"\x1b[?2004h");

    assert!(screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(screen.mouse_reporting_sgr_mode());
    assert!(screen.bracketed_paste());

    screen.process(b"\x1b=");

    assert!(screen.hide_cursor());
    assert!(screen.application_keypad());
    assert!(screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(screen.mouse_reporting_sgr_mode());
    assert!(screen.bracketed_paste());

    screen.process(b"\x1b[?1l");

    assert!(screen.hide_cursor());
    assert!(screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(screen.mouse_reporting_sgr_mode());
    assert!(screen.bracketed_paste());

    screen.process(b"\x1b[?9l");

    assert!(screen.hide_cursor());
    assert!(screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(screen.mouse_reporting_sgr_mode());
    assert!(screen.bracketed_paste());

    screen.process(b"\x1b[?25h");

    assert!(!screen.hide_cursor());
    assert!(screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(screen.mouse_reporting_sgr_mode());
    assert!(screen.bracketed_paste());

    screen.process(b"\x1b[?1000l");

    assert!(!screen.hide_cursor());
    assert!(screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(screen.mouse_reporting_sgr_mode());
    assert!(screen.bracketed_paste());

    screen.process(b"\x1b[?1002l");

    assert!(!screen.hide_cursor());
    assert!(screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(screen.mouse_reporting_sgr_mode());
    assert!(screen.bracketed_paste());

    screen.process(b"\x1b[?1003l");

    assert!(!screen.hide_cursor());
    assert!(screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(screen.mouse_reporting_sgr_mode());
    assert!(screen.bracketed_paste());

    screen.process(b"\x1b[?1005l");

    assert!(!screen.hide_cursor());
    assert!(screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(screen.mouse_reporting_sgr_mode());
    assert!(screen.bracketed_paste());

    screen.process(b"\x1b[?1006l");

    assert!(!screen.hide_cursor());
    assert!(screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(screen.bracketed_paste());

    screen.process(b"\x1b[?2004l");

    assert!(!screen.hide_cursor());
    assert!(screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());

    screen.process(b"\x1b>");

    assert!(!screen.hide_cursor());
    assert!(!screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.mouse_reporting_press());
    assert!(!screen.mouse_reporting_press_release());
    assert!(!screen.mouse_reporting_button_motion());
    assert!(!screen.mouse_reporting_any_motion());
    assert!(!screen.mouse_reporting_utf8_mode());
    assert!(!screen.mouse_reporting_sgr_mode());
    assert!(!screen.bracketed_paste());
}

#[test]
fn alternate_buffer() {
    let mut screen = vt100::Screen::new(24, 80);

    // 47

    screen.process(b"\x1bc");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(screen.cursor_position(), (0, 0));
    assert!(!screen.alternate_buffer_active());

    screen.process(b"\x1b[m\x1b[2J\x1b[H1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n");
    assert_eq!(screen.cursor_position(), (23, 2));
    assert!(!screen.alternate_buffer_active());

    screen.process(b"\x1b[?47h");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(screen.cursor_position(), (0, 0));
    assert!(screen.alternate_buffer_active());

    screen.process(b"foobar");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "foobar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(screen.cursor_position(), (0, 6));
    assert!(screen.alternate_buffer_active());

    screen.process(b"\x1b[?47l");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n");
    assert_eq!(screen.cursor_position(), (23, 2));
    assert!(!screen.alternate_buffer_active());

    screen.process(b"\x1b[?47h");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "foobar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(screen.cursor_position(), (0, 6));
    assert!(screen.alternate_buffer_active());

    screen.process(b"\x1b[?47l");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n");
    assert_eq!(screen.cursor_position(), (23, 2));
    assert!(!screen.alternate_buffer_active());

    // 1049

    screen.process(b"\x1bc");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(screen.cursor_position(), (0, 0));
    assert!(!screen.alternate_buffer_active());

    screen.process(b"\x1b[m\x1b[2J\x1b[H1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n");
    assert_eq!(screen.cursor_position(), (23, 2));
    assert!(!screen.alternate_buffer_active());

    screen.process(b"\x1b[?1049h");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(screen.cursor_position(), (0, 0));
    assert!(screen.alternate_buffer_active());

    screen.process(b"foobar");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "foobar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(screen.cursor_position(), (0, 6));
    assert!(screen.alternate_buffer_active());

    screen.process(b"\x1b[?1049l");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n");
    assert_eq!(screen.cursor_position(), (23, 2));
    assert!(!screen.alternate_buffer_active());

    screen.process(b"\x1b[?1049h");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(screen.cursor_position(), (0, 0));
    assert!(screen.alternate_buffer_active());

    screen.process(b"\x1b[?1049l");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n");
    assert_eq!(screen.cursor_position(), (23, 2));
    assert!(!screen.alternate_buffer_active());
}
