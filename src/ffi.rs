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
    pub fn vt100_screen_get_string_formatted(
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

    // XXX: these wrappers (and all of ffi.c) only exist because rust can't
    // handle bitfields yet - once it can, these should be removed
    pub fn vt100_wrapper_screen_hide_cursor(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_application_keypad(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_application_cursor(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_mouse_reporting_press(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_mouse_reporting_press_release(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_mouse_reporting_button_motion(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_mouse_reporting_mode(screen: *mut types::ScreenImpl) -> libc::c_uchar;
    pub fn vt100_wrapper_screen_bracketed_paste(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_visual_bell(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_audible_bell(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_update_title(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_update_icon_name(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_dirty(screen: *mut types::ScreenImpl) -> libc::c_int;
    pub fn vt100_wrapper_screen_clear_visual_bell(screen: *mut types::ScreenImpl);
    pub fn vt100_wrapper_screen_clear_audible_bell(screen: *mut types::ScreenImpl);
    pub fn vt100_wrapper_screen_clear_update_title(screen: *mut types::ScreenImpl);
    pub fn vt100_wrapper_screen_clear_update_icon_name(screen: *mut types::ScreenImpl);
    pub fn vt100_wrapper_screen_clear_dirty(screen: *mut types::ScreenImpl);
    pub fn vt100_wrapper_cell_is_wide(cell: *mut types::CellImpl) -> libc::c_int;
    pub fn vt100_wrapper_cell_attrs_bold(cell: *mut types::CellAttrs) -> libc::c_int;
    pub fn vt100_wrapper_cell_attrs_italic(cell: *mut types::CellAttrs) -> libc::c_int;
    pub fn vt100_wrapper_cell_attrs_underline(cell: *mut types::CellAttrs) -> libc::c_int;
    pub fn vt100_wrapper_cell_attrs_inverse(cell: *mut types::CellAttrs) -> libc::c_int;
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
