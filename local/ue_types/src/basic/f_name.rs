use crate::*;
use simple_endian::u32le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FName {
    // Size: 0x0C
    pub comparison_idx: u32le,
    pub display_idx: u32le,
    pub number: u32le
}

impl FName {
    pub fn entry<'a>(&'a self) -> Result<FNameEntry, &'static str> { crate::funcs::get_display_name(&self) }
}

impl std::string::ToString for FName {
    fn to_string(&self) -> String {
        match self.entry() {
            Ok(entry) => entry.to_string(),
            Err(err_str) => format!("<error_parsing_fname: {}>", err_str).to_string()
        }
    }
}