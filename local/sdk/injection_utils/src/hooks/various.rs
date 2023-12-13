use retour::static_detour;
use ue_types::*;
use game_base::*;
use utils::debug;
use simple_endian::*;
// use std::io::Write;
// use std::fs::OpenOptions;
use ue_types::ue_widestring::U16CStr;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
struct StringArg {
    pub len: u32le,
    _padding: [u8; 0x4],
    pub s: *const u8
}

impl<'a> std::string::ToString for StringArg {
    fn to_string (&self) -> String {
        let mut bytes: Vec<u8> = vec![];
        for i in 0..self.len.into() {
            unsafe { bytes.push(*self.s.byte_offset(i as isize)) };
        }

        String::from_utf8(bytes).expect("Found invalid UTF-8")
    }
}


static_detour! {
    static SetGameMode: fn(*const UGameInstance, StringArg, StringArg);
    static SetMatchState: fn(*const AGameMode, *const FName);
    static RestartPlayer: fn(*const AGameModeBase, *const APlayerController);
    static AddAssetData: fn(*const UnknownType, *const FAssetData);
    static UAddAssetData: fn(*const UnknownType, *const FAssetData);
    static LoadPrimaryAsset: fn(*const UnknownType, *mut UnknownType, *const FPrimaryAssetId, TArray<FName, FDefaultAllocator>, FDelegateBase, u32) -> *const UnknownType;
    static LoadPrimaryAssets: fn(*const UnknownType, *mut UnknownType, TArray<FPrimaryAssetId, FDefaultAllocator>, TArray<FName, FDefaultAllocator>, FDelegateBase, u32) -> *const UnknownType;
    static LoadPrimaryAssetsWithType: fn(*const UnknownType, *mut UnknownType, FPrimaryAssetType, TArray<FName, FDefaultAllocator>, FDelegateBase, u32) -> *const UnknownType;
    static LoadAssetsFromPath: fn(*const UnknownType, *const FString) -> u32;
    static LoadAssetsFromPaths: fn(*const UnknownType, *const TArray<FString, FDefaultAllocator>) -> u32;
    static SetFromAssetData: fn(*const UnknownType, *const FAssetData) -> bool;
    static UpdateCachedAssetData: fn(*const UnknownType, *const FPrimaryAssetId, *const FAssetData, bool);
    static AddObject: fn(*const UnknownType, *const UObject<*const UnknownType>) -> bool;
    static OpenRead: fn(*const UnknownType, *const u16, bool) -> *const UnknownType;
    static IsNonPakFilenameAllowed: fn(*const UnknownType, *const FString) -> bool;
    static LoadAssetsFromAssetData: fn(*const UnknownType) -> u64le;
}

// static mut WRITTEN_LINES: usize = 0;
// static mut LOG_FILE: Option<std::fs::File> = None;

