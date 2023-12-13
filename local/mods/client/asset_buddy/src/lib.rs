#![feature(pointer_byte_offsets)]
use std::ffi::c_void;
use game_base::*;
use ue_types::*;
use glob::glob;
use utils::debug;

static MOD_NAME: &'static str = "asset_buddy";

#[no_mangle]
fn mod_main(base_addr: *const c_void) {
    GameBase::initialize(MOD_NAME, base_addr);
    GameConsole::wait_for_loaded();

    // Logs debug message to in-game console
    utils::log::set_print_to_console(Box::new(|msg| {
        let console = GameConsole::get();
        if console.is_some() { console.unwrap().output_text(msg) };
    }));
    injection_utils::hooks::asset_manager::bypass_pak_restriction_check().expect("Unable to load asset buddy!");

    let mut contents_dir = std::env::current_exe().unwrap();
    contents_dir.pop();
    contents_dir.pop();
    contents_dir.pop();
    contents_dir.pop();
    contents_dir.push("Mods");
    contents_dir.push("contents");
    for entry in glob(format!("{}/*", contents_dir.display()).as_str()).unwrap() {
        let path = entry.unwrap();
        debug!("Loading custom content folder: {}", path.display().to_string());

        let path: FString = "../../../Mods/contents/ThickCamille".into();
        AssetManager::get().expect("No AssetManager loaded!").scan_path_for_primary_assets(FPrimaryAssetType{name: "pie".into()}, std::ptr::addr_of!(path), std::ptr::null(), false, false, false);
    }
}