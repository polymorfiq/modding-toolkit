#![feature(pointer_byte_offsets)]
use std::ffi::c_void;

use game_base::GameBase;
use game_base::{HasPawn, NavAgentLocatable};
use ue_types::*;
use utils::{debug, logln};

static MOD_NAME: &'static str = "pathfinder";

#[no_mangle]
fn mod_main(base_addr: *const c_void) {
    let game_base = GameBase::initialize(MOD_NAME, base_addr);
    
    // Logs debug message to in-game console
    utils::log::set_console(game_base.console());

    injection_utils::hooks::console::add_command_intercept(intercept_console_command).expect(format!("[{}]: Could not intercept Console Commands!", MOD_NAME).as_str());
}

fn intercept_console_command(_console: &UConsole, cmd: &FString) -> Result<bool, Box<dyn std::error::Error>> {
    let cmd_str = cmd.to_string().trim_end_matches([0x00 as char]).to_string();

    let should_forward = match (cmd_str.as_str(), cmd_str.split_once(" ")) {
        ("getplayerpos", _) => {
            let game_instance = GameBase::singleton().game_instance();
            if game_instance.is_none() {
                debug!("GameInstance not found?!");
                return Ok(false)
            };
            let game_instance = game_instance.unwrap();

            let local_player = game_instance.local_players.at_index(0);
            if !local_player.is_ok() {
                debug!("Local Player not found?!");
                return Ok(false)
            };
            let local_player: *const ULocalPlayer = *local_player.unwrap();

            let pawn = local_player.pawn();
            if pawn.is_none() {
                debug!("Pawn not found?!");
                return Ok(false)
            }

            logln!("Player Position: {:?}", pawn.unwrap().get_nav_agent_location());

            false
        }

        _ => true
    };

    Ok(should_forward)
}