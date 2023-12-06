use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TMulticastScriptDelegate<T> {
    // Size: 0x10
    pub invocation_list: TArray<T, FDefaultAllocator>
}