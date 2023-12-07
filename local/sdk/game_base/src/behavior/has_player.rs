use ue_types::*;

pub trait HasPlayer {
    fn player(self) -> Option<*const UPlayer>;
}

impl HasPlayer for *const ULocalPlayer {
    fn player(self) -> Option<*const UPlayer> { unsafe { Some(std::ptr::addr_of!((*self).base_player)) } }
}

impl HasPlayer for *const UPlayer {
    fn player(self) -> Option<*const UPlayer> { Some(self) }
}