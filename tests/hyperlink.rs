mod helpers;

fn link(
    screen: &vt100::Screen,
    row: u16,
    col: u16,
) -> Option<(String, Option<String>)> {
    let id = screen.cell(row, col)?.hyperlink_id()?;
    let h = screen.hyperlink(id)?;
    Some((h.uri.clone(), h.id.clone()))
}

#[test]
fn osc_8_links_the_text_written_under_it() {
    let mut parser = helpers::new(3, 20, 0);
    parser.process(b"a\x1b]8;;https://e.x/1\x1b\\bc\x1b]8;;\x1b\\d");
    let screen = parser.screen();
    let one = Some(("https://e.x/1".to_owned(), None));
    assert_eq!(link(screen, 0, 0), None);
    assert_eq!(link(screen, 0, 1), one);
    assert_eq!(link(screen, 0, 2), one);
    assert_eq!(link(screen, 0, 3), None, "an empty uri ends it");
}

#[test]
fn a_link_keeps_its_id_and_a_uri_its_semicolons() {
    let mut parser = helpers::new(3, 20, 0);
    parser.process(b"\x1b]8;id=x:k=v;file:///a;b\x07z\x1b]8;;\x07");
    assert_eq!(
        link(parser.screen(), 0, 0),
        Some(("file:///a;b".to_owned(), Some("x".to_owned())))
    );
}

#[test]
fn erasing_drops_a_link_and_a_wide_character_carries_it_in_both_halves() {
    let mut parser = helpers::new(3, 20, 0);
    parser.process("\x1b]8;;u\x07中x\x1b]8;;\x07".as_bytes());
    let screen = parser.screen();
    assert!(link(screen, 0, 0).is_some());
    assert!(
        link(screen, 0, 1).is_some(),
        "the wide character's other half"
    );
    assert!(link(screen, 0, 2).is_some());
    parser.process(b"\x1b[1;3H\x1b[K");
    assert_eq!(link(parser.screen(), 0, 2), None);
}

#[test]
fn the_same_link_is_kept_once() {
    let mut parser = helpers::new(3, 20, 0);
    parser.process(b"\x1b]8;;u\x07a\x1b]8;;\x07b\x1b]8;;u\x07c\x1b]8;;\x07");
    let screen = parser.screen();
    assert_eq!(
        screen.cell(0, 0).unwrap().hyperlink_id(),
        screen.cell(0, 2).unwrap().hyperlink_id()
    );
    assert_eq!(screen.cell(0, 1).unwrap().hyperlink_id(), None);
}

#[test]
fn cleared_links_link_nothing_and_their_numbers_stay_unused() {
    let mut parser = helpers::new(3, 20, 0);
    parser.process(b"\x1b]8;;u\x07a\x1b]8;;\x07");
    let old = parser.screen().cell(0, 0).unwrap().hyperlink_id();
    assert!(old.is_some());
    parser.screen_mut().clear_hyperlinks();
    assert_eq!(link(parser.screen(), 0, 0), None);
    parser.process(b"\x1b]8;;u\x07b\x1b]8;;\x07");
    let screen = parser.screen();
    assert_eq!(link(screen, 0, 1), Some(("u".to_owned(), None)));
    assert_ne!(screen.cell(0, 1).unwrap().hyperlink_id(), old);
    assert_eq!(
        link(screen, 0, 0),
        None,
        "the old cell does not take the new link"
    );
}
