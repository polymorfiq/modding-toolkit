use crate::WorldProxy;
use ue_types::*;

pub struct GameInstance {
    addr: *const UGameInstance
}

impl GameInstance {
    pub fn instance() -> Option<GameInstance> { Some(Self{addr: WorldProxy::world()?.owning_game_instance}) }
    pub fn base(&self) -> Option<&'static UGameInstance> { unsafe { self.addr.as_ref::<'static>() } }

    pub fn local_players(&self) -> Vec<ULocalPlayer> {
        let mut players: Vec<ULocalPlayer> = vec![];

        let player_array = self.base().unwrap().local_players;

        for i in 0..player_array.len() {
            let player = unsafe { **player_array.at_index(i).unwrap() };
            players.push(player)
        }

        players
    }
}