use crate::*;
use ue_types::*;

pub trait HasPawn {
    fn pawn(self) -> Option<*const APawn>;
}

impl HasPawn for *const ULocalPlayer {
    fn pawn(self) -> Option<*const APawn> { unsafe { std::ptr::addr_of!((*self).base_player).pawn() } }
}

impl HasPawn for *const UPlayer {
    fn pawn(self) -> Option<*const APawn> { self.player_controller()?.pawn() }
}

impl HasPawn for *const AController {
    fn pawn(self) -> Option<*const APawn> { unsafe { if (*self).pawn != std::ptr::null() { Some((*self).pawn) } else { None } } }
}

impl HasPawn for *const APlayerController {
    fn pawn(self) -> Option<*const APawn> { self.controller()?.pawn() }
}