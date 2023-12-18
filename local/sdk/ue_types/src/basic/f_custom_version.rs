use crate::*;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C, align(0x4))]
pub struct FCustomVersion {
    // Size: 0x24
    pub key: FGuid,
    pub version: u32,
    pub reference_count: u32
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FCustomVersionDeprecated {
    // Size: 0x24
    pub key: FGuid,
    pub version: u32,
    pub reference_count: u32,
    pub friendly_name: FName
}