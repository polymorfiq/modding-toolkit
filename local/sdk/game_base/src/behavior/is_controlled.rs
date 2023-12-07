use crate::*;
use ue_types::*;

pub trait IsControlled {
    fn controller(&self) -> Option<&AController>;
}

impl<'a> IsControlled for APawn {
    fn controller(&self) -> Option<&AController> { unsafe { self.controller.as_ref() } }
}

impl IsControlled for ULocalPlayer {
    fn controller(&self) -> Option<&AController> { self.player()?.controller() }
}

impl IsControlled for UPlayer {
    fn controller(&self) -> Option<&AController> { self.player_controller()?.controller() }
}

impl IsControlled for AController {
    fn controller(&self) -> Option<&AController> { Some(self) }
}

impl IsControlled for APlayerController {
    fn controller(&self) -> Option<&AController> { Some(&self.base_controller) }
}