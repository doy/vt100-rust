use std::io::Read as _;

#[test]
fn formatted() {
    let mut parser = vt100::Parser::new(24, 80);
    compare_formatted(parser.screen());
    assert_eq!(parser.screen().contents_formatted(), b"");

    parser.process(b"foobar");
    compare_formatted(parser.screen());
    assert!(!parser.screen().cell(0, 2).unwrap().bold());
    assert!(!parser.screen().cell(0, 3).unwrap().bold());
    assert!(!parser.screen().cell(0, 4).unwrap().bold());
    assert!(!parser.screen().cell(0, 5).unwrap().bold());
    assert_eq!(parser.screen().contents_formatted(), b"foobar");

    parser.process(b"\x1b[1;4H\x1b[1;7m\x1b[33mb");
    compare_formatted(parser.screen());
    assert!(!parser.screen().cell(0, 2).unwrap().bold());
    assert!(parser.screen().cell(0, 3).unwrap().bold());
    assert!(!parser.screen().cell(0, 4).unwrap().bold());
    assert!(!parser.screen().cell(0, 5).unwrap().bold());
    assert_eq!(
        parser.screen().contents_formatted(),
        b"foo\x1b[33;1;7mb\x1b[mar"
    );

    parser.process(b"\x1b[1;5H\x1b[22;42ma");
    compare_formatted(parser.screen());
    assert!(!parser.screen().cell(0, 2).unwrap().bold());
    assert!(parser.screen().cell(0, 3).unwrap().bold());
    assert!(!parser.screen().cell(0, 4).unwrap().bold());
    assert!(!parser.screen().cell(0, 5).unwrap().bold());
    assert_eq!(
        parser.screen().contents_formatted(),
        b"foo\x1b[33;1;7mb\x1b[42;22ma\x1b[mr"
    );

    parser.process(b"\x1b[1;6H\x1b[35mr\r\nquux");
    compare_formatted(parser.screen());
    assert_eq!(
        parser.screen().contents_formatted(),
        &b"foo\x1b[33;1;7mb\x1b[42;22ma\x1b[35mr\r\nquux"[..]
    );

    parser.process(b"\x1b[2;1H\x1b[45mquux");
    compare_formatted(parser.screen());
    assert_eq!(
        parser.screen().contents_formatted(),
        &b"foo\x1b[33;1;7mb\x1b[42;22ma\x1b[35mr\r\n\x1b[45mquux"[..]
    );

    parser
        .process(b"\x1b[2;2H\x1b[38;2;123;213;231mu\x1b[38;5;254mu\x1b[39mx");
    compare_formatted(parser.screen());
    assert_eq!(parser.screen().contents_formatted(), &b"foo\x1b[33;1;7mb\x1b[42;22ma\x1b[35mr\r\n\x1b[45mq\x1b[38;2;123;213;231mu\x1b[38;5;254mu\x1b[39mx"[..]);
}

#[test]
fn empty_cells() {
    let mut parser = vt100::Parser::new(24, 80);
    parser.process(b"\x1b[5C\x1b[32m bar\x1b[H\x1b[31mfoo");
    compare_formatted(parser.screen());
    assert_eq!(parser.screen().contents(), "foo   bar");
    assert_eq!(
        parser.screen().contents_formatted(),
        b"\x1b[31mfoo\x1b[m\x1b[C\x1b[C\x1b[32m bar"
    );
}

