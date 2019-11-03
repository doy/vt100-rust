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
        screen.contents(0, 0, 23, 79),
        "foo\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.contents(0, 0, 500, 500),
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
        screen.contents(0, 0, 23, 79),
        "café\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.contents(0, 0, 500, 500),
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
        screen.contents(0, 0, 23, 79),
        "f\noo\nood\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.contents(0, 0, 500, 500),
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
        screen.contents(0, 0, 23, 79),
        "aデbネ\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.contents(0, 0, 500, 500),
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
    assert_eq!(screen.contents(19, 19, 19, 26), "abcdefg\n");
    screen.process("\x1b[20;25H\u{0301}".as_bytes());
    assert_eq!(screen.contents(19, 19, 19, 26), "abcdéfg\n");
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
    assert_eq!(screen.contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    screen.process(b"\x1b[5H01234567890123456789012345678901234567890123456789012345678901234567890123456789");
    screen.process(b"\x1b[6H01234567890123456789012345678901234567890123456789012345678901234567890123456789");
    assert_eq!(screen.contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789\n\n\n01234567890123456789012345678901234567890123456789012345678901234567890123456789\n01234567890123456789012345678901234567890123456789012345678901234567890123456789\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    screen.process(b"\x1b[H\x1b[J");
    screen.process(b"0123456789012345678901234567890123456789012345678901234567890123456789012345678");
    assert_eq!(screen.contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (0, 79));
    screen.process(b"9");
    assert_eq!(screen.contents(0, 0, 23, 79), "01234567890123456789012345678901234567890123456789012345678901234567890123456789\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (0, 80));
    screen.process(b"a");
    assert_eq!(screen.contents(0, 0, 23, 79), "01234567890123456789012345678901234567890123456789012345678901234567890123456789a\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (1, 1));
    screen.process(b"b");
    assert_eq!(screen.contents(0, 0, 23, 79), "01234567890123456789012345678901234567890123456789012345678901234567890123456789ab\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (1, 2));

    screen.process(b"\x1b[H\x1b[J");
    screen.process(b"012345678901234567890123456789012345678901234567890123456789012345678901234567");
    assert_eq!(screen.contents(0, 0, 23, 79), "012345678901234567890123456789012345678901234567890123456789012345678901234567\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (0, 78));
    screen.process("ネ".as_bytes());
    assert_eq!(screen.contents(0, 0, 23, 79), "012345678901234567890123456789012345678901234567890123456789012345678901234567ネ\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (0, 80));
    screen.process(b"a");
    assert_eq!(screen.contents(0, 0, 23, 79), "012345678901234567890123456789012345678901234567890123456789012345678901234567ネa\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (1, 1));
    assert_eq!(screen.cell(0, 77).unwrap().contents(), "7");
    assert_eq!(screen.cell(0, 78).unwrap().contents(), "ネ");
    assert_eq!(screen.cell(0, 79).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "a");
    assert_eq!(screen.cell(1, 1).unwrap().contents(), "");

    screen.process(b"\x1b[H\x1b[J");
    screen.process(b"0123456789012345678901234567890123456789012345678901234567890123456789012345678");
    assert_eq!(screen.contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (0, 79));
    screen.process("ネ".as_bytes());
    assert_eq!(screen.contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678ネ\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (1, 2));
    screen.process(b"a");
    assert_eq!(screen.contents(0, 0, 23, 79), "0123456789012345678901234567890123456789012345678901234567890123456789012345678ネa\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(screen.cursor_position(), (1, 3));
    assert_eq!(screen.cell(0, 77).unwrap().contents(), "7");
    assert_eq!(screen.cell(0, 78).unwrap().contents(), "8");
    assert_eq!(screen.cell(0, 79).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "ネ");
    assert_eq!(screen.cell(1, 1).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 2).unwrap().contents(), "a");
    assert_eq!(screen.cell(1, 3).unwrap().contents(), "");
}

#[test]
fn soft_hyphen() {
    let mut screen = vt100::Screen::new(24, 140);
    screen.process(b"Free En\xc2\xadter\xc2\xadprise is gonna ru\xc2\xadin ev\xc2\xadery\xc2\xadthing good un\xc2\xadless we take a knife to its tes\xc2\xadti\xc2\xadcles first.");
    assert_eq!(screen.contents(0, 0, 0, 139), "Free En\u{00ad}ter\u{00ad}prise is gonna ru\u{00ad}in ev\u{00ad}ery\u{00ad}thing good un\u{00ad}less we take a knife to its tes\u{00ad}ti\u{00ad}cles first.\n");
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "F");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "r");
    assert_eq!(screen.cell(0, 2).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 4).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 5).unwrap().contents(), "E");
    assert_eq!(screen.cell(0, 6).unwrap().contents(), "n\u{00ad}");
    assert_eq!(screen.cell(0, 7).unwrap().contents(), "t");
    assert_eq!(screen.cell(0, 8).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 9).unwrap().contents(), "r\u{00ad}");
    assert_eq!(screen.cell(0, 10).unwrap().contents(), "p");
    assert_eq!(screen.cell(0, 11).unwrap().contents(), "r");
    assert_eq!(screen.cell(0, 12).unwrap().contents(), "i");
    assert_eq!(screen.cell(0, 13).unwrap().contents(), "s");
    assert_eq!(screen.cell(0, 14).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 15).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 16).unwrap().contents(), "i");
    assert_eq!(screen.cell(0, 17).unwrap().contents(), "s");
    assert_eq!(screen.cell(0, 18).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 19).unwrap().contents(), "g");
    assert_eq!(screen.cell(0, 20).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 21).unwrap().contents(), "n");
    assert_eq!(screen.cell(0, 22).unwrap().contents(), "n");
    assert_eq!(screen.cell(0, 23).unwrap().contents(), "a");
    assert_eq!(screen.cell(0, 24).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 25).unwrap().contents(), "r");
    assert_eq!(screen.cell(0, 26).unwrap().contents(), "u\u{00ad}");
    assert_eq!(screen.cell(0, 27).unwrap().contents(), "i");
    assert_eq!(screen.cell(0, 28).unwrap().contents(), "n");
    assert_eq!(screen.cell(0, 29).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 30).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 31).unwrap().contents(), "v\u{00ad}");
    assert_eq!(screen.cell(0, 32).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 33).unwrap().contents(), "r");
    assert_eq!(screen.cell(0, 34).unwrap().contents(), "y\u{00ad}");
    assert_eq!(screen.cell(0, 35).unwrap().contents(), "t");
    assert_eq!(screen.cell(0, 36).unwrap().contents(), "h");
    assert_eq!(screen.cell(0, 37).unwrap().contents(), "i");
    assert_eq!(screen.cell(0, 38).unwrap().contents(), "n");
    assert_eq!(screen.cell(0, 39).unwrap().contents(), "g");
    assert_eq!(screen.cell(0, 40).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 41).unwrap().contents(), "g");
    assert_eq!(screen.cell(0, 42).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 43).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 44).unwrap().contents(), "d");
    assert_eq!(screen.cell(0, 45).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 46).unwrap().contents(), "u");
    assert_eq!(screen.cell(0, 47).unwrap().contents(), "n\u{00ad}");
    assert_eq!(screen.cell(0, 48).unwrap().contents(), "l");
    assert_eq!(screen.cell(0, 49).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 50).unwrap().contents(), "s");
    assert_eq!(screen.cell(0, 51).unwrap().contents(), "s");
    assert_eq!(screen.cell(0, 52).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 53).unwrap().contents(), "w");
    assert_eq!(screen.cell(0, 54).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 55).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 56).unwrap().contents(), "t");
    assert_eq!(screen.cell(0, 57).unwrap().contents(), "a");
    assert_eq!(screen.cell(0, 58).unwrap().contents(), "k");
    assert_eq!(screen.cell(0, 59).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 60).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 61).unwrap().contents(), "a");
    assert_eq!(screen.cell(0, 62).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 63).unwrap().contents(), "k");
    assert_eq!(screen.cell(0, 64).unwrap().contents(), "n");
    assert_eq!(screen.cell(0, 65).unwrap().contents(), "i");
    assert_eq!(screen.cell(0, 66).unwrap().contents(), "f");
    assert_eq!(screen.cell(0, 67).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 68).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 69).unwrap().contents(), "t");
    assert_eq!(screen.cell(0, 70).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 71).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 72).unwrap().contents(), "i");
    assert_eq!(screen.cell(0, 73).unwrap().contents(), "t");
    assert_eq!(screen.cell(0, 74).unwrap().contents(), "s");
    assert_eq!(screen.cell(0, 75).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 76).unwrap().contents(), "t");
    assert_eq!(screen.cell(0, 77).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 78).unwrap().contents(), "s\u{00ad}");
    assert_eq!(screen.cell(0, 79).unwrap().contents(), "t");
    assert_eq!(screen.cell(0, 80).unwrap().contents(), "i\u{00ad}");
    assert_eq!(screen.cell(0, 81).unwrap().contents(), "c");
    assert_eq!(screen.cell(0, 82).unwrap().contents(), "l");
    assert_eq!(screen.cell(0, 83).unwrap().contents(), "e");
    assert_eq!(screen.cell(0, 84).unwrap().contents(), "s");
    assert_eq!(screen.cell(0, 85).unwrap().contents(), " ");
    assert_eq!(screen.cell(0, 86).unwrap().contents(), "f");
    assert_eq!(screen.cell(0, 87).unwrap().contents(), "i");
    assert_eq!(screen.cell(0, 88).unwrap().contents(), "r");
    assert_eq!(screen.cell(0, 89).unwrap().contents(), "s");
    assert_eq!(screen.cell(0, 90).unwrap().contents(), "t");
    assert_eq!(screen.cell(0, 91).unwrap().contents(), ".");
}
