pub mod log;

pub fn init() {
    log::init();
}

#[macro_export]
macro_rules! print {
    () => {
        $crate::log::do_print("\n")
    };
    ($($arg:tt)*) => {{
        $crate::log::do_print(std::format!($($arg)*).as_str())
    }};
}

#[macro_export]
macro_rules! debug {
    () => {
        $crate::log::do_debug("\n")
    };
    ($($arg:tt)*) => {{
        $crate::log::do_debug(std::format!($($arg)*).as_str())
    }};
}

#[macro_export]
macro_rules! info {
    () => {
        $crate::log::do_info("\n")
    };
    ($($arg:tt)*) => {{
        $crate::log::do_info(std::format!($($arg)*).as_str())
    }};
}

#[macro_export]
macro_rules! notice {
    () => {
        $crate::log::do_notice("\n")
    };
    ($($arg:tt)*) => {{
        $crate::log::do_notice(std::format!($($arg)*).as_str())
    }};
}

#[macro_export]
macro_rules! warning {
    () => {
        $crate::log::do_warning("\n")
    };
    ($($arg:tt)*) => {{
        $crate::log::do_warning(std::format!($($arg)*).as_str())
    }};
}

#[macro_export]
macro_rules! error {
    () => {
        $crate::log::do_error("\n")
    };
    ($($arg:tt)*) => {{
        $crate::log::do_error(std::format!($($arg)*).as_str())
    }};
}