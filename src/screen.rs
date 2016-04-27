use libc;
use std;

use cell;
use color;
use ffi;
use types;

pub struct Screen(*mut types::ScreenImpl);

#[repr(C)]
struct ScreenGridPrefix {
    cur: types::Loc,
    max: types::Loc,
}

#[repr(C)]
struct ScreenPrefix {
    grid: *mut ScreenGridPrefix,
    alternate: *mut ScreenGridPrefix,

    title: *mut libc::c_char,
    title_len: libc::size_t,
    icon_name: *mut libc::c_char,
    icon_name_len: libc::size_t,

    attrs: types::CellAttrs,
}

impl Screen {
    pub fn new(rows: i32, cols: i32) -> Screen {
        let screen_impl = unsafe {
            ffi::vt100_screen_new(rows as libc::c_int, cols as libc::c_int)
        };
        Screen(screen_impl)
    }

    pub fn rows(&self) -> i32 {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = unsafe {
            std::mem::transmute(screen_impl)
        };
        unsafe { (*(*prefix).grid).max.row }
    }

    pub fn cols(&self) -> i32 {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = unsafe {
            std::mem::transmute(screen_impl)
        };
        unsafe { (*(*prefix).grid).max.col }
    }

    pub fn set_window_size(&self, rows: i32, cols: i32) {
        let Screen(screen_impl) = *self;
        unsafe { ffi::vt100_screen_set_window_size(screen_impl, rows, cols) };
    }

    pub fn set_scrollback_length(&self, rows: i32) {
        let Screen(screen_impl) = *self;
        unsafe { ffi::vt100_screen_set_scrollback_length(screen_impl, rows) };
    }

    pub fn process(&mut self, s: &[u8]) -> u64 {
        let Screen(screen_impl) = *self;
        unsafe {
            ffi::vt100_screen_process_string(
                screen_impl,
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
        let Screen(screen_impl) = *self;
        let row_start = std::cmp::min(
            std::cmp::max(row_start, 0),
            self.rows() - 1
        );
        let col_start = std::cmp::min(
            std::cmp::max(col_start, 0),
            self.cols() - 1
        );
        let row_end = std::cmp::min(
            std::cmp::max(row_end, 0),
            self.rows() - 1
        );
        let col_end = std::cmp::min(
            std::cmp::max(col_end, 0),
            self.cols() - 1
        );

        let start_loc = types::Loc { row: row_start, col: col_start };
        let end_loc = types::Loc { row: row_end, col: col_end };

        let mut plaintext: *mut libc::c_char = unsafe { std::mem::uninitialized() };
        let mut len: libc::size_t = unsafe { std::mem::uninitialized() };
        unsafe {
            ffi::vt100_screen_get_string_plaintext(
                screen_impl,
                &start_loc as *const types::Loc,
                &end_loc as *const types::Loc,
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

    pub fn cell(&self, row: i32, col: i32) -> Option<cell::Cell> {
        let Screen(screen_impl) = *self;
        if row < 0 || row >= self.rows() || col < 0 || col >= self.cols() {
            return None
        }
        let cell_impl = unsafe {
            ffi::vt100_screen_cell_at(screen_impl, row, col)
        };
        Some(cell::Cell::new(cell_impl))
    }

    pub fn cursor_position(&self) -> (i32, i32) {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = unsafe {
            std::mem::transmute(screen_impl)
        };
        unsafe {
            ((*(*prefix).grid).cur.col, (*(*prefix).grid).cur.col)
        }
    }

    pub fn title(&self) -> Option<&str> {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = unsafe {
            std::mem::transmute(screen_impl)
        };
        if unsafe { (*prefix).title }.is_null() {
            None
        }
        else {
            let slice: &mut [u8] = unsafe {
                std::slice::from_raw_parts_mut(
                    (*prefix).title as *mut u8,
                    (*prefix).title_len
                )
            };
            Some(std::str::from_utf8(slice).unwrap())
        }
    }

    pub fn icon_name(&self) -> Option<&str> {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = unsafe {
            std::mem::transmute(screen_impl)
        };
        if unsafe { (*prefix).icon_name }.is_null() {
            None
        }
        else {
            let slice: &mut [u8] = unsafe {
                std::slice::from_raw_parts_mut(
                    (*prefix).icon_name as *mut u8,
                    (*prefix).icon_name_len
                )
            };
            Some(std::str::from_utf8(slice).unwrap())
        }
    }

    pub fn fgcolor(&self) -> color::Color {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = unsafe {
            std::mem::transmute(screen_impl)
        };
        let attrs = unsafe { &(*prefix).attrs };
        color::Color::new(&attrs.fgcolor)
    }

    pub fn bgcolor(&self) -> color::Color {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = unsafe {
            std::mem::transmute(screen_impl)
        };
        let attrs = unsafe { &(*prefix).attrs };
        color::Color::new(&attrs.bgcolor)
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        let Screen(screen_impl) = *self;
        unsafe { ffi::vt100_screen_delete(screen_impl) };
    }
}
