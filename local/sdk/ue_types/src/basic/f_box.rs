use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FBox {
    // Size: 0x1C
    pub min: FVector,
    pub max: FVector,
    is_valid: u8,
    _padding: [u8; 3]
}