#![feature(pointer_byte_offsets)]
use game_base::*;
use std::ffi::c_void;
use ue_types::*;
use utils::logln;

static MOD_NAME: &'static str = "better_commands";

#[no_mangle]
fn mod_main(base_addr: *const c_void) {
    unsafe { game_base::OFFSETS = game_base::offsets_client::get_offsets() };
    
    GameBase::initialize(MOD_NAME, base_addr);
    GameConsole::wait_for_loaded();

    // Logs debug message to in-game console
    utils::log::set_print_to_console(Box::new(|msg| {
        let console = GameConsole::get();
        if console.is_some() {
            console.unwrap().output_text(msg)
        };
    }));

    injection_utils::hooks::console::add_command_intercept(intercept_console_command).expect(format!("[{}]: Could not intercept Console Commands!", MOD_NAME).as_str());
}

fn intercept_console_command(
    console: Console,
    cmd: &FString,
) -> Result<bool, Box<dyn std::error::Error>> {
    let cmd_str = cmd.to_string().trim_end_matches([0x00 as char]).to_string().to_lowercase();


    let should_forward = match (cmd_str.as_str(), cmd_str.split_once(" ")) {
        (_, Some(("choosecharacterperk", args))) => {
            let perk = match args {
                "characterperk:bp_perk_blood_armor" => Some("CharacterPerk:BP_Perk_Fervor"),
                "characterperk:bp_perk_blight" => Some("CharacterPerk:BP_Perk_Spellslinger"),
                "characterperk:bp_perk_dilution" => Some("CharacterPerk:BP_Perk_Tracking"),
                "characterperk:bp_perk_vivify" => Some("CharacterPerk:BP_Perk_Athletic"),
                "characterperk:bp_perk_specialist" => Some("CharacterPerk:BP_Perk_Harmony"),
                _ => None
            };

            if perk.is_some() {
                console.console_command(format!("ChooseCharacterPerk {}", perk.unwrap()).as_str());
                logln!("Perk {} activated!", perk.unwrap());
                false
            } else {
                true
            }
        },

        ("heal", _) => {
            console.console_command("ApplyPlayerEffect GameplayEffect:BP_Effect_Player_Adjust_Health 150");
            console.console_command("ApplyPlayerEffect GameplayEffect:BP_Effect_Player_Stoneskin 250");
            logln!("Healed!");
            false
        },

        (_, Some(("level", "up"))) => {
            console.console_command("UpgradeCharacterPerk Mind");
            console.console_command("UpgradeCharacterPerk Mind");
            console.console_command("UpgradeCharacterPerk Mind");
            console.console_command("UpgradeCharacterPerk Body");
            console.console_command("UpgradeCharacterPerk Body");
            console.console_command("UpgradeCharacterPerk Body");
            console.console_command("UpgradeCharacterPerk Spirit");
            console.console_command("UpgradeCharacterPerk Spirit");
            console.console_command("UpgradeCharacterPerk Spirit");
            console.console_command("LevelUpCharacterClass");
            console.console_command("LevelUpCharacterClass");
            console.console_command("LevelUpCharacterClass");
            logln!("Leveled up!");
            false
        }


        _ => true
    };

    Ok(should_forward)
}