#[test]
fn rows() {
    let mut parser = vt100::Parser::new(24, 80);
    assert_eq!(
        parser.screen().rows(0, 80).collect::<Vec<String>>(),
        vec![
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ]
    );
    assert_eq!(
        parser
            .screen()
            .rows_formatted(0, 80)
            .collect::<Vec<Vec<u8>>>(),
        vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ]
    );
    assert_eq!(
        parser.screen().rows(5, 15).collect::<Vec<String>>(),
        vec![
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ]
    );
    assert_eq!(
        parser
            .screen()
            .rows_formatted(5, 15)
            .collect::<Vec<Vec<u8>>>(),
        vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ]
    );

    parser
        .process(b"\x1b[31mfoo\x1b[10;10H\x1b[32mbar\x1b[20;20H\x1b[33mbaz");
    assert_eq!(
        parser.screen().rows(0, 80).collect::<Vec<String>>(),
        vec![
            "foo".to_string(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            "         bar".to_string(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            "                   baz".to_string(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ]
    );
    assert_eq!(
        parser
            .screen()
            .rows_formatted(0, 80)
            .collect::<Vec<Vec<u8>>>(),
        vec![
            b"\x1b[31mfoo".to_vec(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            b"\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[32mbar".to_vec(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            b"\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[33mbaz".to_vec(),
            vec![],
            vec![],
            vec![],
            vec![],
        ]
    );
    assert_eq!(
        parser.screen().rows(5, 15).collect::<Vec<String>>(),
        vec![
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            "    bar".to_string(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            "              b".to_string(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ]
    );
    assert_eq!(
        parser
            .screen()
            .rows_formatted(5, 15)
            .collect::<Vec<Vec<u8>>>(),
        vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            b"\x1b[C\x1b[C\x1b[C\x1b[C\x1b[32mbar".to_vec(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            b"\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[C\x1b[33mb".to_vec(),
            vec![],
            vec![],
            vec![],
            vec![],
        ]
    );
}

#[test]
fn diff() {
    let mut parser = vt100::Parser::new(24, 80);
    let screen1 = parser.screen().clone();
    parser.process(b"\x1b[5C\x1b[32m bar");
    let screen2 = parser.screen().clone();
    assert_eq!(screen2.contents_diff(&screen1), b"\x1b[1;6H\x1b[32m bar");
    compare_diff(&screen1, &screen2, b"");

    parser.process(b"\x1b[H\x1b[31mfoo");
    let screen3 = parser.screen().clone();
    assert_eq!(screen3.contents_diff(&screen2), b"\x1b[1;1H\x1b[31mfoo");
    compare_diff(&screen2, &screen3, b"\x1b[5C\x1b[32m bar");

    parser.process(b"\x1b[1;7H\x1b[32mbaz");
    let screen4 = parser.screen().clone();
    assert_eq!(screen4.contents_diff(&screen3), b"\x1b[1;9H\x1b[32mz");
    compare_diff(&screen3, &screen4, b"\x1b[5C\x1b[32m bar\x1b[H\x1b[31mfoo");

    parser.process(b"\x1b[1;8H\x1b[X");
    let screen5 = parser.screen().clone();
    assert_eq!(screen5.contents_diff(&screen4), b"\x1b[1;8H\x1b[X\x1b[C");
    compare_diff(
        &screen4,
        &screen5,
        b"\x1b[5C\x1b[32m bar\x1b[H\x1b[31mfoo\x1b[1;7H\x1b[32mbaz",
    );
}

#[test]
fn diff_crawl() {
    let mut parser = vt100::Parser::new(24, 80);
    let screens: Vec<_> = (1..=30)
        .map(|i| {
            let mut file =
                std::fs::File::open(format!("tests/data/crawl/crawl{}", i))
                    .unwrap();
            let mut frame = vec![];
            file.read_to_end(&mut frame).unwrap();
            parser.process(&frame);
            (frame.clone(), parser.screen().clone())
        })
        .collect();

    let mut all_frames: Vec<u8> = vec![];
    for two_screens in screens.windows(2) {
        eprintln!("loop");
        match two_screens {
            [(prev_frame, prev_screen), (_, screen)] => {
                all_frames.extend(prev_frame);
                compare_diff(prev_screen, screen, &all_frames);
            }
            _ => unreachable!(),
        }
    }
}

fn compare_formatted(screen: &vt100::Screen) {
    let (rows, cols) = screen.size();
    let mut parser = vt100::Parser::new(rows, cols);
    let contents = screen.contents_formatted();
    parser.process(&contents);
    compare_cells(screen, parser.screen());
}

fn compare_diff(
    prev_screen: &vt100::Screen,
    screen: &vt100::Screen,
    prev_parsed: &[u8],
) {
    let (rows, cols) = screen.size();
    let mut parser = vt100::Parser::new(rows, cols);
    parser.process(prev_parsed);
    assert_eq!(
        parser.screen().contents_formatted(),
        prev_screen.contents_formatted()
    );
    compare_cells(parser.screen(), &prev_screen);

    parser.process(&screen.contents_diff(prev_screen));
    if parser.screen().contents_formatted() != screen.contents_formatted() {
        use std::io::Write as _;
        let mut prev_screen_file =
            std::fs::File::create("prev_screen").unwrap();
        prev_screen_file
            .write_all(&prev_screen.contents_formatted())
            .unwrap();
        let mut screen_file = std::fs::File::create("screen").unwrap();
        screen_file.write_all(&screen.contents_formatted()).unwrap();
        let mut diff_file = std::fs::File::create("diff").unwrap();
        diff_file
            .write_all(&screen.contents_diff(prev_screen))
            .unwrap();
    }
    assert_eq!(
        parser.screen().contents_formatted(),
        screen.contents_formatted()
    );
    compare_cells(parser.screen(), &screen);
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
