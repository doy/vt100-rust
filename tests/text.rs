#![allow(clippy::cognitive_complexity)]

#[test]
fn ascii() {
    let mut parser = vt100::Parser::default();
    parser.process(b"foo");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "f");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 0).unwrap().contents(), "");
    assert_eq!(parser.screen().contents(), "foo");
}

#[test]
fn utf8() {
    let mut parser = vt100::Parser::default();
    parser.process("café".as_bytes());
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "c");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "f");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "é");
    assert_eq!(parser.screen().cell(0, 4).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 0).unwrap().contents(), "");
    assert_eq!(parser.screen().contents(), "café");
}

#[test]
fn newlines() {
    let mut parser = vt100::Parser::default();
    parser.process(b"f\r\noo\r\nood");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "f");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 0).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(1, 1).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(1, 2).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(2, 0).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(2, 1).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(2, 2).unwrap().contents(), "d");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(3, 0).unwrap().contents(), "");
    assert_eq!(parser.screen().contents(), "f\noo\nood");
}

#[test]
fn wide() {
    let mut parser = vt100::Parser::default();
    let screen = parser.screen().clone();
    parser.process("aデbネ".as_bytes());
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "デ");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "b");
    assert_eq!(parser.screen().cell(0, 4).unwrap().contents(), "ネ");
    assert_eq!(parser.screen().cell(0, 5).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(0, 6).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 0).unwrap().contents(), "");
    assert_eq!(parser.screen().contents(), "aデbネ");
    assert_eq!(parser.screen().cursor_position(), (0, 6));
    assert_eq!(
        parser.screen().contents_formatted(),
        "\x1b[?25h\x1b[m\x1b[H\x1b[Jaデbネ".as_bytes()
    );
    assert_eq!(parser.screen().contents_diff(&screen), "aデbネ".as_bytes());

    let screen = parser.screen().clone();
    parser.process(b"\x1b[1;1H\x1b[3Cc");
    assert_eq!(parser.screen().contents(), "aデcネ");
    assert_eq!(parser.screen().cursor_position(), (0, 4));
    assert_eq!(
        parser.screen().contents_formatted(),
        "\x1b[?25h\x1b[m\x1b[H\x1b[Jaデcネ\x1b[1;5H".as_bytes()
    );
    assert_eq!(
        parser.screen().contents_diff(&screen),
        "\x1b[1;4Hc".as_bytes()
    );

    let screen = parser.screen().clone();
    parser.process("\x1b[1;7Hfoobar".as_bytes());
    assert_eq!(parser.screen().contents(), "aデcネfoobar");
    assert_eq!(parser.screen().cursor_position(), (0, 12));
    assert_eq!(
        parser.screen().contents_formatted(),
        "\x1b[?25h\x1b[m\x1b[H\x1b[Jaデcネfoobar".as_bytes()
    );
    assert_eq!(
        parser.screen().contents_diff(&screen),
        "\x1b[2Cfoobar".as_bytes()
    );

    let screen = parser.screen().clone();
    parser.process("\x1b[1;1Hデcネfoobar\x1b[K".as_bytes());
    assert_eq!(parser.screen().contents(), "デcネfoobar");
    assert_eq!(parser.screen().cursor_position(), (0, 11));
    assert_eq!(
        parser.screen().contents_formatted(),
        "\x1b[?25h\x1b[m\x1b[H\x1b[Jデcネfoobar".as_bytes()
    );
    assert_eq!(
        parser.screen().contents_diff(&screen),
        "\x1b[Hデcネfo\x1b[Cbar\x1b[K".as_bytes()
    );

    let screen = parser.screen().clone();
    parser.process("\x1b[1;1Haデcネfoobar\x1b[K".as_bytes());
    assert_eq!(parser.screen().contents(), "aデcネfoobar");
    assert_eq!(parser.screen().cursor_position(), (0, 12));
    assert_eq!(
        parser.screen().contents_formatted(),
        "\x1b[?25h\x1b[m\x1b[H\x1b[Jaデcネfoobar".as_bytes()
    );
    assert_eq!(
        parser.screen().contents_diff(&screen),
        "\x1b[Haデcネf\x1b[Cobar".as_bytes()
    );

    let screen = parser.screen().clone();
    parser.process("\x1b[1;1Hデcネfoobar\x1b[K".as_bytes());
    assert_eq!(parser.screen().contents(), "デcネfoobar");
    assert_eq!(parser.screen().cursor_position(), (0, 11));
    assert_eq!(
        parser.screen().contents_formatted(),
        "\x1b[?25h\x1b[m\x1b[H\x1b[Jデcネfoobar".as_bytes()
    );
    assert_eq!(
        parser.screen().contents_diff(&screen),
        "\x1b[Hデcネfo\x1b[Cbar\x1b[K".as_bytes()
    );
}

