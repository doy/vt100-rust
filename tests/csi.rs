mod helpers;

#[test]
fn absolute_movement() {
    helpers::fixture("absolute_movement");
}

#[test]
fn horizontal_vertical_position() {
    let mut vt = vt100::Parser::new(5, 10, 0);

    vt.process(b"\x1b[3;7fX");
    assert_eq!(vt.screen().cell(2, 6).unwrap().contents(), "X");
    assert_eq!(vt.screen().cursor_position(), (2, 7));

    vt.process(b"\x1b[fY");
    assert_eq!(vt.screen().cell(0, 0).unwrap().contents(), "Y");
}

#[test]
fn row_clamp() {
    let mut vt = vt100::Parser::default();
    assert_eq!(vt.screen().cursor_position(), (0, 0));
    vt.process(b"\x1b[15d");
    assert_eq!(vt.screen().cursor_position(), (14, 0));
    vt.process(b"\x1b[150d");
    assert_eq!(vt.screen().cursor_position(), (23, 0));
}

#[test]
fn relative_movement() {
    helpers::fixture("relative_movement");
}

#[test]
fn ed() {
    helpers::fixture("ed");
}

#[test]
fn el() {
    helpers::fixture("el");
}

#[test]
fn ich_dch_ech() {
    helpers::fixture("ich_dch_ech");
}

#[test]
fn il_dl() {
    helpers::fixture("il_dl");
}

#[test]
fn scroll() {
    helpers::fixture("scroll");
}

#[test]
fn xtwinops() {
    struct Callbacks;
    impl vt100::Callbacks for Callbacks {
        fn resize(
            &mut self,
            screen: &mut vt100::Screen,
            (rows, cols): (u16, u16),
        ) {
            screen.set_size(rows, cols);
        }
    }

    let mut vt = vt100::Parser::new_with_callbacks(24, 80, 0, Callbacks);
    assert_eq!(vt.screen().size(), (24, 80));
    vt.process(b"\x1b[8;24;80t");
    assert_eq!(vt.screen().size(), (24, 80));
    vt.process(b"\x1b[8t");
    assert_eq!(vt.screen().size(), (24, 80));
    vt.process(b"\x1b[8;80;24t");
    assert_eq!(vt.screen().size(), (80, 24));
    vt.process(b"\x1b[8;24t");
    assert_eq!(vt.screen().size(), (24, 24));

    let mut vt = vt100::Parser::new_with_callbacks(24, 80, 0, Callbacks);
    assert_eq!(vt.screen().size(), (24, 80));
    vt.process(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    assert_eq!(
        vt.screen().rows(0, 80).next().unwrap(),
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(vt.screen().rows(0, 80).nth(1).unwrap(), "aaaaaaaaaa");
    vt.process(
        b"\x1b[H\x1b[8;24;15tbbbbbbbbbbbbbbbbbbbb\x1b[8;24;80tcccccccccccccccccccc",
    );
    assert_eq!(vt.screen().rows(0, 80).next().unwrap(), "bbbbbbbbbbbbbbb");
    assert_eq!(
        vt.screen().rows(0, 80).nth(1).unwrap(),
        "bbbbbcccccccccccccccccccc"
    );
}

#[test]
fn hpa() {
    let mut vt = vt100::Parser::default();

    // HPA (CSI `) is equivalent to CHA (CSI G) - Horizontal Position Absolute
    vt.process(b"hello");
    assert_eq!(vt.screen().cursor_position(), (0, 5));

    // Move to column 10 (1-indexed, so position 9)
    vt.process(b"\x1b[10`");
    assert_eq!(vt.screen().cursor_position(), (0, 9));

    // Move to column 1 (default)
    vt.process(b"\x1b[`");
    assert_eq!(vt.screen().cursor_position(), (0, 0));

    // Move to column 50
    vt.process(b"\x1b[50`");
    assert_eq!(vt.screen().cursor_position(), (0, 49));

    // Clamp to screen width (80 columns, so max position is 79)
    vt.process(b"\x1b[500`");
    assert_eq!(vt.screen().cursor_position(), (0, 79));
}

#[test]
fn rep() {
    let mut vt = vt100::Parser::default();

    // REP (CSI b) repeats the last printed character
    vt.process(b"x\x1b[5b");
    assert_eq!(vt.screen().rows(0, 80).next().unwrap(), "xxxxxx");
    assert_eq!(vt.screen().cursor_position(), (0, 6));

    // REP with default count of 1
    vt.process(b"\x1b[Hy\x1b[b");
    assert_eq!(vt.screen().rows(0, 80).next().unwrap(), "yyxxxx");
    assert_eq!(vt.screen().cursor_position(), (0, 2));

    // REP on a new line
    vt.process(b"\x1b[2;1Ha\x1b[3b");
    assert_eq!(vt.screen().rows(0, 80).nth(1).unwrap(), "aaaa");

    // REP should not do anything if no character was printed yet
    let mut vt2 = vt100::Parser::default();
    vt2.process(b"\x1b[5b");
    assert_eq!(vt2.screen().rows(0, 80).next().unwrap(), "");
    assert_eq!(vt2.screen().cursor_position(), (0, 0));
}
