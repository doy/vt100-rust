use libc;

pub enum ScreenImpl {}
pub enum CellImpl {}
#[repr(C)]
pub struct ColorImpl(pub libc::uint32_t);

#[repr(C)]
pub struct CellAttrs {
    pub fgcolor: ColorImpl,
    pub bgcolor: ColorImpl,
    pub attrs: libc::c_uchar,
}

#[repr(C)]
pub struct Loc {
    pub row: libc::c_int,
    pub col: libc::c_int,
}
