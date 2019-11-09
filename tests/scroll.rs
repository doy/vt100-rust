#[test]
fn scroll_regions() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    parser.process(b"\x1b[m\x1b[2J\x1b[H1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");

    parser.process(b"\x1b[24;50H\n");
    assert_eq!(parser.screen().contents(), "2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");

    parser.process(b"\x1b[m\x1b[2J\x1b[H1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");

    parser.process(b"\x1b[10;20r");
    assert_eq!(parser.screen().cursor_position(), (9, 0));

    parser.process(b"\x1b[20;50H");
    assert_eq!(parser.screen().cursor_position(), (19, 49));

    parser.process(b"\n");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n\n21\n22\n23\n24");
    assert_eq!(parser.screen().cursor_position(), (19, 49));

    parser.process(b"\x1b[B");
    assert_eq!(parser.screen().cursor_position(), (19, 49));

    parser.process(b"\x1b[20A");
    assert_eq!(parser.screen().cursor_position(), (9, 49));
    parser.process(b"\x1b[1;24r\x1b[m\x1b[2J\x1b[H1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    parser.process(b"\x1b[10;20r\x1b[15;50H\x1b[2L");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n\n\n15\n16\n17\n18\n21\n22\n23\n24");
    parser.process(b"\x1b[10;50H\x1bM");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n\n10\n11\n12\n13\n14\n\n\n15\n16\n17\n21\n22\n23\n24");

    assert_eq!(parser.screen().cursor_position(), (9, 49));
    parser.process(b"\x1b[23d");
    assert_eq!(parser.screen().cursor_position(), (22, 49));
    parser.process(b"\n");
    assert_eq!(parser.screen().cursor_position(), (23, 49));
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n\n10\n11\n12\n13\n14\n\n\n15\n16\n17\n21\n22\n23\n24");
}

#[test]
fn origin_mode() {
    let mut parser = vt100::Parser::new(24, 80, 0);

    parser.process(b"\x1b[5;15r");
    assert_eq!(parser.screen().cursor_position(), (4, 0));

    parser.process(b"\x1b[10;50H");
    assert_eq!(parser.screen().cursor_position(), (9, 49));

    parser.process(b"\x1b[?6h");
    assert_eq!(parser.screen().cursor_position(), (4, 0));

    parser.process(b"\x1b[10;50H");
    assert_eq!(parser.screen().cursor_position(), (13, 49));

    parser.process(b"\x1b[?6l");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[10;50H");
    assert_eq!(parser.screen().cursor_position(), (9, 49));

    parser.process(b"\x1b[?6h\x1b[?47h\x1b[6;16r\x1b[H");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[?6h");
    assert_eq!(parser.screen().cursor_position(), (5, 0));

    parser.process(b"\x1b[?47l\x1b[H");
    assert_eq!(parser.screen().cursor_position(), (4, 0));
}
