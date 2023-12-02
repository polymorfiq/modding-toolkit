use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TIndirectArray<T> {
    // Size: 0x10
    pub data: TArray<*const T>,
}