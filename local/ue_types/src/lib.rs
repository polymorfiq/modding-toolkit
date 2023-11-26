#![feature(pointer_byte_offsets)]
use core::ffi::c_void;

pub mod f_name;
pub use f_name::{FName, FNameEntry, FNameEntryHeader, TNameEntryArray};

pub mod u_object;
pub use u_object::{UObjectBase, FUObjectArray, FChunkedFixedUObjectArray};

pub mod u_class;
pub use u_class::{UClass};

use injection_utils::InjectionBase;

#[derive(Debug, Copy, Clone)]
pub struct GameBase {}

static mut GAME_BASE: GameBase = GameBase::empty();

impl GameBase {
    pub const fn empty() -> Self {
        Self {}
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

        Self{}
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