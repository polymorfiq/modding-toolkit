use crate::*;

static mut _GET_DISPLAY_NAME: Option<fn(*const FName) -> *const FNameEntryHeader> = None;
pub fn get_display_name<'a>(f_name: &'a FName) -> Result<FNameEntry, &'static str> {
    let display_name_fn = unsafe { _GET_DISPLAY_NAME.expect("get_display_name() called when not hooked!") };
    let entry_header = unsafe { (display_name_fn)(std::ptr::addr_of!(*f_name)).as_ref::<'a>() };

    if entry_header.is_some() {
        Ok(FNameEntry::from_header_ptr(entry_header.unwrap()))
    } else {
        Err("Could not unwrap entry_header pointer..")
    }
}

pub fn set_get_display_name(fnc_ptr: Option<fn(*const FName) -> *const FNameEntryHeader>) {
    unsafe { _GET_DISPLAY_NAME = fnc_ptr };
}