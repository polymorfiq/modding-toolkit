use simple_endian::*;
use std::ffi::c_void;
use crate::{FStructBaseChain, TArray, UField, UProperty};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UStruct {
    // Size: 0xA0
    base_field: UField,
    base_chain: FStructBaseChain,
    super_struct: *const UStruct, // 0x48
    children: *const UField,
    property_size: u32le,
    min_alignment: u32le,
    script: *const TArray<u8>,
    property_link: *const UProperty,
    ref_link: *const UProperty,
    destructor_link: *const UProperty,
    post_construct_link: *const UProperty,
    script_object_references: *const TArray<c_void>,
}