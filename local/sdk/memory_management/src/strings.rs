use ue_types::*;
use ue_types::ue_widestring::{WideString, WideChar};

static mut STRINGS: Vec<WideString> = vec![];
static mut FSTRINGS: Vec<FString> = vec![];

pub fn fstring(data: String) -> &'static FString {
    let ws = unsafe {
        let ws = WideString::from_str(data.as_str());
        STRINGS.push(ws);
        STRINGS.last().unwrap()
    };

    let char_ptr = ws.as_ptr() as *const WideChar;
    let str_len = ws.len() as u32;
    let capacity = ws.capacity() as u32;

    let str_array = TArray::<WideChar, FDefaultAllocator>::from_data(char_ptr, str_len, capacity);

    unsafe {
        let f_str = FString{data: str_array};
        FSTRINGS.push(f_str);
        FSTRINGS.last().unwrap()
    }
}