use vt100;

pub trait TestHelpers {
    fn assert_process(&mut self, s: &[u8]) -> u64;
}

impl TestHelpers for vt100::Screen {
    fn assert_process(&mut self, s: &[u8]) -> u64 {
        let ret = self.process(s);
        assert_eq!(ret, s.len() as u64);
        ret
    }
}
