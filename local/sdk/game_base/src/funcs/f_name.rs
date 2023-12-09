use crate::*;
use ue_types::*;

pub trait FNameFuncs {
    fn to_fstring(&self) -> Option<FString>;
}

impl FNameFuncs for FName {
    fn to_fstring(&self) -> Option<FString> {
        let to_string_fn: fn(this: *const FName, result: *mut FString) -> *const FString = unsafe { 
            std::mem::transmute(GameBase::singleton().at_offset(offsets::OFFSET_FUNC_FNAME_TO_STRING))
        };

        let result: Box<FString> = Box::new("".into());
        let result = (to_string_fn)(self, Box::into_raw(result));
        unsafe { Some(*result) }
    }
}

impl From<&dyn FNameFuncs> for FString {
    fn from(name: &dyn FNameFuncs) -> FString {
        name.to_fstring().unwrap()
    }
}