use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FSoftObjectPath {
    pub asset_path_name: FName,
    _padding: [u8; 0x4],
    pub sub_path_string: FString
}