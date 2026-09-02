mod helpers;

#[test]
fn ascii() {
    helpers::fixture("ascii");
}

#[test]
fn utf8() {
    helpers::fixture("utf8");
}

#[test]
fn newlines() {
    helpers::fixture("newlines");
}

#[test]
fn wide() {
    helpers::fixture("wide");
}

#[test]
fn combining() {
    helpers::fixture("combining");
}

#[test]
fn wrap() {
    helpers::fixture("wrap");
}

#[test]
fn wrap_weird() {
    helpers::fixture("wrap_weird");
}

/// U+17D8 KHMER SIGN BEYYAL is reported by `unicode-width` as three columns
/// wide. This crate draws every character wider than one column in exactly
/// two columns, so it has to treat this one as two columns wide too.
const THREE_COLUMNS: &str = "\u{17d8}";

#[test]
fn a_character_wider_than_two_columns_is_drawn_in_two_columns() {
    let mut parser = helpers::new(2, 10, 0);
    parser.process(format!("a{THREE_COLUMNS}b").as_bytes());
    let screen = parser.screen();
    assert_eq!(screen.contents(), "a\u{17d8}b");
    assert_eq!(screen.cursor_position(), (0, 4));
    assert!(screen.cell(0, 1).unwrap().is_wide());
    assert!(screen.cell(0, 2).unwrap().is_wide_continuation());
    assert_eq!(screen.cell(0, 3).unwrap().contents(), "b");
}

#[test]
fn a_character_wider_than_two_columns_fits_on_a_two_column_screen() {
    let mut parser = helpers::new(1, 2, 0);
    parser.process(THREE_COLUMNS.as_bytes());
    assert_eq!(parser.screen().contents(), "\u{17d8}");
    assert!(parser.screen().cell(0, 0).unwrap().is_wide());
    assert!(parser.screen().cell(0, 1).unwrap().is_wide_continuation());
}

#[test]
fn a_character_wider_than_two_columns_is_discarded_on_a_one_column_screen() {
    // Two columns is the narrowest screen it can be drawn on, the same as any
    // other wide character.
    let mut parser = helpers::new(1, 1, 0);
    parser.process(THREE_COLUMNS.as_bytes());
    assert_eq!(parser.screen().contents(), "");
    assert_eq!(parser.screen().cursor_position(), (0, 0));
}

#[test]
fn a_character_wider_than_two_columns_only_wraps_after_two_columns() {
    // Three columns is enough room for `a` plus a two column character, so
    // nothing wraps onto the second row.
    let mut parser = helpers::new(2, 3, 0);
    parser.process(format!("a{THREE_COLUMNS}").as_bytes());
    assert_eq!(parser.screen().contents(), "a\u{17d8}");
    assert_eq!(parser.screen().cursor_position(), (0, 3));
    assert!(!parser.screen().row_wrapped(0));
}

#[test]
fn a_character_wider_than_two_columns_wraps_when_two_columns_dont_fit() {
    let mut parser = helpers::new(2, 3, 0);
    parser.process(format!("abc{THREE_COLUMNS}").as_bytes());
    assert!(parser.screen().row_wrapped(0));
    assert_eq!(parser.screen().contents(), "abc\u{17d8}");
    // Both of its columns are on the second row; it isn't split across the
    // wrap.
    assert!(parser.screen().cell(1, 0).unwrap().is_wide());
    assert!(parser.screen().cell(1, 1).unwrap().is_wide_continuation());
}

#[test]
fn a_character_wider_than_two_columns_round_trips() {
    assert!(helpers::contents_formatted_reproduces_state(
        format!("a{THREE_COLUMNS}b").as_bytes()
    ));
    assert!(helpers::rows_formatted_reproduces_state(
        format!("a{THREE_COLUMNS}b").as_bytes()
    ));
}
