use retour::static_detour;
use ue_types::*;
use game_base::*;
use utils::debug;

static_detour! {
    static IsNonPakFilenameAllowed: fn(*const UnknownType, *const FString) -> bool;
    static FPakFileFind: fn(*const UnknownType, *const FString, *const UnknownType) -> u8;
}

pub fn bypass_pak_restriction_check() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let is_non_pak_filename_allowed: fn(*const UnknownType, *const FString) -> bool = std::mem::transmute(GameBase::singleton().at_offset(game_base::offsets::OFFSET_FUNC_ASSET_MANAGER_IS_NON_PAK_FILENAME_ALLOWED));
        IsNonPakFilenameAllowed.initialize(is_non_pak_filename_allowed, bypassed_is_non_pak_filename_allowed).expect("Could not setup Pak restriction bypass");
        IsNonPakFilenameAllowed.enable().expect("Could not enable Pak restriction bypass");
    }

    debug!("[bypass_pak_restriction_check] Disabled asset Pak restriction checks!");
    Ok(())
}

pub fn force_asset_loads_from_disk() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let fpakfile_find: fn(*const UnknownType, *const FString, *const UnknownType) -> u8 = std::mem::transmute(GameBase::singleton().at_offset(game_base::offsets::OFFSET_FUNC_FPAKFILE_FIND));
        FPakFileFind.initialize(fpakfile_find, bypassed_fpakfile_find).expect("Could not setup 'Force asset files to be loaded from the disk'");
        FPakFileFind.enable().expect("Could not enable 'Force asset files to be loaded from the disk'");
    }

    Ok(())
}

fn bypassed_is_non_pak_filename_allowed(_this: *const UnknownType, _filename: *const FString) -> bool {
    // debug!("Is Non Pak Filename Allowed?: {:?}", unsafe { (*filename).to_string() });

    true
}

fn bypassed_fpakfile_find(this: *const UnknownType, filename: *const FString, out_entry: *const UnknownType) -> u8 {
    let filename_str = unsafe { (*filename).to_string() };
    let exe_dir = std::env::current_exe();
    let local_file_path = exe_dir.unwrap().join("../").join(filename_str.as_str());

    if filename_str.contains("/Content/") && local_file_path.exists() {
        // println!("Forcing to be loaded from the disk: {}", local_file_path.as_path().file_name().unwrap().to_str().unwrap());
        0x0
    } else {
        FPakFileFind.call(this, filename, out_entry)
    }
}