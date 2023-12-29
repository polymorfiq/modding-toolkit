use retour::static_detour;
use ue_types::*;
use game_base::*;
use utils::debug;
use ue_types::ue_widestring::*;

static_detour! {
    static IsNonPakFilenameAllowed: fn(*const UnknownType, *const FString) -> bool;
    static FPakFileFind: fn(*const UnknownType, *const FString, *const UnknownType) -> u8;
    static PakFileSetupSignedPakReader: fn(*const UnknownType, *const UnknownType, *const UnknownType) -> *const UnknownType;
    static FPakDoSignatureCheck: fn(bool, *const UnknownType, i32);
}

pub fn bypass_pak_sig_check() -> Result<(), Box<dyn std::error::Error>> {
    // unsafe {
    //     let setup_signed_pak_reader: fn(*const UnknownType, *const UnknownType, *const UnknownType) -> *const UnknownType = std::mem::transmute(GameBase::singleton().at_offset(game_base::OFFSETS.asset_funcs.fpakfile_setup_signed_pak_reader));
    //     PakFileSetupSignedPakReader.initialize(setup_signed_pak_reader, bypassed_setup_signed_pak_reader).expect("Could not setup Pak Signature bypass");
    //     PakFileSetupSignedPakReader.enable().expect("Could not enable Pak Signature bypass");
    // }

    // unsafe {
    //     let do_signature_check: fn(bool, *const UnknownType, i32) = std::mem::transmute(GameBase::singleton().at_offset(0x1ff8250));

    //     FPakDoSignatureCheck.initialize(do_signature_check, bypassed_do_signature_check).expect("Could not overload DoSignatureCheck()");
    //     FPakDoSignatureCheck.enable().expect("Could not enable Pak DoSignatureCheck() bypass");
    // }

    debug!("[bypass_pak_restriction_check] Disabled asset Pak Signature checks!");
    Ok(())
}

pub fn bypass_pak_restriction_check() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let is_non_pak_filename_allowed: fn(*const UnknownType, *const FString) -> bool = std::mem::transmute(GameBase::singleton().at_offset(game_base::OFFSETS.asset_funcs.asset_manager_is_non_pak_filename_allowed));
        IsNonPakFilenameAllowed.initialize(is_non_pak_filename_allowed, bypassed_is_non_pak_filename_allowed).expect("Could not setup Pak restriction bypass");
        IsNonPakFilenameAllowed.enable().expect("Could not enable Pak restriction bypass");
    }

    debug!("[bypass_pak_restriction_check] Disabled asset Pak restriction checks!");
    Ok(())
}

pub fn mount_pak_file(filename: &str, pak_order: u32, in_path: &str, load_index: bool) -> Result<(), Box<dyn std::error::Error>> {
    let get_platform_file_manager: fn() -> *const UnknownType = unsafe {
        std::mem::transmute(GameBase::singleton().at_offset(0xfdba60))
    };
    let platform_file_manager = (get_platform_file_manager)();
    println!("Platform File Mgr: {:p}", platform_file_manager);

    let find_platform_file: fn(*const UnknownType, *const U16CStr) -> *const UnknownType = unsafe {
        std::mem::transmute(GameBase::singleton().at_offset(0xfd9270))
    };

    let fpak_platform_file = (find_platform_file)(platform_file_manager, u16cstr!("PakFile"));
    println!("Platform File: {:p}", fpak_platform_file);

    let mount: fn(*const UnknownType, *const U16CStr, u32, *const U16CStr, bool) = unsafe {
        std::mem::transmute(GameBase::singleton().at_offset(game_base::OFFSETS.asset_funcs.fpakplatformfile_mount))
    };

    let filename_w = U16CString::from_str(filename).unwrap();
    let in_path_w = U16CString::from_str(in_path).unwrap();
    (mount)(fpak_platform_file, filename_w.as_ref(), pak_order, in_path_w.as_ref(), load_index);
    Ok(())
}

pub fn register_mount_point(root_path: *const FString, content_path: *const FString) -> Result<(), Box<dyn std::error::Error>> {
    let register_mount_point: fn(*const FString, *const FString) = unsafe { std::mem::transmute(GameBase::singleton().at_offset(game_base::OFFSETS.asset_funcs.fpackagename_register_mount_point)) };

    (register_mount_point)(root_path, content_path);
    Ok(())
}

pub fn force_asset_loads_from_disk() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let fpakfile_find: fn(*const UnknownType, *const FString, *const UnknownType) -> u8 = std::mem::transmute(GameBase::singleton().at_offset(game_base::OFFSETS.asset_funcs.fpakfile_find));
        FPakFileFind.initialize(fpakfile_find, bypassed_fpakfile_find).expect("Could not setup 'Force asset files to be loaded from the disk'");
        FPakFileFind.enable().expect("Could not enable 'Force asset files to be loaded from the disk'");
    }

    Ok(())
}

fn bypassed_setup_signed_pak_reader(_this: *const UnknownType, reader_archive: *const UnknownType, _filename: *const UnknownType) -> *const UnknownType {
    reader_archive
}

fn bypassed_do_signature_check(_was_cancelled: bool, _request: *const UnknownType, _index: i32) {
    ()
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