use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FRelativeBitReference {
    // Size: 0x8
    pub dword_index: u32le,
    pub mask: u32le
}