use std::ffi::CStr;
use simple_endian::u32le;
use widestring::U16CString;

const FNAME_WIDE_FLAG: u32 = 0x1;
const FNAME_ENTRY_HEADER_SIZE: isize = 0xC;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FNameEntry<'a> {
    // Size: 0xC
    pub header: FNameEntryHeader,
    pub value: &'a CStr
}

impl<'a> FNameEntry<'a> {
    pub fn from_header_ptr(entry_ptr: *const FNameEntryHeader) -> Self {
        let value_ptr: *const i8 = unsafe { std::mem::transmute(entry_ptr.byte_offset(FNAME_ENTRY_HEADER_SIZE)) };
        let value = unsafe { CStr::from_ptr(value_ptr) };

        Self {
            header: unsafe { *entry_ptr },
            value: value
        }
    }
    
    pub fn is_wide(&self) -> bool {
        self.header.name_idx.to_native() & FNAME_WIDE_FLAG > 0
    }

    pub fn string_value(&self) -> Option<String> {
        match self.value.to_str() {
            Ok(s) => Some(s.to_owned()),
            _ => None
        }
    }

    pub fn value(&self) -> &CStr { self.value }

    pub fn wide_value(&self) -> U16CString {
        unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(*self.value) as *const u16) }
    }

    pub fn byte_len(&self) -> usize {
        if self.is_wide() {
            self.wide_value().len() * 2
        } else { 
            self.value.to_bytes().len()
        }
    }

    pub fn char_len(&self) -> usize {
        if self.is_wide() {
            self.byte_len() / 2
        } else {
            self.byte_len()
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

impl<'a> std::string::ToString for FNameEntry<'a> {
    fn to_string(&self) -> String {
        match self.value.to_str() {
            Ok(str_val) => str_val.to_owned(),
            _ => "<fname_entry_error>".to_string()
        }
    }
}