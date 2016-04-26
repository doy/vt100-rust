use libc;

enum ScreenImpl {}
pub struct Screen(*mut ScreenImpl);

impl Screen {
    pub fn new(rows: u32, cols: u32) -> Screen {
        let screen_impl = unsafe {
            vt100_screen_new(rows as libc::c_int, cols as libc::c_int)
        };
        Screen(screen_impl)
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        let &mut Screen(screen_impl) = self;
        unsafe { vt100_screen_delete(screen_impl) };
    }
}

extern "C" {
    fn vt100_screen_new(rows: libc::c_int, cols: libc::c_int) -> *mut ScreenImpl;
    fn vt100_screen_delete(screen: *mut ScreenImpl);
}

#[cfg(test)]
mod tests {
    #[test]
    fn ffi() {
        let ptr = unsafe { super::vt100_screen_new(24, 80) };
        assert!(!ptr.is_null());
        unsafe { super::vt100_screen_delete(ptr) };
    }
}
