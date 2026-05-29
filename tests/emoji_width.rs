// Regression tests for emoji-presentation width: a base char + U+FE0F
// (VS16) is two columns, but vt100 used to store it as one narrow cell,
// shifting everything after it and leaving on-screen residue.

fn cells(parser: &vt100::Parser, n: u16) -> Vec<(String, bool, bool)> {
    let screen = parser.screen();
    (0..n)
        .map(|col| {
            let c = screen.cell(0, col).unwrap();
            (
                c.contents().to_string(),
                c.is_wide(),
                c.is_wide_continuation(),
            )
        })
        .collect()
}

#[test]
fn vs16_emoji_occupies_two_columns() {
    let mut parser = vt100::Parser::new(1, 20, 0);
    parser.process("A\u{2764}\u{FE0F}B".as_bytes());

    let c = cells(&parser, 5);
    assert_eq!(c[0], ("A".into(), false, false), "col0 = A");
    assert_eq!(
        c[1],
        ("\u{2764}\u{FE0F}".into(), true, false),
        "col1 = ❤️ and must be WIDE"
    );
    assert_eq!(
        c[2],
        (String::new(), false, true),
        "col2 must be a wide continuation"
    );
    assert_eq!(c[3], ("B".into(), false, false), "col3 = B (not shifted)");
    assert_eq!(parser.screen().cursor_position(), (0, 4));
}

#[test]
fn vs16_promotion_over_existing_wide_char_does_not_orphan_continuation() {
    let mut parser = vt100::Parser::new(1, 6, 0);
    parser.process("a\u{4E2D}".as_bytes());
    // Move the cursor back to col0 and write a text-presentation base + VS16.
    // The base lands on col0; its continuation lands on col1, which is the
    // first half of 中 — so promotion must also clear 中's now-stranded
    // continuation at col2 instead of leaving it orphaned.
    parser.process(b"\x1b[1;1H");
    parser.process("\u{2764}\u{FE0F}".as_bytes());

    let c = cells(&parser, 4);
    assert_eq!(
        c[0],
        ("\u{2764}\u{FE0F}".into(), true, false),
        "col0 = ❤️ wide"
    );
    assert_eq!(c[1].2, true, "col1 = ❤️ continuation");
    assert!(
        !c[2].2,
        "col2 (clobbered 中's old continuation) must be cleared, not orphaned"
    );
}
