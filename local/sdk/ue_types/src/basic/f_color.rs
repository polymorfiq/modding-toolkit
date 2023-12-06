use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FColor {
    // Size: 0x4
    data: u32le
}