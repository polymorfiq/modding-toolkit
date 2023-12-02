#![feature(pointer_byte_offsets)]
use std::ffi::c_void;
use ue_types::*;

static mut GAME_BASE: GameBase = GameBase::empty("static_init");

#[derive(Debug, Copy, Clone)]
pub struct GameBase {
    pub mod_name: &'static str,
    addr_base: Option<*const c_void>,
    addr_gobjects: Option<*const c_void>,
    addr_gnames: Option<*const c_void>,
    addr_get_display_name: Option<*const c_void>,
    game_instance: Option<&'static UGameInstance<'static>>,
    world: Option<&'static UWorld<'static>>,
    game_console: Option<&'static UConsole>
}

#[cfg(feature = "server-sdk")]
const OFFSET_FUNC_GETNAMES: isize = 0xF08E80;
#[cfg(feature = "client-sdk")]
const OFFSET_FUNC_GETNAMES: isize = 0x10E94B0;

#[cfg(feature = "server-sdk")]
const OFFSET_STRUCT_GOBJECTS: isize = 0x645FEC8;
#[cfg(feature = "client-sdk")]
const OFFSET_STRUCT_GOBJECTS: isize = 0x753EC50;

#[cfg(feature = "server-sdk")]
const OFFSET_FUNC_GET_DISPLAY_NAME: isize = 0xF08E10;
#[cfg(feature = "client-sdk")]
const OFFSET_FUNC_GET_DISPLAY_NAME: isize = 0x10E9440;


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
            game_instance: None,
            game_console: None,
            world: None
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
        let mut game_engine: Option<&UGameEngine> = None;
        let g_objects = self.gobjects();

        for i in 0..(g_objects.num_elements.to_native()-1) {
            let item = g_objects.item_at_idx(i as usize);
            let object = if item.is_some() { item.unwrap().object::<UObject>() } else { None };
            let obj_name = if object.is_some() { Some(object.unwrap().full_name()) } else { None };

            if obj_name == Some("/Engine/Transient.GameEngine".to_string()) {
                game_engine = Some(item.expect("Unable to unwrap Game Engine item").object::<UGameEngine>().expect("Unable to unwrap Game Engine object"));
                let game_instance = game_engine.expect("Failed to unwrap Game Engine").game_instance();
                self.game_instance = game_instance;
                self.world = if game_instance.is_some() { Some(game_instance.unwrap().world_context().world()) } else { None };

            } else if obj_name == Some("/Engine/Transient.GameEngine.GGameViewportClient.Console".to_string()) {
                self.game_console = item.expect("Unable to unwrap Game Console Item").object::<UConsole>();
            }
        }

        if game_engine.is_some() && self.game_console.is_some() {
            true
        } else {
            false
        }
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

    pub fn game_instance(&self) -> &'static UGameInstance { self.game_instance.unwrap() }
    pub fn world(&self) -> &'static UWorld { self.world.unwrap() }
    pub fn console(&self) -> Option<&'static UConsole> { self.game_console }

    pub fn get_display_name_fn(&self) -> Option<fn(*const FName) -> *const FNameEntryHeader> {
        println!("self.addr_get_display_name: {:?}", self.addr_get_display_name);
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