use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct AXMainMenuGameMode {
    pub base_game_mode: AGameMode,
    pub main_menu_instance: *const UnknownType,
    pub test_server_ip: FString
}