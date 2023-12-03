use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x10))]
pub struct FTransform {
    pub rotation: FQuat,
    pub translation: FVector,
    _padding: [u8; 4],
    pub scale_3d: FVector,
    _padding_b: [u8; 4]
}