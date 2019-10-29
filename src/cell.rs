use libc;
use std;

use color;
use ffi;
use types;

pub struct Cell(*mut types::CellImpl);

#[repr(C)]
struct CellPrefix {
    pub contents: [libc::c_char; 8],
    pub len: libc::size_t,
    pub attrs: types::CellAttrs,
}

impl Cell {
    pub fn new(cell_impl: *mut types::CellImpl) -> Cell {
        Cell(cell_impl)
    }

    pub fn contents(&self) -> &str {
        let Cell(cell_impl) = *self;
        let prefix: *mut CellPrefix = cell_impl as *mut CellPrefix;
        let contents: &[u8] = unsafe {
            std::slice::from_raw_parts(
                &(*prefix).contents as *const i8 as *const u8,
                (*prefix).len,
            )
        };
        std::str::from_utf8(contents).unwrap()
    }

    pub fn fgcolor(&self) -> color::Color {
        let Cell(cell_impl) = *self;
        let prefix: *mut CellPrefix = cell_impl as *mut CellPrefix;
        let attrs = unsafe { &(*prefix).attrs };
        color::Color::new(&attrs.fgcolor)
    }

    pub fn bgcolor(&self) -> color::Color {
        let Cell(cell_impl) = *self;
        let prefix: *mut CellPrefix = cell_impl as *mut CellPrefix;
        let attrs = unsafe { &(*prefix).attrs };
        color::Color::new(&attrs.bgcolor)
    }

    pub fn is_wide(&self) -> bool {
        let Cell(cell_impl) = *self;
        unsafe { ffi::vt100_wrapper_cell_is_wide(cell_impl) != 0 }
    }

    pub fn bold(&self) -> bool {
        let Cell(cell_impl) = *self;
        let prefix: *mut CellPrefix = cell_impl as *mut CellPrefix;
        unsafe {
            ffi::vt100_wrapper_cell_attrs_bold(&mut (*prefix).attrs) != 0
        }
    }

    pub fn italic(&self) -> bool {
        let Cell(cell_impl) = *self;
        let prefix: *mut CellPrefix = cell_impl as *mut CellPrefix;
        unsafe {
            ffi::vt100_wrapper_cell_attrs_italic(&mut (*prefix).attrs) != 0
        }
    }

    pub fn underline(&self) -> bool {
        let Cell(cell_impl) = *self;
        let prefix: *mut CellPrefix = cell_impl as *mut CellPrefix;
        unsafe {
            ffi::vt100_wrapper_cell_attrs_underline(&mut (*prefix).attrs) != 0
        }
    }

    pub fn inverse(&self) -> bool {
        let Cell(cell_impl) = *self;
        let prefix: *mut CellPrefix = cell_impl as *mut CellPrefix;
        unsafe {
            ffi::vt100_wrapper_cell_attrs_inverse(&mut (*prefix).attrs) != 0
        }
    }
}
