#![feature(pointer_byte_offsets)]
use std::ffi::c_void;
use ue_types::*;

static mut GAME_BASE: GameBase = GameBase::empty();

#[derive(Debug, Copy, Clone)]
pub struct GameBase {
    addr_base: Option<*const c_void>,
    addr_gobjects: Option<*const c_void>,
    addr_gnames: Option<*const c_void>,
    addr_get_display_name: Option<*const c_void>,
    game_instance: Option<&'static UGameInstance<'static>>,
    world: Option<&'static UWorld<'static>>,
    game_console: Option<&'static UConsole>
}


impl Default for GameBase {
    fn default() -> Self { Self::empty() }
}

impl GameBase {
    pub const fn empty() -> Self {
        Self {
            addr_base: None,
            addr_gobjects: None,
            addr_gnames: None,
            addr_get_display_name: None,
            game_instance: None,
            game_console: None,
            world: None
        }
    }

    pub fn generate(
        base_addr: *const c_void,
        offset_struct_gobjects: isize,
        offset_func_getnames: isize,
        offset_func_get_display_name: isize,
    ) -> &'static Self {
        let addr_gobjects = unsafe { base_addr.byte_offset(offset_struct_gobjects) };
        
        let get_names: fn() -> *const c_void = unsafe { 
            std::mem::transmute(base_addr.byte_offset(offset_func_getnames))
        };
        let addr_gnames = (get_names)();
        let addr_get_display_name = unsafe { base_addr.byte_offset(offset_func_get_display_name) };

        let mut generated = Self {
            addr_base: Some(base_addr),
            addr_gobjects: Some(addr_gobjects),
            addr_gnames: Some(addr_gnames),
            addr_get_display_name: Some(addr_get_display_name),
            ..Default::default()
        };

        ue_types::funcs::set_get_display_name(generated.get_display_name_fn());
        generated.initialize();
        Self::set_singleton(generated);

        unsafe { &GAME_BASE }
    }

    pub fn initialize(&mut self) {
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

        println!("Found Game Engine at {:p}", if game_engine.is_some() { game_engine.unwrap() } else { std::ptr::null() });
        println!("Found Game Console at {:p}", if self.game_console.is_some() { self.game_console.unwrap() } else { std::ptr::null() });
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