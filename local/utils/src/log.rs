// UConsole VF Table Client Addr: 0x7FF7506D1450
use ue_types::{FString, UConsole};
use widestring::WideString;

static mut CONSOLE: Option<&UConsole> = None;
pub fn set_console(console: Option<&'static UConsole>) {
    println!("CONSOLE SET: {:?}", console);
    unsafe { CONSOLE = console };
}

pub fn do_logln(message: &str) {
    print_to_game_console(format!("{}", message));
    println!("{}", message);
}

pub fn do_debug(message: &str) {
    print_to_game_console(format!("Debug: {}", message));
    println!("Debug: {}", message);
}

pub fn do_info(message: &str) {
    print_to_game_console(format!("Info: {}", message));
    println!("Info: {}", message);
}

pub fn do_notice(message: &str) {
    print_to_game_console(format!("Notice: {}", message));
    println!("Notice: {}", message);
}

pub fn do_warning(message: &str) {
    print_to_game_console(format!("Warning: {}", message));
    println!("Warning: {}", message);
}

pub fn do_error(message: &str) {
    print_to_game_console(format!("Error: {}", message));
    println!("Error: {}", message);
}

fn print_to_game_console(message: String) {
    let msg: WideString = format!("{}\0", message).into();
    let f_str_msg: FString = msg.into();

    match unsafe { CONSOLE } {
        Some(console) => console.output_text(&f_str_msg),
        None => ()
    }
}