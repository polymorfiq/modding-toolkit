use retour::static_detour;
use ue_types::*;
use game_base::*;
use utils::debug;
use simple_endian::*;

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
}

pub fn intercept() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {

        let set_game_mode: fn(*const UGameInstance, StringArg, StringArg) = std::mem::transmute(GameBase::singleton().at_offset(0x39910C0));
        SetGameMode.initialize(set_game_mode, my_set_game_mode)?;
        SetGameMode.enable()?;

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