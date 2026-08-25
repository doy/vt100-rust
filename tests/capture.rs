//! Tests for capturing terminal data in a streaming fashion via
//! [`vt100::Callbacks::on_scroll`].
//!
//! The intended use is to create a [`vt100::Parser`] with no scrollback and
//! append each row to a buffer as it scrolls off the top of the screen, and
//! then, at the end, append the visible contents of the terminal to the same
//! buffer. The result is the same as if the data had been processed by a
//! terminal tall enough to hold all of it at once.

use vt100::capture::{BasicFormattedCaptureState, RowContents};

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
        .write_contents_formatted_basic(&mut capture.buf, &mut capture.state)
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
    // `on_scroll` fires for every row that leaves the screen, whether or not
    // the parser keeps it in its scrollback.
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
    assert_eq!(parser.callbacks().buf, "");
    parser.process(b"\x1b[5S");
    assert_eq!(parser.callbacks().buf, "1\n2\n3\n4\n5");
}
