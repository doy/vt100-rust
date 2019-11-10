#[test]
fn intermediate_control() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"[");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\n");
    assert_eq!(parser.screen().cursor_position(), (1, 0));

    parser.process(b"C");
    assert_eq!(parser.screen().cursor_position(), (1, 1));
}
