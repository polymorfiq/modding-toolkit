use retour::static_detour;
use ue_types::{FString, UConsole, UnknownType, UObject};
use game_base::GameBase;
use utils::{logln, debug, warning};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

static_detour! {
    static ConsoleCommand: fn(*const UConsole, *const FString);
}

#[derive(Debug, Clone)]
pub enum UserCommand {
    FindWithVFTable(*const *const UnknownType),
    UnknownCommand(String),
    CloseClient,
    Noop
}

static mut CMD_CHANNEL: Option<Sender<UserCommand>> = None;

pub fn initialize_shivs() -> Receiver<UserCommand> {
    let game_base = GameBase::singleton();

    let (tx, rx): (Sender<UserCommand>, Receiver<UserCommand>) = mpsc::channel();
    unsafe { CMD_CHANNEL = Some(tx) };
    
    match game_base.console() {
        Some(console) => unsafe {
            let placed_shiv = ConsoleCommand.initialize(console._console_command(), console_command_shiv);
            if placed_shiv.is_ok() {
                ConsoleCommand.enable().expect("Failed enabling UConsole shiv!");
                debug!("Successfully placed Console Shiv!");
            } else {
                warning!("Could not place Console Shiv!");
            }
        }

        None => ()
    }

    rx
}

fn console_command_shiv(console_ptr: *const UConsole, cmd_ptr: *const FString) {
    let cmd = unsafe { cmd_ptr.as_ref::<'static>().unwrap() };

    let mut cmd_str = cmd.to_string();
    cmd_str.truncate(cmd_str.len()-1);

    let user_cmd = to_user_cmd(&cmd_str);
    let tx = unsafe {
        if CMD_CHANNEL.is_some() { Some(CMD_CHANNEL.as_ref().unwrap().clone()) } else { None }
    };

    println!("Detected console command: {} - {:?}", cmd_str, user_cmd);
    use UserCommand::*;
    match user_cmd {
        UnknownCommand(_) => ConsoleCommand.call(console_ptr, cmd_ptr),
        FindWithVFTable(table_addr) => find_with_vf_table(table_addr),
        CloseClient => {
            debug!("Told to close client! Exiting!");
            std::process::exit(1)
        },
        _ => if tx.is_some() { tx.unwrap().send(user_cmd).unwrap() }
    };
}

fn to_user_cmd(cmd_str: &String) -> UserCommand {
    match (cmd_str.as_str(), cmd_str.split_once(" ")) {
        ("close_client", _) => UserCommand::CloseClient,
        ("noop", _) => UserCommand::Noop,
        (_, Some(("vftable", args))) => {
            let without_prefix = args.trim_start_matches("0x");

            match usize::from_str_radix(without_prefix, 16) {
                Ok(num) => UserCommand::FindWithVFTable(num as *const *const UnknownType),
                _ => UserCommand::UnknownCommand(cmd_str.clone())
            }
            
        },
        (_, Some(("noop", _))) => UserCommand::Noop,
        _ => UserCommand::UnknownCommand(cmd_str.clone())
    }
}

fn find_with_vf_table(table_addr: *const *const UnknownType) {
    let g_objects = GameBase::singleton().gobjects();

    logln!("Finding objects with VFTable: {:p}", table_addr);
    let mut count: usize = 0;

    for i in 0..(g_objects.num_elements.to_native()-1) {
        match g_objects.item_at_idx(i as usize) {
            Some(item) => {
                match item.object::<UObject>() {
                    Some(obj) => {
                        if obj.virtual_funcs() == table_addr {
                            count += 1;
                            logln!("GOBJECTS[{:?}]: {:?} ({:?})", i, obj.full_name(), obj.class().full_name());
                        }
                    }
                    _ => ()
                }
                
            }

            None => ()
        }
    }

    if count == 0 { logln!("No results found for VFTable: {:p}", table_addr); }
}