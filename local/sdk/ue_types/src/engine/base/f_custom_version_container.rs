use crate::*;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C, align(0x8))]
pub struct FCustomVersionContainer {
    // Size: 0x10
    pub versions: TArray<FCustomVersion, FDefaultAllocator>
}