//! Tests for capturing terminal data in a streaming fashion via
//! [`vt100::Callbacks::on_scroll`].
//!
//! The intended use is to create a [`vt100::Parser`] with no scrollback and
//! append each row to a buffer as it scrolls off the top of the screen, and
//! then, at the end, append the visible contents of the terminal to the same
//! buffer. The result is the same as if the data had been processed by a
//! terminal tall enough to hold all of it at once.

use vt100::capture::{
    BasicFormattedCaptureRange, BasicFormattedCaptureState, RowContents,
    basic_formatted_rows, basic_formatted_to_plain,
};

mod helpers;

/// Appends every row that scrolls off the screen to a buffer.
#[derive(Debug, Default)]
struct Capture {
    buf: String,
    state: BasicFormattedCaptureState,
    /// Rows that scrolled off the alternate screen.
    alternate: Vec<String>,
}

impl vt100::Callbacks for Capture {
    fn on_scroll(&mut self, contents: RowContents<'_>, alternate: bool) {
        let (buf, state) = if alternate {
            self.alternate.push(String::new());
            // Store each row from the alternate screen separately, with a
            // fresh state.
            (self.alternate.last_mut().unwrap(), &mut Default::default())
        } else {
            (&mut self.buf, &mut self.state)
        };
        contents.write_formatted_basic(buf, state).unwrap();
    }
}

fn parser(rows: u16, cols: u16) -> vt100::Parser<Capture> {
    helpers::new_with_callbacks(rows, cols, 0, Capture::default())
}

/// Returns the capture so far, without the visible contents of the screen.
fn scrolled(parser: &vt100::Parser<Capture>) -> &str {
    &parser.callbacks().buf
}

/// Finishes a capture by appending the visible contents of the screen to the
/// rows which have already scrolled off of it.
fn finish(parser: &mut vt100::Parser<Capture>) -> String {
    let mut capture = std::mem::take(parser.callbacks_mut());
    parser
        .screen()
        .write_contents_formatted_basic(
            &mut capture.buf,
            BasicFormattedCaptureRange::Full(&mut capture.state),
        )
        .unwrap();
    capture.buf
}

/// Processes `data` with a screen tall enough that nothing scrolls off of it,
/// which is what a capture of the same data is expected to reproduce.
fn tall(rows: u16, cols: u16, data: &[u8]) -> String {
    let mut parser = helpers::new(rows, cols, 0);
    parser.process(data);
    let mut contents = parser.screen().contents_formatted_basic();
    // Drop the blank rows below the ones that were actually written to.
    let trimmed = contents.trim_end_matches('\n');
    contents.truncate(trimmed.len());
    contents
}

#[test]
fn nothing_is_captured_until_the_screen_scrolls() {
    let mut parser = parser(3, 10);
    parser.process(b"one\r\ntwo\r\nthree");
    assert_eq!(scrolled(&parser), "");
    assert_eq!(finish(&mut parser), "one\ntwo\nthree");
}

#[test]
fn rows_scrolled_off_the_screen_are_captured() {
    let mut parser = parser(3, 10);
    parser.process(b"one\r\ntwo\r\nthree\r\nfour\r\nfive");
    assert_eq!(parser.screen().contents(), "three\nfour\nfive");
    // The two rows that are no longer on the screen were captured as they
    // scrolled off, and aren't followed by a newline until something is
    // written after them.
    assert_eq!(scrolled(&parser), "one\ntwo");
    assert_eq!(finish(&mut parser), "one\ntwo\nthree\nfour\nfive");
}

#[test]
fn blank_rows_scrolled_off_the_screen_are_captured() {
    let mut parser = parser(3, 10);
    parser.process(b"one\r\n\r\n\r\n\r\ntwo");
    assert_eq!(finish(&mut parser), "one\n\n\n\ntwo");
}

#[test]
fn rows_scrolled_off_by_an_explicit_scroll_up_are_captured() {
    let mut parser = parser(4, 10);
    parser.process(b"one\r\ntwo\r\nthree\x1b[2S");
    assert_eq!(scrolled(&parser), "one\ntwo");
    // `three` moved to the top of the screen and the three rows below it are
    // blank.
    assert_eq!(finish(&mut parser), "one\ntwo\nthree\n\n\n");
}

