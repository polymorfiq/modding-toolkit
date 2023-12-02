use retour::static_detour;
use ue_types::{FString, UConsole};
use game_base::GameBase;
use utils::{debug, warning};

static_detour! {
    static ConsoleCommand: fn(*const UConsole, *const FString);
}

pub fn initialize_shivs() {
    let game_base = GameBase::singleton();
    
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

}

fn console_command_shiv(console_ptr: *const UConsole, cmd_ptr: *const FString) {
    let cmd = unsafe { cmd_ptr.as_ref::<'static>().unwrap() };

    println!("Detected console command: {}", cmd.to_string());
    ConsoleCommand.call(console_ptr, cmd_ptr);
}