#[test]
fn absolute_movement() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[10;10H");
    assert_eq!(parser.screen().cursor_position(), (9, 9));

    parser.process(b"\x1b[d");
    assert_eq!(parser.screen().cursor_position(), (0, 9));

    parser.process(b"\x1b[15d");
    assert_eq!(parser.screen().cursor_position(), (14, 9));

    parser.process(b"\x1b[H");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[8H");
    assert_eq!(parser.screen().cursor_position(), (7, 0));

    parser.process(b"\x1b[15G");
    assert_eq!(parser.screen().cursor_position(), (7, 14));

    parser.process(b"\x1b[G");
    assert_eq!(parser.screen().cursor_position(), (7, 0));

    parser.process(b"\x1b[0;0H");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[1;1H");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[500;500H");
    assert_eq!(parser.screen().cursor_position(), (23, 79));
}

#[test]
fn relative_movement() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[C");
    assert_eq!(parser.screen().cursor_position(), (0, 1));

    parser.process(b"\x1b[C");
    assert_eq!(parser.screen().cursor_position(), (0, 2));

    parser.process(b"\x1b[20C");
    assert_eq!(parser.screen().cursor_position(), (0, 22));

    parser.process(b"\x1b[D");
    assert_eq!(parser.screen().cursor_position(), (0, 21));

    parser.process(b"\x1b[D");
    assert_eq!(parser.screen().cursor_position(), (0, 20));

    parser.process(b"\x1b[9D");
    assert_eq!(parser.screen().cursor_position(), (0, 11));

    parser.process(b"\x1b[500C");
    assert_eq!(parser.screen().cursor_position(), (0, 79));

    parser.process(b"\x1b[500D");
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[B");
    assert_eq!(parser.screen().cursor_position(), (1, 0));

    parser.process(b"\x1b[B");
    assert_eq!(parser.screen().cursor_position(), (2, 0));

    parser.process(b"\x1b[20B");
    assert_eq!(parser.screen().cursor_position(), (22, 0));

    parser.process(b"\x1b[A");
    assert_eq!(parser.screen().cursor_position(), (21, 0));

    parser.process(b"\x1b[A");
    assert_eq!(parser.screen().cursor_position(), (20, 0));

    parser.process(b"\x1b[9A");
    assert_eq!(parser.screen().cursor_position(), (11, 0));

    parser.process(b"\x1b[500B");
    assert_eq!(parser.screen().cursor_position(), (23, 0));

    parser.process(b"\x1b[500A");
    assert_eq!(parser.screen().cursor_position(), (0, 0));
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn ed() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"foo\x1b[5;5Hbar\x1b[10;10Hbaz\x1b[20;20Hquux");
    assert_eq!(parser.screen().contents(), "foo\n\n\n\n    bar\n\n\n\n\n         baz\n\n\n\n\n\n\n\n\n\n                   quux");

    parser.process(b"\x1b[10;12H\x1b[0J");
    assert_eq!(
        parser.screen().contents(),
        "foo\n\n\n\n    bar\n\n\n\n\n         ba"
    );

    parser.process(b"\x1b[5;6H\x1b[1J");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n      r\n\n\n\n\n         ba"
    );

    parser.process(b"\x1b[7;7H\x1b[2J");
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"\x1b[2J\x1b[H");
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"foo\x1b[5;5Hbar\x1b[10;10Hbaz\x1b[20;20Hquux");
    assert_eq!(parser.screen().contents(), "foo\n\n\n\n    bar\n\n\n\n\n         baz\n\n\n\n\n\n\n\n\n\n                   quux");

    parser.process(b"\x1b[10;12H\x1b[J");
    assert_eq!(
        parser.screen().contents(),
        "foo\n\n\n\n    bar\n\n\n\n\n         ba"
    );

    parser.process(b"\x1b[2J\x1b[H");
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"foo\x1b[5;5Hbar\x1b[10;10Hbaz\x1b[20;20Hquux");
    assert_eq!(parser.screen().contents(), "foo\n\n\n\n    bar\n\n\n\n\n         baz\n\n\n\n\n\n\n\n\n\n                   quux");

    parser.process(b"\x1b[10;12H\x1b[?0J");
    assert_eq!(
        parser.screen().contents(),
        "foo\n\n\n\n    bar\n\n\n\n\n         ba"
    );

    parser.process(b"\x1b[5;6H\x1b[?1J");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n      r\n\n\n\n\n         ba"
    );

    parser.process(b"\x1b[7;7H\x1b[?2J");
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"\x1b[2J\x1b[H");
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"foo\x1b[5;5Hbar\x1b[10;10Hbaz\x1b[20;20Hquux");
    assert_eq!(parser.screen().contents(), "foo\n\n\n\n    bar\n\n\n\n\n         baz\n\n\n\n\n\n\n\n\n\n                   quux");

    parser.process(b"\x1b[10;12H\x1b[?J");
    assert_eq!(
        parser.screen().contents(),
        "foo\n\n\n\n    bar\n\n\n\n\n         ba"
    );

    parser.process(b"\x1bc\x1b[5;5H");
    assert_eq!(
        parser.screen().cell(3, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(5, 5).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;5H"
    );

    parser.process(b"\x1b[41m\x1b[J");
    assert_eq!(
        parser.screen().cell(3, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(5, 5).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        format!(
            "\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;5H\x1b[41m{}\x1b[X\r\n{}{}\x1b[X\x1b[5;5H",
            "\x1b[X\x1b[C".repeat(75),
            format!("{}\x1b[X\r\n", "\x1b[X\x1b[C".repeat(79)).repeat(18),
            "\x1b[X\x1b[C".repeat(79),
        )
        .as_bytes()
    );

    parser.process(b"\x1bc\x1b[5;5H");
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;5H"
    );

    parser.process(b"\x1b[41m\x1b[1J");
    assert_eq!(
        parser.screen().cell(3, 3).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(5, 5).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        format!(
            "\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[41m{}{}\x1b[X",
            format!("{}\x1b[X\r\n", "\x1b[X\x1b[C".repeat(79)).repeat(4),
            "\x1b[X\x1b[C".repeat(4),
        )
        .as_bytes()
    );

    parser.process(b"\x1bc\x1b[5;5H");
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;5H"
    );

    parser.process(b"\x1b[41m\x1b[2J");
    assert_eq!(
        parser.screen().cell(3, 3).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(5, 5).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        format!(
            "\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[41m{}{}\x1b[5;5H",
            format!("{}\x1b[X\r\n", "\x1b[X\x1b[C".repeat(79)).repeat(23),
            format!("{}\x1b[X", "\x1b[X\x1b[C".repeat(79)),
        )
        .as_bytes()
    );
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn el() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"foo\x1b[5;5Hbarbar\x1b[10;10Hbazbaz\x1b[20;20Hquux");
    assert_eq!(parser.screen().contents(), "foo\n\n\n\n    barbar\n\n\n\n\n         bazbaz\n\n\n\n\n\n\n\n\n\n                   quux");

    parser.process(b"\x1b[5;8H\x1b[0K");
    assert_eq!(parser.screen().contents(), "foo\n\n\n\n    bar\n\n\n\n\n         bazbaz\n\n\n\n\n\n\n\n\n\n                   quux");

    parser.process(b"\x1b[10;12H\x1b[1K");
    assert_eq!(parser.screen().contents(), "foo\n\n\n\n    bar\n\n\n\n\n            baz\n\n\n\n\n\n\n\n\n\n                   quux");

    parser.process(b"\x1b[20;22H\x1b[2K");
    assert_eq!(
        parser.screen().contents(),
        "foo\n\n\n\n    bar\n\n\n\n\n            baz"
    );

    parser.process(b"\x1b[1;2H\x1b[K");
    assert_eq!(
        parser.screen().contents(),
        "f\n\n\n\n    bar\n\n\n\n\n            baz"
    );

    parser.process(b"\x1b[2J\x1b[H");
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"foo\x1b[5;5Hbarbar\x1b[10;10Hbazbaz\x1b[20;20Hquux");
    assert_eq!(parser.screen().contents(), "foo\n\n\n\n    barbar\n\n\n\n\n         bazbaz\n\n\n\n\n\n\n\n\n\n                   quux");

    parser.process(b"\x1b[5;8H\x1b[?0K");
    assert_eq!(parser.screen().contents(), "foo\n\n\n\n    bar\n\n\n\n\n         bazbaz\n\n\n\n\n\n\n\n\n\n                   quux");

    parser.process(b"\x1b[10;12H\x1b[?1K");
    assert_eq!(parser.screen().contents(), "foo\n\n\n\n    bar\n\n\n\n\n            baz\n\n\n\n\n\n\n\n\n\n                   quux");

    parser.process(b"\x1b[20;22H\x1b[?2K");
    assert_eq!(
        parser.screen().contents(),
        "foo\n\n\n\n    bar\n\n\n\n\n            baz"
    );

    parser.process(b"\x1b[1;2H\x1b[?K");
    assert_eq!(
        parser.screen().contents(),
        "f\n\n\n\n    bar\n\n\n\n\n            baz"
    );

    parser.process(b"\x1b[2J\x1b[H");
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890");
    assert_eq!(
        parser.screen().contents(),
        "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"
    );

    parser.process(b"\x1b[1;21H\x1b[K");
    assert_eq!(
        parser.screen().contents(),
        "12345678901234567890\n12345678901234567890"
    );

    parser.process(b"\x1b[1;10H\x1b[1K");
    assert_eq!(
        parser.screen().contents(),
        "          1234567890\n12345678901234567890"
    );

    parser.process(b"\x1bc\x1b[5;5H");
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;5H"
    );

    parser.process(b"\x1b[41m\x1b[K");
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        format!(
            "\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;5H\x1b[41m{}\x1b[X\x1b[5;5H",
            "\x1b[X\x1b[C".repeat(75)
        )
        .as_bytes()
    );

    parser.process(b"\x1bc\x1b[5;5H");
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;5H"
    );

    parser.process(b"\x1b[41m\x1b[1K");
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        format!(
            "\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;1H\x1b[41m{}\x1b[X",
            "\x1b[X\x1b[C".repeat(4),
        )
        .as_bytes()
    );

    parser.process(b"\x1bc\x1b[5;5H");
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;5H"
    );

    parser.process(b"\x1b[41m\x1b[2K");
    assert_eq!(
        parser.screen().cell(4, 3).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 4).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(4, 5).unwrap().bgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().contents_formatted(),
        format!(
            "\x1b[?25h\x1b[m\x1b[H\x1b[J\x1b[5;1H\x1b[41m{}\x1b[X\x1b[5;5H",
            "\x1b[X\x1b[C".repeat(79),
        )
        .as_bytes()
    );
}

