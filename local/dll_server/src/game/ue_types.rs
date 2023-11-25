use simple_endian::*;
use core::ffi::c_void;

const GOBJECTS_NUM_ELEMS_PER_CHUNK: u32 = 64 * 1024;

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
    pub objects_ptr: *mut *mut FUObjectItem,
    pub pre_allocated_objects_ptr: *mut FUObjectItem,
    pub max_elements: u32le,
    pub num_elements: u32le,
    pub max_chunks: u32le,
    pub num_chunks: u32le,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FUObjectItem {
    object: *mut c_void,
    flags: u32,
    cluster_root_idx: u32,
    serial_number: u32
}

const TNAME_ENTRY_FNAME_MAX_ENTRIES: usize = 4 * 1024 * 1024;
const TNAME_ENTRY_ELEMS_PER_CHUNK: usize = 16384;
const TNAME_ENTRY_CHUNK_TABLE_SIZE: usize = (TNAME_ENTRY_FNAME_MAX_ENTRIES + TNAME_ENTRY_ELEMS_PER_CHUNK - 1) / TNAME_ENTRY_ELEMS_PER_CHUNK;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TNameEntryArray {
    chunks: [*mut *mut FNameEntry; TNAME_ENTRY_CHUNK_TABLE_SIZE],
    num_elems: u32le,
    num_chunks: u32le
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FNameEntry {
    something: u64
}