use libc;
use std;

use types;

pub struct Cell(*mut types::CellImpl);

#[repr(C)]
struct CellPrefix {
    pub contents: [libc::c_char; 8],
    pub len: libc::size_t,
}

impl Cell {
    pub fn new(cell_impl: *mut types::CellImpl) -> Cell {
        Cell(cell_impl)
    }

    pub fn contents(&self) -> &str {
        let Cell(cell_impl) = *self;
        let contents: &[u8] = unsafe {
            let prefix: *mut CellPrefix = std::mem::transmute(cell_impl);
            std::slice::from_raw_parts(
                &(*prefix).contents as *const i8 as *const u8,
                (*prefix).len
            )
        };
        std::str::from_utf8(contents).unwrap()
    }
}
