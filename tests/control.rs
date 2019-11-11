#![allow(clippy::cognitive_complexity)]

#[test]
fn bel() {
    let mut parser = vt100::Parser::default();

    assert!(!parser.screen_mut().check_audible_bell());

    parser.process(b"\x07");
    assert!(parser.screen_mut().check_audible_bell());
    assert!(!parser.screen_mut().check_audible_bell());
}

#[test]
fn bs() {
    let mut parser = vt100::Parser::default();

    parser.process(b"foo\x08\x08aa");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "f");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 0).unwrap().contents(), "");
    assert_eq!(parser.screen().contents(), "faa");

    parser.process(b"\r\nquux\x08\x08\x08\x08\x08\x08bar");
    assert_eq!(parser.screen().cell(1, 0).unwrap().contents(), "b");
    assert_eq!(parser.screen().cell(1, 1).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(1, 2).unwrap().contents(), "r");
    assert_eq!(parser.screen().cell(1, 3).unwrap().contents(), "x");
    assert_eq!(parser.screen().cell(1, 4).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(2, 0).unwrap().contents(), "");
    assert_eq!(parser.screen().contents(), "faa\nbarx");
}

#[test]
fn tab() {
    let mut parser = vt100::Parser::default();

    parser.process(b"foo\tbar");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "f");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(0, 4).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(0, 5).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(0, 6).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(0, 7).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(0, 8).unwrap().contents(), "b");
    assert_eq!(parser.screen().cell(0, 9).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(0, 10).unwrap().contents(), "r");
    assert_eq!(parser.screen().cell(0, 11).unwrap().contents(), "");
    assert_eq!(parser.screen().contents(), "foo     bar");
}

fn lf_with(b: u8) {
    let mut parser = vt100::Parser::default();

    parser.process(b"foo");
    parser.process(&[b]);
    parser.process(b"bar");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "f");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 0).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 1).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 2).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 3).unwrap().contents(), "b");
    assert_eq!(parser.screen().cell(1, 4).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(1, 5).unwrap().contents(), "r");
    assert_eq!(parser.screen().cell(1, 6).unwrap().contents(), "");
    assert_eq!(parser.screen().contents(), "foo\n   bar");
}

#[test]
fn lf() {
    lf_with(b'\x0a');
}

#[test]
fn vt() {
    lf_with(b'\x0b');
}

#[test]
fn ff() {
    lf_with(b'\x0c');
}

#[test]
fn cr() {
    let mut parser = vt100::Parser::default();

    parser.process(b"fooo\rbar");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "b");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "r");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 4).unwrap().contents(), "");
    assert_eq!(parser.screen().cell(1, 0).unwrap().contents(), "");
    assert_eq!(parser.screen().contents(), "baro");
}
