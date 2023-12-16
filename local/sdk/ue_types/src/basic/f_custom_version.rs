use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FCustomVersion {
    // Size: 0x24
    pub key: FGuid,
    pub version: u32le,
    pub reference_count: u32le,
    pub friendly_name: FName
}