#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FQuat {
    // Size: 0x10
    x: f32,
    y: f32,
    z: f32,
    w: f32
}