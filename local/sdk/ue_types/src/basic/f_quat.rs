use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FQuat {
    // Size: 0x10
    x: f32le,
    y: f32le,
    z: f32le,
    w: f32le
}