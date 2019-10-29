extern crate vt100;

mod support;
use support::TestHelpers;

#[test]
fn bel() {
    let mut screen = vt100::Screen::new(24, 80);

    assert!(!screen.check_audible_bell());

    screen.assert_process(b"\x07");
    assert!(screen.check_audible_bell());
    assert!(!screen.check_audible_bell());
}

#[test]
fn bs() {
    let mut screen = vt100::Screen::new(24, 80);

    screen.assert_process(b"foo\x08\x08aa");
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "f");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "a");
    assert_eq!(screen.cell(0, 2).unwrap().contents(), "a");
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "faa\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );

    screen.assert_process(b"\r\nquux\x08\x08\x08\x08\x08\x08bar");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "b");
    assert_eq!(screen.cell(1, 1).unwrap().contents(), "a");
    assert_eq!(screen.cell(1, 2).unwrap().contents(), "r");
    assert_eq!(screen.cell(1, 3).unwrap().contents(), "x");
    assert_eq!(screen.cell(1, 4).unwrap().contents(), "");
    assert_eq!(screen.cell(2, 0).unwrap().contents(), "");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "faa\nbarx\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
}

#[test]
fn tab() {
    let mut screen = vt100::Screen::new(24, 80);

    screen.assert_process(b"foo\tbar");
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "f");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 2).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "");
    assert_eq!(screen.cell(0, 4).unwrap().contents(), "");
    assert_eq!(screen.cell(0, 5).unwrap().contents(), "");
    assert_eq!(screen.cell(0, 6).unwrap().contents(), "");
    assert_eq!(screen.cell(0, 7).unwrap().contents(), "");
    assert_eq!(screen.cell(0, 8).unwrap().contents(), "b");
    assert_eq!(screen.cell(0, 9).unwrap().contents(), "a");
    assert_eq!(screen.cell(0, 10).unwrap().contents(), "r");
    assert_eq!(screen.cell(0, 11).unwrap().contents(), "");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "foo     bar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
}

#[test]
fn lf() {
    let mut screen = vt100::Screen::new(24, 80);

    screen.assert_process(b"foo\nbar");
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "f");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 2).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 1).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 2).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 3).unwrap().contents(), "b");
    assert_eq!(screen.cell(1, 4).unwrap().contents(), "a");
    assert_eq!(screen.cell(1, 5).unwrap().contents(), "r");
    assert_eq!(screen.cell(1, 6).unwrap().contents(), "");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "foo\n   bar\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
}

#[test]
fn cr() {
    let mut screen = vt100::Screen::new(24, 80);

    screen.assert_process(b"fooo\rbar");
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "b");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "a");
    assert_eq!(screen.cell(0, 2).unwrap().contents(), "r");
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "o");
    assert_eq!(screen.cell(0, 4).unwrap().contents(), "");
    assert_eq!(screen.cell(1, 0).unwrap().contents(), "");
    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "baro\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
}
