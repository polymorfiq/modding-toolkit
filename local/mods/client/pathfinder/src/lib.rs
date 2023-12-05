#![feature(pointer_byte_offsets)]
use std::ffi::c_void;

use game_base::GameBase;
use ue_types::*;
use utils::logln;

static MOD_NAME: &'static str = "pathfinder";

#[no_mangle]
fn mod_main(base_addr: *const c_void) {
    let game_base = GameBase::initialize(MOD_NAME, base_addr);
    
    // Logs debug message to in-game console
    utils::log::set_console(game_base.console());

    injection_utils::hooks::console::add_command_intercept(intercept_console_command).expect(format!("[{}]: Could not intercept Console Commands!", MOD_NAME).as_str());
}

fn intercept_console_command(_console: &UConsole, cmd: &FString) -> Result<bool, Box<dyn std::error::Error>> {
    let mut cmd_str = cmd.to_string();
    cmd_str.truncate(cmd_str.len()-1); // Remove null byte off end of string

    let should_forward = match (cmd_str.as_str(), cmd_str.split_once(" ")) {
        ("getplayerpos", _) => {
            let g_objects = GameBase::singleton().gobjects();

            let mut player_i = 0;
            for i in 0..(g_objects.num_elements.to_native()-1) {
                let item = g_objects.item_at_idx(i as usize);
                if item.is_none() { continue };

                let obj = item.unwrap().object::<UObject>();
                if obj.is_none() { continue };
                let obj = obj.unwrap();
                if obj.class().full_name() != "/Script/Engine.Default__Character".to_string() { continue };

                let controller = item.unwrap().object::<ACharacter>();
                if controller.is_none() { continue };
                
                logln!("Players[{:?}]: {:?}", player_i, ACharacter::get_nav_agent_location(item.unwrap().object_addr as *const ACharacter));
                player_i += 1;
            }

            false
        }

        _ => true
    };

    Ok(should_forward)
}