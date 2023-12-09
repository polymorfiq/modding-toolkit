use crate::*;
use ue_types::*;

#[derive(Copy, Clone)]
pub struct GameName(*const FName);

pub trait VirtualGameName {
    fn game_name(&self) -> GameName;
    
    fn to_string(&self) -> String { self.to_fstring().to_string() }
    
    fn to_fstring(&self) -> FString {
        let to_string: fn(*const FName, *mut FString) -> *const FString = unsafe {
            std::mem::transmute(GameBase::singleton().at_offset(crate::offsets::OFFSET_FUNC_FNAME_TO_STRING))
        };

        let result: Box<FString> = Box::new("".into());
        unsafe { *(to_string)(self.game_name().0, Box::into_raw(result)) }
    }
    
    fn display_name(&self) -> FNameEntry {
        let get_display_name: fn(*const FName) -> *const FNameEntryHeader = unsafe {
            std::mem::transmute(GameBase::singleton().at_offset(crate::offsets::OFFSET_FUNC_GET_DISPLAY_NAME))
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