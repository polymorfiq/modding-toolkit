use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UConsole {
    // Size: 0x140
    pub base_object: UObject<UConsoleVFTable>,
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

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UConsoleVFTable {
    addr: *const std::ffi::c_void
}

impl UConsoleVFTable {
    pub fn console_command(&self) -> *const fn(this: *const UConsole, cmd: *const FString) {
        unsafe { std::mem::transmute(self.addr.byte_offset(0x248)) }
    }

    pub fn output_text(&self) -> *const fn(this: *const UConsole, text: *const FString) {
        unsafe { std::mem::transmute(self.addr.byte_offset(0x258)) }
    }
    
}