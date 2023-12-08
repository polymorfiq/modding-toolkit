use crate::*;
use simple_endian::*;
use widestring::WideChar;
use std::marker::PhantomData;

pub mod object_flags {
    pub const NONE: u32 = 0;
	pub const REACHABLE_IN_CLUSTER: u32 = 1 << 23; // External reference to object in cluster exists
	pub const CLUSTER_ROOT: u32 = 1 << 24; // Root of a cluster
	pub const NATIVE: u32 = 1 << 25; // Native (UClass only). 
	pub const ASYNC_ONLY: u32 = 1 << 26; // Object exists only on a different thread than the game thread.
	pub const ASYNC_LOADING: u32 = 1 << 27; // Object is being asynchronously loaded.
	pub const UNREACHABLE: u32 = 1 << 28; // Object is not reachable on the object graph.
	pub const PENDING_KILL: u32 = 1 << 29; // Objects that are pending destruction (invalid for gameplay but valid objects)
	pub const ROOT_SET: u32 = 1 << 30; // Object will not be garbage collected, even if unreferenced.
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct UObject {
    // Size: 0x30
    base: UObjectBase
}

impl UObject {
    pub fn virtual_funcs(&self) -> *const *const UnknownType { self.base.vf_table }
    pub fn name(&self) -> FName { self.base.name_private }

    pub fn class(&self) -> &UClass {
        unsafe { self.base.class_private.as_ref::<'static>().unwrap() }
    }

    pub fn outer(&self) -> Option<UObject> {
        if self.base.outer_private != std::ptr::null() {
            Some(unsafe { *self.base.outer_private })
        } else {
            None
        }
    }

    pub fn full_name(&self) -> String {
        let my_name = self.name();
        let my_name_string = my_name.to_string();

        match self.outer() {
            Some(outer) => [outer.full_name(), my_name_string].join("."),
            None => my_name_string
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
struct UObjectBase {
    // Size: 0x30
    pub vf_table: *const *const UnknownType,
    pub obj_flags: u32,
    internal_idx: u32le,
    pub class_private: *const UClass,
    name_private: FName,
    _padding: [u8; 4],
    outer_private: *const UObject
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UObjectVFTable<T> {
    _unknown_a: *const UnknownType,
    _unknown_b: *const UnknownType,
    pub deferred_register: *const fn(this: *const UObject, u_class_static_class: *const UClass, package_name: *const WideChar, in_name: *const WideChar),
    _unknown_c: *const UnknownType,
    pub can_be_in_cluster: *const fn(this: *const UObject) -> bool,
    pub create_cluster: *const fn(this: *const UObject),
    _unknown_d: *const UnknownType,
    pub get_detailed_info_internal: *const fn(this: *const UObject, result: *const FString) -> *const FString,
    _unknown_e: *const UnknownType,
    _unknown_f: *const UnknownType,
    _unknown_g: *const UnknownType,
    _unknown_h: *const UnknownType,
    _unknown_i: *const UnknownType,
    pub modify: *const fn(this: *const UObject, b_always_mark_dirty: bool) -> bool,
    pub post_load: *const fn(this: *const UObject),
    pub post_load_sub_objects: *const fn(this: *const UObject, outer_instance_graph: *const UnknownType),
    pub begin_destroy: *const fn(this: *const UObject),
    _unknown_j: *const UnknownType,
    pub finish_destroy: *const fn(this: *const UObject),
    _unknown_k: *const UnknownType,
    pub serialize_object: *const fn(this: *const UObject, ar: *const UnknownType),
    _unknown_l: *const UnknownType,
    pub post_duplicate: *const fn(this: *const UObject, duplicate_mode: UnknownType),
    _unknown_m: *const UnknownType,
    pub needs_load_for_client: *const fn(this: *const UObject) -> bool,
    pub needs_load_for_server: *const fn(this: *const UObject) -> bool,
    _unknown_n: *const UnknownType,
    _unknown_o: [*const UnknownType; 3],
    pub get_preload_dependencies: *const fn(this: *const UObject, out_deps: TArray<*const UObject, FDefaultAllocator>),
    _unknown_p: [*const UnknownType; 5],
    pub rename_object: *const fn(this: *const UObject, in_name: *const WideChar, new_outer: *const UObject, flags: u32le) -> bool,
    _unknown_q: *const UnknownType,
    pub get_world: *const fn(this: *const UObject) -> *const UWorld,
    _unknown_r: *const UnknownType,
    pub get_resource_size_ex: *const fn(this: *const UObject, cumulative_resource_size: *const UnknownType),
    pub get_exporter_name: *const fn(this: *const UObject, result: *mut FName) -> *const FName,
    _unknown_s: *const UnknownType,
    _unknown_t: *const UnknownType,
    pub get_asset_registry_tags: *const fn(this: *const UObject, out_tags: TArray<UnknownType, FDefaultAllocator>),
    pub is_asset: *const fn(this: *const UObject) -> bool,
    pub get_primary_asset_id: *const fn(this: *const UObject, result: *const UnknownType) -> *const UnknownType,
    pub is_localized_resource: *const fn(this: *const UObject) -> bool,
    pub is_safe_for_root_set: *const fn(this: *const UObject) -> bool,
    pub tag_sub_objects: *const fn(this: *const UnknownType, new_flags: UnknownType),
    _unknown_u: *const UnknownType,
    pub is_name_stable_for_networking: *const fn(this: *const UObject) -> bool,
    pub is_full_name_stable_for_networking: *const fn(this: *const UObject) -> bool,
    pub is_supported_for_networking: *const fn(this: *const UObject) -> bool,
    _unknown_v: [*const UnknownType; 5],
    _build_sub_object_mapping: *const UnknownType,
    _unknown_w: *const UnknownType,
    _unknown_x: *const UnknownType,
    pub process_event: *const fn(this: *const UObject, function: *const UnknownType, params: *const UnknownType),
    _get_max_child_nodes: *const UnknownType,
    _unknown_y: *const UnknownType,
    _process_console_exec: *const UnknownType,
    _unknown_z: [*const UnknownType; 2],
    pub check_default_subobjects_internal: *const fn(this: *const UObject) -> bool,
    pub set_input_text: *const fn(this: *const UConsole, text: *const FString),
    _set_response_state_discovery_channel: *const UnknownType,
    pub console_command: *const fn(cmd: *const FString),
    pub clear_output: *const fn(this: *const UConsole),
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
    pub fake_goto_state: *const fn(this: *const UConsole, next_state_name: FName),
    _phantom: PhantomData<T>
}