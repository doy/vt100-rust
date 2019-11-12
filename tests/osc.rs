#![allow(clippy::cognitive_complexity)]

#[test]
fn title() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().title(), "");
    assert_eq!(parser.screen().icon_name(), "");

    let screen = parser.screen().clone();
    parser.process(b"\x1b]2;it's a title\x07");
    assert_eq!(parser.screen().title(), "it's a title");
    assert_eq!(parser.screen().icon_name(), "");
    assert_eq!(
        parser.screen().title_formatted(),
        &b"\x1b]2;it's a title\x07"[..]
    );
    assert_eq!(
        parser.screen().title_diff(&screen),
        &b"\x1b]2;it's a title\x07"[..]
    );

    let screen = parser.screen().clone();
    parser.process(b"\x1b]2;\x07");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(parser.screen().icon_name(), "");
    assert_eq!(parser.screen().title_formatted(), &b""[..]);
    assert_eq!(parser.screen().title_diff(&screen), &b"\x1b]0;\x07"[..]);
}

#[test]
fn icon_name() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().title(), "");
    assert_eq!(parser.screen().icon_name(), "");

    let screen = parser.screen().clone();
    parser.process(b"\x1b]1;it's an icon name\x07");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(parser.screen().icon_name(), "it's an icon name");
    assert_eq!(
        parser.screen().title_formatted(),
        &b"\x1b]1;it's an icon name\x07"[..]
    );
    assert_eq!(
        parser.screen().title_diff(&screen),
        &b"\x1b]1;it's an icon name\x07"[..]
    );

    let screen = parser.screen().clone();
    parser.process(b"\x1b]1;\x07");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(parser.screen().icon_name(), "");
    assert_eq!(parser.screen().title_formatted(), &b""[..]);
    assert_eq!(parser.screen().title_diff(&screen), &b"\x1b]0;\x07"[..]);
}

#[test]
fn title_icon_name() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().title(), "");
    assert_eq!(parser.screen().icon_name(), "");

    let screen = parser.screen().clone();
    parser.process(b"\x1b]0;it's both\x07");
    assert_eq!(parser.screen().title(), "it's both");
    assert_eq!(parser.screen().icon_name(), "it's both");
    assert_eq!(
        parser.screen().title_formatted(),
        &b"\x1b]0;it's both\x07"[..]
    );
    assert_eq!(
        parser.screen().title_diff(&screen),
        &b"\x1b]0;it's both\x07"[..]
    );

    let screen = parser.screen().clone();
    parser.process(b"\x1b]0;\x07");
    assert_eq!(parser.screen().title(), "");
    assert_eq!(parser.screen().icon_name(), "");
    assert_eq!(parser.screen().title_formatted(), &b""[..]);
    assert_eq!(parser.screen().title_diff(&screen), &b"\x1b]0;\x07"[..]);

    let screen = parser.screen().clone();
    parser.process(b"\x1b]2;it's a title\x07\x1b]1;it's an icon name\x07");
    assert_eq!(parser.screen().title(), "it's a title");
    assert_eq!(parser.screen().icon_name(), "it's an icon name");
    assert_eq!(
        parser.screen().title_formatted(),
        &b"\x1b]1;it's an icon name\x07\x1b]2;it's a title\x07"[..]
    );
    assert_eq!(
        parser.screen().title_diff(&screen),
        &b"\x1b]1;it's an icon name\x07\x1b]2;it's a title\x07"[..]
    );

    let screen = parser.screen().clone();
    parser.process(b"\x1b]1;it's a title\x07");
    assert_eq!(
        parser.screen().title_formatted(),
        &b"\x1b]0;it's a title\x07"[..]
    );
    assert_eq!(
        parser.screen().title_diff(&screen),
        &b"\x1b]0;it's a title\x07"[..]
    );

    let screen = parser.screen().clone();
    parser.process(b"\x1b]1;it's an icon name\x07");
    assert_eq!(
        parser.screen().title_formatted(),
        &b"\x1b]1;it's an icon name\x07\x1b]2;it's a title\x07"[..]
    );
    assert_eq!(
        parser.screen().title_diff(&screen),
        &b"\x1b]1;it's an icon name\x07"[..]
    );
}

#[test]
fn unknown_sequence() {
    let mut parser = vt100::Parser::default();
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "");
    parser.process(b"\x1b]499;some long, long string?\x07");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "");
}
