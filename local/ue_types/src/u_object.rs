use simple_endian::*;
use std::ffi::c_void;

use super::{FName, UClass};

const GOBJECTS_NUM_ELEMS_PER_CHUNK: usize = 64 * 1024;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FUObjectArray {
    obj_first_gc_idx: u32le,
    obj_last_non_gc_idx: u32le,
    max_objects_not_considered_by_gc: u32le,
    open_for_disregard_for_gc: u8,
    _padding: [u8; 3],
    pub objects_array: FChunkedFixedUObjectArray,
    various_data: [u8; 990]
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FChunkedFixedUObjectArray {
    pub objects: *mut *mut FUObjectItem,
    pub pre_allocated_objects: *mut FUObjectItem,
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
    object: *mut UObjectBase,
    pub flags: u32,
    pub cluster_root_idx: u32,
    pub serial_number: u32
}

impl FUObjectItem {
    pub fn object(&self) -> Option<UObjectBase> {
        if (self.object as *const c_void) == std::ptr::null() {
            None
        } else {
            unsafe { Some(*self.object) }
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UObjectBase {
    __vf_table: *mut c_void,
    pub obj_flags: u32,
    internal_idx: u32le,
    pub class_private: *mut UClass,
    name_private: FName,
    _padding: [u8; 4],
    outer_private: *const UObjectBase
}

impl UObjectBase {
    pub fn name(&self) -> FName {
        self.name_private
    }

    pub fn outer(&self) -> Option<UObjectBase> {
        if self.outer_private != std::ptr::null() {
            Some(unsafe { *self.outer_private })
        } else {
            None
        }
    }

    pub fn get_full_name(&self) -> String {
        let my_name = self.name();
        let my_name_str = my_name.entry().unwrap().value().to_str().unwrap().to_string();

        match self.outer() {
            Some(outer) => {
                [outer.get_full_name(), my_name_str].join(".")
            }

            None => my_name_str
        }
    }
}