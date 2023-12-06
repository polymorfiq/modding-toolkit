use simple_endian::*;
use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FMulticastDelegateBase<T> {
    // Size: 0x18
    pub invocation_list: TArray<T, FDefaultAllocator>,
    pub compaction_threshold: u32le,
    pub invocation_list_lock_count: u32le
}