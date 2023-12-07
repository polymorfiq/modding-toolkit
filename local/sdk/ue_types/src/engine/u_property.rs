use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UProperty {
    pub base_field: UField,
    pub array_dim: u32,
    pub element_size: u32,
    pub e_property_flags: u64,
    pub rep_index: u16,
    pub blueprint_replication_condition: u8,
    pub offset_interval: u32,
    pub rep_notify_func: FName,
    pub property_link_next: *const UProperty,
    pub next_ref: *const UProperty,
    pub destructor_link_next: *const UProperty,
    pub post_construct_link_next: *const UProperty
}

impl UProperty {}