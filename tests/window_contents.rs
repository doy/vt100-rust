#[test]
fn formatted() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(
        screen.contents_formatted(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );

    screen.process(b"foobar");
    assert!(!screen.cell(0, 2).unwrap().bold());
    assert!(!screen.cell(0, 3).unwrap().bold());
    assert!(!screen.cell(0, 4).unwrap().bold());
    assert!(!screen.cell(0, 5).unwrap().bold());
    assert_eq!(
        screen.contents_formatted(0, 0, 23, 79),
        "foobar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );

    screen.process(b"\x1b[1;4H\x1b[1;7m\x1b[33mb");
    assert!(!screen.cell(0, 2).unwrap().bold());
    assert!(screen.cell(0, 3).unwrap().bold());
    assert!(!screen.cell(0, 4).unwrap().bold());
    assert!(!screen.cell(0, 5).unwrap().bold());
    assert_eq!(screen.contents_formatted(0, 0 ,23, 79), "foo\x1b[33;1;7mb\x1b[mar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    screen.process(b"\x1b[1;5H\x1b[22;42ma");
    assert!(!screen.cell(0, 2).unwrap().bold());
    assert!(screen.cell(0, 3).unwrap().bold());
    assert!(!screen.cell(0, 4).unwrap().bold());
    assert!(!screen.cell(0, 5).unwrap().bold());
    assert_eq!(screen.contents_formatted(0, 0 ,23, 79), "foo\x1b[33;1;7mb\x1b[42;22ma\x1b[mr\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
}
