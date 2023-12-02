// UConsole VF Table Client Addr: 0x7FF7506D1450
use ue_types::GameBase;

#[cfg(not(feature = "in-game-console"))]
pub fn init() {}

#[cfg(feature = "in-game-console")]
pub fn init() {}

#[cfg(not(feature = "in-game-console"))]
pub fn do_print(message: &str) {
    println!("{}", message);
}

#[cfg(feature = "in-game-console")]
pub fn do_print(message: &str) {
    print_to_game_console(format!("{}", message));
    println!("{}", message);
}

#[cfg(not(feature = "in-game-console"))]
pub fn do_debug(message: &str) {
    println!("Debug: {}", message);
}

#[cfg(feature = "in-game-console")]
pub fn do_debug(message: &str) {
    print_to_game_console(format!("Debug: {}", message));
    println!("Debug: {}", message);
}

#[cfg(not(feature = "in-game-console"))]
pub fn do_info(message: &str) {
    println!("Info: {}", message);
}

#[cfg(feature = "in-game-console")]
pub fn do_info(message: &str) {
    print_to_game_console(format!("Info: {}", message));
    println!("Info: {}", message);
}

#[cfg(not(feature = "in-game-console"))]
pub fn do_notice(message: &str) {
    println!("Notice: {}", message);
}

#[cfg(feature = "in-game-console")]
pub fn do_notice(message: &str) {
    print_to_game_console(format!("Notice: {}", message));
    println!("Notice: {}", message);
}

#[cfg(not(feature = "in-game-console"))]
pub fn do_warning(message: &str) {
    println!("Warning: {}", message);
}

#[cfg(feature = "in-game-console")]
pub fn do_warning(message: &str) {
    print_to_game_console(format!("Warning: {}", message));
    println!("Warning: {}", message);
}

#[cfg(not(feature = "in-game-console"))]
pub fn do_error(message: &str) {
    println!("Error: {}", message);
}

#[cfg(feature = "in-game-console")]
pub fn do_error(message: &str) {
    print_to_game_console(format!("Error: {}", message));
    println!("Error: {}", message);
}

fn print_to_game_console(message: String) {
    let game_base = GameBase::singleton();
    let console = game_base.console();

    if console.is_some() {
        let console = console.unwrap();
        console.console_command(format!("Say {}\0", message).to_string());
        println!("Wrote to console!!");
    } else {
        println!("Could not find the in-game console!!");
    }
}