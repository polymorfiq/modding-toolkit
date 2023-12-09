use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UEngine {
    // Size: 0xEC8
    pub base_object: UObject<UnknownType>,
    _unknown_a: [u8; 0xCF0],
    pub world_list: TIndirectArray<FWorldContext, FDefaultAllocator>,
    _unknown_b: [u8; 0x198]
}