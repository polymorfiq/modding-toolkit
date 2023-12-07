use crate::*;
use ue_types::*;

pub trait IsPlayerControlled {
    fn player_controller(&self) -> Option<&APlayerController>;
}

impl IsPlayerControlled for ULocalPlayer {
    fn player_controller(&self) -> Option<&APlayerController> { self.player()?.player_controller() }
}

impl IsPlayerControlled for UPlayer {
    fn player_controller(&self) -> Option<&APlayerController> { unsafe { self.player_controller.as_ref() } }
}