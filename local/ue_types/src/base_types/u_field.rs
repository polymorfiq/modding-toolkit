use crate::{FName, UObject};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UField {
    // Size: 0x38
    base_object: UObject,
    next: *const UField
}

impl UField {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }
}