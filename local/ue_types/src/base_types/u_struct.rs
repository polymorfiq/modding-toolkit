use simple_endian::*;
use crate::{TArray, UField, UObject, UProperty};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UStruct {
    // Size: 0xA0
    base_field: UField,
    base_chain: FStructBaseChain,
    super_struct: *const UStruct, // 0x48
    children: *const UField,
    property_size: u32le,
    min_alignment: u32le,
    script: TArray<u8>,
    property_link: *const UProperty,
    ref_link: *const UProperty,
    destructor_link: *const UProperty,
    post_construct_link: *const UProperty,
    script_object_references: TArray<*const UObject>,
}

impl UStruct {
    pub fn field(&self) -> UField { self.base_field }
    pub fn chain(&self) -> FStructBaseChain { self.base_chain }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FStructBaseChain {
    // Size: 0x4
    struct_base_chain_array: *const *const FStructBaseChain,
    num_struct_bases_in_chain_minus_one: u32le,
    _padding: [u8; 4]
}