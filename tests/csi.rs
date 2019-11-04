#[test]
fn absolute_movement() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"\x1b[10;10H");
    assert_eq!(screen.cursor_position(), (9, 9));

    screen.process(b"\x1b[d");
    assert_eq!(screen.cursor_position(), (0, 9));

    screen.process(b"\x1b[15d");
    assert_eq!(screen.cursor_position(), (14, 9));

    screen.process(b"\x1b[H");
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"\x1b[8H");
    assert_eq!(screen.cursor_position(), (7, 0));

    screen.process(b"\x1b[15G");
    assert_eq!(screen.cursor_position(), (7, 14));

    screen.process(b"\x1b[G");
    assert_eq!(screen.cursor_position(), (7, 0));

    screen.process(b"\x1b[0;0H");
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"\x1b[1;1H");
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"\x1b[500;500H");
    assert_eq!(screen.cursor_position(), (23, 79));
}

#[test]
fn relative_movement() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"\x1b[C");
    assert_eq!(screen.cursor_position(), (0, 1));

    screen.process(b"\x1b[C");
    assert_eq!(screen.cursor_position(), (0, 2));

    screen.process(b"\x1b[20C");
    assert_eq!(screen.cursor_position(), (0, 22));

    screen.process(b"\x1b[D");
    assert_eq!(screen.cursor_position(), (0, 21));

    screen.process(b"\x1b[D");
    assert_eq!(screen.cursor_position(), (0, 20));

    screen.process(b"\x1b[9D");
    assert_eq!(screen.cursor_position(), (0, 11));

    screen.process(b"\x1b[500C");
    assert_eq!(screen.cursor_position(), (0, 79));

    screen.process(b"\x1b[500D");
    assert_eq!(screen.cursor_position(), (0, 0));

    screen.process(b"\x1b[B");
    assert_eq!(screen.cursor_position(), (1, 0));

    screen.process(b"\x1b[B");
    assert_eq!(screen.cursor_position(), (2, 0));

    screen.process(b"\x1b[20B");
    assert_eq!(screen.cursor_position(), (22, 0));

    screen.process(b"\x1b[A");
    assert_eq!(screen.cursor_position(), (21, 0));

    screen.process(b"\x1b[A");
    assert_eq!(screen.cursor_position(), (20, 0));

    screen.process(b"\x1b[9A");
    assert_eq!(screen.cursor_position(), (11, 0));

    screen.process(b"\x1b[500B");
    assert_eq!(screen.cursor_position(), (23, 0));

    screen.process(b"\x1b[500A");
    assert_eq!(screen.cursor_position(), (0, 0));
}

#[test]
fn ed() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"foo\x1b[5;5Hbar\x1b[10;10Hbaz\x1b[20;20Hquux");
    assert_eq!(screen.contents(0, 0, 23, 79), "foo\n\n\n\n    bar\n\n\n\n\n         baz\n\n\n\n\n\n\n\n\n\n                   quux");

    screen.process(b"\x1b[10;12H\x1b[0J");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "foo\n\n\n\n    bar\n\n\n\n\n         ba"
    );

    screen.process(b"\x1b[5;6H\x1b[1J");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n      r\n\n\n\n\n         ba"
    );

    screen.process(b"\x1b[7;7H\x1b[2J");
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"\x1b[2J\x1b[H");
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"foo\x1b[5;5Hbar\x1b[10;10Hbaz\x1b[20;20Hquux");
    assert_eq!(screen.contents(0, 0, 23, 79), "foo\n\n\n\n    bar\n\n\n\n\n         baz\n\n\n\n\n\n\n\n\n\n                   quux");

    screen.process(b"\x1b[10;12H\x1b[J");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "foo\n\n\n\n    bar\n\n\n\n\n         ba"
    );

    screen.process(b"\x1b[2J\x1b[H");
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"foo\x1b[5;5Hbar\x1b[10;10Hbaz\x1b[20;20Hquux");
    assert_eq!(screen.contents(0, 0, 23, 79), "foo\n\n\n\n    bar\n\n\n\n\n         baz\n\n\n\n\n\n\n\n\n\n                   quux");

    screen.process(b"\x1b[10;12H\x1b[?0J");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "foo\n\n\n\n    bar\n\n\n\n\n         ba"
    );

    screen.process(b"\x1b[5;6H\x1b[?1J");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n      r\n\n\n\n\n         ba"
    );

    screen.process(b"\x1b[7;7H\x1b[?2J");
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"\x1b[2J\x1b[H");
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"foo\x1b[5;5Hbar\x1b[10;10Hbaz\x1b[20;20Hquux");
    assert_eq!(screen.contents(0, 0, 23, 79), "foo\n\n\n\n    bar\n\n\n\n\n         baz\n\n\n\n\n\n\n\n\n\n                   quux");

    screen.process(b"\x1b[10;12H\x1b[?J");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "foo\n\n\n\n    bar\n\n\n\n\n         ba"
    );
}

