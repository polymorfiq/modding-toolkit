use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TIndirectArray<T, Allocator> {
    // Size: 0x10
    pub data: TArray<*const T, Allocator>,
}