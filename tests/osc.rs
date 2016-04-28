extern crate vt100;

mod support;
use support::TestHelpers;

#[test]
fn title() {
    let mut screen = vt100::Screen::new(24, 80);
    assert!(screen.title().is_none());
    assert!(screen.icon_name().is_none());
    screen.assert_process(b"\x1b]2;it's a title\x07");
    assert_eq!(screen.title().unwrap(), "it's a title");
    assert!(screen.icon_name().is_none());
    screen.assert_process(b"\x1b]2;\x07");
    assert_eq!(screen.title().unwrap(), "");
    assert!(screen.icon_name().is_none());
}

#[test]
fn icon_name() {
    let mut screen = vt100::Screen::new(24, 80);
    assert!(screen.title().is_none());
    assert!(screen.icon_name().is_none());
    screen.assert_process(b"\x1b]1;it's an icon name\x07");
    assert!(screen.title().is_none());
    assert_eq!(screen.icon_name().unwrap(), "it's an icon name");
    screen.assert_process(b"\x1b]1;\x07");
    assert!(screen.title().is_none());
    assert_eq!(screen.icon_name().unwrap(), "");
}

#[test]
fn title_icon_name() {
    let mut screen = vt100::Screen::new(24, 80);
    assert!(screen.title().is_none());
    assert!(screen.icon_name().is_none());
    screen.assert_process(b"\x1b]0;it's both\x07");
    assert_eq!(screen.title().unwrap(), "it's both");
    assert_eq!(screen.icon_name().unwrap(), "it's both");
    screen.assert_process(b"\x1b]0;\x07");
    assert_eq!(screen.title().unwrap(), "");
    assert_eq!(screen.icon_name().unwrap(), "");
}

#[test]
fn unknown_sequence() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "");
    screen.assert_process(b"\x1b]499;some long, long string?\x07");
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "");
}
