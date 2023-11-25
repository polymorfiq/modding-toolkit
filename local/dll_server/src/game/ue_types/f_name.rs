use simple_endian::*;
use std::ffi::{c_void, CStr};
use widestring::U16CString;

const FNAME_HEADER_SIZE: isize = 12;
const FNAME_WIDE_FLAG: u32 = 0x1;
const TNAME_ENTRY_FNAME_MAX_ENTRIES: usize = 4 * 1024 * 1024;
const TNAME_ENTRY_ELEMS_PER_CHUNK: usize = 16384;
const TNAME_ENTRY_CHUNK_TABLE_SIZE: usize = (TNAME_ENTRY_FNAME_MAX_ENTRIES + TNAME_ENTRY_ELEMS_PER_CHUNK - 1) / TNAME_ENTRY_ELEMS_PER_CHUNK;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FName {
    comparison_idx: u32le,
    display_idx: u32le,
    number: u32
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TNameEntryArray {
    pub chunks: *mut *mut FNameEntryHeader,
    pub num_elems: u32le,
    pub num_chunks: u32le
}

impl TNameEntryArray {
    pub fn entry_for_fname(&self, name: FName) -> Option<FNameEntry> {
        self.entry_at_idx(name.display_idx.to_native() as usize)
    }

    pub fn entry_at_idx(&self, idx: usize) -> Option<FNameEntry> {
        let chunk_idx = idx / TNAME_ENTRY_ELEMS_PER_CHUNK;
        let elem_idx = idx % TNAME_ENTRY_ELEMS_PER_CHUNK;

        self.get_entry(chunk_idx, elem_idx)
    }

    pub fn get_entry(&self, chunk: usize, elem_idx: usize) -> Option<FNameEntry> {
        let chunk = unsafe { *self.chunks.offset(chunk as isize) };
        let header = unsafe { *chunk.offset(elem_idx as isize) };

        let value_ptr = unsafe { (chunk.offset(elem_idx as isize) as *const u8).offset(FNAME_HEADER_SIZE) as *const i8 };

        Some(FNameEntry {
            header: header,
            value_ptr: value_ptr
        })
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