#[test]
fn attributes_carry_over_from_scrolled_rows_to_visible_rows() {
    let mut parser = parser(3, 10);
    parser.process(b"\x1b[31mone\r\ntwo\r\nthree\r\nfour");
    // The color is written out once, in the row that scrolled off; the rows
    // still on the screen inherit it rather than repeating it.
    assert_eq!(finish(&mut parser), "\x1b[31mone\ntwo\nthree\nfour");
}

#[test]
fn attributes_carry_over_between_scrolled_rows() {
    let mut parser = parser(3, 10);
    parser.process(b"\x1b[32mone\r\ntwo\r\n\x1b[mthree\r\nfour\r\nfive");
    assert_eq!(scrolled(&parser), "\x1b[32mone\ntwo");
    assert_eq!(
        finish(&mut parser),
        "\x1b[32mone\ntwo\n\x1b[mthree\nfour\nfive"
    );
}

#[test]
fn a_row_that_scrolls_off_while_wrapped_isnt_split_by_a_newline() {
    let mut parser = parser(3, 5);
    parser.process(b"hello world");
    assert!(parser.screen().row_wrapped(0));
    parser.process(b"\r\nnext");
    // The first row scrolled off, but it wraps onto the second row, which is
    // still visible, so no newline separates them.
    assert_eq!(scrolled(&parser), "hello");
    assert_eq!(finish(&mut parser), "hello world\nnext");
}

#[test]
fn a_wrapped_row_is_rejoined_after_scrolling_off_entirely() {
    let mut parser = parser(3, 5);
    parser.process(b"hello world\r\nnext\r\nlast\r\nafter");
    // Both halves of the wrapped row have scrolled off, and they're rejoined
    // in the capture without a newline between them.
    assert_eq!(scrolled(&parser), "hello world");
    assert_eq!(finish(&mut parser), "hello world\nnext\nlast\nafter");
}

#[test]
fn wide_characters_that_scroll_off_are_captured_once() {
    let mut parser = parser(2, 6);
    parser.process("aあ\x1b[32mbい\r\nsecond\r\nthird".as_bytes());
    assert_eq!(scrolled(&parser), "aあ\x1b[32mbい");
    assert_eq!(finish(&mut parser), "aあ\x1b[32mbい\nsecond\nthird");
}

#[test]
fn erased_cells_that_scroll_off_keep_their_background() {
    let mut parser = parser(2, 4);
    // Erasing the row while a background color is set fills it with cells
    // which aren't empty, so they're captured as spaces with that background.
    parser.process(b"\x1b[41m\x1b[2K\x1b[m\r\ntwo\r\nsix");
    assert_eq!(scrolled(&parser), "\x1b[41m    ");
    assert_eq!(finish(&mut parser), "\x1b[41m    \n\x1b[mtwo\nsix");
}

#[test]
fn rows_scrolled_off_the_alternate_screen_are_reported_separately() {
    let mut parser = parser(3, 10);
    parser.process(b"one\r\ntwo\r\nthree\r\nfour");
    assert_eq!(scrolled(&parser), "one");

    parser.process(b"\x1b[?1049h");
    parser.process(b"alt1\r\nalt2\r\nalt3\r\nalt4");
    assert_eq!(parser.callbacks().alternate, ["alt1"]);
    // The main screen's capture is untouched by the alternate screen.
    assert_eq!(scrolled(&parser), "one");

    parser.process(b"\x1b[?1049l");
    // Switching back restores the main screen, so the capture picks up
    // exactly where it left off.
    assert_eq!(finish(&mut parser), "one\ntwo\nthree\nfour");
}

#[test]
fn a_capture_matches_a_screen_tall_enough_to_hold_everything() {
    let data: &[u8] = b"\x1b[33mfoo\r\n\x1b[1mbar baz quux\r\n\
        \x1b[mplugh\r\n\x1b[7mxyzzy\r\n\x1b[27m\r\nlast";
    let mut parser = parser(3, 8);
    parser.process(data);
    assert_eq!(finish(&mut parser), tall(20, 8, data));
}

