#[test]
fn title() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.title(), "");
    assert_eq!(screen.icon_name(), "");
    screen.process(b"\x1b]2;it's a title\x07");
    assert_eq!(screen.title(), "it's a title");
    assert_eq!(screen.icon_name(), "");
    screen.process(b"\x1b]2;\x07");
    assert_eq!(screen.title(), "");
    assert_eq!(screen.icon_name(), "");
}

#[test]
fn icon_name() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.title(), "");
    assert_eq!(screen.icon_name(), "");
    screen.process(b"\x1b]1;it's an icon name\x07");
    assert_eq!(screen.title(), "");
    assert_eq!(screen.icon_name(), "it's an icon name");
    screen.process(b"\x1b]1;\x07");
    assert_eq!(screen.title(), "");
    assert_eq!(screen.icon_name(), "");
}

#[test]
fn title_icon_name() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.title(), "");
    assert_eq!(screen.icon_name(), "");
    screen.process(b"\x1b]0;it's both\x07");
    assert_eq!(screen.title(), "it's both");
    assert_eq!(screen.icon_name(), "it's both");
    screen.process(b"\x1b]0;\x07");
    assert_eq!(screen.title(), "");
    assert_eq!(screen.icon_name(), "");
}

#[test]
fn unknown_sequence() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "");
    screen.process(b"\x1b]499;some long, long string?\x07");
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "");
}
