use crate::*;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C, align(0x8))]
pub struct FEngineVersion {
    // Size: 0x20
    pub base_version: FEngineVersionBase,
    _padding: [u8; 0x4],
    pub branch: FString
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C, align(0x4))]
pub struct FEngineVersionBase {
    // Size: 0xC
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    _padding: [u8; 0x2],
    pub change_list: u32
}