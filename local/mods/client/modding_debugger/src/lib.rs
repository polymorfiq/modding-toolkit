#![feature(pointer_byte_offsets)]
use std::ffi::c_void;

use game_base::*;
use ue_types::*;
use utils::{debug, logln};

static MOD_NAME: &'static str = "modding_debugger";

#[no_mangle]
fn mod_main_sync(base_addr: *const c_void) {
    GameBase::initialize(MOD_NAME, base_addr);
    injection_utils::hooks::various::intercept().unwrap();

    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(10000));

        // Logs debug message to in-game console
        utils::log::set_print_to_console(Box::new(|msg| {
            let console = GameConsole::get();
            if console.is_some() { console.unwrap().output_text(msg) };
        }));
        debug!("Loading mod-debugger async code...");

        GObjects::filter(|obj| {
            if obj.class().is_some() && obj.class().unwrap().full_name().contains("ObjectLibrary") {
                debug!("OBJ FOUND: {:?} - {:?} - {:?}", Object::new(obj), obj.full_name(), obj.class().unwrap().full_name());
            }

            false
        });

        injection_utils::hooks::console::add_command_intercept(intercept_console_command).expect(format!("[{}]: Could not intercept Console Commands!", MOD_NAME).as_str());
    });
}

fn intercept_console_command(_console: Console, cmd: &FString) -> Result<bool, Box<dyn std::error::Error>> {
    let cmd_str = cmd.to_string().trim_end_matches([0x00 as char]).to_string();

    let should_forward = match (cmd_str.as_str(), cmd_str.split_once(" ")) {
        (_, Some(("start_game", _))) => {
            MainMenu::connect_to_server("127.0.0.1", "7777", "");
            false
        },

        (_, Some(("withname", args))) => {
            find_objects_with_name(args);
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
    let g_objects = GObjects::objects::<UnknownType>();

    logln!("Finding objects with VFTable: {:p}", table_addr);
    let mut count: usize = 0;

    for i in 0..(g_objects.num_elements.to_native()-1) {
        match g_objects.item_at_idx(i as usize) {
            Some(item) => {
                match unsafe { (*item).object::<UObject<*const UnknownType>>() } {
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
    let g_objects = GObjects::objects::<UnknownType>();

    let mut count: usize = 0;
    logln!("Finding objects with Class Name: {:?}", args);
    for i in 0..(g_objects.num_elements.to_native()-1) {
        let item = g_objects.item_at_idx(i as usize);
        if !item.is_some() { continue };

        let obj = unsafe { (*item.unwrap()).object::<UObject<*const UnknownType>>() };
        if !obj.is_some() { continue };

        let obj = obj.unwrap();

        match obj.class() {
            Some(class) => {
                if class.full_name().contains(args) {
                    logln!("GOBJECTS[{:?}]: {:?} ({:?})", i, obj.full_name(), obj.class().unwrap().full_name());
                    count += 1;
                }
            }

            None => ()
        }
        
        
    }
    if count == 0 { logln!("No results found for Class Name: {:?}", args); }
}

fn find_objects_with_name(args: &str) {
    let g_objects = GObjects::objects::<UnknownType>();

    let mut count: usize = 0;
    logln!("Finding objects with Class Name: {:?}", args);
    for i in 0..(g_objects.num_elements.to_native()-1) {
        let item = g_objects.item_at_idx(i as usize);
        if !item.is_some() { continue };

        let obj = unsafe { (*item.unwrap()).object::<UObject<*const UnknownType>>() };
        if !obj.is_some() { continue };

        let obj = obj.unwrap();
        
        if obj.full_name().contains(args) {
            let class_name = match obj.class() {
                Some(class) => class.full_name(),
                None => "?".to_string()
            };

            logln!("GOBJECTS[{:?}]: {:?} ({:?})", i, obj.full_name(), class_name);
            count += 1;
        }
    }
    if count == 0 { logln!("No results found for Object Name: {:?}", args); }
}