use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct AGMainMenuGameMode {
    pub base_game_mode: AXMainMenuGameMode,
    pub menu_manager: *const UnknownType,
    pub modal_manager: *const UnknownType,
    pub main_menu_template: *const UClass,
    // More stuff...
}