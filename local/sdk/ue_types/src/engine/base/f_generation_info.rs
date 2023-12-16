use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FGenerationInfo {
    pub export_count: u32le,
    pub name_count: u32le
}