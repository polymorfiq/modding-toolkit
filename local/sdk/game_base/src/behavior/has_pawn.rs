use crate::*;
use ue_types::*;

pub trait HasPawn {
    fn pawn(&self) -> Option<&APawn>;
}

impl HasPawn for ULocalPlayer {
    fn pawn(&self) -> Option<&APawn> { self.base_player.pawn() }
}

impl HasPawn for UPlayer {
    fn pawn(&self) -> Option<&APawn> { self.player_controller()?.pawn() }
}

impl HasPawn for AController {
    fn pawn(&self) -> Option<&APawn> { unsafe { self.pawn.as_ref() } }
}

impl HasPawn for APlayerController {
    fn pawn(&self) -> Option<&APawn> { self.controller()?.pawn() }
}