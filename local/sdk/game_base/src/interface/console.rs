use crate::*;
use ue_types::*;

#[derive(Debug, Copy, Clone)]
pub struct Console(*const UConsole);

impl Console {
    pub fn new(console: *const UConsole) -> Self {
        Self(console)
    }

    pub fn console_command(&self, cmd: String) {
        let cmd_fstr: Box<FString> = Box::new(cmd.into());
        unsafe { (*self.object_virtual_funcs().console_command())(self.0, Box::into_raw(cmd_fstr)); }
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