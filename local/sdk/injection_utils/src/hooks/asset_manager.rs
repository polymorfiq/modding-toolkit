use retour::static_detour;
use ue_types::*;
use game_base::*;
use utils::debug;

static_detour! {
    static IsNonPakFilenameAllowed: fn(*const UnknownType, *const FString) -> bool;
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

fn bypassed_is_non_pak_filename_allowed(_this: *const UnknownType, _filename: *const FString) -> bool {
    true
}