#[test]
fn el() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"foo\x1b[5;5Hbarbar\x1b[10;10Hbazbaz\x1b[20;20Hquux");
    assert_eq!(screen.contents(0, 0, 23, 79), "foo\n\n\n\n    barbar\n\n\n\n\n         bazbaz\n\n\n\n\n\n\n\n\n\n                   quux");

    screen.process(b"\x1b[5;8H\x1b[0K");
    assert_eq!(screen.contents(0, 0, 23, 79), "foo\n\n\n\n    bar\n\n\n\n\n         bazbaz\n\n\n\n\n\n\n\n\n\n                   quux");

    screen.process(b"\x1b[10;12H\x1b[1K");
    assert_eq!(screen.contents(0, 0, 23, 79), "foo\n\n\n\n    bar\n\n\n\n\n            baz\n\n\n\n\n\n\n\n\n\n                   quux");

    screen.process(b"\x1b[20;22H\x1b[2K");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "foo\n\n\n\n    bar\n\n\n\n\n            baz"
    );

    screen.process(b"\x1b[1;2H\x1b[K");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "f\n\n\n\n    bar\n\n\n\n\n            baz"
    );

    screen.process(b"\x1b[2J\x1b[H");
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"foo\x1b[5;5Hbarbar\x1b[10;10Hbazbaz\x1b[20;20Hquux");
    assert_eq!(screen.contents(0, 0, 23, 79), "foo\n\n\n\n    barbar\n\n\n\n\n         bazbaz\n\n\n\n\n\n\n\n\n\n                   quux");

    screen.process(b"\x1b[5;8H\x1b[?0K");
    assert_eq!(screen.contents(0, 0, 23, 79), "foo\n\n\n\n    bar\n\n\n\n\n         bazbaz\n\n\n\n\n\n\n\n\n\n                   quux");

    screen.process(b"\x1b[10;12H\x1b[?1K");
    assert_eq!(screen.contents(0, 0, 23, 79), "foo\n\n\n\n    bar\n\n\n\n\n            baz\n\n\n\n\n\n\n\n\n\n                   quux");

    screen.process(b"\x1b[20;22H\x1b[?2K");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "foo\n\n\n\n    bar\n\n\n\n\n            baz"
    );

    screen.process(b"\x1b[1;2H\x1b[?K");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "f\n\n\n\n    bar\n\n\n\n\n            baz"
    );

    screen.process(b"\x1b[2J\x1b[H");
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"
    );

    screen.process(b"\x1b[1;21H\x1b[K");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "12345678901234567890\n12345678901234567890"
    );

    screen.process(b"\x1b[1;10H\x1b[1K");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "          1234567890\n12345678901234567890"
    );
}

#[test]
fn ich_dch_ech() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"\x1b[10;10Hfoobar");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         foobar"
    );

    screen.process(b"\x1b[10;12H\x1b[3@");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         fo   obar"
    );
    assert_eq!(screen.cursor_position(), (9, 11));

    screen.process(b"\x1b[4P");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         fobar"
    );
    assert_eq!(screen.cursor_position(), (9, 11));

    screen.process(b"\x1b[100@");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         fo"
    );
    assert_eq!(screen.cursor_position(), (9, 11));

    screen.process(b"obar");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(screen.cursor_position(), (9, 15));

    screen.process(b"\x1b[10;12H\x1b[100P");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         fo"
    );
    assert_eq!(screen.cursor_position(), (9, 11));

    screen.process(b"obar");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(screen.cursor_position(), (9, 15));

    screen.process(b"\x1b[10;13H\x1b[X");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         foo ar"
    );
    assert_eq!(screen.cursor_position(), (9, 12));

    screen.process(b"\x1b[10;11H\x1b[4X");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         f    r"
    );
    assert_eq!(screen.cursor_position(), (9, 10));

    screen.process(b"\x1b[10;11H\x1b[400X");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         f"
    );
    assert_eq!(screen.cursor_position(), (9, 10));
}

#[test]
fn il_dl() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"\x1b[10;10Hfoobar\x1b[3D");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(screen.cursor_position(), (9, 12));

    screen.process(b"\x1b[L");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(screen.cursor_position(), (9, 12));

    screen.process(b"\x1b[3L");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(screen.cursor_position(), (9, 12));

    screen.process(b"\x1b[500L");
    assert_eq!(screen.contents(0, 0, 23, 79), "");
    assert_eq!(screen.cursor_position(), (9, 12));

    screen.process(b"\x1b[10;10Hfoobar\x1b[3D\x1b[6A");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(screen.cursor_position(), (3, 12));

    screen.process(b"\x1b[M");
    assert_eq!(
        screen.contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n         foobar"
    );
    assert_eq!(screen.cursor_position(), (3, 12));

    screen.process(b"\x1b[3M");
    assert_eq!(screen.contents(0, 0, 23, 79), "\n\n\n\n\n         foobar");
    assert_eq!(screen.cursor_position(), (3, 12));

    screen.process(b"\x1b[500M");
    assert_eq!(screen.contents(0, 0, 23, 79), "");
    assert_eq!(screen.cursor_position(), (3, 12));
}

#[test]
fn scroll() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.contents(0, 0, 23, 79), "");

    screen.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    assert_eq!(screen.contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");

    screen.process(b"\x1b[15;15H");
    assert_eq!(screen.cursor_position(), (14, 14));

    screen.process(b"\x1b[S");
    assert_eq!(screen.contents(0, 0, 23, 79), "2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(screen.cursor_position(), (14, 14));

    screen.process(b"\x1b[3S");
    assert_eq!(screen.contents(0, 0, 23, 79), "5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(screen.cursor_position(), (14, 14));

    screen.process(b"\x1b[T");
    assert_eq!(screen.contents(0, 0, 23, 79), "\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24");
    assert_eq!(screen.cursor_position(), (14, 14));

    screen.process(b"\x1b[5T");
    assert_eq!(screen.contents(0, 0, 23, 79), "\n\n\n\n\n\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22");
    assert_eq!(screen.cursor_position(), (14, 14));
}
