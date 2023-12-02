use ue_types::*;

static mut F_STRING_STORAGE: Vec<OwnedFString> = vec![];

#[derive(Debug, Clone)]
#[repr(C)]
pub struct OwnedFString {
    str_data: WideString
}

impl OwnedFString {}

impl From<OwnedFString> for FString {
    fn from(owned: OwnedFString) -> FString {
        let char_ptr = self.str_data.as_ptr() as *const WideChar;
        let str_len = self.str_data.len() as u32;
        let capacity = self.str_data.capacity() as u32;

        let str_array = TArray::<WideChar>::from_data(char_ptr, str_len, capacity);

        FString{data: str_array}
    }
}

impl From<&str> for OwnedFString {
    fn from(s: &str) -> OwnedFString {
        let wide_str = WideString::from_str(s);
        Self{str_data: wide_str}
    }
}

impl From<String> for OwnedFString {
    fn from(s: String) -> OwnedFString {
        s.as_str().into()
    }
}