pub fn intercept() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // LOG_FILE = Some(OpenOptions::new()
        // .create(true)
        // .write(true)
        // .append(true)
        // .open("C:\\Users\\cory\\Documents\\Spellbreak\\assets.log")
        // .unwrap());

        // let set_game_mode: fn(*const UGameInstance, StringArg, StringArg) = std::mem::transmute(GameBase::singleton().at_offset(0x39910C0));
        // SetGameMode.initialize(set_game_mode, my_set_game_mode)?;
        // SetGameMode.enable()?;

        // let add_asset_data: fn(*const UnknownType, *const FAssetData) = std::mem::transmute(GameBase::singleton().at_offset(game_base::offsets::OFFSET_FUNC_ASSET_REGISTRY_ADD_ASSET_DATA));
        // println!("Add Asset Data - {:p}", GameBase::singleton().at_offset(game_base::offsets::OFFSET_FUNC_ASSET_REGISTRY_ADD_ASSET_DATA));
        // AddAssetData.initialize(add_asset_data, my_add_asset_data).expect("BOOM1");
        // AddAssetData.enable().expect("BOOM");

        let load_primary_asset: fn(*const UnknownType, *mut UnknownType, *const FPrimaryAssetId, TArray<FName, FDefaultAllocator>, FDelegateBase, u32) -> *const UnknownType = std::mem::transmute(GameBase::singleton().at_offset(0x2139EE0));
        println!("Load Primary Asset - {:p}", GameBase::singleton().at_offset(0x2139EE0));
        LoadPrimaryAsset.initialize(load_primary_asset, my_load_primary_asset).expect("BOOM1");
        LoadPrimaryAsset.enable().expect("BOOM");

        let load_primary_assets: fn(*const UnknownType, *mut UnknownType, TArray<FPrimaryAssetId, FDefaultAllocator>, TArray<FName, FDefaultAllocator>, FDelegateBase, u32) -> *const UnknownType = std::mem::transmute(GameBase::singleton().at_offset(0x213A000));
        println!("Load Primary Asset - {:p}", GameBase::singleton().at_offset(0x213A000));
        LoadPrimaryAssets.initialize(load_primary_assets, my_load_primary_assets).expect("BOOM1");
        LoadPrimaryAssets.enable().expect("BOOM");

        let load_primary_assets_with_type: fn(*const UnknownType, *mut UnknownType, FPrimaryAssetType, TArray<FName, FDefaultAllocator>, FDelegateBase, u32) -> *const UnknownType = std::mem::transmute(GameBase::singleton().at_offset(0x213A0F0));
        println!("Load Primary Asset - {:p}", GameBase::singleton().at_offset(0x213A0F0));
        LoadPrimaryAssetsWithType.initialize(load_primary_assets_with_type, my_load_primary_assets_with_type).expect("BOOM1");
        LoadPrimaryAssetsWithType.enable().expect("BOOM");

        let load_assets_from_path: fn(*const UnknownType, *const FString) -> u32 = std::mem::transmute(GameBase::singleton().at_offset(0x2479280));
        println!("Load Assets With Path - {:p}", GameBase::singleton().at_offset(0x2479280));
        LoadAssetsFromPath.initialize(load_assets_from_path, my_load_assets_from_path).expect("BOOM1");
        LoadAssetsFromPath.enable().expect("BOOM");

        let load_assets_from_paths: fn(*const UnknownType, *const TArray<FString, FDefaultAllocator>) -> u32 = std::mem::transmute(GameBase::singleton().at_offset(0x2479330));
        println!("Load Assets With Paths - {:p}", GameBase::singleton().at_offset(0x2479330));
        LoadAssetsFromPaths.initialize(load_assets_from_paths, my_load_assets_from_paths).expect("BOOM1");
        LoadAssetsFromPaths.enable().expect("BOOM");

        let set_from_asset_data: fn(*const UnknownType, *const FAssetData) -> bool = std::mem::transmute(GameBase::singleton().at_offset(0x15B6240));
        println!("Set From Asset Data - {:p}", GameBase::singleton().at_offset(0x15B6240));
        SetFromAssetData.initialize(set_from_asset_data, my_set_from_asset_data).expect("BOOM1");
        SetFromAssetData.enable().expect("BOOM");

        let update_cached_asset_data: fn(*const UnknownType, *const FPrimaryAssetId, *const FAssetData, bool) = std::mem::transmute(GameBase::singleton().at_offset(0x2145030));
        println!("Update Cached Asset Data - {:p}", GameBase::singleton().at_offset(0x2145030));
        UpdateCachedAssetData.initialize(update_cached_asset_data, my_update_cached_asset_data).expect("BOOM1");
        UpdateCachedAssetData.enable().expect("BOOM");

        let add_object: fn(*const UnknownType, *const UObject<*const UnknownType>) -> bool = std::mem::transmute(GameBase::singleton().at_offset(0x2468830));
        println!("Add object - {:p}", GameBase::singleton().at_offset(0x2468830));
        AddObject.initialize(add_object, my_add_object).expect("BOOM1");
        AddObject.enable().expect("BOOM");

        let open_read: fn(*const UnknownType, *const u16, bool) -> *const UnknownType = std::mem::transmute(GameBase::singleton().at_offset(0x2001870));
        OpenRead.initialize(open_read, my_open_read).expect("BOOM1");
        OpenRead.enable().expect("BOOM");

        let load_assets_from_asset_data: fn(this: *const UnknownType) -> u64le = std::mem::transmute(GameBase::singleton().at_offset(0x2478C30));
        LoadAssetsFromAssetData.initialize(load_assets_from_asset_data, my_load_assets_from_asset_data).expect("BOOM1");
        LoadAssetsFromAssetData.enable().expect("BOOM");

        

        // let load_asset_datas_from_paths: fn(*const UnknownType, *const TArray<FString, FDefaultAllocator>, bool) = std::mem::transmute(GameBase::singleton().at_offset(0x24789F0));
        // println!("Load Asset Datas From Paths - {:p}", GameBase::singleton().at_offset(0x24789F0));
        // GObjects::filter(|obj| {
        //     if obj.full_name().contains("UObjectLibrary") {
        //         println!("UOBJECTLIBRARY: {:?} - {:?}", obj.full_name(), obj.class().expect("EXPECTED CLASS").object().full_name());
        //     }

        //     false
        // });


        // let u_add_asset_data: fn(*const UnknownType, *const FAssetData) = std::mem::transmute(GameBase::singleton().at_offset(0x1599000));
        // println!("Add Asset Data - {:p}", GameBase::singleton().at_offset(0x1599000));
        // UAddAssetData.initialize(u_add_asset_data, my_u_add_asset_data).expect("BOOM1");
        // UAddAssetData.enable().expect("BOOM");

        // let set_match_state: fn(*const AGameMode, *const FName) = std::mem::transmute(GameBase::singleton().at_offset(0x39910C0));
        // SetMatchState.initialize(set_match_state, my_set_match_state)?;
        // SetMatchState.enable()?;

        // let restart_player: fn(*const AGameModeBase, *const APlayerController) = std::mem::transmute(GameBase::singleton().at_offset(0x233CE50));
        // RestartPlayer.initialize(restart_player, my_restart_player)?;
        // RestartPlayer.enable()?;
    }

    Ok(())
}