#[cfg(feature = "unicode-normalization")]
#[test]
fn combining() {
    let mut parser = vt100::Parser::default();
    parser.process(b"a");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "a");
    parser.process("\u{0301}".as_bytes());
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "á");
    parser.process(b"\x1bcabcdefg");
    assert_eq!(parser.screen().contents(), "abcdefg");
    parser.process("\x1b[1;6H\u{0301}".as_bytes());
    assert_eq!(parser.screen().contents(), "abcdéfg");
    parser.process(b"\x1b[10;78Haaa");
    assert_eq!(parser.screen().cell(9, 79).unwrap().contents(), "a");
    parser.process("\r\n\u{0301}".as_bytes());
    assert_eq!(parser.screen().cell(9, 79).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(10, 0).unwrap().contents(), "");

    parser.process("\x1bcabcdefg\x1b[1;3H\u{0301}".as_bytes());
    assert_eq!(parser.screen().contents(), "ab́cdefg");
    parser.process("\x1b[1;2Hb\x1b[1;8H".as_bytes());
    assert_eq!(parser.screen().contents(), "abcdefg");
    let screen = parser.screen().clone();
    parser.process(b"\x1bcabcdefg");
    assert_eq!(parser.screen().contents_diff(&screen), b"");

    parser.process(b"\x1bcaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    assert_eq!(parser.screen().cursor_position(), (0, 80));
    assert_eq!(parser.screen().contents(), "a".repeat(80));

    parser.process("\u{0301}".as_bytes());
    assert_eq!(parser.screen().cursor_position(), (1, 0));
    assert_eq!(parser.screen().contents(), format!("{}á", "a".repeat(79)));
}

