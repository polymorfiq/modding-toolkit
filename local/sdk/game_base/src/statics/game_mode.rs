use crate::GameWorld;
use ue_types::*;

pub struct GameMode {
    addr: *const AXGameMode
}

impl GameMode {
    pub fn mode() -> Option<GameMode> {
        let game_mode = GameWorld::world()?.base()?.authority_game_mode;
        if game_mode == std::ptr::null() { return None }

        Some(Self{addr: game_mode})
    }

    pub fn base(&self) -> Option<&'static AXGameMode> { unsafe { (self.addr as *const AXGameMode).as_ref::<'static>() } }
    pub fn state(&self) -> Option<&'static AXGameState> { unsafe { self.base()?.base_game_mode.game_state.as_ref() } }
    pub fn players(&self) -> Vec<&'static APlayerState> {
        let state = self.state();
        if state.is_none() { return vec![] };
        let state = state.unwrap();

        let mut players: Vec<&'static APlayerState> = vec![];
        for i in 0..state.base_game_state.player_array.len() {
            let player = state.base_game_state.player_array.at_index(i);
            if !player.is_ok() { continue };

            let player_ref = unsafe { player.unwrap().as_ref() };
            if player_ref.is_some() {
                players.push(player_ref.unwrap());
            }
        }

        players
    }
}