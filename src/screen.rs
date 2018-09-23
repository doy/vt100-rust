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
    saved: types::Loc,
    scroll_top: libc::c_int,
    scroll_bottom: libc::c_int,
    row_count: libc::c_int,
    row_capacity: libc::c_int,
    row_top: libc::c_int,
}

enum ScreenParserState {}

#[repr(C)]
struct ScreenPrefix {
    grid: *mut ScreenGridPrefix,
    alternate: *mut ScreenGridPrefix,

    parser_state: *mut ScreenParserState,

    title: *mut libc::c_char,
    title_len: libc::size_t,
    icon_name: *mut libc::c_char,
    icon_name_len: libc::size_t,

    scrollback_length: libc::c_int,

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
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
        unsafe { (*(*prefix).grid).max.row }
    }

    pub fn cols(&self) -> i32 {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
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
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;

        // XXX not super happy about this - can we maybe disable the
        // optimization in libvt100 if no scrollback at all was requested?
        let grid_max_row = unsafe { (*(*prefix).grid).max.row };
        let row_count = unsafe { (*(*prefix).grid).row_count };

        let row_start = std::cmp::min(
            std::cmp::max(row_start, 0),
            self.rows() - 1
        ) + row_count - grid_max_row;
        let col_start = std::cmp::min(
            std::cmp::max(col_start, 0),
            self.cols() - 1
        );
        let row_end = std::cmp::min(
            std::cmp::max(row_end, 0),
            self.rows() - 1
        ) + row_count - grid_max_row;
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
        unsafe { libc::free(plaintext as *mut libc::c_void) };
        std::string::String::from_utf8(rust_plaintext).unwrap()
    }

    pub fn window_contents_formatted(&self,
        row_start: i32,
        col_start: i32,
        row_end: i32,
        col_end: i32
    ) -> String {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;

        let grid_max_row = unsafe { (*(*prefix).grid).max.row };
        let row_count = unsafe { (*(*prefix).grid).row_count };

        let row_start = std::cmp::min(
            std::cmp::max(row_start, 0),
            self.rows() - 1
        ) + row_count - grid_max_row;
        let col_start = std::cmp::min(
            std::cmp::max(col_start, 0),
            self.cols() - 1
        );
        let row_end = std::cmp::min(
            std::cmp::max(row_end, 0),
            self.rows() - 1
        ) + row_count - grid_max_row;
        let col_end = std::cmp::min(
            std::cmp::max(col_end, 0),
            self.cols() - 1
        );

        let start_loc = types::Loc { row: row_start, col: col_start };
        let end_loc = types::Loc { row: row_end, col: col_end };

        let mut formatted: *mut libc::c_char = unsafe { std::mem::uninitialized() };
        let mut len: libc::size_t = unsafe { std::mem::uninitialized() };
        unsafe {
            ffi::vt100_screen_get_string_formatted(
                screen_impl,
                &start_loc as *const types::Loc,
                &end_loc as *const types::Loc,
                &mut formatted as *mut *mut libc::c_char,
                &mut len as *mut libc::size_t,
            )
        };
        let rust_formatted = unsafe {
            std::slice::from_raw_parts(
                formatted as *mut libc::c_uchar,
                len
            )
        }.to_vec();
        std::string::String::from_utf8(rust_formatted).unwrap()
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
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
        unsafe {
            ((*(*prefix).grid).cur.row, (*(*prefix).grid).cur.col)
        }
    }

    pub fn title(&self) -> Option<&str> {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
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
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
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
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
        let attrs = unsafe { &(*prefix).attrs };
        color::Color::new(&attrs.fgcolor)
    }

    pub fn bgcolor(&self) -> color::Color {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
        let attrs = unsafe { &(*prefix).attrs };
        color::Color::new(&attrs.bgcolor)
    }

    pub fn bold(&self) -> bool {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
        unsafe {
            ffi::vt100_wrapper_cell_attrs_bold(&mut (*prefix).attrs) != 0
        }
    }

    pub fn italic(&self) -> bool {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
        unsafe {
            ffi::vt100_wrapper_cell_attrs_italic(&mut (*prefix).attrs) != 0
        }
    }

    pub fn underline(&self) -> bool {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
        unsafe {
            ffi::vt100_wrapper_cell_attrs_underline(&mut (*prefix).attrs) != 0
        }
    }

    pub fn inverse(&self) -> bool {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
        unsafe {
            ffi::vt100_wrapper_cell_attrs_inverse(&mut (*prefix).attrs) != 0
        }
    }

    pub fn hide_cursor(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            ffi::vt100_wrapper_screen_hide_cursor(screen_impl) != 0
        }
    }

    pub fn application_keypad(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            ffi::vt100_wrapper_screen_application_keypad(screen_impl) != 0
        }
    }

    pub fn application_cursor(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            ffi::vt100_wrapper_screen_application_cursor(screen_impl) != 0
        }
    }

    pub fn mouse_reporting_press(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            ffi::vt100_wrapper_screen_mouse_reporting_press(screen_impl) != 0
        }
    }

    pub fn mouse_reporting_press_release(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            ffi::vt100_wrapper_screen_mouse_reporting_press_release(screen_impl) != 0
        }
    }

    pub fn mouse_reporting_button_motion(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            ffi::vt100_wrapper_screen_mouse_reporting_button_motion(screen_impl) != 0
        }
    }

    pub fn mouse_reporting_sgr_mode(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            ffi::vt100_wrapper_screen_mouse_reporting_mode(screen_impl) == 2
        }
    }

    pub fn bracketed_paste(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            ffi::vt100_wrapper_screen_bracketed_paste(screen_impl) != 0
        }
    }

    pub fn alternate_buffer_active(&self) -> bool {
        let Screen(screen_impl) = *self;
        let prefix: *mut ScreenPrefix = screen_impl as *mut ScreenPrefix;
        !unsafe { (*prefix).alternate }.is_null()
    }

    pub fn check_visual_bell(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            let state = ffi::vt100_wrapper_screen_visual_bell(screen_impl) != 0;
            ffi::vt100_wrapper_screen_clear_visual_bell(screen_impl);
            state
        }
    }

    pub fn check_audible_bell(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            let state = ffi::vt100_wrapper_screen_audible_bell(screen_impl) != 0;
            ffi::vt100_wrapper_screen_clear_audible_bell(screen_impl);
            state
        }
    }

    pub fn check_update_title(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            let state = ffi::vt100_wrapper_screen_update_title(screen_impl) != 0;
            ffi::vt100_wrapper_screen_clear_update_title(screen_impl);
            state
        }
    }

    pub fn check_update_icon_name(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            let state = ffi::vt100_wrapper_screen_update_icon_name(screen_impl) != 0;
            ffi::vt100_wrapper_screen_clear_update_icon_name(screen_impl);
            state
        }
    }

    pub fn check_dirty(&self) -> bool {
        let Screen(screen_impl) = *self;
        unsafe {
            let state = ffi::vt100_wrapper_screen_dirty(screen_impl) != 0;
            ffi::vt100_wrapper_screen_clear_dirty(screen_impl);
            state
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        let Screen(screen_impl) = *self;
        unsafe { ffi::vt100_screen_delete(screen_impl) };
    }
}
