use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FRotationConversionCache {
    // Size: 0x20
    pub cached_quat: FQuat,
    pub cached_rotator: FRotator,
}