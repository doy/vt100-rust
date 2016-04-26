use libc;
use std;

enum ScreenImpl {}

#[repr(C)]
struct Loc {
    pub row: libc::c_int,
    pub col: libc::c_int,
}

pub struct Screen {
    pub rows: i32,
    pub cols: i32,

    screen_impl: *mut ScreenImpl,
}

impl Screen {
    pub fn new(rows: i32, cols: i32) -> Screen {
        let screen_impl = unsafe {
            vt100_screen_new(rows as libc::c_int, cols as libc::c_int)
        };
        Screen {
            rows: rows,
            cols: cols,
            screen_impl: screen_impl,
        }
    }

    pub fn process(&mut self, s: &str) -> u64 {
        unsafe {
            vt100_screen_process_string(
                self.screen_impl,
                s.as_ptr() as *const libc::c_char,
                s.len()
            ) as u64
        }
    }

    pub fn window_contents(&self,
        row_start: i32,
        col_start: i32,
        row_end: i32,
        col_end: i32
    ) -> String {
        let row_start = std::cmp::min(std::cmp::max(row_start, 0), self.rows - 1);
        let col_start = std::cmp::min(std::cmp::max(col_start, 0), self.cols - 1);
        let row_end = std::cmp::min(std::cmp::max(row_end, 0), self.rows - 1);
        let col_end = std::cmp::min(std::cmp::max(col_end, 0), self.cols - 1);

        let start_loc = Loc { row: row_start, col: col_start };
        let end_loc = Loc { row: row_end, col: col_end };

        let mut plaintext: *mut libc::c_char = unsafe { std::mem::uninitialized() };
        let mut len: libc::size_t = unsafe { std::mem::uninitialized() };
        unsafe {
            vt100_screen_get_string_plaintext(
                self.screen_impl,
                &start_loc as *const Loc,
                &end_loc as *const Loc,
                &mut plaintext as *mut *mut libc::c_char,
                &mut len as *mut libc::size_t,
            )
        };
        let rust_plaintext = unsafe {
            std::slice::from_raw_parts(
                plaintext as *mut libc::c_uchar,
                len
            )
        }.to_vec();
        std::string::String::from_utf8(rust_plaintext).unwrap()
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        unsafe { vt100_screen_delete(self.screen_impl) };
    }
}

extern "C" {
    fn vt100_screen_new(
        rows: libc::c_int,
        cols: libc::c_int
    ) -> *mut ScreenImpl;
    fn vt100_screen_delete(screen: *mut ScreenImpl);

    fn vt100_screen_process_string(
        screen: *mut ScreenImpl,
        buf: *const libc::c_char,
        len: libc::size_t,
    ) -> libc::c_int;
    fn vt100_screen_get_string_plaintext(
        screen: *mut ScreenImpl,
        start: *const Loc,
        end: *const Loc,
        outp: *mut *mut libc::c_char,
        outlen: *mut libc::size_t,
    );
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
