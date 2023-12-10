use crate::*;
use ue_types::*;
use utils::warning;

pub struct MainMenu {}

impl MainMenu {
    pub fn connect_to_server(ip: &str, port: &str, args: &str) {
        let game_mode = Self::game_mode();
        if game_mode.is_none() { warning!("No MainMenuGameMode found!") }

        let ip_and_port_fstr: FString = format!("{}:{}", ip, port).into();
        let args_fstr: FString = args.into();

        let connect_to_server: fn(*const AXMainMenuGameMode, *const FString, *const FString) = unsafe {
            std::mem::transmute(GameBase::singleton().at_offset(crate::offsets::OFFSET_FUNC_MAIN_MENU_CONNECT_TO_SERVER))
        };

        let base_game_mode = (*game_mode.unwrap()).base_game_mode;
        println!("GAME MODE: {:p}", std::ptr::addr_of!(base_game_mode));

        (connect_to_server)(
            std::ptr::addr_of!((*game_mode.unwrap()).base_game_mode),
            std::ptr::addr_of!(ip_and_port_fstr),
            std::ptr::addr_of!(args_fstr)
        );
    }

    pub fn game_mode<'a>() -> Option<&'a AGMainMenuGameMode> {
        let main_menu_game_modes = GObjects::filter(|object| object.name().to_string().as_str() == "Default__GMainMenuGameMode");

        if main_menu_game_modes.len() > 0 {
            unsafe { (main_menu_game_modes[0] as *const AGMainMenuGameMode).as_ref() }
        } else {
            None
        }
    }
}