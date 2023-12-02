#![feature(pointer_byte_offsets)]
use core::ffi::c_void;

pub mod base_types;
pub use base_types::*;

use injection_utils::InjectionBase;

#[derive(Debug, Copy, Clone)]
pub struct GameBase {
    game_instance: Option<&'static UGameInstance<'static>>,
    world: Option<&'static UWorld<'static>>,
    game_console: Option<&'static UConsole>
}

static mut GAME_BASE: GameBase = GameBase::empty();

impl GameBase {
    pub const fn empty() -> Self {
        Self {
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
        let mut game_engine: Option<&UGameEngine> = None;
        let g_objects = GameBase::singleton().gobjects();
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

        println!("Found Game Engine at {:?}", game_engine);
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

    pub fn game_instance(&self) -> &'static UGameInstance { self.game_instance.unwrap() }
    pub fn world(&self) -> &'static UWorld { self.world.unwrap() }
    pub fn console(&self) -> Option<&'static UConsole> { self.game_console }

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

    pub fn get_offset_from_addr<T>(&self, addr: *const T) -> usize {
        (addr as usize) - (InjectionBase::singleton().addr_base as usize)
    }

    pub fn get_offset<T>(&self, addr: &T) -> usize {
        (std::ptr::addr_of!(*addr) as usize) - (InjectionBase::singleton().addr_base as usize)
    }
}
#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn correct_data_sizes() {
        assert_eq!(std::mem::size_of::<TEnumAsByte<UnknownType>>(), 0x1);
        assert_eq!(std::mem::size_of::<TWeakPtr<UnknownType>>(), 0x10);
        assert_eq!(std::mem::size_of::<TWeakPtr<FOutputDevice>>(), 0x10);
        assert_eq!(std::mem::size_of::<FExec>(), 0x8);
        assert_eq!(std::mem::size_of::<FAutoCompleteNode>(), 0x28);
        assert_eq!(std::mem::size_of::<FGuid>(), 0x10);
        assert_eq!(std::mem::size_of::<AActor>(), 0x348);
        assert_eq!(std::mem::size_of::<UObject>(), 0x30);
        assert_eq!(std::mem::size_of::<UField>(), 0x38);
        assert_eq!(std::mem::size_of::<TArray<UObject>>(), 0x10);
        assert_eq!(std::mem::size_of::<FString>(), 0x10);
        assert_eq!(std::mem::size_of::<FURL>(), 0x68);
        assert_eq!(std::mem::size_of::<ULevel>(), 0x288);
        assert_eq!(std::mem::size_of::<UWorld>(), 0x730);
        assert_eq!(std::mem::size_of::<FSeamlessTravelHandler>(), 0xA8);
        assert_eq!(std::mem::size_of::<FWorldContext>(), 0x278);
        assert_eq!(std::mem::size_of::<UEngine>(), 0xEC8);
        assert_eq!(std::mem::size_of::<UGameEngine>(), 0xF18);
        assert_eq!(std::mem::size_of::<UConsole>(), 0x140);
    }
}