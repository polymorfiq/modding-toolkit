use crate::*;
pub use widestring::utfstring::Utf16String;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct FString {
    // Size: 0x10
    pub data: TArray<u16, FDefaultAllocator>
}

impl FString {
    pub fn to_wide_string(&self) -> Utf16String {
        let mut chars: Vec<u16> = vec![];
        if self.data.len() == 0 { return "".to_string().into() };

        for i in 0..self.data.len() {
            match self.data.at_index(i) {
                Ok(curr) => chars.push(*curr),
                _ => chars.push('?' as u16)
            }
        }

        Utf16String::from_vec(chars).unwrap()
    }
    pub fn len(&self) -> usize { self.data.len() }
}

impl std::string::ToString for FString {
    fn to_string (&self) -> String {
        self.to_wide_string().to_string().trim_end_matches([0x00 as char]).to_string()
    }
}

impl From<String> for FString {
    fn from(str_data: String) -> FString {
        Utf16String::from_str(str_data.as_str()).into()
    }
}

impl From<&str> for FString {
    fn from(str_data: &str) -> FString {
        Utf16String::from_str(str_data).into()
    }
}

impl From<Utf16String> for FString {
    fn from(ws: Utf16String) -> FString {
        let char_ptr = ws.as_ptr() as *const u16;
        let str_len = ws.len() as u32;
        let capacity = ws.capacity() as u32;

        let str_array = TArray::<u16, FDefaultAllocator>::from_data(char_ptr, str_len, capacity);

        FString{data: str_array}
    }
}

impl std::fmt::Debug for FString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FString")
         .field("str", &self.to_string())
         .finish()
    }
}