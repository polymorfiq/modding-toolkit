#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FVector {
    // Size: 0xC
    pub x: f32,
    pub y: f32,
    pub z: f32
}