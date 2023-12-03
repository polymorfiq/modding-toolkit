use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UEngine {
    // Size: 0xEC8
    base_object: UObject,
    _unknown_a: [u8; 0xCF0],
    pub world_list: TIndirectArray<FWorldContext<'static>>,
    _unknown_b: [u8; 0x198]
}