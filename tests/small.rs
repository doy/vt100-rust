//! Tests for small screens (only one row tall or one column wide).

use std::num::NonZeroU16;

fn sizes() -> impl Iterator<Item = (NonZeroU16, NonZeroU16)> {
    let dimensions =
        || [1, 2, 10].into_iter().map(|n| NonZeroU16::new(n).unwrap());
    dimensions()
        .flat_map(move |rows| dimensions().map(move |cols| (rows, cols)))
}

// https://github.com/doy/vt100-rust/issues/37
#[test]
fn wide_char_on_one_column_screen() {
    for (rows, cols) in sizes() {
        let mut parser = vt100::Parser::new(rows, cols, 0);
        parser.process("あ".as_bytes());
    }
}

// https://github.com/doy/vt100-rust/issues/37
#[test]
fn wrapping_on_one_row_screen() {
    for (rows, cols) in sizes() {
        let mut parser = vt100::Parser::new(rows, cols, 0);
        parser.process(b"abcdefghijk");
    }
}

// https://github.com/doy/vt100-rust/issues/28
#[test]
fn resize_screen_containing_wide_char_to_one_column_and_clear() {
    for (rows, cols) in sizes() {
        let mut parser = vt100::Parser::new(rows, cols, 0);
        parser.process("あ".as_bytes());
        parser.screen_mut().set_size(rows, NonZeroU16::MIN);
        parser.process(b"\x1b[K");
    }
}
