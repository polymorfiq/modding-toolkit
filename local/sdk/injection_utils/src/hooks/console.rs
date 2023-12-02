use retour::static_detour;
use ue_types::{FString, UConsole};
use game_base::GameBase;
use utils::{debug, warning};

static_detour! {
    static CommandIntercept: fn(*const UConsole, *const FString);
}

type CommandInterceptShiv = fn(&UConsole, &FString) -> Result<bool, Box<dyn std::error::Error>>;
pub fn add_command_intercept(shiv_fn: CommandInterceptShiv) -> Result<(), Box<dyn std::error::Error>> {
    match GameBase::singleton().console() {
        Some(console) => {
            unsafe {
                CommandIntercept.initialize(console._console_command(), call_intercept_shiv(shiv_fn))?;
                CommandIntercept.enable()?;
            }

            debug!("[{}]: Intercepting in-game console commands!", GameBase::singleton().mod_name);
            Ok(())
        },

        None => {
            Err("No console attached to GameBase!".into())
        }
    }
}

fn call_intercept_shiv(shiv_fn: CommandInterceptShiv) -> impl Fn(*const UConsole, *const FString) {
    return move |console: *const UConsole, cmd: *const FString| {
        let console_ref = unsafe { console.as_ref::<'static>().unwrap() };
        let cmd_ref = unsafe { cmd.as_ref::<'static>().unwrap() };

        match shiv_fn(console_ref, cmd_ref) {
            Ok(do_forward_cmd) => {
                if do_forward_cmd { CommandIntercept.call(console, cmd) };
            }

            Err(err) => {
                warning!("[{}]: Error intercepting console command: {:?}", GameBase::singleton().mod_name, err);
                CommandIntercept.call(console, cmd)
            }
        }
    }
}