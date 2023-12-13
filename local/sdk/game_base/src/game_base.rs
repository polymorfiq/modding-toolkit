use std::ffi::c_void;
use crate::GAME_BASE;

#[derive(Debug, Copy, Clone)]
pub struct GameBase {
    pub mod_name: &'static str,
    addr_base: Option<*const c_void>
}

impl Default for GameBase {
    fn default() -> Self { Self::empty("default_init") }
}

impl GameBase {
    pub const fn empty(mod_name: &'static str) -> Self {
        Self {
            mod_name: mod_name,
            addr_base: None
        }
    }

    pub fn initialize(
        mod_name: &'static str,
        base_addr: *const c_void
    ) -> &'static mut Self {
        unsafe {
            GAME_BASE.mod_name = mod_name;
            GAME_BASE.addr_base = Some(base_addr);
        };

        println!("GAME ADDRESS BASE: {:p}", base_addr);
        unsafe { &mut GAME_BASE }
    }

    pub fn singleton() -> &'static mut Self {
        unsafe { &mut GAME_BASE }
    }

    pub fn at_offset(&self, offset: isize) -> *const c_void {
        let ptr = unsafe { self.addr_base.expect("Base Address missing...").byte_offset(offset) };
        ptr
    }
}