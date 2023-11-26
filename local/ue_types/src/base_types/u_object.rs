use simple_endian::*;
use std::ffi::c_void;

use super::{FName, UClass};

const GOBJECTS_NUM_ELEMS_PER_CHUNK: usize = 64 * 1024;
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
#[repr(C)]
pub struct UObject {
    // Size: 0x30
    base: UObjectBase
}

impl UObject {
    pub fn name(&self) -> FName {
        self.base.name_private
    }

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

    pub fn get_full_name(&self) -> String {
        let my_name = self.name();
        let my_name_string = match my_name.to_string() {
            Some(name) => name,
            None => "?".to_string()
        };

        match self.outer() {
            Some(outer) => {
                [outer.get_full_name(), my_name_string].join(".")
            }

            None => my_name_string
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FUObjectArray {
    // Size: 0x130
    obj_first_gc_idx: u32le,
    obj_last_non_gc_idx: u32le,
    max_objects_not_considered_by_gc: u32le,
    open_for_disregard_for_gc: u8,
    _padding: [u8; 3],
    pub objects_array: FChunkedFixedUObjectArray,
    various_data: [u8; 0x100] // 0x30
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FChunkedFixedUObjectArray {
    // Size: 0x20
    pub objects: *const *const FUObjectItem,
    pub pre_allocated_objects: *const FUObjectItem,
    pub max_elements: u32le,
    pub num_elements: u32le,
    pub max_chunks: u32le,
    pub num_chunks: u32le,
}

impl FChunkedFixedUObjectArray {
    pub fn item_at_idx(&self, idx: usize) -> Option<FUObjectItem> {
		let chunk_idx = idx / GOBJECTS_NUM_ELEMS_PER_CHUNK;
        let elem_idx = idx % (GOBJECTS_NUM_ELEMS_PER_CHUNK - 1);

        if idx >= self.max_elements.to_native() as usize { return None; }
        if chunk_idx >= (self.num_chunks.to_native() as usize) { return None; }
        if elem_idx >= (self.num_elements.to_native() as usize) { return None; }

        let chunk_ptr = unsafe { self.objects.offset(chunk_idx as isize) };
        if (chunk_ptr as *const c_void) == std::ptr::null() { return None; }
        let chunk = unsafe { *chunk_ptr };

        let elem_ptr = unsafe { chunk.offset(elem_idx as isize) };
        if (elem_ptr as *const c_void) == std::ptr::null() { return None; }

        unsafe { Some(*elem_ptr) }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FUObjectItem {
    // Size: 0x14
    pub object_addr: *mut UObject,
    pub flags: u32,
    pub cluster_root_idx: u32,
    pub serial_number: u32
}

impl FUObjectItem {
    pub fn object(&self) -> Option<UObject> {
        if (self.object_addr as *const c_void) == std::ptr::null() {
            None
        } else {
            unsafe { Some(*self.object_addr) }
        }
    }

    pub fn is_root_set(&self) -> bool {
        self.flags & object_flags::ROOT_SET > 0
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct UObjectBase {
    // Size: 0x30
    __vf_table: *mut c_void,
    pub obj_flags: u32,
    internal_idx: u32le,
    pub class_private: *mut UClass,
    name_private: FName,
    _padding: [u8; 4],
    outer_private: *const UObject
}