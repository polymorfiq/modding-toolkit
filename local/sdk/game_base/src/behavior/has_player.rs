use ue_types::*;

pub trait HasPlayer {
    fn player(&self) -> Option<&UPlayer>;
}

impl HasPlayer for ULocalPlayer {
    fn player(&self) -> Option<&UPlayer> { Some(&self.base_player) }
}

impl HasPlayer for UPlayer {
    fn player(&self) -> Option<&UPlayer> { Some(self) }
}