use crate::*;
use simple_endian::*;
use widestring::WideChar;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UConsole {
    // Size: 0x140
    pub base_object: UObject<UConsoleUObjectVFTable>,
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
pub struct UConsoleVFTable {}

impl UConsoleVFTable {
    pub fn console_command(&self) -> *const fn(this: *const UConsole, cmd: *const FString) {
        let base_addr = std::ptr::addr_of!(*self);
        unsafe { std::mem::transmute(base_addr.byte_offset(0x248)) }
    }

    pub fn output_text(&self) -> *const fn(this: *const UConsole, text: *const FString) {
        let base_addr = std::ptr::addr_of!(*self);
        unsafe { std::mem::transmute(base_addr.byte_offset(0x258)) }
    }
    
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UConsoleUObjectVFTable {
    _unknown_a: *const UnknownType,
    _unknown_b: *const UnknownType,
    pub deferred_register: *const fn(this: *const UObject<UnknownType>, u_class_static_class: *const UClass, package_name: *const WideChar, in_name: *const WideChar),
    _unknown_c: *const UnknownType,
    pub can_be_in_cluster: *const fn(this: *const UObject<UnknownType>) -> bool,
    pub create_cluster: *const fn(this: *const UObject<UnknownType>),
    _unknown_d: *const UnknownType,
    pub get_detailed_info_internal: *const fn(this: *const UObject<UnknownType>, result: *const FString) -> *const FString,
    _unknown_e: *const UnknownType,
    _unknown_f: *const UnknownType,
    _unknown_g: *const UnknownType,
    _unknown_h: *const UnknownType,
    _unknown_i: *const UnknownType,
    pub modify: *const fn(this: *const UObject<UnknownType>, b_always_mark_dirty: bool) -> bool,
    pub post_load: *const fn(this: *const UObject<UnknownType>),
    pub post_load_sub_objects: *const fn(this: *const UObject<UnknownType>, outer_instance_graph: *const UnknownType),
    pub begin_destroy: *const fn(this: *const UObject<UnknownType>),
    _unknown_j: *const UnknownType,
    pub finish_destroy: *const fn(this: *const UObject<UnknownType>),
    _unknown_k: *const UnknownType,
    pub serialize_object: *const fn(this: *const UObject<UnknownType>, ar: *const UnknownType),
    _unknown_l: *const UnknownType,
    pub post_duplicate: *const fn(this: *const UObject<UnknownType>, duplicate_mode: UnknownType),
    _unknown_m: *const UnknownType,
    pub needs_load_for_client: *const fn(this: *const UObject<UnknownType>) -> bool,
    pub needs_load_for_server: *const fn(this: *const UObject<UnknownType>) -> bool,
    _unknown_n: *const UnknownType,
    _unknown_o: [*const UnknownType; 3],
    pub get_preload_dependencies: *const fn(this: *const UObject<UnknownType>, out_deps: TArray<*const UObject<UnknownType>, FDefaultAllocator>),
    _unknown_p: [*const UnknownType; 5],
    pub rename_object: *const fn(this: *const UObject<UnknownType>, in_name: *const WideChar, new_outer: *const UObject<UnknownType>, flags: u32le) -> bool,
    _unknown_q: *const UnknownType,
    pub get_world: *const fn(this: *const UObject<UnknownType>) -> *const UWorld,
    _unknown_r: *const UnknownType,
    pub get_resource_size_ex: *const fn(this: *const UObject<UnknownType>, cumulative_resource_size: *const UnknownType),
    pub get_exporter_name: *const fn(this: *const UObject<UnknownType>, result: *mut FName) -> *const FName,
    _unknown_s: *const UnknownType,
    _unknown_t: *const UnknownType,
    pub get_asset_registry_tags: *const fn(this: *const UObject<UnknownType>, out_tags: TArray<UnknownType, FDefaultAllocator>),
    pub is_asset: *const fn(this: *const UObject<UnknownType>) -> bool,
    pub get_primary_asset_id: *const fn(this: *const UObject<UnknownType>, result: *const UnknownType) -> *const UnknownType,
    pub is_localized_resource: *const fn(this: *const UObject<UnknownType>) -> bool,
    pub is_safe_for_root_set: *const fn(this: *const UObject<UnknownType>) -> bool,
    pub tag_sub_objects: *const fn(this: *const UnknownType, new_flags: UnknownType),
    _unknown_u: *const UnknownType,
    pub is_name_stable_for_networking: *const fn(this: *const UObject<UnknownType>) -> bool,
    pub is_full_name_stable_for_networking: *const fn(this: *const UObject<UnknownType>) -> bool,
    pub is_supported_for_networking: *const fn(this: *const UObject<UnknownType>) -> bool,
    _unknown_v: [*const UnknownType; 5],
    _build_sub_object_mapping: *const UnknownType,
    _unknown_w: *const UnknownType,
    _unknown_x: *const UnknownType,
    pub process_event: *const fn(this: *const UObject<UnknownType>, function: *const UnknownType, params: *const UnknownType),
    _get_max_child_nodes: *const UnknownType,
    _unknown_y: *const UnknownType,
    _process_console_exec: *const UnknownType,
    _unknown_z: [*const UnknownType; 2],
    pub check_default_subobjects_internal: *const fn(this: *const UObject<UnknownType>) -> bool,
    pub set_input_text: *const fn(this: *const UConsole, text: *const FString),
    _set_response_state_discovery_channel: *const UnknownType,
    pub console_command: *const fn(this: *const UConsole, cmd: *const FString),
    pub clear_output: *const fn(this: *const UConsole),
    pub output_text: *const fn(this: *const UConsole, text: *const FString),
    pub start_typing: *const fn(cmd: *const FString),
    pub flush_player_input: *const fn(this: *const UConsole),
    pub process_shift_key: *const fn(this: *const UConsole, key: UnknownType, event: UnknownType) -> bool,
    pub append_input_text: *const fn(text: *const FString),
    pub build_runtime_auto_complete_list: *const fn(this: *const UConsole, b_force: bool),
    _unknown_za: *const UnknownType,
    pub update_complete_indices: *const fn(this: *const UConsole),
    pub input_char_typing: *const fn(this: *const UConsole, controller_id: u32le, unicode: *const FString) -> bool,
    pub post_render_console_typing: *const fn(this: *const UConsole, canvas: *const UnknownType),
    pub begin_state_typing: *const fn(this: *const UConsole, previous_state_name: FName),
    pub end_state_typing: *const fn(this: *const UConsole, next_state_name: FName),
    pub input_key_open: *const fn(this: *const UConsole, controller_id: u32le, key: UnknownType, event: UnknownType, amount_depressed: f32le, b_gamepad: bool) -> bool,
    pub post_render_console_open: *const fn(this: *const UConsole, canvas: *const UnknownType),
    _unknown_zb: *const UnknownType,
    pub input_char: *const fn(this: *const UConsole, controller_id: u32le, unicode: *const FString) -> bool,
    pub input_key: *const fn(this: *const UConsole, controller_id: u32le, key: UnknownType, event: UnknownType, amount_depressed: f32le, b_gamepad: bool) -> bool,
    pub input_axis: *const fn(this: *const UConsole, controller_id: u32le, key: UnknownType, delta: f32le, delta_time: f32le, num_samples: u32le, b_gamepad: bool) -> bool,
    _unknown_zc: *const UnknownType,
    pub post_render_console: *const fn(this: *const UConsole, canvas: *const UnknownType),
    pub fake_goto_state: *const fn(this: *const UConsole, next_state_name: FName)
}