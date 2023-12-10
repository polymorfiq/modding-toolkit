use crate::*;
use simple_endian::*;

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
pub struct UObject<VFTable> {
    // Size: 0x30
    pub base: UObjectBase<VFTable>
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct UObjectBase<VFTable> {
    // Size: 0x30
    pub vftable: VFTable,
    pub object_flags: u32,
    pub internal_idx: u32le,
    pub class_private: *const UClass,
    pub name_private: FName,
    _padding: [u8; 4],
    pub outer_private: *const UObject<*const UnknownType>
}