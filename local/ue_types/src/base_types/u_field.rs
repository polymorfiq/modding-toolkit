use crate::UObject;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UField {
    // Size: 0x38
    base_object: UObject,
    next: *const UField
}

impl UField {
    pub fn object(&self) -> UObject { self.base_object }
}