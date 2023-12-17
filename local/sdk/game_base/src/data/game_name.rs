use crate::*;
use ue_types::*;
use std::ffi::CString;

#[derive(Copy, Clone)]
pub struct GameName(*const FName);

pub trait VirtualGameName {
    fn game_name(&self) -> GameName;

    fn f_name(&self) -> FName {
        unsafe { *self.game_name().0 }
    }
    
    fn to_string(&self) -> String { self.to_fstring().to_string() }
    
    fn to_fstring(&self) -> FString {
        let to_string: fn(*const FName, *mut FString) -> *const FString = unsafe {
            std::mem::transmute(GameBase::singleton().at_offset(OFFSETS.base_funcs.fname_to_string))
        };

        let result: Box<FString> = Box::new("".into());
        unsafe { *(to_string)(self.game_name().0, Box::into_raw(result)) }
    }
    
    fn display_name(&self) -> FNameEntry {
        let get_display_name: fn(*const FName) -> *const FNameEntryHeader = unsafe {
            std::mem::transmute(GameBase::singleton().at_offset(OFFSETS.base_funcs.get_display_name))
        };

        let header_ptr = (get_display_name)(self.game_name().0);
        FNameEntry::from_header_ptr(header_ptr)
    }
}

impl VirtualGameName for *const FName {
    fn game_name(&self) -> GameName { GameName(*self) }
}

impl VirtualGameName for FName {
    fn game_name(&self) -> GameName { GameName(std::ptr::addr_of!(*self)) }
}

impl VirtualGameName for GameName {
    fn game_name(&self) -> GameName { *self }
}

impl std::fmt::Debug for GameName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameName")
         .field("str", &VirtualGameName::to_string(self))
         .finish()
    }
}

impl<'a> std::string::ToString for GameName {
    fn to_string (&self) -> String { VirtualGameName::to_string(self) }
}

impl From<String> for GameName {
    fn from(s: String) -> GameName {
        let result: Box<FName> = Box::new(FName{
            comparison_idx: 0.into(),
            display_idx: 0.into(),
            number: 0.into()
        });

        let static_name = Box::leak(result);
        let fname_constructor: fn(*mut FName, *const i8, u32) = unsafe {
            std::mem::transmute(GameBase::singleton().at_offset(OFFSETS.base_funcs.fname_str_constructor) )
        };

        let contents = CString::new(s.as_str()).unwrap();
        let e_find_name: u32 = 0x1; // FName_Add

        (fname_constructor)(&mut *static_name, contents.as_ptr(), e_find_name);
        
        GameName(std::ptr::addr_of!(*static_name))
    }
}


impl From<&str> for GameName {
    fn from(s: &str) -> GameName {
        s.to_string().into()
    }
}