// fn my_add_asset_data(this: *const UnknownType, asset_data: *const FAssetData) {
//     let package_path = unsafe { (*asset_data).package_path.to_string() };
//     if !package_path.contains("Human") {
//         AddAssetData.call(this, asset_data);
//     } else {
//         // unsafe {
//         //     writeln!(LOG_FILE.as_ref().unwrap(), "Asset Data Added - {:?}", AssetData::new(asset_data)).unwrap();

//         //     WRITTEN_LINES += 1;
//         //     if WRITTEN_LINES % 100 == 0 {
//         //         LOG_FILE.as_ref().unwrap().sync_data().unwrap();
//         //     }
//         // }
//     }
// }

// fn my_u_add_asset_data(this: *const UnknownType, asset_data: *const FAssetData) {
//     // UAddAssetData.call(this, asset_data);
// }

fn my_load_primary_asset(this: *const UnknownType, result: *mut UnknownType, asset_to_load: *const FPrimaryAssetId, load_bundle: TArray<FName, FDefaultAllocator>, delegate_to_call: FDelegateBase, priority: u32) -> *const UnknownType {
    let asset_name = unsafe { (*asset_to_load).primary_asset_name.to_string() };
    debug!("Primary Asset Loaded: {:?}", asset_name);
    // unsafe {
    //     writeln!(LOG_FILE.as_ref().unwrap(), "Primary Asset Loaded - {:?}", asset_name).unwrap();

    //     WRITTEN_LINES += 1;
    //     if WRITTEN_LINES % 100 == 0 {
    //         LOG_FILE.as_ref().unwrap().sync_data().unwrap();
    //     }
    // }
    LoadPrimaryAsset.call(this, result, asset_to_load, load_bundle, delegate_to_call, priority)
}

fn my_load_primary_assets(this: *const UnknownType, result: *mut UnknownType, assets_to_load: TArray<FPrimaryAssetId, FDefaultAllocator>, load_bundle: TArray<FName, FDefaultAllocator>, delegate_to_call: FDelegateBase, priority: u32) -> *const UnknownType {
    for i in 0..assets_to_load.len() {
        let asset_id = assets_to_load.at_index(i).unwrap();
        println!("Assets ID loading: {:?}", asset_id.primary_asset_name.to_string());
    }

    LoadPrimaryAssets.call(this, result, assets_to_load, load_bundle, delegate_to_call, priority)
}

fn my_load_primary_assets_with_type(this: *const UnknownType, result: *mut UnknownType, asset_type: FPrimaryAssetType, load_bundle: TArray<FName, FDefaultAllocator>, delegate_to_call: FDelegateBase, priority: u32) -> *const UnknownType {
    println!("Asset type loading: {:?}", asset_type.name.to_string());

    LoadPrimaryAssetsWithType.call(this, result, asset_type, load_bundle, delegate_to_call, priority)
}

fn my_load_assets_from_asset_data(this: *const UnknownType) -> u64le {
    println!("Load Assets From Asset Data: {:p}", this);

    LoadAssetsFromAssetData.call(this)
}

fn my_load_assets_from_path(this: *const UnknownType, path: *const FString) -> u32 {
    println!("Load Assets From Path: {:?}", unsafe { (*path).to_string() });

    LoadAssetsFromPath.call(this, path)
}

fn my_load_assets_from_paths(this: *const UnknownType, paths: *const TArray<FString, FDefaultAllocator>) -> u32 {
    let path_array = unsafe { *paths };

    for i in 0..path_array.len() {
        let path = path_array.at_index(i).unwrap();
        println!("Assets Path loading: {:?}", path.to_string());
    }

    LoadAssetsFromPaths.call(this, paths)
}

fn my_set_from_asset_data(this: *const UnknownType, asset_data: *const FAssetData) -> bool {
    // let data = AssetData::new(asset_data);
    // let package_data = data.asset_data().package_data.to_string();

    // if package_data.contains("Marla") || package_data.contains("Blade") {
    //     // println!("Set From Asset Data: {:?}", data);
    //     false
    // } else {
        SetFromAssetData.call(this, asset_data)
    // }
}

fn my_update_cached_asset_data(this: *const UnknownType, primary_asset_id: *const FPrimaryAssetId, asset_data: *const FAssetData, b_allow_duplicates: bool) {
    // let asset_id = unsafe { *primary_asset_id };
    // let data = AssetData::new(asset_data);
    // let package_data = data.asset_data().package_data.to_string();

    // println!("Updating cached Asset data: {:p}", this);

    UpdateCachedAssetData.call(this, primary_asset_id, asset_data, b_allow_duplicates);
}

fn my_add_object(this: *const UnknownType, object: *const UObject<*const UnknownType>) -> bool {
    println!("ADD OBJECT: {:p}", this);

    AddObject.call(this, object)
}

fn my_open_read(this: *const UnknownType, filename: *const u16, allow_write: bool) -> *const UnknownType {
    println!("OpenRead: {}", unsafe { U16CStr::from_ptr_str(filename).to_string().unwrap() });

    OpenRead.call(this, filename, allow_write)
}