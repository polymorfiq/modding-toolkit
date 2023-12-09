use crate::GameBase;
use ue_types::*;

pub struct GNames {}

impl GNames {
    pub fn gnames() -> &'static TNameEntryArray<'static> {
        let get_names: fn() -> *const TNameEntryArray<'static> = unsafe { 
            std::mem::transmute(GameBase::singleton().at_offset(crate::offsets::OFFSET_FUNC_GETNAMES))
        };

        unsafe { std::mem::transmute((get_names)()) }
    }
}