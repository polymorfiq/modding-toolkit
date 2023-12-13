#![feature(pointer_byte_offsets)]
use std::ffi::c_void;
use game_base::*;
use ue_types::*;
use utils::logln;

static MOD_NAME: &'static str = "tdm_helper";

#[no_mangle]
fn mod_main(base_addr: *const c_void) {
    GameBase::initialize(MOD_NAME, base_addr);
    std::thread::sleep(std::time::Duration::from_millis(5000));
    
    // Logs debug message to in-game console
    utils::log::set_print_to_console(Box::new(|msg| {
        let console = GameConsole::get();
        if console.is_some() { console.unwrap().output_text(msg) };
    }));

    injection_utils::hooks::console::add_command_intercept(intercept_console_command).expect(format!("[{}]: Could not intercept Console Commands!", MOD_NAME).as_str());
    injection_utils::hooks::player_events::on_respawn(on_player_respawn).unwrap();
}

fn intercept_console_command(console: Console, cmd: &FString) -> Result<bool, Box<dyn std::error::Error>> {
    let cmd_str = cmd.to_string().trim_end_matches([0x00 as char]).to_string();

    let should_forward = match (cmd_str.as_str(), cmd_str.split_once(" ")) {
        ("guns", _) => {
            console.console_command("LevelUpCharacterClass");
            console.console_command("LevelUpCharacterClass");
            console.console_command("LevelUpCharacterClass");
            console.console_command("LevelUpCharacterClass");
            console.console_command("SpawnGauntlet Loot:BP_Item_Weapon_Ice_Tier_5 1");
            console.console_command("SpawnGauntlet Loot:BP_Item_Weapon_Fire_Tier_5 1");
            console.console_command("SpawnGauntlet Loot:BP_Item_Weapon_Poison_Tier_5 1");
            console.console_command("SpawnGauntlet Loot:BP_Item_Weapon_Wind_Tier_5 1");
            console.console_command("SpawnGauntlet Loot:BP_Item_Weapon_Earth_Tier_5 1");
            console.console_command("SpawnGauntlet Loot:BP_Item_Weapon_Shock_Tier_5 1");
            console.console_command("SpawnAmulet Loot:BP_Item_Amulet_Tier_5 1");
            console.console_command("SpawnBoot Loot:BP_Item_Boots_Tier_5 1");
            console.console_command("SpawnBelt Loot:BP_Item_Belt_Tier_5 1");
            console.console_command("SpawnScroll Scroll:BP_Item_Scroll_Spirit_Full");
            console.console_command("SpawnScroll Scroll:BP_Item_Scroll_Mind_Full");
            console.console_command("SpawnScroll Scroll:BP_Item_Scroll_Body_Full");
            false
        },

        _ => true
    };

    Ok(should_forward)
}


fn on_player_respawn(_this: *const AGameModeBase, player: *const APlayerController) {
    logln!("[tdm_helper] Player respawned - refilling health...");

    let cmd_1: Box<FString> = Box::new("ApplyPlayerEffect GameplayEffect:BP_Effect_Player_Adjust_Health 50".into());
    let cmd_2: Box<FString> = Box::new("ApplyPlayerEffect GameplayEffect:BP_Effect_Player_Adjust_Armor 200".into());
    let result: Box<FString> = Box::new("".into());

    unsafe {
        let player_console_command: fn(*const APlayerController, *mut FString, *const FString, bool) = std::mem::transmute(GameBase::singleton().at_offset(0x258AFD0));
        (player_console_command)(player, Box::into_raw(result.clone()), Box::into_raw(cmd_1), false);
        (player_console_command)(player, Box::into_raw(result), Box::into_raw(cmd_2), false);
    }
}