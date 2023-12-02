#![feature(pointer_byte_offsets)]
use std::ffi::c_void;

// use ue_types::*;
// use game_base::GameBase;
// use utils::debug;

// const OFFSET_FUNC_GETNAMES: isize = 0x10E94B0;
// const OFFSET_STRUCT_GOBJECTS: isize = 0x753EC50;
// const OFFSET_FUNC_GET_DISPLAY_NAME: isize = 0x10E9440;

#[no_mangle]
fn mod_main(base_addr: *const c_void) {
    let game_base = GameBase::initialize(base_addr);

    utils::log::set_console(game_base.console());
    debug!("Injected - Game Base: {:p}", GameBase::singleton());

    let console_cmd_recv = injection_utils::hooks::console::initialize_shivs();

    loop {
        match console_cmd_recv.recv() {
            Ok(cmd) => debug!("Received fallthrough command: {:?}", cmd),
            _ => {
                debug!("Stopped watching for fallthrough commands...");
                break
            }
        }
        
    }
}