extern crate vt100;

#[cfg(test)]
mod tests {
    use vt100;

    #[test]
    fn object_creation() {
        let _ = vt100::Screen::new(24, 80);
    }
}
