use crate::*;
use simple_endian::u32le;

const OFFSET_VFTABLE_SET_INPUT_TEXT: isize = 0x238;
const OFFSET_VFTABLE_CONSOLE_COMMAND: isize = 0x248;
const OFFSET_VFTABLE_OUTPUT_TEXT: isize = 0x258;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UConsole {
    // Size: 0x140
    pub base_object: UObject,
    pub base_output_device: FOutputDevice,
    pub console_target_player: *const UnknownType,
    pub default_texture_black: *const UnknownType,
    pub default_texture_white: *const UnknownType,
    pub scrollback: TArray<FString, FDefaultAllocator>,
    pub sb_head: u32le,
    pub sb_pos: u32le,
    pub history_buffer: TArray<FString, FDefaultAllocator>,
    pub typed_str: FString,
    pub typed_str_pos: u32le,
    _padding_a: [u8; 4],
    pub pre_computed_input_line: FString,
    pub last_autocompleted_command: FString,
    _b_something_a: u8,
    _padding_b: [u8; 7],
    pub auto_complete_list: TArray<UnknownType, FDefaultAllocator>,
    _b_something_b: u8,
    _padding_c: [u8; 3],
    pub auto_complete_idx: u32le,
    pub auto_complete_cursor: u32le,
    _b_something_c: u8,
    _padding_d: [u8; 3],
    pub console_state: FName,
    _padding_e: [u8; 4],
    pub auto_complete_tree: FAutoCompleteNode,
    pub auto_complete: TArray<UnknownType, FDefaultAllocator>,
    pub console_settings: *const UnknownType,
    pub previous_focused_widget: TWeakPtr<UnknownType>
}

impl UConsole {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }

    pub fn output_text(&self, text: &FString) {
        let vftable = self.object().virtual_funcs();
        let output_text_fn: fn(*const UConsole, *const FString) = unsafe {
            std::mem::transmute(*vftable.byte_offset(OFFSET_VFTABLE_OUTPUT_TEXT))
        };

        (output_text_fn)(std::ptr::addr_of!(*self), std::ptr::addr_of!(*text));
    }

    pub fn set_input_text(&self, text: &FString) {
        let vftable = self.object().virtual_funcs();
        let set_input_text_fn: fn(*const UConsole, *const FString) = unsafe {
            std::mem::transmute(*vftable.byte_offset(OFFSET_VFTABLE_SET_INPUT_TEXT))
        };

        (set_input_text_fn)(std::ptr::addr_of!(*self), std::ptr::addr_of!(*text));
    }

    pub fn console_command(&self, cmd: &FString) {
        (self._console_command())(std::ptr::addr_of!(*self), std::ptr::addr_of!(*cmd));
    }

    pub fn _console_command(&self) -> fn(*const UConsole, *const FString) {
        let vftable = self.object().virtual_funcs();
        let console_cmd_addr = unsafe { *vftable.byte_offset(OFFSET_VFTABLE_CONSOLE_COMMAND) };
        let console_command_fn: fn(*const UConsole, *const FString) = unsafe { std::mem::transmute(console_cmd_addr) };
        console_command_fn
    }
}