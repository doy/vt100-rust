mod helpers;

#[test]
fn object_creation() {
    let parser = vt100::Parser::default();
    assert_eq!(helpers::size(parser.screen()), (24, 80));
}

#[test]
fn process_text() {
    let mut parser = vt100::Parser::default();
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    parser.process(input);
    assert_eq!(parser.screen().contents(), "foobar");
}

#[test]
fn set_size() {
    let mut parser = vt100::Parser::default();
    assert_eq!(helpers::size(parser.screen()), (24, 80));
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    helpers::set_size(parser.screen_mut(), 34, 8);
    assert_eq!(helpers::size(parser.screen()), (34, 8));
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[30;5H");
    assert_eq!(parser.screen().cursor_position(), (29, 4));

    helpers::set_size(parser.screen_mut(), 24, 80);
    assert_eq!(helpers::size(parser.screen()), (24, 80));
    assert_eq!(parser.screen().cursor_position(), (23, 4));

    // The six rows that were pushed off the top to make the screen shorter
    // come back at the top when it grows again, even though this parser has
    // no scrollback to hold them, so the cursor moves back down with them.
    helpers::set_size(parser.screen_mut(), 34, 8);
    assert_eq!(helpers::size(parser.screen()), (34, 8));
    assert_eq!(parser.screen().cursor_position(), (29, 4));

    parser.process(b"\x1b[?1049h");
    assert_eq!(helpers::size(parser.screen()), (34, 8));
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    helpers::set_size(parser.screen_mut(), 24, 80);
    assert_eq!(helpers::size(parser.screen()), (24, 80));
    assert_eq!(parser.screen().cursor_position(), (0, 0));

    parser.process(b"\x1b[?1049l");
    assert_eq!(helpers::size(parser.screen()), (24, 80));
    assert_eq!(parser.screen().cursor_position(), (23, 4));

    helpers::set_size(parser.screen_mut(), 34, 8);
    parser.process(b"\x1bc01234567890123456789");
    assert_eq!(parser.screen().contents(), "01234567890123456789");

    helpers::set_size(parser.screen_mut(), 24, 80);
    assert_eq!(parser.screen().contents(), "01234567\n89012345\n6789");

    helpers::set_size(parser.screen_mut(), 34, 8);
    assert_eq!(parser.screen().contents(), "01234567\n89012345\n6789");

    let mut parser = vt100::Parser::default();
    assert_eq!(helpers::size(parser.screen()), (24, 80));
    helpers::set_size(parser.screen_mut(), 30, 100);
    assert_eq!(helpers::size(parser.screen()), (30, 100));
    parser.process(b"\x1b[75Cfoobar");
    assert_eq!(
        parser.screen().contents(),
        "                                                                           foobar"
    );

    let mut parser = vt100::Parser::default();
    assert_eq!(helpers::size(parser.screen()), (24, 80));
    helpers::set_size(parser.screen_mut(), 30, 100);
    assert_eq!(helpers::size(parser.screen()), (30, 100));
    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24\x1b[24;99Hfoobar");
    assert_eq!(
        parser.screen().contents(),
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24                                                                                                foobar"
    );
}

/// The rows that don't fit on a shrinking screen are pushed off the top and
/// into the scrollback, and pulled back out when the screen grows again.
#[test]
fn set_size_shrink_pushes_rows_into_the_scrollback() {
    let mut parser = helpers::new(6, 10, 10);
    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6");
    assert_eq!(parser.screen().cursor_position(), (5, 1));

    helpers::set_size(parser.screen_mut(), 3, 10);
    assert_eq!(parser.screen().contents(), "4\n5\n6");
    assert_eq!(parser.screen().cursor_position(), (2, 1));

    // The rows that were pushed off the top are in the scrollback.
    parser.screen_mut().set_scrollback(3);
    assert_eq!(parser.screen().contents(), "1\n2\n3");
    parser.screen_mut().set_scrollback(0);

    // Growing the screen again pulls them back out of the scrollback.
    helpers::set_size(parser.screen_mut(), 6, 10);
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6");
    assert_eq!(parser.screen().cursor_position(), (5, 1));
    assert_eq!(parser.screen().scrollback(), 0);
}

/// Rows are only pushed off the top of a shrinking screen when they have to
/// be to keep the cursor on the screen; otherwise the excess rows are removed
/// from the bottom, leaving the visible contents where they are.
#[test]
fn set_size_shrink_keeps_the_contents_of_a_screen_that_isnt_full() {
    let mut parser = helpers::new(6, 10, 10);
    parser.process(b"1\r\n2\r\n3");
    assert_eq!(parser.screen().cursor_position(), (2, 1));

    helpers::set_size(parser.screen_mut(), 4, 10);
    assert_eq!(parser.screen().contents(), "1\n2\n3");
    assert_eq!(parser.screen().cursor_position(), (2, 1));
    // Nothing was pushed into the scrollback.
    parser.screen_mut().set_scrollback(3);
    assert_eq!(parser.screen().scrollback(), 0);

    // Shrinking past the cursor starts pushing rows off the top again.
    helpers::set_size(parser.screen_mut(), 2, 10);
    assert_eq!(parser.screen().contents(), "2\n3");
    assert_eq!(parser.screen().cursor_position(), (1, 1));
    parser.screen_mut().set_scrollback(1);
    assert_eq!(parser.screen().contents(), "1\n2");
}

/// When fewer rows have scrolled off the screen than are needed to fill out a
/// growing screen, the remaining rows are added at the bottom, so the visible
/// contents stay where they are.
#[test]
fn set_size_grow_adds_blank_rows_at_the_bottom() {
    let mut parser = helpers::new(3, 10, 10);
    parser.process(b"1\r\n2\r\n3\r\n4");
    // One row is in the scrollback at this point.
    assert_eq!(parser.screen().contents(), "2\n3\n4");
    assert_eq!(parser.screen().cursor_position(), (2, 1));

    helpers::set_size(parser.screen_mut(), 6, 10);
    // The single scrollback row moved back onto the screen, pushing the
    // contents down one row, and the other two new rows are blank ones at the
    // bottom.
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4");
    assert_eq!(parser.screen().cursor_position(), (3, 1));

    // When nothing has scrolled off, growing doesn't move the contents at all.
    let mut parser = helpers::new(6, 10, 0);
    parser.process(b"1\r\n2");
    helpers::set_size(parser.screen_mut(), 10, 10);
    assert_eq!(parser.screen().contents(), "1\n2");
    assert_eq!(parser.screen().cursor_position(), (1, 1));
}

/// When the scrollback is too small to have held every row that scrolled off
/// the screen, growing the screen still adds rows at the top; the rows the
/// scrollback no longer holds are replaced with blank ones above the rows it
/// does hold, since they're the older ones.
#[test]
fn set_size_grow_with_a_small_scrollback() {
    let mut parser = helpers::new(2, 10, 1);
    parser.process(b"1\r\n2\r\n3\r\n4");
    // Two rows scrolled off, but the scrollback only held on to `2`.
    assert_eq!(parser.screen().contents(), "3\n4");
    parser.screen_mut().set_scrollback(1);
    assert_eq!(parser.screen().contents(), "2\n3");
    parser.screen_mut().set_scrollback(0);

    helpers::set_size(parser.screen_mut(), 4, 10);
    // Both rows come back at the top, with a blank row standing in for `1`.
    assert_eq!(parser.screen().contents(), "\n2\n3\n4");
    assert_eq!(parser.screen().cursor_position(), (3, 1));
    assert_eq!(parser.screen().scrollback(), 0);

    // With no scrollback at all, every row that comes back is blank.
    let mut parser = helpers::new(2, 10, 0);
    parser.process(b"1\r\n2\r\n3\r\n4");
    assert_eq!(parser.screen().contents(), "3\n4");
    helpers::set_size(parser.screen_mut(), 4, 10);
    assert_eq!(parser.screen().contents(), "\n\n3\n4");
    assert_eq!(parser.screen().cursor_position(), (3, 1));

    // Growing further than the rows that scrolled off still adds the extra
    // rows at the bottom.
    helpers::set_size(parser.screen_mut(), 6, 10);
    assert_eq!(parser.screen().contents(), "\n\n3\n4");
    assert_eq!(parser.screen().cursor_position(), (3, 1));
}

/// Rows can be pushed out of the scrollback entirely when the screen shrinks.
#[test]
fn set_size_shrink_with_a_small_scrollback() {
    let mut parser = helpers::new(6, 10, 2);
    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6");

    helpers::set_size(parser.screen_mut(), 2, 10);
    assert_eq!(parser.screen().contents(), "5\n6");
    // Only the last two of the four rows that were pushed off the top fit in
    // the scrollback; `1` and `2` are gone.
    parser.screen_mut().set_scrollback(2);
    assert_eq!(parser.screen().contents(), "3\n4");
}

/// Resizing always resets the scroll region and the scrollback offset, which
/// is what xterm does.
#[test]
fn set_size_resets_the_scroll_region_and_scrollback_offset() {
    let mut parser = helpers::new(6, 10, 10);
    parser.process(b"\x1b[2;4r");
    helpers::set_size(parser.screen_mut(), 6, 20);
    // The scroll region is gone, so writing past the bottom row scrolls the
    // whole screen.
    parser.process(b"\x1b[6;1Hx\r\ny");
    assert_eq!(parser.screen().contents(), "\n\n\n\nx\ny");

    let mut parser = helpers::new(3, 10, 10);
    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6");
    parser.screen_mut().set_scrollback(2);
    assert_eq!(parser.screen().scrollback(), 2);
    helpers::set_size(parser.screen_mut(), 4, 10);
    assert_eq!(parser.screen().scrollback(), 0);
}

/// Rows in the scrollback keep their old width until they're pulled back onto
/// the screen, at which point they're resized like any other row.
#[test]
fn set_size_resizes_scrollback_rows_when_they_come_back_onto_the_screen() {
    let mut parser = helpers::new(2, 10, 10);
    parser.process(b"aaaaaaaaaa\r\nbbbbbbbbbb\r\ncccccccccc");
    helpers::set_size(parser.screen_mut(), 2, 5);

    // The scrollback row is still 10 columns wide while it's in the
    // scrollback, so it has cells past the last column of the screen, even
    // though only the first 5 columns of it are displayed.
    parser.screen_mut().set_scrollback(1);
    assert_eq!(parser.screen().contents(), "aaaaa\nbbbbb");
    assert!(parser.screen().cell(0, 5).is_some());
    parser.screen_mut().set_scrollback(0);

    // Growing the screen pulls it back out of the scrollback, which truncates
    // it to the width of the screen.
    helpers::set_size(parser.screen_mut(), 3, 5);
    assert_eq!(parser.screen().contents(), "aaaaa\nbbbbb\nccccc");
    assert!(parser.screen().cell(0, 4).is_some());
    assert!(parser.screen().cell(0, 5).is_none());
}

/// Changing only the height leaves the `wrapped` flag of each row alone.
#[test]
fn set_size_height_only_preserves_wrapping() {
    let mut parser = helpers::new(3, 5, 0);
    parser.process(b"abcdefg");
    assert!(parser.screen().row_wrapped(0));

    helpers::set_size(parser.screen_mut(), 6, 5);
    assert!(parser.screen().row_wrapped(0));
    assert_eq!(parser.screen().contents(), "abcdefg");

    // Changing the width clears it, because the rows aren't reflowed.
    helpers::set_size(parser.screen_mut(), 6, 8);
    assert!(!parser.screen().row_wrapped(0));
    assert_eq!(parser.screen().contents(), "abcde\nfg");
}

/// Removing rows from the bottom of the screen can't leave the new bottom row
/// marked as wrapped, because there's nothing below it left to wrap onto.
#[test]
fn set_size_shrink_unwraps_the_new_bottom_row() {
    let mut parser = helpers::new(4, 3, 0);
    parser.process(b"abcdef\x1b[H");
    assert!(parser.screen().row_wrapped(0));

    // The cursor is at the top, so the three bottom rows are removed and the
    // wrapped row becomes the bottom row.
    helpers::set_size(parser.screen_mut(), 1, 3);
    assert_eq!(parser.screen().contents(), "abc");
    assert!(!parser.screen().row_wrapped(0));
    // A wrapped bottom row can't be written out, so leaving the flag set
    // would make the screen impossible to reproduce.
    assert!(helpers::contents_formatted_reproduces_sized_screen(
        parser.screen()
    ));
}

/// The saved cursor moves with the screen contents, and stays in bounds.
#[test]
fn set_size_moves_the_saved_cursor() {
    let mut parser = helpers::new(6, 10, 10);
    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6\x1b[5;9H\x1b7\x1b[6;1H");

    // Two rows are pushed off the top to keep the cursor on the screen, so
    // the saved cursor moves up two rows too, and is clamped to the new
    // width.
    helpers::set_size(parser.screen_mut(), 4, 5);
    parser.process(b"\x1b8");
    assert_eq!(parser.screen().cursor_position(), (2, 4));

    // The saved cursor is clamped to the last row when the rows it was on are
    // removed from the bottom of the screen.
    let mut parser = helpers::new(6, 10, 10);
    parser.process(b"\x1b[6;1H\x1b7\x1b[H");
    helpers::set_size(parser.screen_mut(), 2, 10);
    parser.process(b"\x1b8");
    assert_eq!(parser.screen().cursor_position(), (1, 0));
}

/// The alternate screen is resized the same way as the main screen, and the
/// two are resized independently of each other.
#[test]
fn set_size_alternate_screen() {
    let mut parser = helpers::new(4, 10, 10);
    parser.process(b"1\r\n2\r\n3\r\n4");
    parser.process(b"\x1b[?1049halt1\r\nalt2\r\nalt3\r\nalt4");

    helpers::set_size(parser.screen_mut(), 2, 10);
    // The alternate screen has no scrollback, so the rows pushed off the top
    // of it are simply gone.
    assert_eq!(parser.screen().contents(), "alt3\nalt4");
    assert_eq!(parser.screen().cursor_position(), (1, 4));

    parser.process(b"\x1b[?1049l");
    assert_eq!(parser.screen().contents(), "3\n4");
    assert_eq!(parser.screen().cursor_position(), (1, 1));
    // The main screen kept its rows in its own scrollback.
    parser.screen_mut().set_scrollback(2);
    assert_eq!(parser.screen().contents(), "1\n2");
}

#[test]
fn cell_contents() {
    let mut parser = vt100::Parser::default();
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    parser.process(input);
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "f");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "o");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "b");
    assert_eq!(parser.screen().cell(0, 4).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(0, 5).unwrap().contents(), "r");
    assert_eq!(parser.screen().cell(0, 6).unwrap().contents(), "");
}

#[test]
fn cell_colors() {
    let mut parser = vt100::Parser::default();
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    parser.process(input);

    assert_eq!(
        parser.screen().cell(0, 0).unwrap().fgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(0, 3).unwrap().fgcolor(),
        vt100::Color::Idx(2)
    );
    assert_eq!(
        parser.screen().cell(0, 4).unwrap().fgcolor(),
        vt100::Color::Idx(2)
    );
    assert_eq!(
        parser.screen().cell(0, 4).unwrap().bgcolor(),
        vt100::Color::Idx(2)
    );
}

#[test]
fn cell_attrs() {
    let mut parser = vt100::Parser::default();
    let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
    parser.process(input);

    assert!(parser.screen().cell(0, 4).unwrap().italic());
}

// Regression test for the panic where shrinking a screen truncates the
// continuation cell of a wide character, then erasing the line tries
// to clear past the end of the row. The repro is from
// https://github.com/doy/vt100-rust/issues/28.
#[test]
fn resize_truncating_wide_char_then_erase_line_does_not_panic() {
    let mut parser = helpers::new(2, 4, 0);
    parser.process("你".as_bytes());
    helpers::set_size(parser.screen_mut(), 2, 1);
    parser.process(b"\x1b[K");
    assert_eq!(helpers::size(parser.screen()), (2, 1));
}
