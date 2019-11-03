#[test]
fn intermediate_control() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"\x1b");
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"[");
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"\n");
    assert_eq!(screen.cursor_position(), (1, 0));

    screen.process(b"C");
    assert_eq!(screen.cursor_position(), (1, 1));
}
