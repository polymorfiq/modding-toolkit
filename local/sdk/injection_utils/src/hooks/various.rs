use retour::static_detour;
use ue_types::*;
use game_base::*;
use utils::debug;
use simple_endian::*;
use std::fs::OpenOptions;
use std::io::prelude::*;

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

        let set_game_mode: fn(*const UGameInstance, StringArg, StringArg) = std::mem::transmute(GameBase::singleton().at_offset(0x39910C0));
        SetGameMode.initialize(set_game_mode, my_set_game_mode)?;
        SetGameMode.enable()?;

        let add_asset_data: fn(*const UnknownType, *const FAssetData) = std::mem::transmute(GameBase::singleton().at_offset(game_base::offsets::OFFSET_FUNC_ASSET_REGISTRY_ADD_ASSET_DATA));
        println!("Add Asset Data - {:p}", GameBase::singleton().at_offset(game_base::offsets::OFFSET_FUNC_ASSET_REGISTRY_ADD_ASSET_DATA));
        AddAssetData.initialize(add_asset_data, my_add_asset_data).expect("BOOM1");
        AddAssetData.enable().expect("BOOM");

        let u_add_asset_data: fn(*const UnknownType, *const FAssetData) = std::mem::transmute(GameBase::singleton().at_offset(0x1599000));
        println!("Add Asset Data - {:p}", GameBase::singleton().at_offset(0x1599000));
        UAddAssetData.initialize(u_add_asset_data, my_u_add_asset_data).expect("BOOM1");
        UAddAssetData.enable().expect("BOOM");

        let set_match_state: fn(*const AGameMode, *const FName) = std::mem::transmute(GameBase::singleton().at_offset(0x39910C0));
        SetMatchState.initialize(set_match_state, my_set_match_state)?;
        SetMatchState.enable()?;

        let restart_player: fn(*const AGameModeBase, *const APlayerController) = std::mem::transmute(GameBase::singleton().at_offset(0x233CE50));
        RestartPlayer.initialize(restart_player, my_restart_player)?;
        RestartPlayer.enable()?;
    }

    Ok(())
}

fn my_set_match_state(this: *const AGameMode, next_state: *const FName) {
    debug!("SET_MATCH_STATE - {:p} - {:?}", this, unsafe { (*next_state).to_string() });
    SetMatchState.call(this, next_state);
}

fn my_set_game_mode(this: *const UGameInstance, mode: StringArg, variant: StringArg) {
    debug!("SET_GAME_MODE - {:p} - {:?} - {:?}", this, mode.to_string(), variant.to_string());
    SetGameMode.call(this, mode, variant);
}

fn my_restart_player(this: *const AGameModeBase, player: *const APlayerController) {
    debug!("RESTART_PLAYER called - {:p}", this);
    RestartPlayer.call(this, player);
}

fn my_add_asset_data(this: *const UnknownType, asset_data: *const FAssetData) {
    unsafe {
        writeln!(LOG_FILE.as_ref().unwrap(), "Asset Data Added - {:?}", AssetData::new(asset_data)).unwrap();

        WRITTEN_LINES += 1;
        if WRITTEN_LINES % 100 == 0 {
            LOG_FILE.as_ref().unwrap().sync_data().unwrap();
        }
    }

    AddAssetData.call(this, asset_data);
}

fn my_u_add_asset_data(this: *const UnknownType, asset_data: *const FAssetData) {
    UAddAssetData.call(this, asset_data);
}