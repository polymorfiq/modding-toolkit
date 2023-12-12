use retour::static_detour;
use ue_types::*;
use game_base::*;

static_detour! {
    static RestartPlayer: fn(*const AGameModeBase, *const APlayerController);
}

type PlayerRespawnShiv = fn(*const AGameModeBase, *const APlayerController);
static mut RESPAWN_SHIV: Option<PlayerRespawnShiv> = None;

pub fn on_respawn(shiv: PlayerRespawnShiv) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        RESPAWN_SHIV = Some(shiv);
        let restart_player: fn(*const AGameModeBase, *const APlayerController) = std::mem::transmute(GameBase::singleton().at_offset(0x233CE50));
        RestartPlayer.initialize(restart_player, my_restart_player)?;
        RestartPlayer.enable()?;
    }

    Ok(())
}


fn my_restart_player(this: *const AGameModeBase, player: *const APlayerController) {
    RestartPlayer.call(this, player);
    
    unsafe {
        if RESPAWN_SHIV.is_some() {
            (RESPAWN_SHIV.unwrap())(this, player);
        }
    }
}