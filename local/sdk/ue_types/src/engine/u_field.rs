use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UField {
    // Size: 0x38
    pub base_object: UObject<*const UnknownType>,
    pub next: *const UField
}