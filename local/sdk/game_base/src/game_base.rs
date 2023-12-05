use std::ffi::c_void;
use ue_types::*;

use crate::{
    GAME_BASE,
    OFFSET_FUNC_GET_DISPLAY_NAME,
    OFFSET_FUNC_GETNAMES,
    OFFSET_STRUCT_GOBJECTS,
    OFFSET_STRUCT_UWORLD_PROXY
};

#[derive(Debug, Copy, Clone)]
pub struct GameBase {
    pub mod_name: &'static str,
    addr_base: Option<*const c_void>,
    addr_gobjects: Option<*const c_void>,
    addr_gnames: Option<*const c_void>,
    addr_get_display_name: Option<*const c_void>,
    pub game_console: Option<&'static UConsole>
}

impl Default for GameBase {
    fn default() -> Self { Self::empty("default_init") }
}

impl GameBase {
    pub const fn empty(mod_name: &'static str) -> Self {
        Self {
            mod_name: mod_name,
            addr_base: None,
            addr_gobjects: None,
            addr_gnames: None,
            addr_get_display_name: None,
            game_console: None
        }
    }

    pub fn initialize(
        mod_name: &'static str,
        base_addr: *const c_void
    ) -> &'static Self {
        let addr_gobjects = unsafe { base_addr.byte_offset(OFFSET_STRUCT_GOBJECTS) };
        
        let get_names: fn() -> *const c_void = unsafe { 
            std::mem::transmute(base_addr.byte_offset(OFFSET_FUNC_GETNAMES))
        };
        let addr_gnames = (get_names)();
        let addr_get_display_name = unsafe { base_addr.byte_offset(OFFSET_FUNC_GET_DISPLAY_NAME) };

        let mut generated = Self {
            mod_name: mod_name,
            addr_base: Some(base_addr),
            addr_gobjects: Some(addr_gobjects),
            addr_gnames: Some(addr_gnames),
            addr_get_display_name: Some(addr_get_display_name),
            ..Default::default()
        };

        ue_types::funcs::set_get_display_name(generated.get_display_name_fn());
        generated.search_game_objects(mod_name);
        Self::set_singleton(generated);

        unsafe { &GAME_BASE }
    }

    pub fn search_game_objects(&mut self, mod_name: &'static str) {
        let mut attempt_num:usize = 0;
        loop {
            if self.search_game_objects_attempt() {
                break
            } else if attempt_num > 20 {
                println!("Elefrac Game SDK ({}) - Couldn't find Game Engine & Console! Giving up!", mod_name);
                break;
            } else {
                attempt_num += 1;
                std::thread::sleep(std::time::Duration::from_millis(5000));
            }
        }
    }

    pub fn search_game_objects_attempt(&mut self) -> bool {
        let g_objects = self.gobjects();

        for i in 0..(g_objects.num_elements.to_native()-1) {
            let item = g_objects.item_at_idx(i as usize);
            let object = if item.is_some() { item.unwrap().object::<UObject>() } else { None };
            let obj_name = if object.is_some() { Some(object.unwrap().full_name()) } else { None };

            if obj_name == Some("/Engine/Transient.GameEngine.GGameViewportClient.Console".to_string()) {
                self.game_console = item.expect("Unable to unwrap Game Console Item").object::<UConsole>();
            }
        }

        self.game_console.is_some()
    }

    pub fn singleton() -> &'static Self {
        unsafe { &GAME_BASE }
    }
    
    pub fn set_singleton(base: Self) -> &'static Self {
        unsafe {
            GAME_BASE = base;
            &GAME_BASE
        }
    }

    pub fn gobjects(&self) -> FChunkedFixedUObjectArray {
        let ptr = self.addr_gobjects.expect("GOBJECT address missing...") as *const FUObjectArray;
        unsafe { (*ptr).objects_array }
    }

    pub fn gnames(&self) -> TNameEntryArray {
        let ptr = self.addr_gnames.expect("GNAMES address missing...") as *const TNameEntryArray;
        unsafe { *ptr }
    }

    pub fn game_instance(&self) -> Option<&'static UGameInstance> {
        let world = self.world();
        if world.is_none() { return None };

        // unsafe {
        //     world.unwrap().owning_game_instance.as_ref::<'static>()
        // }
        None
    }

    pub fn level(&self) -> Option<&'static ULevel> {
        let world = self.world();
        if world.is_none() { return None };
        world.unwrap().level()
    }

    pub fn world(&self) -> Option<&'static UWorld> {
        if !self.addr_base.is_some() { return None };

        let uworld_proxy = unsafe {
            let ptr = self.addr_base.unwrap().byte_offset(OFFSET_STRUCT_UWORLD_PROXY) as *const UWorldProxy;
            ptr.as_ref::<'static>().unwrap()
        };
        
        unsafe { uworld_proxy.world.unwrap().as_ref::<'static>() }
    }

    pub fn console(&self) -> Option<&'static UConsole> { self.game_console }

    pub fn get_display_name_fn(&self) -> Option<fn(*const FName) -> *const FNameEntryHeader> {
        match self.addr_get_display_name {
            Some(fn_addr) => {
                let get_display_name: fn(*const FName) -> *const FNameEntryHeader = unsafe {
                    std::mem::transmute(fn_addr)
                };
                
                Some(get_display_name)
            },

            None => None
        }
    }

    pub fn at_offset(&self, offset: isize) -> *const c_void {
        let ptr = unsafe { self.addr_base.expect("Base Address missing...").byte_offset(offset) };
        ptr
    }

    pub fn get_offset_from_addr<T>(&self, addr: *const T) -> usize {
        (addr as usize) - (self.addr_base.expect("Base Address missing...") as usize)
    }

    pub fn get_offset<T>(&self, addr: &T) -> usize {
        (std::ptr::addr_of!(*addr) as usize) - (self.addr_base.expect("Base Address missing...") as usize)
    }
}