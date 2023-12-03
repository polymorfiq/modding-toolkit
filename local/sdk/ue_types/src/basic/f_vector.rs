use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FVector {
    // Size: 0xC
    pub x: f32le,
    pub y: f32le,
    pub z: f32le
}