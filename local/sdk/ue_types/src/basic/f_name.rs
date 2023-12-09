use simple_endian::u32le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FName {
    // Size: 0x0C
    pub comparison_idx: u32le,
    pub display_idx: u32le,
    pub number: u32le
}