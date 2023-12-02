#![feature(pointer_byte_offsets)]
use core::ffi::c_void;
use retour::static_detour;

static_detour! {
    static CheatGod: extern "C" fn(*const c_void);
}

pub struct InjectionBase {
    pub addr_base: *const c_void,
    pub addr_gobjects: *const c_void,
    pub addr_gnames: *const c_void,
    pub addr_func_get_display_name: *const c_void
}

pub const EMPTY_BASE: InjectionBase = InjectionBase {
    addr_base: std::ptr::null(),
    addr_gobjects: std::ptr::null(),
    addr_gnames: std::ptr::null(),
    addr_func_get_display_name: std::ptr::null()
};

static mut INJECTION_BASE: InjectionBase = EMPTY_BASE;

impl InjectionBase {
    pub fn singleton() -> &'static Self {
        unsafe { &INJECTION_BASE }
    }

    pub fn set_singleton(base: Self) -> &'static Self {
        unsafe {
            INJECTION_BASE = base;
            &INJECTION_BASE
        }
    }

    pub unsafe fn override_god() -> () {
        let singleton = Self::singleton();
        let addr_cheat_god = unsafe { singleton.addr_base.byte_offset(0xb83300) };
        CheatGod.initialize(std::mem::transmute(addr_cheat_god), Self::cheat_god_shiv).unwrap();
        CheatGod.enable().unwrap();
    }

    fn cheat_god_shiv(_manager: *const c_void) {
        // Let's do this instead of incrementing the counter
        println!("Someone tried to cheat by using God!");
    }
}