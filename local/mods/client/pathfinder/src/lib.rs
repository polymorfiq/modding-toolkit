#![feature(pointer_byte_offsets)]
use std::ffi::c_void;

use game_base::*;
use ue_types::*;
use utils::{debug, logln};

static MOD_NAME: &'static str = "pathfinder";

#[no_mangle]
fn mod_main(base_addr: *const c_void) {
    unsafe { game_base::OFFSETS = game_base::offsets_client::get_offsets() };
    
    GameBase::initialize(MOD_NAME, base_addr);
    std::thread::sleep(std::time::Duration::from_millis(5000));
    
    // Logs debug message to in-game console
    utils::log::set_print_to_console(Box::new(|msg| {
        let console = GameConsole::get();
        if console.is_some() { console.unwrap().output_text(msg) };
    }));

    injection_utils::hooks::console::add_command_intercept(intercept_console_command).expect(format!("[{}]: Could not intercept Console Commands!", MOD_NAME).as_str());
}

fn intercept_console_command(_console: Console, cmd: &FString) -> Result<bool, Box<dyn std::error::Error>> {
    let cmd_str = cmd.to_string().trim_end_matches([0x00 as char]).to_string();

    let should_forward = match (cmd_str.as_str(), cmd_str.split_once(" ")) {
        ("getplayerpos", _) => {
            let game_instance = GameInstance::instance().expect("Could not fetch game instance");
            let local_player = game_instance.base().unwrap().local_players.at_index(0);
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

            logln!("Player Position: {:?}", unsafe { (*pawn.unwrap()).get_world_location() });

            false
        }

        _ => true
    };

    Ok(should_forward)
}