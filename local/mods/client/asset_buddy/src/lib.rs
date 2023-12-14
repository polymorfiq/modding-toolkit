#![feature(pointer_byte_offsets)]
use std::ffi::c_void;
use game_base::*;
use ue_types::*;
use glob::glob;
use utils::debug;

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

        ObjectLibrary::get().unwrap().load_assets_from_path(path.display().to_string());

        
        let f_path: FString = path.display().to_string().into();
        let primary_asset_name: GameName = "RawSkin".into();
        let asset_manager = AssetManager::get().expect("No AssetManager loaded!");

        let package_class = GObjects::find_first(|obj| {
            obj.full_name() == "/Script/CoreUObject.Package"
        });

        asset_manager.scan_path_for_primary_assets(
            FPrimaryAssetType{name: primary_asset_name.f_name()},
            std::ptr::addr_of!(f_path),
            package_class.unwrap() as *const UClass,
            false,
            false,
            false
        );
    }
}