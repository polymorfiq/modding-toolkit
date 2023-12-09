#![feature(pointer_byte_offsets)]
use std::ffi::c_void;

use game_base::GameBase;
use game_base::{VirtualUObject};
use ue_types::*;
use utils::{debug, logln};

static MOD_NAME: &'static str = "modding_debugger";

#[no_mangle]
fn mod_main(base_addr: *const c_void) {
    let game_base = GameBase::initialize(MOD_NAME, base_addr);
    
    // Logs debug message to in-game console
    utils::log::set_print_to_console(Box::new(|msg| {
        let console = game_base.console();
        if console.is_some() { unsafe { (*(*console.unwrap().virtual_funcs()).output_text())(console.unwrap(), msg) } };
    }));

    injection_utils::hooks::console::add_command_intercept(intercept_console_command).expect(format!("[{}]: Could not intercept Console Commands!", MOD_NAME).as_str());
}

fn intercept_console_command(_console: &UConsole, cmd: &FString) -> Result<bool, Box<dyn std::error::Error>> {
    let cmd_str = cmd.to_string().trim_end_matches([0x00 as char]).to_string();

    let should_forward = match (cmd_str.as_str(), cmd_str.split_once(" ")) {
        (_, Some(("world", args))) => {
            find_actors(args);
            false
        },

        (_, Some(("withclass", args))) => {
            find_objects_with_class(args);
            false
        },

        (_, Some(("vftable", args))) => {
            let without_prefix = args.trim_start_matches("0x");
            let from_hex = usize::from_str_radix(without_prefix, 16)?;
            find_with_vf_table(from_hex as *const *const UnknownType);

            false
        }

        _ => true
    };

    Ok(should_forward)
}

fn find_with_vf_table(table_addr: *const *const UnknownType) {
    let g_objects = GameBase::singleton().gobjects();

    logln!("Finding objects with VFTable: {:p}", table_addr);
    let mut count: usize = 0;

    for i in 0..(g_objects.num_elements.to_native()-1) {
        match g_objects.item_at_idx(i as usize) {
            Some(item) => {
                match unsafe { (*item).object::<UObject<UnknownType>>() } {
                    Some(obj) => {
                        count += 1;
                        logln!("GOBJECTS[{:?}]: {:?}", i, obj.full_name());
                    }
                    _ => ()
                }
                
            }

            None => ()
        }
    }

    if count == 0 { logln!("No results found for VFTable: {:p}", table_addr); }
}

fn find_objects_with_class(args: &str) {
    let g_objects = GameBase::singleton().gobjects();

    let mut count: usize = 0;
    logln!("Finding objects with Class Name: {:?}", args);
    for i in 0..(g_objects.num_elements.to_native()-1) {
        let item = g_objects.item_at_idx(i as usize);
        if !item.is_some() { continue };

        let obj = unsafe { (*item.unwrap()).object::<UObject<UnknownType>>() };
        if !obj.is_some() { continue };

        let obj = obj.unwrap();
        
        if obj.full_name().contains(args) {
            logln!("GOBJECTS[{:?}]: ({:?})", i, obj.full_name());
            count += 1;
        }
    }
    if count == 0 { logln!("No results found for Class Name: {:?}", args); }
}

fn find_actors(_args: &str) {
    let world = GameBase::singleton().world();
    if world.is_none() {
        debug!("NO WORLD FOUND?!");
        return
    }
    let world = world.unwrap();

    debug!("WORLD: {:#01x?}", world);
    debug!("LEVEL: {:#01x?}", world.level());
    debug!("GAME INSTANCE: {:#01x?}", GameBase::singleton().game_instance());
}