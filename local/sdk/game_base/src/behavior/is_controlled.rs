use crate::*;
use ue_types::*;

pub trait IsControlled {
    fn controller(self) -> Option<*const AController>;
}

impl IsControlled for *const AController {
    fn controller(self) -> Option<*const AController> { Some(self) }
}

impl IsControlled for *const APlayerController {
    fn controller(self) -> Option<*const AController> { unsafe { Some(std::ptr::addr_of!((*self).base_controller)) } }
}

impl<'a> IsControlled for *const APawn {
    fn controller(self) -> Option<*const AController> { unsafe { Some((*self).controller) } }
}

impl IsControlled for *const ULocalPlayer {
    fn controller(self) -> Option<*const AController> { self.player()?.controller() }
}

impl IsControlled for *const UPlayer {
    fn controller(self) -> Option<*const AController> { self.player_controller()?.controller() }
}