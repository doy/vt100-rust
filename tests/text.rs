#![allow(clippy::cognitive_complexity)]

#[test]
fn ascii() {
    let mut screen = vt100::Screen::new(24, 80);
    screen.process(b"foo");
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "f");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 2).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "foo\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.window_contents(0, 0, 500, 500),
        "foo\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
}

#[test]
fn utf8() {
    let mut screen = vt100::Screen::new(24, 80);
    screen.process("café".as_bytes());
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "c");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "a");
    assert_eq!(screen.cell(0, 2).unwrap().contents(), "f");
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "é");
    assert_eq!(screen.cell(0, 4).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "café\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.window_contents(0, 0, 500, 500),
        "café\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
}

#[test]
fn newlines() {
    let mut screen = vt100::Screen::new(24, 80);
    screen.process(b"f\r\noo\r\nood");
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "f");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "");
    assert_eq!(screen.cell(0, 2).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "o");
    assert_eq!(screen.cell(1, 1).unwrap().contents(), "o");
    assert_eq!(screen.cell(1, 2).unwrap().contents(), "");
    assert_eq!(screen.cell(2, 0).unwrap().contents(), "o");
    assert_eq!(screen.cell(2, 1).unwrap().contents(), "o");
    assert_eq!(screen.cell(2, 2).unwrap().contents(), "d");
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "");
    assert_eq!(screen.cell(3, 0).unwrap().contents(), "");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "f\noo\nood\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.window_contents(0, 0, 500, 500),
        "f\noo\nood\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
}

#[test]
fn wide() {
    let mut screen = vt100::Screen::new(24, 80);
    screen.process("aデbネ".as_bytes());
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "a");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "デ");
    assert_eq!(screen.cell(0, 2).unwrap().contents(), "");
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "b");
    assert_eq!(screen.cell(0, 4).unwrap().contents(), "ネ");
    assert_eq!(screen.cell(0, 5).unwrap().contents(), "");
    assert_eq!(screen.cell(0, 6).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "aデbネ\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.window_contents(0, 0, 500, 500),
        "aデbネ\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
}

#[test]
fn combining() {
    let mut screen = vt100::Screen::new(24, 80);
    screen.process(b"a");
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "a");
    screen.process("\u{0301}".as_bytes());
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "á");
    screen.process(b"\x1b[20;20Habcdefg");
    assert_eq!(screen.window_contents(19, 19, 19, 26), "abcdefg\n");
    screen.process("\x1b[20;25H\u{0301}".as_bytes());
    assert_eq!(screen.window_contents(19, 19, 19, 26), "abcdéfg\n");
    screen.process(b"\x1b[10;78Haaa");
    assert_eq!(screen.cell(9, 79).unwrap().contents(), "a");
    screen.process("\r\n\u{0301}".as_bytes());
    assert_eq!(screen.cell(9, 79).unwrap().contents(), "a");
    assert_eq!(screen.cell(10, 0).unwrap().contents(), "");
}

#[test]
fn wrap() {
    let mut screen = vt100::Screen::new(24, 80);
    screen.process(b"0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    screen.process(b"\x1b[5H01234567890123456789012345678901234567890123456789012345678901234567890123456789");
    screen.process(b"\x1b[6H01234567890123456789012345678901234567890123456789012345678901234567890123456789");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789\n\n\n01234567890123456789012345678901234567890123456789012345678901234567890123456789\n01234567890123456789012345678901234567890123456789012345678901234567890123456789\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    screen.process(b"\x1b[H\x1b[J");
    screen.process(b"0123456789012345678901234567890123456789012345678901234567890123456789012345678");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (0, 79));
    screen.process(b"9");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "01234567890123456789012345678901234567890123456789012345678901234567890123456789\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (0, 80));
    screen.process(b"a");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "01234567890123456789012345678901234567890123456789012345678901234567890123456789a\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (1, 1));
    screen.process(b"b");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "01234567890123456789012345678901234567890123456789012345678901234567890123456789ab\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (1, 2));

    screen.process(b"\x1b[H\x1b[J");
    screen.process(b"012345678901234567890123456789012345678901234567890123456789012345678901234567");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "012345678901234567890123456789012345678901234567890123456789012345678901234567\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (0, 78));
    screen.process("ネ".as_bytes());
    assert_eq!(screen.window_contents(0, 0, 23, 79), "012345678901234567890123456789012345678901234567890123456789012345678901234567ネ\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (0, 80));
    screen.process(b"a");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "012345678901234567890123456789012345678901234567890123456789012345678901234567ネa\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (1, 1));
    assert_eq!(screen.cell(0, 77).unwrap().contents(), "7");
    assert_eq!(screen.cell(0, 78).unwrap().contents(), "ネ");
    assert_eq!(screen.cell(0, 79).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "a");
    assert_eq!(screen.cell(1, 1).unwrap().contents(), "");

    screen.process(b"\x1b[H\x1b[J");
    screen.process(b"0123456789012345678901234567890123456789012345678901234567890123456789012345678");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (0, 79));
    screen.process("ネ".as_bytes());
    assert_eq!(screen.window_contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678ネ\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (1, 2));
    screen.process(b"a");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678ネa\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (1, 3));
    assert_eq!(screen.cell(0, 77).unwrap().contents(), "7");
    assert_eq!(screen.cell(0, 78).unwrap().contents(), "8");
    assert_eq!(screen.cell(0, 79).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "ネ");
    assert_eq!(screen.cell(1, 1).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 2).unwrap().contents(), "a");
    assert_eq!(screen.cell(1, 3).unwrap().contents(), "");
}
