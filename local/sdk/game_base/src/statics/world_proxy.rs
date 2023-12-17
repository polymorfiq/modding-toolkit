use crate::GameBase;
use ue_types::*;

pub struct WorldProxy {}

impl WorldProxy {
    pub fn proxy() -> &'static UWorldProxy {
        unsafe { (GameBase::singleton().at_offset(crate::OFFSETS.base_structs.uworld_proxy) as *const UWorldProxy).as_ref::<'static>().unwrap() }
    }

    pub fn world() -> &'static UWorld {
        unsafe {
            Self::proxy().world.unwrap().as_ref::<'static>().unwrap()
        }
    }

    pub fn level<'a>() -> &'a ULevel {
        unsafe {
            (*Self::proxy().world.unwrap()).level().as_ref().unwrap()
        }
    }
}