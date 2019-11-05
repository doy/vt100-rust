#[test]
fn formatted() {
    let mut parser = vt100::Parser::new(24, 80);
    compare_formatted(&parser);
    assert_eq!(parser.screen().contents_formatted(0, 0, 23, 79), b"");

    parser.process(b"foobar");
    compare_formatted(&parser);
    assert!(!parser.screen().cell(0, 2).unwrap().bold());
    assert!(!parser.screen().cell(0, 3).unwrap().bold());
    assert!(!parser.screen().cell(0, 4).unwrap().bold());
    assert!(!parser.screen().cell(0, 5).unwrap().bold());
    assert_eq!(parser.screen().contents_formatted(0, 0, 23, 79), b"foobar");

    parser.process(b"\x1b[1;4H\x1b[1;7m\x1b[33mb");
    compare_formatted(&parser);
    assert!(!parser.screen().cell(0, 2).unwrap().bold());
    assert!(parser.screen().cell(0, 3).unwrap().bold());
    assert!(!parser.screen().cell(0, 4).unwrap().bold());
    assert!(!parser.screen().cell(0, 5).unwrap().bold());
    assert_eq!(
        parser.screen().contents_formatted(0, 0, 23, 79),
        b"foo\x1b[33;1;7mb\x1b[mar"
    );

    parser.process(b"\x1b[1;5H\x1b[22;42ma");
    compare_formatted(&parser);
    assert!(!parser.screen().cell(0, 2).unwrap().bold());
    assert!(parser.screen().cell(0, 3).unwrap().bold());
    assert!(!parser.screen().cell(0, 4).unwrap().bold());
    assert!(!parser.screen().cell(0, 5).unwrap().bold());
    assert_eq!(
        parser.screen().contents_formatted(0, 0, 23, 79),
        b"foo\x1b[33;1;7mb\x1b[42;22ma\x1b[mr"
    );

    parser.process(b"\x1b[1;6H\x1b[35mr\r\nquux");
    compare_formatted(&parser);
    assert_eq!(
        parser.screen().contents_formatted(0, 0, 23, 79),
        &b"foo\x1b[33;1;7mb\x1b[42;22ma\x1b[35mr\r\nquux"[..]
    );

    parser.process(b"\x1b[2;1H\x1b[45mquux");
    compare_formatted(&parser);
    assert_eq!(
        parser.screen().contents_formatted(0, 0, 23, 79),
        &b"foo\x1b[33;1;7mb\x1b[42;22ma\x1b[35mr\r\n\x1b[45mquux"[..]
    );

    parser
        .process(b"\x1b[2;2H\x1b[38;2;123;213;231mu\x1b[38;5;254mu\x1b[39mx");
    compare_formatted(&parser);
    assert_eq!(parser.screen().contents_formatted(0, 0 ,23, 79), &b"foo\x1b[33;1;7mb\x1b[42;22ma\x1b[35mr\r\n\x1b[45mq\x1b[38;2;123;213;231mu\x1b[38;5;254mu\x1b[39mx"[..]);
}

#[test]
fn empty_cells() {
    let mut parser = vt100::Parser::new(24, 80);
    parser.process(b"\x1b[5C\x1b[32m bar\x1b[H\x1b[31mfoo");
    compare_formatted(&parser);
    assert_eq!(parser.screen().contents(0, 0, 23, 79), "foo   bar");
    assert_eq!(
        parser.screen().contents_formatted(0, 0, 23, 79),
        b"\x1b[31mfoo\x1b[m\x1b[C\x1b[C\x1b[32m bar"
    );
}

fn compare_formatted(parser: &vt100::Parser) {
    let (rows, cols) = parser.screen().size();
    let contents =
        parser.screen().contents_formatted(0, 0, rows - 1, cols - 1);
    let mut parser2 = vt100::Parser::new(rows, cols);
    parser2.process(&contents);
    compare_cells(parser, &parser2);
}

fn compare_cells(parser1: &vt100::Parser, parser2: &vt100::Parser) {
    assert_eq!(parser1.screen().size(), parser2.screen().size());
    let (rows, cols) = parser1.screen().size();

    for row in 0..rows {
        for col in 0..cols {
            let cell1 = parser1.screen().cell(row, col).unwrap();
            let cell2 = parser2.screen().cell(row, col).unwrap();

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
