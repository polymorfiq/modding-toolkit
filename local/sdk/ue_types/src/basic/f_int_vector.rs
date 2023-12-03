use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FIntVector {
    // Size: 0xC
    x: i32le,
    y: i32le,
    z: i32le
}