#[test]
fn a_capture_matches_a_tall_screen_when_fed_in_arbitrary_chunks() {
    let data: &[u8] = b"\x1b[36;44malpha beta gamma\r\ndelta\r\n\
        \x1b[3mepsilon zeta\r\n\x1b[23;1meta\r\ntheta iota kappa\r\n\x1b[mmu";
    let expected = tall(20, 7, data);
    for chunk in 1..=data.len() {
        let mut parser = parser(4, 7);
        for bytes in data.chunks(chunk) {
            parser.process(bytes);
        }
        assert_eq!(finish(&mut parser), expected, "chunk size {chunk}");
    }
}

#[test]
fn a_capture_matches_a_tall_screen_for_every_screen_height() {
    let data: &[u8] =
        b"one\r\ntwo three four\r\n\x1b[4mfive\r\n\x1b[24msix\r\n\
        seven eight\r\nnine";
    let expected = tall(20, 6, data);
    for rows in 2..=12 {
        let mut parser = parser(rows, 6);
        parser.process(data);
        let captured = finish(&mut parser);
        assert_eq!(captured.trim_end_matches('\n'), expected, "{rows} rows");
    }
}

#[test]
fn a_capture_of_a_parser_with_scrollback_includes_the_scrollback() {
    let data: &[u8] = b"one\r\ntwo\r\n\x1b[35mthree\r\nfour\r\n\x1b[mfive";
    // `on_scroll` only fires for rows that leave the *scrollback*, so nothing
    // is captured here; the rows that scrolled off the screen are picked up
    // from the scrollback by `finish`.
    let mut parser =
        helpers::new_with_callbacks(3, 10, 10, Capture::default());
    parser.process(data);
    assert_eq!(finish(&mut parser), tall(20, 10, data));

    // The captured rows are the ones that the parser put into its scrollback.
    parser.screen_mut().set_scrollback(2);
    assert_eq!(parser.screen().contents(), "one\ntwo\nthree");
}

#[test]
fn scroll_up_with_small_scrollback() {
    let mut parser =
        helpers::new_with_callbacks(6, 20, 2, Capture::default());
    parser.process(b"1\r\n2\r\n3\r\n4\r\n5\r\n6");
    assert_eq!(parser.screen().contents(), "1\n2\n3\n4\n5\n6");
    assert_eq!(scrolled(&parser), "");
    parser.process(b"\x1b[5S");
    // Five rows scrolled off, but the scrollback only holds the last two of
    // them, so the other three went straight back out of it and were
    // captured.
    assert_eq!(scrolled(&parser), "1\n2\n3");
    parser.screen_mut().set_scrollback(2);
    assert_eq!(parser.screen().contents(), "4\n5\n6");
    parser.screen_mut().set_scrollback(0);
    // The capture still reproduces everything, because `finish` includes the
    // rows that are still in the scrollback.
    assert_eq!(finish(&mut parser), "1\n2\n3\n4\n5\n6\n\n\n\n\n");
}

/// A row that goes into the scrollback and is later pushed out of it is only
/// captured once, when it leaves the scrollback.
#[test]
fn rows_are_captured_when_they_leave_the_scrollback() {
    let mut parser =
        helpers::new_with_callbacks(2, 10, 2, Capture::default());
    parser.process(b"one\r\ntwo\r\nthree");
    // `one` is in the scrollback, not captured yet.
    assert_eq!(scrolled(&parser), "");

    parser.process(b"\r\nfour");
    // `one` and `two` fill the scrollback; still nothing has left it.
    assert_eq!(scrolled(&parser), "");

    parser.process(b"\r\nfive");
    // `one` was pushed out of the scrollback to make room for `three`.
    assert_eq!(scrolled(&parser), "one");
    assert_eq!(finish(&mut parser), "one\ntwo\nthree\nfour\nfive");
}

