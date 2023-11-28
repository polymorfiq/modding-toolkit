use crate::{FName, UField};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UProperty {
    base_field: UField,
    array_dim: u32,
    element_size: u32,
    e_property_flags: u64,
    rep_index: u16,
    blueprint_replication_condition: u8,
    offset_interval: u32,
    rep_notify_func: FName,
    property_link_next: *const UProperty,
    next_ref: *const UProperty,
    destructor_link_next: *const UProperty,
    post_construct_link_next: *const UProperty
}

impl UProperty {
    pub fn base_field(&self) -> UField { self.base_field }
    pub fn name(&self) -> FName { self.base_field.name() }
    pub fn full_name(&self) -> String { self.base_field.full_name() }
}