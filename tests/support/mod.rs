pub trait TestHelpers {
    fn assert_process(&mut self, s: &[u8]) -> usize;
}

impl TestHelpers for vt100::Screen {
    fn assert_process(&mut self, s: &[u8]) -> usize {
        let ret = self.process(s);
        assert_eq!(ret, s.len());
        ret
    }
}