/// Shrinking the screen with [`vt100::Parser::set_size`] captures the rows
/// that get pushed out of the scrollback as a result.
#[test]
fn rows_pushed_out_of_the_scrollback_by_a_resize_are_captured() {
    let mut parser =
        helpers::new_with_callbacks(6, 10, 2, Capture::default());
    parser.process(b"one\r\ntwo\r\nthree\r\nfour\r\nfive\r\nsix");
    assert_eq!(scrolled(&parser), "");

    helpers::set_parser_size(&mut parser, 2, 10);
    // Four rows were pushed off the top of the screen, and the scrollback
    // only holds two of them.
    assert_eq!(scrolled(&parser), "one\ntwo");
    assert_eq!(parser.screen().contents(), "five\nsix");
    assert_eq!(finish(&mut parser), "one\ntwo\nthree\nfour\nfive\nsix");
}

#[test]
fn a_capture_across_resizes_matches_a_screen_tall_enough_to_hold_everything()
{
    let data: &[u8] = b"\x1b[33mone\r\n\x1b[1mtwo three\r\n\x1b[mfour\r\n\
        \x1b[7mfive\r\n\x1b[27msix\r\nseven";
    let expected = tall(20, 8, data);

    let heights = [(6, 2), (2, 6), (4, 4), (3, 9), (9, 3)];
    for (before, after, scrollback_len) in heights
        .into_iter()
        .flat_map(|(h1, h2)| (0..=10).map(move |len| (h1, h2, len)))
    {
        let mut parser = helpers::new_with_callbacks(
            before,
            8,
            scrollback_len,
            Capture::default(),
        );
        for chunk in data.split_inclusive(|&b| b == b'\n') {
            parser.process(chunk);
        }
        helpers::set_parser_size(&mut parser, after, 8);
        let captured = finish(&mut parser);
        assert_eq!(
            captured.trim_end_matches('\n'),
            expected,
            "{before} rows -> {after} rows"
        );
    }
}

/// Resizing through [`vt100::Screen::set_size`] doesn't call `on_scroll`.
#[test]
fn screen_set_size_does_not_capture_anything() {
    let mut parser = parser(6, 10);
    parser.process(b"one\r\ntwo\r\nthree\r\nfour\r\nfive\r\nsix");
    assert_eq!(scrolled(&parser), "");

    helpers::set_size(parser.screen_mut(), 2, 10);
    assert_eq!(scrolled(&parser), "");
    assert_eq!(parser.screen().contents(), "five\nsix");
}

/// Rows pushed off the top of the alternate screen by a resize are reported
/// as coming from the alternate screen.
#[test]
fn rows_pushed_off_the_alternate_screen_by_a_resize_are_reported_separately()
{
    let mut parser = parser(4, 10);
    parser.process(b"one\r\ntwo\r\nthree\r\nfour\r\nfive");
    parser.process(b"\x1b[?1049halt1\r\nalt2\r\nalt3\r\nalt4");
    assert_eq!(scrolled(&parser), "one");

    helpers::set_parser_size(&mut parser, 2, 10);
    assert_eq!(parser.callbacks().alternate, ["alt1", "alt2"]);
    // The main screen was resized too, and its rows went into the main
    // screen's capture.
    assert_eq!(scrolled(&parser), "one\ntwo\nthree");

    parser.process(b"\x1b[?1049l");
    assert_eq!(parser.screen().contents(), "four\nfive");
    assert_eq!(finish(&mut parser), "one\ntwo\nthree\nfour\nfive");
}

/// Like [`tall`], but returns the plain text contents of the screen.
fn tall_plain(rows: u16, cols: u16, data: &[u8]) -> String {
    let mut parser = helpers::new(rows, cols, 0);
    parser.process(data);
    parser.screen().contents()
}

/// Like [`tall_plain`], but splits the output into rows. A row that wraps is
/// considered two rows.
fn tall_rows(rows: u16, cols: u16, data: &[u8]) -> Vec<String> {
    let mut parser = helpers::new(rows, cols, 0);
    parser.process(data);
    let mut rows: Vec<String> = parser.screen().rows(0, cols).collect();
    trim_trailing_blank_rows(&mut rows);
    rows
}

fn trim_trailing_blank_rows(rows: &mut Vec<String>) {
    while rows.last().is_some_and(String::is_empty) {
        rows.pop();
    }
}

