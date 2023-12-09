use crate::WorldProxy;
use ue_types::*;

pub struct GameInstance {}

impl GameInstance {
    pub fn instance() -> &'static UGameInstance {
        unsafe { WorldProxy::world().owning_game_instance.as_ref::<'static>().unwrap() }
    }
}