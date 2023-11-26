use simple_endian::*;
use std::ffi::{CStr, c_void};
use widestring::U16CString;

use crate::GameBase;

pub const FNAME_HEADER_SIZE: isize = 12;
const FNAME_WIDE_FLAG: u32 = 0x1;
// const TNAME_ENTRY_FNAME_MAX_ENTRIES: isize = 4 * 1024 * 1024;
// const TNAME_ENTRY_ELEMS_PER_CHUNK: isize = 16384;
// const TNAME_ENTRY_CHUNK_TABLE_SIZE: isize = (TNAME_ENTRY_FNAME_MAX_ENTRIES + TNAME_ENTRY_ELEMS_PER_CHUNK - 1) / TNAME_ENTRY_ELEMS_PER_CHUNK;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FName {
    // Size: 0x0C
    comparison_idx: u32le,
    display_idx: u32le,
    pub number: u32be
}

impl FName {
    pub fn to_string(self) -> Option<String> {
        let entry_ptr = GameBase::singleton().get_display_name(&self);

        if (entry_ptr as *const c_void) == std::ptr::null() {
            None
        } else {
            FNameEntry::from_header_ptr(entry_ptr).string_value()
        }
    }

    pub fn entry(self) -> Option<FNameEntry> {
        let entry_ptr = GameBase::singleton().get_display_name(&self);

        if (entry_ptr as *const c_void) == std::ptr::null() {
            None
        } else {
            Some(FNameEntry::from_header_ptr(entry_ptr))
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TNameEntryArray {
    pub chunks: *mut *mut FNameEntryHeader,
    pub num_elems: u32le,
    pub num_chunks: u32le
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

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FNameEntry {
    pub header: FNameEntryHeader,
    value_ptr: *const i8
}

impl FNameEntry {
    pub fn from_header_ptr(entry_ptr: *const FNameEntryHeader) -> Self {
        let value_ptr = unsafe { entry_ptr.byte_offset(FNAME_HEADER_SIZE) as *const i8 };

        Self {
            header: unsafe { *entry_ptr },
            value_ptr: value_ptr
        }
    }
    
    pub fn is_wide(&self) -> bool {
        self.header.name_idx.to_native() & FNAME_WIDE_FLAG > 0
    }

    pub fn string_value(&self) -> Option<String> {
        let c_str = unsafe { CStr::from_ptr(self.value_ptr) };
        let str_val = c_str.to_str();

        if str_val.is_ok() {
            let str_slice: &str = str_val.unwrap();
            Some(str_slice.to_owned())
        } else {
            None
        }
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