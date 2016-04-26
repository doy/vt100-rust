extern crate vt100;

#[cfg(test)]
mod tests {
    use vt100;

    #[test]
    fn object_creation() {
        let screen = vt100::Screen::new(24, 80);
        assert_eq!(screen.rows, 24);
        assert_eq!(screen.cols, 80);
    }
}
