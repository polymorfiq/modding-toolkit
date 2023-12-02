use std::ffi::CStr;
use simple_endian::u32le;
use widestring::U16CString;

const FNAME_WIDE_FLAG: u32 = 0x1;
const FNAME_ENTRY_HEADER_SIZE: isize = 0xC;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FNameEntry {
    // Size: 0xC
    pub header: FNameEntryHeader,
    pub value_ptr: *const i8
}

impl FNameEntry {
    pub fn from_header_ptr(entry_ptr: *const FNameEntryHeader) -> Self {
        let value_ptr = unsafe { entry_ptr.byte_offset(FNAME_ENTRY_HEADER_SIZE) as *const i8 };

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

impl std::string::ToString for FNameEntry {
    fn to_string(&self) -> String {
        let c_str = unsafe { CStr::from_ptr(self.value_ptr) };

        match c_str.to_str() {
            Ok(str_val) => str_val.to_owned(),
            _ => "<fname_entry_error>".to_string()
        }
    }
}