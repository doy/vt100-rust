use libc;

use types;

extern "C" {
    pub fn vt100_screen_new(
        rows: libc::c_int,
        cols: libc::c_int
    ) -> *mut types::ScreenImpl;
    pub fn vt100_screen_delete(screen: *mut types::ScreenImpl);

    pub fn vt100_screen_process_string(
        screen: *mut types::ScreenImpl,
        buf: *const libc::c_char,
        len: libc::size_t,
    ) -> libc::c_int;
    pub fn vt100_screen_get_string_plaintext(
        screen: *mut types::ScreenImpl,
        start: *const types::Loc,
        end: *const types::Loc,
        outp: *mut *mut libc::c_char,
        outlen: *mut libc::size_t,
    );

    pub fn vt100_wrapper_rows(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_cols(screen: *mut types::ScreenImpl) -> libc::c_int;
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
