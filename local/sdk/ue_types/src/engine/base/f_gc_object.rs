use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FGCObject {
    // Size: 0x10
    vf_table: *const *const UnknownType,
    pub b_reference_added: u8,
    _padding: [u8; 7]
}