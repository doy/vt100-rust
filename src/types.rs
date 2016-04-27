use libc;

pub enum ScreenImpl {}
pub enum CellImpl {}

#[repr(C)]
pub struct Loc {
    pub row: libc::c_int,
    pub col: libc::c_int,
}
