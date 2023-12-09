use std::ffi::c_void;
use ue_types::*;

use crate::GAME_BASE;
use crate::{Console, GObjects, VirtualObject};

#[derive(Debug, Copy, Clone)]
pub struct GameBase {
    pub mod_name: &'static str,
    addr_base: Option<*const c_void>,
    pub game_console: Option<Console>
}

impl Default for GameBase {
    fn default() -> Self { Self::empty("default_init") }
}

impl GameBase {
    pub const fn empty(mod_name: &'static str) -> Self {
        Self {
            mod_name: mod_name,
            addr_base: None,
            game_console: None
        }
    }

    pub fn initialize(
        mod_name: &'static str,
        base_addr: *const c_void
    ) -> &'static Self {
        unsafe {
            GAME_BASE.mod_name = mod_name;
            GAME_BASE.addr_base = Some(base_addr);
        };

        println!("GAME ADDRESS BASE: {:p}", base_addr);

        unsafe {
            GAME_BASE.search_game_objects();
            &GAME_BASE
        }
    }

    pub fn search_game_objects(&mut self) {
        let mut attempt_num:usize = 0;
        loop {
            if self.search_game_objects_attempt() {
                break
            } else if attempt_num > 20 {
                println!("Elefrac Game SDK ({}) - Couldn't find Game Engine & Console! Giving up!", unsafe { GAME_BASE.mod_name });
                break;
            } else {
                attempt_num += 1;
                std::thread::sleep(std::time::Duration::from_millis(5000));
            }
        }
    }

    pub fn search_game_objects_attempt(&mut self) -> bool {
        let g_objects = GObjects::objects::<UnknownType>();
        let console_regex = regex::Regex::new(r"/Engine/Transient.GameEngine_([0-9]+)\.GGameViewportClient_([0-9]+)\.Console_([0-9]+)").unwrap();

        for i in 0..(g_objects.num_elements.to_native()-1) {
            let item = g_objects.item_at_idx(i as usize);
            let object = if item.is_some() { unsafe { (*item.unwrap()).object::<UObject<*const UnknownType>>() } } else { None };
            let obj_name = if object.is_some() { Some(object.unwrap().full_name()) } else { None };

            if obj_name.is_some() && console_regex.is_match(obj_name.clone().unwrap().as_str()) {
                let console = unsafe { (*item.expect("Unable to unwrap Game Console Item")).object_addr as *const UConsole };
                println!("GAME CONSOLE: {:p}", console);

                self.game_console = Some(Console::new(console));
            }
        }

        self.game_console.is_some()
    }

    pub fn singleton() -> &'static Self {
        unsafe { &GAME_BASE }
    }

    pub fn console(&self) -> Option<Console> { self.game_console }

    pub fn at_offset(&self, offset: isize) -> *const c_void {
        let ptr = unsafe { self.addr_base.expect("Base Address missing...").byte_offset(offset) };
        ptr
    }
}