#![feature(pointer_byte_offsets)]
use core::ffi::c_void;
use std::thread;
use winapi::um::libloaderapi::GetModuleHandleA;

use ue_types::*;
use game_base::GameBase;
use utils::debug;

const OFFSET_FUNC_GETNAMES: isize = 0x10E94B0;
const OFFSET_STRUCT_GOBJECTS: isize = 0x753EC50;
const OFFSET_FUNC_GET_DISPLAY_NAME: isize = 0x10E9440;

#[ctor::ctor]
fn ctor() {
    let base_addr = unsafe { GetModuleHandleA(std::ptr::null()) as *const c_void };
    let game_base = GameBase::generate(
        base_addr,
        OFFSET_STRUCT_GOBJECTS,
        OFFSET_FUNC_GETNAMES,
        OFFSET_FUNC_GET_DISPLAY_NAME
    );

    utils::log::set_console(game_base.console());
    
    debug!("Injected - Game Base: {:p}", GameBase::singleton());

    injection_utils::hooks::console::initialize_shivs();

    thread::spawn(|| {
        listen_for_commands();
    });

    debug!("Command listener started!!");
}

pub fn listen_for_commands() {
    // let game_base = GameBase::singleton();

    // loop {
    //     match game_base.next_console_command() {
            
    //     }
    // }
}

pub fn do_a_thing() {
    let game_base = GameBase::singleton();
    let g_objects = game_base.gobjects();

    for i in 0..(g_objects.num_elements.to_native()-1) {
        let item = g_objects.item_at_idx(i as usize);
        let object = if item.is_some() { item.unwrap().object::<UObject>() } else { None };

        if object.is_some() &&  game_base.get_offset_from_addr(object.unwrap().virtual_funcs()) == 0x5ab1450 {
            let obj: &UObject = object.unwrap();
            debug!("GOBJECTS[{:?}]: {:?} ({:?})", i, obj.full_name(), obj.class().full_name());
        }
    }
}