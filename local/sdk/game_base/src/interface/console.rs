use crate::*;
use ue_types::*;
use memory_management::strings::fstring;

#[derive(Debug, Copy, Clone)]
pub struct Console(*const UConsole);

impl Console {
    pub fn new(console: *const UConsole) -> Self {
        Self(console)
    }

    pub fn console_command(&self, cmd: &str) {
        let cmd_str = format!("{}\0", cmd.to_string());
        let f_str = fstring(cmd_str);

        unsafe { (*self.object_virtual_funcs().console_command())(self.0, std::ptr::addr_of!(*f_str)); }
    }

    pub fn output_text(&self, text: *const FString) {
        unsafe { (*self.object_virtual_funcs().output_text())(self.0, text); }
    }
}

impl VirtualObject<UConsoleVFTable> for Console {
    fn object(&self) -> Object<UConsoleVFTable> {
        unsafe { Object::<UConsoleVFTable>::new(std::ptr::addr_of!((*self.0).base_object)) }
    }
}