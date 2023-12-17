use crate::GameBase;
use ue_types::*;

pub struct WorldProxy {}

impl WorldProxy {
    pub fn proxy() -> Option<&'static UWorldProxy> {
        unsafe { (GameBase::singleton().at_offset(crate::OFFSETS.base_structs.uworld_proxy) as *const UWorldProxy).as_ref::<'static>() }
    }

    pub fn world() -> Option<&'static UWorld> {
        unsafe {
            Self::proxy()?.world.as_ref::<'static>()
        }
    }

    pub fn level<'a>() -> Option<&'a ULevel> {
        Self::world()?.level()
    }
}