use retour::static_detour;
use ue_types::*;
use game_base::*;
use utils::debug;
use simple_endian::*;
use std::io::Write;
use std::fs::OpenOptions;

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
}

static mut WRITTEN_LINES: usize = 0;
static mut LOG_FILE: Option<std::fs::File> = None;

pub fn intercept() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        LOG_FILE = Some(OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("C:\\Users\\cory\\Documents\\Spellbreak\\assets.log")
        .unwrap());

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

fn my_add_asset_data(this: *const UnknownType, asset_data: *const FAssetData) {
    let package_path = unsafe { (*asset_data).package_path.to_string() };
    if !package_path.contains("Human") {
        AddAssetData.call(this, asset_data);
    } else {
        // unsafe {
        //     writeln!(LOG_FILE.as_ref().unwrap(), "Asset Data Added - {:?}", AssetData::new(asset_data)).unwrap();

        //     WRITTEN_LINES += 1;
        //     if WRITTEN_LINES % 100 == 0 {
        //         LOG_FILE.as_ref().unwrap().sync_data().unwrap();
        //     }
        // }
    }
}

fn my_u_add_asset_data(this: *const UnknownType, asset_data: *const FAssetData) {
    // UAddAssetData.call(this, asset_data);
}

fn my_load_primary_asset(this: *const UnknownType, result: *mut UnknownType, asset_to_load: *const FPrimaryAssetId, load_bundle: TArray<FName, FDefaultAllocator>, delegate_to_call: FDelegateBase, priority: u32) -> *const UnknownType {
    let asset_name = unsafe { (*asset_to_load).primary_asset_name.to_string() };
    debug!("Primary Asset Loaded: {:?}", asset_name);
    unsafe {
        writeln!(LOG_FILE.as_ref().unwrap(), "Primary Asset Loaded - {:?}", asset_name).unwrap();

        WRITTEN_LINES += 1;
        if WRITTEN_LINES % 100 == 0 {
            LOG_FILE.as_ref().unwrap().sync_data().unwrap();
        }
    }
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
    let data = AssetData::new(asset_data);
    let package_data = data.asset_data().package_data.to_string();

    if package_data.contains("Marla") || package_data.contains("Blade") {
        println!("Set From Asset Data: {:?}", data);
        false
    } else {
        SetFromAssetData.call(this, asset_data)
    }
}