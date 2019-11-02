use std::convert::TryInto as _;
use unicode_width::UnicodeWidthChar as _;

// soft hyphen is defined as width 1, but in a terminal setting it should
// always be width 0
pub fn char_width(c: char) -> u16 {
    match c {
        '\u{00ad}' => 0,
        _ => c.width().unwrap_or(0).try_into().unwrap(),
    }
}

// strings in this context should always be an arbitrary character followed by
// zero or more zero-width characters, so we should only have to look at the
// first character
pub fn str_width(s: &str) -> u16 {
    s.chars().next().map_or(0, char_width)
}
