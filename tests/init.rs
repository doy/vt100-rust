#![allow(clippy::cognitive_complexity)]

#[test]
fn init() {
    let mut screen = vt100::Screen::new(24, 80);
    assert_eq!(screen.window_size(), (24, 80));
    assert_eq!(screen.cursor_position(), (0, 0));

    let cell = screen.cell(0, 0);
    assert_eq!(cell.unwrap().contents(), "");
    let cell = screen.cell(23, 79);
    assert_eq!(cell.unwrap().contents(), "");
    let cell = screen.cell(24, 0);
    assert!(cell.is_none());
    let cell = screen.cell(0, 80);
    assert!(cell.is_none());

    assert_eq!(
        screen.window_contents(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );
    assert_eq!(
        screen.window_contents_formatted(0, 0, 23, 79),
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    );

    assert_eq!(screen.title(), "");
    assert_eq!(screen.icon_name(), "");

    assert_eq!(screen.fgcolor(), vt100::Color::Default);
    assert_eq!(screen.bgcolor(), vt100::Color::Default);

    assert!(!screen.bold());
    assert!(!screen.italic());
    assert!(!screen.underline());
    assert!(!screen.inverse());

    assert!(!screen.check_visual_bell());
    assert!(!screen.check_audible_bell());
    assert!(!screen.application_keypad());
    assert!(!screen.application_cursor());
    assert!(!screen.hide_cursor());
    assert!(!screen.bracketed_paste());
    assert_eq!(screen.mouse_protocol_mode(), vt100::MouseProtocolMode::None);
    assert_eq!(
        screen.mouse_protocol_encoding(),
        vt100::MouseProtocolEncoding::Default
    );
}
