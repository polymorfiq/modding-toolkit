// UConsole VF Table Client Addr: 0x7FF7506D1450
use ue_types::FString;
use widestring::WideString;

static mut PRINT_TO_CONSOLE: Option<Box<dyn Fn(&FString)>> = None;
pub fn set_print_to_console(print_fn: Box<dyn Fn(&FString)>) {
    unsafe { PRINT_TO_CONSOLE = Some(print_fn) };
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

    unsafe {
        if PRINT_TO_CONSOLE.is_some() {
            (PRINT_TO_CONSOLE.as_ref().unwrap())(&f_str_msg)
        }
    }
}