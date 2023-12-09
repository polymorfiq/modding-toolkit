use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FUniqueNetIdRepl {
    _unknown: [u8; 0x18],
    pub replication_bytes: TArray<u8, FDefaultAllocator>
}