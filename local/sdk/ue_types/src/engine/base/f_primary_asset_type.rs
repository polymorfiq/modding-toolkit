use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FPrimaryAssetType {
    // Size: 0xC
    pub name: FName
}