#[test]
fn ich_dch_ech() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"\x1b[10;10Hfoobar");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n         foobar"
    );

    parser.process(b"\x1b[10;12H\x1b[3@");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n         fo   obar"
    );
    assert_eq!(parser.screen().cursor_position(), (9, 11));

    parser.process(b"\x1b[4P");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n         fobar"
    );
    assert_eq!(parser.screen().cursor_position(), (9, 11));

    parser.process(b"\x1b[100@");
    assert_eq!(parser.screen().contents(), "\n\n\n\n\n\n\n\n\n         fo");
    assert_eq!(parser.screen().cursor_position(), (9, 11));

    parser.process(b"obar");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(parser.screen().cursor_position(), (9, 15));

    parser.process(b"\x1b[10;12H\x1b[100P");
    assert_eq!(parser.screen().contents(), "\n\n\n\n\n\n\n\n\n         fo");
    assert_eq!(parser.screen().cursor_position(), (9, 11));

    parser.process(b"obar");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(parser.screen().cursor_position(), (9, 15));

    parser.process(b"\x1b[10;13H\x1b[X");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n         foo ar"
    );
    assert_eq!(parser.screen().cursor_position(), (9, 12));

    parser.process(b"\x1b[10;11H\x1b[4X");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n         f    r"
    );
    assert_eq!(parser.screen().cursor_position(), (9, 10));

    parser.process(b"\x1b[10;11H\x1b[400X");
    assert_eq!(parser.screen().contents(), "\n\n\n\n\n\n\n\n\n         f");
    assert_eq!(parser.screen().cursor_position(), (9, 10));
}

