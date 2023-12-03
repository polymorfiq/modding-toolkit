use crate::*;
use widestring::{WideChar, WideString};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FString {
    // Size: 0x10
    pub data: TArray<WideChar>
}

impl FString {
    pub fn to_wide_string(&self) -> WideString {
        let mut chars: Vec<WideChar> = vec![];
        if self.data.len() == 0 { return "".to_string().into() };

        for i in 0..self.data.len() {
            match self.data.at_index(i) {
                Ok(curr) => chars.push(*curr),
                _ => chars.push('?' as WideChar)
            }
        }

        WideString::from_vec(chars)
    }
    pub fn len(&self) -> usize { self.data.len() }
}

impl std::string::ToString for FString {
    fn to_string (&self) -> String {
        match self.to_wide_string().to_string() {
            Ok(string_data) => string_data,
            _ => "<fstring_parse_error>".to_string()
        }
    }
}

impl From<WideString> for FString {
    fn from(ws: WideString) -> FString {
        let char_ptr = ws.as_ptr() as *const WideChar;
        let str_len = ws.len() as u32;
        let capacity = ws.capacity() as u32;

        let str_array = TArray::<WideChar>::from_data(char_ptr, str_len, capacity);

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