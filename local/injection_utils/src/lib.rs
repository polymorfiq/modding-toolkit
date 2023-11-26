#![feature(pointer_byte_offsets)]
use core::ffi::c_void;

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
}