#[test]
fn il_dl() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"\x1b[10;10Hfoobar\x1b[3D");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(parser.screen().cursor_position(), (9, 12));

    parser.process(b"\x1b[L");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(parser.screen().cursor_position(), (9, 12));

    parser.process(b"\x1b[3L");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(parser.screen().cursor_position(), (9, 12));

    parser.process(b"\x1b[500L");
    assert_eq!(parser.screen().contents(), "");
    assert_eq!(parser.screen().cursor_position(), (9, 12));

    parser.process(b"\x1b[10;10Hfoobar\x1b[3D\x1b[6A");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(parser.screen().cursor_position(), (3, 12));

    parser.process(b"\x1b[M");
    assert_eq!(
        parser.screen().contents(),
        "\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(parser.screen().cursor_position(), (3, 12));

    parser.process(b"\x1b[3M");
    assert_eq!(parser.screen().contents(), "\n\n\n\n\n         foobar");
    assert_eq!(parser.screen().cursor_position(), (3, 12));

    parser.process(b"\x1b[500M");
    assert_eq!(parser.screen().contents(), "");
    assert_eq!(parser.screen().cursor_position(), (3, 12));
}

#[test]
fn scroll() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().contents(), "");

    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");

    parser.process(b"\x1b[15;15H");
    assert_eq!(parser.screen().cursor_position(), (14, 14));

    parser.process(b"\x1b[S");
    assert_eq!(parser.screen().contents(), "2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(parser.screen().cursor_position(), (14, 14));

    parser.process(b"\x1b[3S");
    assert_eq!(parser.screen().contents(), "5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(parser.screen().cursor_position(), (14, 14));

    parser.process(b"\x1b[T");
    assert_eq!(parser.screen().contents(), "\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(parser.screen().cursor_position(), (14, 14));

    parser.process(b"\x1b[5T");
    assert_eq!(parser.screen().contents(), "\n\n\n\n\n\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22");
    assert_eq!(parser.screen().cursor_position(), (14, 14));
}
