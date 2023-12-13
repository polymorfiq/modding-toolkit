use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FPrimaryAssetId {
    // Size: 0x18
    pub primary_asset_type: FPrimaryAssetType,
    pub primary_asset_name: FName
}