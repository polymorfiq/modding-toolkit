use crate::*;
use ue_types::*;

pub struct GameConsole;

static mut CONSOLE_ADDR: Option<*const UConsole> = None;
static mut NEVER_WAITED: bool = true;

impl GameConsole {
    pub fn get() -> Option<Console> {
        let existing_addr = unsafe { CONSOLE_ADDR };
        if existing_addr.is_some() { return Some(Console::new(existing_addr.unwrap())) };

        let console_regex = regex::Regex::new(r"/Engine/Transient.GameEngine_([0-9]+)\.GGameViewportClient_([0-9]+)\.Console_([0-9]+)").unwrap();

        let consoles = GObjects::filter(|object| {
            console_regex.is_match(object.full_name().as_str())
        });

        if consoles.len() > 0 {
            unsafe{
                CONSOLE_ADDR = Some(consoles[0] as *const UConsole);
                Some(Console::new(CONSOLE_ADDR.unwrap()))
            }
        } else {
            None
        }
    }

    pub fn wait_for_loaded() {
        if unsafe { NEVER_WAITED } {
            // Give GObjects a little time to start up...
            std::thread::sleep(std::time::Duration::from_millis(5000));
        }

        let mut waited_count: usize = 0;
        loop {
            if Self::get().is_some() {
                break;
            } else if waited_count <= 5 {
                std::thread::sleep(std::time::Duration::from_millis(5000));
                waited_count += 1;
            } else {
                break;
            }
        }
    }
}