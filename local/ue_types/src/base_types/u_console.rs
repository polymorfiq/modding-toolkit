use simple_endian::*;
use crate::{FName, FString, OwnedFString, TArray, TWeakPtr, UObject, UnknownType};

const OFFSET_VFTABLE_SET_INPUT_TEXT: isize = 0x238;
const OFFSET_VFTABLE_CONSOLE_COMMAND: isize = 0x248;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UConsole {
    // Size: 0x140
    base_object: UObject,
    base_output_device: FOutputDevice,
    console_target_player: *const UnknownType,
    default_texture_black: *const UnknownType,
    default_texture_white: *const UnknownType,
    scrollback: TArray<FString>,
    sb_head: u32le,
    sb_pos: u32le,
    history_buffer: TArray<FString>,
    typed_str: FString,
    typed_str_pos: u32le,
    _padding_a: [u8; 4],
    pre_computed_input_line: FString,
    last_autocompleted_command: FString,
    _b_something_a: u8,
    _padding_b: [u8; 7],
    auto_complete_list: TArray<UnknownType>,
    _b_something_b: u8,
    _padding_c: [u8; 3],
    auto_complete_idx: u32le,
    auto_complete_cursor: u32le,
    _b_something_c: u8,
    _padding_d: [u8; 3],
    console_state: FName,
    _padding_e: [u8; 4],
    auto_complete_tree: FAutoCompleteNode,
    auto_complete: TArray<UnknownType>,
    console_settings: *const UnknownType,
    previous_focused_widget: TWeakPtr<UnknownType>
}

impl UConsole {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }

    pub fn set_input_text(&self, cmd: String) {
        let vftable = self.object().virtual_funcs();
        let owned_f_str = OwnedFString::from_string(cmd);
        let f_str = owned_f_str.fstring();

        let set_input_text_fn: fn(*const UConsole, *const FString) = unsafe {
            std::mem::transmute(*vftable.byte_offset(OFFSET_VFTABLE_SET_INPUT_TEXT))
        };
        println!("SET INPUT TEXT: {:p}", set_input_text_fn);

        (set_input_text_fn)(std::ptr::addr_of!(*self), std::ptr::addr_of!(f_str));
    }

    pub fn console_command(&self, cmd: String) {
        let vftable = self.object().virtual_funcs();
        let owned_f_str = OwnedFString::from_string(cmd);
        let f_str = owned_f_str.fstring();

        let console_command_fn: fn(*const UConsole, *const FString) = unsafe {
            std::mem::transmute(*vftable.byte_offset(OFFSET_VFTABLE_CONSOLE_COMMAND))
        };

        (console_command_fn)(std::ptr::addr_of!(*self), std::ptr::addr_of!(f_str));
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FOutputDevice {
    // Size: 0x10
    __vf_table: *const UnknownType,
    b_suppress_event_tags: u8,
    b_auto_emit_line_terminator: u8,
    _padding: [u8; 6]
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FAutoCompleteNode {
    // Size: 0x28
    _something: [u8; 0x28]
}