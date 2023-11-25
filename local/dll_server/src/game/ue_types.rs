use simple_endian::*;
use std::ffi::{c_void, CStr};
use widestring::U16CString;

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
pub struct TNameEntryArray<'a> {
    pub chunks: [&'a [*mut FNameEntryHeader; TNAME_ENTRY_ELEMS_PER_CHUNK]; TNAME_ENTRY_CHUNK_TABLE_SIZE],
    pub num_elems: u32le,
    pub num_chunks: u32le
}

const FNAME_HEADER_SIZE: isize = 12;
impl<'a> TNameEntryArray<'a> {
    pub fn get_fname_entry(&self, chunk: usize, idx: usize) -> FNameEntry {
        let chunk = *self.chunks[chunk];
        let header = unsafe { *chunk[idx] };

        let value_ptr = unsafe { (chunk[idx] as *const u8).offset(FNAME_HEADER_SIZE) as *const i8 };
        println!("{:?} + {:?} = {:?}", chunk[idx], FNAME_HEADER_SIZE, value_ptr);

        FNameEntry {
            header: header,
            value_ptr: value_ptr
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FNameEntryHeader {
    pub next_hash: *mut FNameEntryHeader,
    name_idx: u32le
}

impl FNameEntryHeader {
    pub fn idx(&self) -> u32 {
        self.name_idx.to_native() >> 1
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FNameEntry {
    pub header: FNameEntryHeader,
    value_ptr: *const i8
}

const FNAME_WIDE_FLAG: u32 = 0x1;
impl FNameEntry {
    pub fn is_wide(&self) -> bool {
        self.header.name_idx.to_native() & FNAME_WIDE_FLAG > 0
    }

    pub fn value(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.value_ptr) }
    }

    pub fn wide_value(&self) -> U16CString {
        unsafe { U16CString::from_ptr_str(self.value_ptr as *const u16) }
    }

    pub fn byte_len(&self) -> usize {
        let mut len: usize = 0;

        while unsafe { *((self.value_ptr as *const u8).offset(len as isize)) } != 0 {
            if self.is_wide() {
                len += 2;
            } else {
                len += 1;
            }
        }
        
        len
    }

    pub fn char_len(&self) -> usize {
        if self.is_wide() {
            self.byte_len() / 2
        } else {
            self.byte_len()
        }
    }
}