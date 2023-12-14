#![feature(pointer_byte_offsets)]
use std::ffi::c_void;
use game_base::*;

static MOD_NAME: &'static str = "asset_buddy";

#[no_mangle]
fn mod_main_sync(base_addr: *const c_void) {
    GameBase::initialize(MOD_NAME, base_addr);
    injection_utils::hooks::asset_manager::bypass_pak_restriction_check().expect("Unable to load asset buddy (bypass pak restriction)!");
    injection_utils::hooks::asset_manager::force_asset_loads_from_disk().expect("Unable to load asset buddy (force load from disk)!");
}

#[no_mangle]
fn mod_main(base_addr: *const c_void) {
    GameBase::initialize(MOD_NAME, base_addr);
    GameConsole::wait_for_loaded();

    // Logs debug message to in-game console
    utils::log::set_print_to_console(Box::new(|msg| {
        let console = GameConsole::get();
        if console.is_some() { console.unwrap().output_text(msg) };
    }));
}