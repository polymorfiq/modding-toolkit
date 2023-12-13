use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FDelegateBase {
    // Size: 0x10
    pub delegate_allocator: FHeapAllocatorForElementType<TAlignedBytes8<16>>,
    pub delegate_size: u32le,
    _padding: [u8; 0x4]
}