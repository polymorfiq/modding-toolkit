use crate::*;
use simple_endian::u32le;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UStruct {
    // Size: 0xA0
    pub base_field: UField,
    pub base_chain: FStructBaseChain,
    pub super_struct: *const UStruct, // 0x48
    pub children: *const UField,
    pub property_size: u32le,
    pub min_alignment: u32le,
    pub script: TArray<u8, FDefaultAllocator>,
    pub property_link: *const UProperty,
    pub ref_link: *const UProperty,
    pub destructor_link: *const UProperty,
    pub post_construct_link: *const UProperty,
    pub script_object_references: TArray<*const UObject, FDefaultAllocator>,
}

impl UStruct {
    pub fn field(&self) -> UField { self.base_field }
    pub fn chain(&self) -> FStructBaseChain { self.base_chain }
}