use simple_endian::*;
use std::ffi::{c_void, CStr};
use widestring::U16CString;

use super::f_name::{FName, FNameEntry, FNameEntryHeader, TNameEntryArray};

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
        let elem_idx = idx % GOBJECTS_NUM_ELEMS_PER_CHUNK;

        if idx >= (self.max_elements.to_native() as usize) || chunk_idx >= (self.num_chunks.to_native() as usize) {
            None
        } else {
            let chunk = unsafe { *self.objects.offset(chunk_idx as isize) };
            unsafe { Some(*chunk.offset(elem_idx as isize)) }
        }
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
    pub fn object(&self) -> UObjectBase {
        unsafe { *self.object }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UObjectBase {
    __vf_table: *mut c_void,
    pub obj_flags: u32,
    internal_idx: u32le,
    class_private: *mut c_void,
    name_private: FName,
    _padding: [u8; 4],
    outer_private: *mut UObjectBase
}

impl UObjectBase {
    pub fn name(&self) -> FName {
        self.name_private
    }
}