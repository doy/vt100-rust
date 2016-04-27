extern crate vt100;

#[cfg(test)]
mod tests {
    use vt100;

    #[test]
    fn object_creation() {
        let mut screen = vt100::Screen::new(24, 80);
        assert_eq!(screen.rows(), 24);
        assert_eq!(screen.cols(), 80);

        let input = b"foo\x1b[31m\x1b[32mb\x1b[3;7;42ma\x1b[23mr";
        screen.process(input);
        assert_eq!(screen.window_contents(0, 0, 0, 50), "foobar\n");
    }

    #[test]
    fn set_window_size() {
        let screen = vt100::Screen::new(24, 80);
        assert_eq!(screen.rows(), 24);
        assert_eq!(screen.cols(), 80);

        screen.set_window_size(34, 8);
        assert_eq!(screen.rows(), 34);
        assert_eq!(screen.cols(), 8);
    }
}
