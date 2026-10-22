mod helpers;

#[test]
fn modes() {
    helpers::fixture("modes");
}

#[test]
fn alternate_buffer() {
    helpers::fixture("alternate_buffer");
}

#[test]
fn normal_screen_rows_read_under_the_alternate_screen() {
    let mut parser = helpers::new(3, 10, 0);
    let text = |cells: Option<&[vt100::Cell]>| -> Option<String> {
        cells.map(|c| c.iter().map(vt100::Cell::contents).collect())
    };
    parser.process(b"normal\r\n0123456789wrap");
    assert_eq!(parser.screen().normal_screen_row_cells(0), None);
    assert!(!parser.screen().normal_screen_row_wrapped(1));

    parser.process(b"\x1b[?1049h\x1b[H\x1b[Jalt");
    let screen = parser.screen();
    assert_eq!(
        text(screen.normal_screen_row_cells(0)).as_deref(),
        Some("normal")
    );
    assert_eq!(
        text(screen.normal_screen_row_cells(1)).as_deref(),
        Some("0123456789")
    );
    assert!(screen.normal_screen_row_wrapped(1));
    assert!(!screen.normal_screen_row_wrapped(0));
    assert_eq!(
        text(screen.normal_screen_row_cells(2)).as_deref(),
        Some("wrap")
    );
    assert_eq!(screen.normal_screen_row_cells(3), None);

    parser.process(b"\x1b[?1049l");
    assert_eq!(parser.screen().normal_screen_row_cells(0), None);
}
