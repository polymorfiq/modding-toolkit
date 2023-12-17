use crate::WorldProxy;
use ue_types::*;

pub struct GameLevel {
    addr: *const ULevel
}

impl GameLevel {
    pub fn level() -> Option<GameLevel> { Some(Self{addr: std::ptr::addr_of!(*WorldProxy::level()?)}) }
    pub fn base(&self) -> Option<&'static ULevel> { unsafe { self.addr.as_ref::<'static>() } }
}