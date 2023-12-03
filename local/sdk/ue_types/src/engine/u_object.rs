use crate::*;
use simple_endian::u32le;

pub mod object_flags {
    pub const NONE: u32 = 0;
	pub const REACHABLE_IN_CLUSTER: u32 = 1 << 23; // External reference to object in cluster exists
	pub const CLUSTER_ROOT: u32 = 1 << 24; // Root of a cluster
	pub const NATIVE: u32 = 1 << 25; // Native (UClass only). 
	pub const ASYNC_ONLY: u32 = 1 << 26; // Object exists only on a different thread than the game thread.
	pub const ASYNC_LOADING: u32 = 1 << 27; // Object is being asynchronously loaded.
	pub const UNREACHABLE: u32 = 1 << 28; // Object is not reachable on the object graph.
	pub const PENDING_KILL: u32 = 1 << 29; // Objects that are pending destruction (invalid for gameplay but valid objects)
	pub const ROOT_SET: u32 = 1 << 30; // Object will not be garbage collected, even if unreferenced.
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct UObject {
    // Size: 0x30
    base: UObjectBase
}

impl UObject {
    pub fn virtual_funcs(&self) -> *const *const UnknownType { self.base.vf_table }
    pub fn name(&self) -> FName { self.base.name_private }

    pub fn class(&self) -> &UClass {
        unsafe { self.base.class_private.as_ref::<'static>().unwrap() }
    }

    pub fn outer(&self) -> Option<UObject> {
        if self.base.outer_private != std::ptr::null() {
            Some(unsafe { *self.base.outer_private })
        } else {
            None
        }
    }

    pub fn full_name(&self) -> String {
        let my_name = self.name();
        let my_name_string = my_name.to_string();

        match self.outer() {
            Some(outer) => [outer.full_name(), my_name_string].join("."),
            None => my_name_string
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
struct UObjectBase {
    // Size: 0x30
    pub vf_table: *const *const UnknownType,
    pub obj_flags: u32,
    internal_idx: u32le,
    pub class_private: *const UClass,
    name_private: FName,
    _padding: [u8; 4],
    outer_private: *const UObject
}