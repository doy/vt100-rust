use unicode_width::UnicodeWidthChar as _;

// soft hyphen is defined as width 1, but in a terminal setting it should
// always be width 0
pub fn char_width(c: char) -> usize {
    match c {
        '\u{00ad}' => 0,
        _ => c.width().unwrap_or(0),
    }
}

pub fn str_width(s: &str) -> usize {
    s.chars().map(char_width).sum()
}