#[test]
fn to_plain_strips_sgr_sequences() {
    let capture = "\x1b[31mfoo\x1b[1;32mbar\x1b[mbaz";
    assert_eq!(
        basic_formatted_to_plain(capture).collect::<String>(),
        "foobarbaz"
    );
}

#[test]
fn to_plain_leaves_text_without_sgr_sequences_alone() {
    // Text with no escapes at all is yielded as a single borrowed piece.
    assert_eq!(
        basic_formatted_to_plain("one\ntwo").collect::<Vec<_>>(),
        ["one\ntwo"]
    );
}

#[test]
fn to_plain_of_a_blank_row_yields_nothing() {
    for capture in ["", "\x1b[33m", "\x1b[33m\x1b[m"] {
        assert_eq!(basic_formatted_to_plain(capture).next(), None);
    }
}

#[test]
fn to_plain_doesnt_yield_empty_pieces() {
    assert_eq!(
        basic_formatted_to_plain("\x1b[33m\x1b[1mfoo\x1b[m")
            .collect::<Vec<_>>(),
        ["foo"]
    );
    assert_eq!(basic_formatted_to_plain("\x1b[33m\x1b[m").next(), None);
}

#[test]
fn to_plain_discards_a_truncated_trailing_sgr_sequence() {
    // A capture should never end mid-escape, but if it does, we drop the
    // partial escape rather than emitting it or looping forever.
    assert_eq!(
        basic_formatted_to_plain("foo\x1b[3").collect::<String>(),
        "foo"
    );
    assert_eq!(
        basic_formatted_to_plain("foo\x1b").collect::<String>(),
        "foo"
    );
}

#[test]
fn to_plain_of_a_capture_matches_the_plain_contents_of_a_tall_screen() {
    let data: &[u8] = b"\x1b[33mfoo\r\n\x1b[1mbar baz quux\r\n\
        \x1b[mplugh\r\n\x1b[7mxyzzy\r\n\x1b[27m\r\nlast";
    let mut parser = parser(3, 8);
    parser.process(data);
    let capture = finish(&mut parser);
    assert_eq!(
        basic_formatted_to_plain(&capture).collect::<String>(),
        tall_plain(20, 8, data)
    );
}

#[test]
fn rows_splits_a_capture_at_the_screen_width() {
    // `hello world` on a five column screen occupies three rows.
    assert_eq!(
        basic_formatted_rows("hello world", 5).collect::<Vec<_>>(),
        ["hello", " worl", "d"]
    );
}

#[test]
fn rows_splits_a_capture_at_newlines() {
    assert_eq!(
        basic_formatted_rows("one\ntwo\nthree", 10).collect::<Vec<_>>(),
        ["one", "two", "three"]
    );
}

#[test]
fn rows_keeps_blank_rows_in_the_middle_of_a_capture() {
    assert_eq!(
        basic_formatted_rows("one\n\n\ntwo", 10).collect::<Vec<_>>(),
        ["one", "", "", "two"]
    );
}

#[test]
fn rows_of_a_capture_include_the_blank_rows_below_the_written_ones() {
    let mut parser = parser(4, 10);
    parser.process(b"one\r\ntwo\r\nthree\x1b[2S");
    let capture = finish(&mut parser);
    assert_eq!(capture, "one\ntwo\nthree\n\n\n");
    // Two rows scrolled off. `three` plus the three blank rows below it are
    // still on the screen.
    assert_eq!(
        basic_formatted_rows(&capture, 10).collect::<Vec<_>>(),
        ["one", "two", "three", "", "", ""]
    );
}

#[test]
fn rows_yields_a_blank_row_after_a_trailing_newline() {
    // A capture never ends with a newline just to terminate its last row, so
    // a trailing newline means there's a blank row after it.
    assert_eq!(
        basic_formatted_rows("one\ntwo\n", 10).collect::<Vec<_>>(),
        ["one", "two", ""]
    );
}

#[test]
fn rows_of_an_empty_capture_yields_a_single_blank_row() {
    // An empty capture is indistinguishable from a capture of a single blank
    // row. A capture can't actually be empty -- the terminal is always at
    // least one row tall -- so a single blank row is the better reading.
    assert_eq!(basic_formatted_rows("", 10).collect::<Vec<_>>(), [""]);
}

