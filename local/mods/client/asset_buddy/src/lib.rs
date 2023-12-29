use std::ffi::c_void;
use game_base::*;
use injection_utils::hooks::asset_manager;
use memory_management::strings::fstring;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use glob::glob;
use utils::warning;

static MOD_NAME: &'static str = "asset_buddy";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub mounts: BTreeMap<String, String>
}

#[no_mangle]
fn mod_main_sync(base_addr: *const c_void) {
    unsafe { game_base::OFFSETS = game_base::offsets_client::get_offsets() };

    GameBase::initialize(MOD_NAME, base_addr);
    asset_manager::bypass_pak_restriction_check().expect("Unable to load asset buddy (bypass pak restriction)!");
    asset_manager::force_asset_loads_from_disk().expect("Unable to load asset buddy (force load from disk)!");
    asset_manager::bypass_pak_sig_check().expect("Unable to load asset buddy (bypass pak signatue check)!");
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

    let mut dir = std::env::current_exe().unwrap();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.pop();
    let mods_dir = dir.join("Mods").join("contents");
    println!("MODS DIR {}", mods_dir.display().to_string());

    let pak_glob = mods_dir.clone().join("**").join("*.pak");
    for entry in glob(pak_glob.display().to_string().as_str()).unwrap() {
        let path = entry.unwrap();
        println!("Mounting Pak File: {}", path.display().to_string());
        let _ = asset_manager::mount_pak_file(path.display().to_string().as_str(), 0, "/Game/", true).unwrap();
        println!("Mounted Pak File: {}", path.display().to_string());
    }

    let manifest_path = mods_dir.clone().join("manifest.yaml");
    if manifest_path.is_file() {
        let f = std::fs::File::open(manifest_path).expect("Could not open manifest file");
        let manifest: Result<Manifest, serde_yaml::Error> = serde_yaml::from_reader(f);
        if manifest.is_ok() {
            let manifest = manifest.unwrap();

            for (mount_dir, game_path) in manifest.mounts {
                let mount_dir = if mount_dir.starts_with("/") { mount_dir } else {
                    mods_dir.clone().join(mount_dir).display().to_string()
                };

                println!("{} -> {}", game_path, mount_dir);

                let _ = asset_manager::register_mount_point(fstring(mount_dir), fstring(game_path)).expect("Could not load paks from Mods/contents folder");
            }
        } else {
            warning!("Could not read manifest file: {:?}", manifest);
        }
    }
}