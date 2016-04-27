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

    pub fn vt100_screen_set_window_size(
        screen: *mut types::ScreenImpl,
        rows: libc::c_int,
        cols: libc::c_int,
    );
    pub fn vt100_screen_set_scrollback_length(
        screen: *mut types::ScreenImpl,
        rows: libc::c_int,
    );

    pub fn vt100_screen_cell_at(
        screen: *mut types::ScreenImpl,
        row: libc::c_int,
        col: libc::c_int,
    ) -> *mut types::CellImpl;

    pub fn vt100_wrapper_rows(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_cols(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_cell_is_wide(cell: *mut types::CellImpl) -> libc::c_int;
    pub fn vt100_wrapper_cell_bold(cell: *mut types::CellImpl) -> libc::c_int;
    pub fn vt100_wrapper_cell_italic(cell: *mut types::CellImpl) -> libc::c_int;
    pub fn vt100_wrapper_cell_underline(cell: *mut types::CellImpl) -> libc::c_int;
    pub fn vt100_wrapper_cell_inverse(cell: *mut types::CellImpl) -> libc::c_int;
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
