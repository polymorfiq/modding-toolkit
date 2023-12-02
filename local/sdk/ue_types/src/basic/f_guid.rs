use simple_endian::u32le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FGuid {
    // Size: 0x10
    pub a: u32le,
    pub b: u32le,
    pub c: u32le,
    pub d: u32le
}