use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FVector2D {
    // Size: 0x8
    x: f32le,
    y: f32le
}