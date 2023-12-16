use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FEngineVersion {
    // Size: 0x20
    pub base_version: FEngineVersionBase,
    _padding: [u8; 0x4],
    pub branch: FString
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FEngineVersionBase {
    // Size: 0xC
    pub major: u16le,
    pub minor: u16le,
    pub patch: u16le,
    _padding: [u8; 0x2],
    pub change_list: u32le
}