#[test]
fn rows_doesnt_count_sgr_sequences_towards_the_width() {
    // The escapes don't take up any columns.
    assert_eq!(
        basic_formatted_rows("\x1b[31mab\x1b[1mcde\x1b[mfgh", 5)
            .collect::<Vec<_>>(),
        ["\x1b[31mab\x1b[1mcde\x1b[m", "fgh"]
    );
}

#[test]
fn rows_counts_wide_characters_as_two_columns() {
    // Three wide characters fill six columns, so the fourth starts a new row.
    assert_eq!(
        basic_formatted_rows("あいうえ", 6).collect::<Vec<_>>(),
        ["あいう", "え"]
    );
    // A wide character that doesn't fit in the remaining column is pushed to
    // the next row, the same way the terminal itself wraps it.
    assert_eq!(
        basic_formatted_rows("aあいb", 3).collect::<Vec<_>>(),
        ["aあ", "いb"]
    );
}

#[test]
fn rows_doesnt_count_combining_characters_towards_the_width() {
    // The combining acute accent is part of the `e` before it, so `resume`
    // still fits in six columns.
    assert_eq!(
        basic_formatted_rows("r\u{301}esume\u{301}d", 6).collect::<Vec<_>>(),
        ["r\u{301}esume\u{301}", "d"]
    );
}

#[test]
fn rows_counts_characters_wider_than_two_columns_as_two_columns() {
    // U+17D8 KHMER SIGN BEYYAL is reported by `unicode-width` as three
    // columns wide, but the screen draws it in two, so it has to be counted
    // as two here as well.
    assert_eq!(
        basic_formatted_rows("a\u{17d8}b", 3).collect::<Vec<_>>(),
        ["a\u{17d8}", "b"]
    );
}

#[test]
fn rows_yields_a_character_wider_than_the_screen_on_its_own_row() {
    // A one column screen discards wide characters, so this can only happen
    // if the terminal was resized after the capture was taken. Yield the
    // oversized character on a row of its own rather than looping forever on
    // a row we can never fit it into.
    assert_eq!(
        basic_formatted_rows("あいう", 1).collect::<Vec<_>>(),
        ["あ", "い", "う"]
    );
    assert_eq!(
        basic_formatted_rows("aあb", 1).collect::<Vec<_>>(),
        ["a", "あ", "b"]
    );
    // A combining character still belongs to the oversized character before
    // it, rather than being pushed onto the next row on its own.
    assert_eq!(
        basic_formatted_rows("あ\u{301}い", 1).collect::<Vec<_>>(),
        ["あ\u{301}", "い"]
    );
}

#[test]
fn rows_of_a_capture_match_the_rows_of_a_tall_screen() {
    let data: &[u8] = b"\x1b[33mfoo\r\n\x1b[1mbar baz quux\r\n\
        \x1b[mplugh\r\n\x1b[7mxyzzy\r\n\x1b[27m\r\nlast";
    let mut parser = parser(3, 8);
    parser.process(data);
    let capture = finish(&mut parser);

    // `basic_formatted_rows` should separate wrapped rows, which don't contain
    // any newlines in `capture`.
    let rows: Vec<String> = basic_formatted_rows(&capture, 8)
        .map(|row| basic_formatted_to_plain(row).collect())
        .collect();
    assert_eq!(rows, tall_rows(20, 8, data));
}

#[test]
fn rows_of_a_capture_match_a_tall_screen_for_every_screen_width() {
    let data = "\x1b[36;44malpha beta\r\n\x1b[3mgamma delta\r\n\
        \x1b[23muo\u{308}mlaut\r\nwide \x1b[1mあ\u{17d8}い\x1b[m\r\nlast"
        .as_bytes();
    for cols in 3..=20 {
        let mut parser = parser(3, cols);
        parser.process(data);
        let capture = finish(&mut parser);

        let rows: Vec<String> = basic_formatted_rows(&capture, cols)
            .map(|row| basic_formatted_to_plain(row).collect())
            .collect();
        assert_eq!(rows, tall_rows(30, cols, data), "{cols} cols");
    }
}
