#[test]
fn formatted() {
    let mut screen = vt100::Screen::new(24, 80);
    compare_formatted(&screen);
    assert_eq!(
        screen.contents_formatted(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );

    screen.process(b"foobar");
    compare_formatted(&screen);
    assert!(!screen.cell(0, 2).unwrap().bold());
    assert!(!screen.cell(0, 3).unwrap().bold());
    assert!(!screen.cell(0, 4).unwrap().bold());
    assert!(!screen.cell(0, 5).unwrap().bold());
    assert_eq!(
        screen.contents_formatted(0, 0, 23, 79),
        "foobar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );

    screen.process(b"\x1b[1;4H\x1b[1;7m\x1b[33mb");
    compare_formatted(&screen);
    assert!(!screen.cell(0, 2).unwrap().bold());
    assert!(screen.cell(0, 3).unwrap().bold());
    assert!(!screen.cell(0, 4).unwrap().bold());
    assert!(!screen.cell(0, 5).unwrap().bold());
    assert_eq!(screen.contents_formatted(0, 0 ,23, 79), "foo\x1b[33;1;7mb\x1b[mar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    screen.process(b"\x1b[1;5H\x1b[22;42ma");
    compare_formatted(&screen);
    assert!(!screen.cell(0, 2).unwrap().bold());
    assert!(screen.cell(0, 3).unwrap().bold());
    assert!(!screen.cell(0, 4).unwrap().bold());
    assert!(!screen.cell(0, 5).unwrap().bold());
    assert_eq!(screen.contents_formatted(0, 0 ,23, 79), "foo\x1b[33;1;7mb\x1b[42;22ma\x1b[mr\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    screen.process(b"\x1b[1;6H\x1b[35mr\r\nquux");
    compare_formatted(&screen);
    assert_eq!(screen.contents_formatted(0, 0 ,23, 79), "foo\x1b[33;1;7mb\x1b[42;22ma\x1b[35mr\nquux\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    screen.process(b"\x1b[2;1H\x1b[45mquux");
    compare_formatted(&screen);
    assert_eq!(screen.contents_formatted(0, 0 ,23, 79), "foo\x1b[33;1;7mb\x1b[42;22ma\x1b[35mr\n\x1b[45mquux\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    screen
        .process(b"\x1b[2;2H\x1b[38;2;123;213;231mu\x1b[38;5;254mu\x1b[39mx");
    compare_formatted(&screen);
    assert_eq!(screen.contents_formatted(0, 0 ,23, 79), "foo\x1b[33;1;7mb\x1b[42;22ma\x1b[35mr\n\x1b[45mq\x1b[38;2;123;213;231mu\x1b[38;5;254mu\x1b[39mx\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
}

fn compare_formatted(screen: &vt100::Screen) {
    let (rows, cols) = screen.size();
    let contents = screen.contents_formatted(0, 0, rows - 1, cols - 1);
    let mut screen2 = vt100::Screen::new(rows, cols);
    let input =
        contents
            .trim_end()
            .as_bytes()
            .iter()
            .fold(vec![], |mut acc, &c| {
                if c == b'\n' {
                    acc.push(b'\r');
                    acc.push(b'\n');
                } else {
                    acc.push(c);
                }
                acc
            });
    screen2.process(&input);
    compare_cells(screen, &screen2);
}

fn compare_cells(screen1: &vt100::Screen, screen2: &vt100::Screen) {
    assert_eq!(screen1.size(), screen2.size());
    let (rows, cols) = screen1.size();

    for row in 0..rows {
        for col in 0..cols {
            let cell1 = screen1.cell(row, col).unwrap();
            let cell2 = screen2.cell(row, col).unwrap();

            assert_eq!(cell1.contents(), cell2.contents());
            assert_eq!(cell1.fgcolor(), cell2.fgcolor());
            assert_eq!(cell1.bgcolor(), cell2.bgcolor());
            assert_eq!(cell1.bold(), cell2.bold());
            assert_eq!(cell1.italic(), cell2.italic());
            assert_eq!(cell1.underline(), cell2.underline());
            assert_eq!(cell1.inverse(), cell2.inverse());
        }
    }
}
