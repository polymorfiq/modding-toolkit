#![feature(pointer_byte_offsets)]
use core::ffi::c_void;

pub mod base_types;
pub use base_types::*;

use injection_utils::InjectionBase;

#[derive(Debug, Copy, Clone)]
pub struct GameBase {
    addr_game_instance: *const UGameInstance<'static>,
}

static mut GAME_BASE: GameBase = GameBase::empty();
const GAME_INSTANCE_GOBJECTS_IDX: isize = 79;

impl GameBase {
    pub const fn empty() -> Self {
        Self {
            addr_game_instance: std::ptr::null()
        }
    }

    pub fn generate(
        base_addr: *const c_void,
        offset_struct_gobjects: isize,
        offset_func_getnames: isize,
        offset_func_get_display_name: isize,
    ) -> Self {
        let addr_gobjects = unsafe { base_addr.byte_offset(offset_struct_gobjects) };
        
        let get_names: fn() -> *const c_void = unsafe { 
            std::mem::transmute(base_addr.byte_offset(offset_func_getnames))
        };
        let addr_gnames = (get_names)();
        let addr_get_display_name = unsafe { base_addr.byte_offset(offset_func_get_display_name) };
        
        InjectionBase::set_singleton(InjectionBase{
            addr_base: base_addr,
            addr_gobjects: addr_gobjects,
            addr_gnames: addr_gnames,
            addr_func_get_display_name: addr_get_display_name
        });

        let mut generated = Self::empty();
        generated.initialize();

        generated
    }

    pub fn initialize(&mut self) {
        let game_instance_item = self.gobjects().item_at_idx(GAME_INSTANCE_GOBJECTS_IDX as usize).expect("Failed to find GameInstance");
        let game_instance_obj = game_instance_item.object().expect("Unable to unwrap GameInstance object");
        if game_instance_obj.name().entry().unwrap().value().to_str() != Ok("GameInstance") {
            panic!("Picked wrong GObject out of GObjects - expected GameInstance!");
        }

        self.addr_game_instance = game_instance_item.object_addr as *const UGameInstance;
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
        let ptr = InjectionBase::singleton().addr_gobjects as *const FUObjectArray;
        unsafe { (*ptr).objects_array }
    }

    pub fn gnames(&self) -> TNameEntryArray {
        let ptr = InjectionBase::singleton().addr_gnames as *const TNameEntryArray;
        unsafe { *ptr }
    }

    pub fn game_instance(&self) -> &'static UGameInstance {
        unsafe { self.addr_game_instance.as_ref::<'static>().unwrap() }
    }

    pub fn get_display_name(&self, f_name: &FName) -> *const FNameEntryHeader {
        let get_display_name: fn(*const FName) -> *const FNameEntryHeader = unsafe {
            std::mem::transmute(InjectionBase::singleton().addr_func_get_display_name)
        };

        (get_display_name)(f_name)
    }

    pub fn at_offset(&self, offset: isize) -> *const c_void {
        let ptr = unsafe { InjectionBase::singleton().addr_base.byte_offset(offset) };
        ptr
    }
}