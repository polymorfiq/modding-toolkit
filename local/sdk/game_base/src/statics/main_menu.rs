use crate::*;
use ue_types::*;

pub struct MainMenu {}

impl MainMenu {
    pub fn game_mode<'a>() -> Option<&'a AGMainMenuGameMode> {
        let main_menu_game_modes = GObjects::filter(|object| object.name().to_string().as_str() == "Default__GMainMenuGameMode");

        if main_menu_game_modes.len() > 0 {
            unsafe { (main_menu_game_modes[0] as *const AGMainMenuGameMode).as_ref() }
        } else {
            None
        }
    }
}