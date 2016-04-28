extern crate vt100;

#[test]
fn split_escape_sequences() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.process(b"abc"), 3);
    assert_eq!(screen.process(b"abc\x1b[12;24Hdef"), 14);

    assert_eq!(screen.process(b"\x1b"), 0);
    assert_eq!(screen.process(b"\x1b["), 0);
    assert_eq!(screen.process(b"\x1b[1"), 0);
    assert_eq!(screen.process(b"\x1b[12"), 0);
    assert_eq!(screen.process(b"\x1b[12;"), 0);
    assert_eq!(screen.process(b"\x1b[12;2"), 0);
    assert_eq!(screen.process(b"\x1b[12;24"), 0);
    assert_eq!(screen.process(b"\x1b[12;24H"), 8);

    assert_eq!(screen.process(b"abc\x1b"), 3);
    assert_eq!(screen.process(b"abc\x1b["), 3);
    assert_eq!(screen.process(b"abc\x1b[1"), 3);
    assert_eq!(screen.process(b"abc\x1b[12"), 3);
    assert_eq!(screen.process(b"abc\x1b[12;"), 3);
    assert_eq!(screen.process(b"abc\x1b[12;2"), 3);
    assert_eq!(screen.process(b"abc\x1b[12;24"), 3);
    assert_eq!(screen.process(b"abc\x1b[12;24H"), 11);

    assert_eq!(screen.process(b"\x1b"), 0);
    assert_eq!(screen.process(b"\x1b["), 0);
    assert_eq!(screen.process(b"\x1b[?"), 0);
    assert_eq!(screen.process(b"\x1b[?1"), 0);
    assert_eq!(screen.process(b"\x1b[?10"), 0);
    assert_eq!(screen.process(b"\x1b[?100"), 0);
    assert_eq!(screen.process(b"\x1b[?1000"), 0);
    assert_eq!(screen.process(b"\x1b[?1000h"), 8);

    assert_eq!(screen.process(b"\x1b]"), 0);
    assert_eq!(screen.process(b"\x1b]4"), 0);
    assert_eq!(screen.process(b"\x1b]49"), 0);
    assert_eq!(screen.process(b"\x1b]499"), 0);
    assert_eq!(screen.process(b"\x1b]499;"), 0);
    assert_eq!(screen.process(b"\x1b]499;a"), 0);
    assert_eq!(screen.process(b"\x1b]499;a "), 0);
    assert_eq!(screen.process(b"\x1b]499;a '"), 0);
    assert_eq!(screen.process(b"\x1b]499;a '["), 0);
    assert_eq!(screen.process(b"\x1b]499;a '[]"), 0);
    assert_eq!(screen.process(b"\x1b]499;a '[]_"), 0);
    assert_eq!(screen.process(b"\x1b]499;a '[]_\x07"), 13);
}

#[test]
fn split_utf8() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.process(b"a"), 1);

    assert_eq!(screen.process(b"\xc3"), 0);
    assert_eq!(screen.process(b"\xc3\xa1"), 2);

    assert_eq!(screen.process(b"\xe3"), 0);
    assert_eq!(screen.process(b"\xe3\x82"), 0);
    assert_eq!(screen.process(b"\xe3\x82\xad"), 3);

    assert_eq!(screen.process(b"\xf0"), 0);
    assert_eq!(screen.process(b"\xf0\x9f"), 0);
    assert_eq!(screen.process(b"\xf0\x9f\x92"), 0);
    assert_eq!(screen.process(b"\xf0\x9f\x92\xa9"), 4);
}
