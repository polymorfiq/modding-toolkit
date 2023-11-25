use core::ffi::c_void;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::io::{Error, ErrorKind, Read, Write};
use retour::static_detour;
use winapi::um::libloaderapi::GetModuleHandleA;

pub mod ue_types;
use ue_types::*;

pub static mut ADDR_BASE: *const u8 = std::ptr::null(); // Probably going to be 0x7FF674A80000
pub static mut ADDR_GNAMES: *const TNameEntryArray = std::ptr::null();

const OFFSET_FUNC_GETNAMES: usize = 0xf08e80;
const OFFSET_LIST_PLAYERS: usize = 0x3084F20;
const OFFSET_STRUCT_GOBJECTS: usize = 0x645FEC8;
const OFFSET_FUNC_GET_GAME_STATE: usize = 0xB05A20;

pub fn init() {
    unsafe {
        ADDR_BASE = GetModuleHandleA(std::ptr::null()) as *const u8;

        let get_names: fn() -> *const TNameEntryArray = std::mem::transmute(offset_addr(OFFSET_FUNC_GETNAMES));
        let gname_addr = (get_names)();
        ADDR_GNAMES = (get_names)();
    }
}

pub fn get_base_addr() -> *const c_void {
    unsafe { ADDR_BASE as *const c_void }
}

pub fn get_gnames_addr<'a>() -> *const TNameEntryArray {
    unsafe { ADDR_GNAMES as *const TNameEntryArray }
}

pub fn get_gobjects() -> FChunkedFixedUObjectArray {
    let ptr = offset_addr(OFFSET_STRUCT_GOBJECTS) as *const FUObjectArray;
    let g_obj_ptr: &FUObjectArray = unsafe { std::mem::transmute(ptr) };
    (*g_obj_ptr).objects_array
}

pub fn get_gnames() -> TNameEntryArray {
    let g_name_ptr: &TNameEntryArray = unsafe { std::mem::transmute(ADDR_GNAMES as *const TNameEntryArray) };
    *g_name_ptr
}

fn offset_addr(offset: usize) -> *const u8 {
    unsafe { ADDR_BASE.offset(offset as isize) }
}

impl FName {
    pub fn entry(self) -> Option<FNameEntry> {
        let gnames = get_gnames();
        gnames.entry_for_fname(self)
    }
}