#[test]
fn wrap() {
    let mut parser = vt100::Parser::default();
    parser.process(b"0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    assert_eq!(parser.screen().contents(), "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    parser.process(b"\x1b[5H01234567890123456789012345678901234567890123456789012345678901234567890123456789");
    parser.process(b"\x1b[6H01234567890123456789012345678901234567890123456789012345678901234567890123456789");
    assert_eq!(parser.screen().contents(), "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789\n\n\n01234567890123456789012345678901234567890123456789012345678901234567890123456789\n01234567890123456789012345678901234567890123456789012345678901234567890123456789");

    parser.process(b"\x1b[H\x1b[J");
    parser.process(b"0123456789012345678901234567890123456789012345678901234567890123456789012345678");
    assert_eq!(parser.screen().contents(), "0123456789012345678901234567890123456789012345678901234567890123456789012345678");
    assert_eq!(parser.screen().cursor_position(), (0, 79));
    parser.process(b"9");
    assert_eq!(parser.screen().contents(), "01234567890123456789012345678901234567890123456789012345678901234567890123456789");
    assert_eq!(parser.screen().cursor_position(), (0, 80));
    parser.process(b"a");
    assert_eq!(parser.screen().contents(), "01234567890123456789012345678901234567890123456789012345678901234567890123456789a");
    assert_eq!(parser.screen().cursor_position(), (1, 1));
    parser.process(b"b");
    assert_eq!(parser.screen().contents(), "01234567890123456789012345678901234567890123456789012345678901234567890123456789ab");
    assert_eq!(parser.screen().cursor_position(), (1, 2));

    parser.process(b"\x1b[H\x1b[J");
    parser.process(b"012345678901234567890123456789012345678901234567890123456789012345678901234567");
    assert_eq!(parser.screen().contents(), "012345678901234567890123456789012345678901234567890123456789012345678901234567");
    assert_eq!(parser.screen().cursor_position(), (0, 78));
    parser.process("ネ".as_bytes());
    assert_eq!(parser.screen().contents(), "012345678901234567890123456789012345678901234567890123456789012345678901234567ネ");
    assert_eq!(parser.screen().cursor_position(), (0, 80));
    parser.process(b"a");
    assert_eq!(parser.screen().contents(), "012345678901234567890123456789012345678901234567890123456789012345678901234567ネa");
    assert_eq!(parser.screen().cursor_position(), (1, 1));
    assert_eq!(parser.screen().cell(0, 77).unwrap().contents(), "7");
    assert_eq!(parser.screen().cell(0, 78).unwrap().contents(), "ネ");
    assert_eq!(parser.screen().cell(0, 79).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 0).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(1, 1).unwrap().contents(), "");

    parser.process(b"\x1b[H\x1b[J");
    parser.process(b"0123456789012345678901234567890123456789012345678901234567890123456789012345678");
    assert_eq!(parser.screen().contents(), "0123456789012345678901234567890123456789012345678901234567890123456789012345678");
    assert_eq!(parser.screen().cursor_position(), (0, 79));
    parser.process("ネ".as_bytes());
    assert_eq!(parser.screen().contents(), "0123456789012345678901234567890123456789012345678901234567890123456789012345678ネ");
    assert_eq!(parser.screen().cursor_position(), (1, 2));
    parser.process(b"a");
    assert_eq!(parser.screen().contents(), "0123456789012345678901234567890123456789012345678901234567890123456789012345678ネa");
    assert_eq!(parser.screen().cursor_position(), (1, 3));
    assert_eq!(parser.screen().cell(0, 77).unwrap().contents(), "7");
    assert_eq!(parser.screen().cell(0, 78).unwrap().contents(), "8");
    assert_eq!(parser.screen().cell(0, 79).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 0).unwrap().contents(), "ネ");
    assert_eq!(parser.screen().cell(1, 1).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 2).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(1, 3).unwrap().contents(), "");

    parser.process(b"\x1b[H\x1b[J");
    assert_eq!(parser.screen().contents(), "");
    parser.process(b"                                                                                ");
    assert_eq!(parser.screen().contents(), "                                                                                ");
    parser.process(b"\n");
    assert_eq!(parser.screen().contents(), "                                                                                ");
    parser.process(b"\n");
    assert_eq!(parser.screen().contents(), "                                                                                ");
    parser.process(b" ");
    assert_eq!(parser.screen().contents(), "                                                                                \n\n\n ");
}

#[test]
fn wrap_weird() {
    let mut parser = vt100::Parser::default();

    let screen = parser.screen().clone();
    parser.process(b"foo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo");
    assert_eq!(parser.screen().contents_formatted(), &b"\x1b[?25h\x1b[m\x1b[H\x1b[Jfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo"[..]);
    assert_eq!(parser.screen().contents_diff(&screen), &b"foo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo"[..]);

    let screen = parser.screen().clone();
    parser.process(b"\x1b[3;80H ");
    assert_eq!(parser.screen().contents_formatted(), &b"\x1b[?25h\x1b[m\x1b[H\x1b[Jfoo\r\nfoo\r\nfoo\x1b[76C \r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\r\nfoo\x1b[3;80H "[..]);
    assert_eq!(parser.screen().contents_diff(&screen), &b"\x1b[3;80H "[..]);
}
