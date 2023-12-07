use crate::*;
use ue_types::*;

pub trait IsPlayerControlled {
    fn player_controller(self) -> Option<*const APlayerController>;
}

impl IsPlayerControlled for *const ULocalPlayer {
    fn player_controller(self) -> Option<*const APlayerController> { self.player()?.player_controller() }
}

impl IsPlayerControlled for *const UPlayer {
    fn player_controller(self) -> Option<*const APlayerController> { unsafe { Some((*self).player_controller) } }
}