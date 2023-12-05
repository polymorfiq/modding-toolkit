use simple_endian::*;
use crate::*;

#[derive(Debug, Clone)]
#[repr(C, align(0x8))]
pub struct APlayerController<'a> {
    // Size: 0x0698
    pub base_controller: AController<'a>,
    pub player: *const UPlayer<'a>,
    pub acknowledge_pawn: *const UnknownType,
    pub controlling_dir_track_inst: *const UnknownType,
    pub my_hud: *const UnknownType,
    pub player_camera_manager: *const UnknownType,
    pub player_camera_manager_class: TSubclassOf<UnknownType>,
    pub b_auto_manage_active_camera_target: u8, // 0x3F8,
    _padding_a: [u8; 3],
    pub target_view_rotation: FRotator,
    pub blended_target_view_rotation: FRotator,
    pub smooth_target_view_rotation_speed: u32le,
    pub local_player_cached_lod_distance_factor: u32le,
    _padding_b: [u8; 4],
    pub hidden_actors: TArray<&'a AActor<'a>>,
    pub hidden_primitive_components: TArray<UnknownType>,
    pub b_render_primitive_components: u8,
    _padding_c: [u8; 3],
    pub last_spectator_state_sync_time: u32le,
    pub last_spectator_sync_location: FVector,
    pub last_spectator_sync_rotation: FRotator,
    pub client_cap: u32le,
    _padding_d: [u8; 4],
    pub cheat_manager: *const UnknownType,
    pub cheat_class: TSubclassOf<UnknownType>,
    pub player_input: *const UnknownType,
    pub active_force_feedback_effects: TArray<UnknownType>,
    pub _dynamic_force_feedbacks: TSortedMap<UnknownType, UnknownType>,
    pub _latent_dynamic_force_feedbacks: TSortedMap<UnknownType, UnknownType>,
    pub active_haptic_effect_left: TSharedPtr<UnknownType>,
    pub active_haptic_effect_right: TSharedPtr<UnknownType>,
    pub active_haptic_effect_gun: TSharedPtr<UnknownType>,
    pub _force_feedback_values: ForceFeedbackValues,
    pub pending_map_change_level_names: TArray<FName>,
    _bf_500: u8,
    _padding_e: [u8; 3],
    pub net_player_idx: u8,
    _padding_f: [u8; 3],
    _mute_list: [u8; 0x38],
    pub pending_swap_connection: *const UnknownType,
    pub net_connection: *const UnknownType,
    pub rotation_input: FRotator,
    pub input_yaw_scale: u32le,
    pub input_pitch_scale: u32le,
    pub input_roll_scale: u32le,
    _bf_568: u8,
    _padding_g: [u8; 0x3],
    pub force_feedback_scale: u32le,
    pub click_event_keys: TArray<UnknownType>,
    pub default_mouse_cursor: TEnumAsByte<UnknownType>,
    pub current_mouse_cursor: TEnumAsByte<UnknownType>,
    pub default_click_trace_channel: TEnumAsByte<UnknownType>,
    pub current_click_trace_channel: TEnumAsByte<UnknownType>,
    pub hit_result_trace_distance: u32le,
    pub seamless_travel_count: u16le,
    pub last_completed_seamless_travel_count: u16le,
    pub current_clickable_primitives: TWeakObjectPtr<UnknownType>,
    pub _current_touchable_primitives: [u8; 0x58],
    _padding_h: [u8; 4],
    pub current_input_stack: TArray<UnknownType>,
    pub inactive_state_input_component: *const UnknownType,
    _bf_608: u8,
    _padding_i: [u8; 7],
    pub virtual_joystick: TSharedPtr<UnknownType>,
    pub current_touch_interface: *const UnknownType,
    _timer_handle_unfreeze: FTimerHandle,
    _timer_handle_delayed_prepare_map_change: FTimerHandle,
    _time_handle_client_commit_map_change: FTimerHandle,
    _bf_640: u8,
    _padding_j: [u8; 3],
    pub audio_listener_component: TWeakObjectPtr<USceneComponent<'a>>,
    pub audio_listener_attenuation_component: TWeakObjectPtr<USceneComponent<'a>>,
    pub audio_listener_location_override: FVector,
    pub audio_listener_rotation_override: FRotator,
    pub audio_listener_attenuation_override: FVector,
    pub spectator_pawn: Option<&'a APawn<'a>>,
    pub last_retry_player_time: u32le,
    pub b_is_local_player_controller: u8,
    _padding_k: [u8; 3],
    pub spawn_location: FVector,
    _bf_694: u8,
    _padding_l: [u8; 3]
}

impl APlayerController<'_> {
    pub fn get_nav_agent_location(&self) -> FVector {
        let vf_table = self.base_controller.base_nav_agent_interface.vftable;
        let get_nav_agent_location: fn(*const AController, *mut FVector) = unsafe {
            std::mem::transmute(*vf_table.byte_offset(0x18))
        };

        let mut result: FVector = FVector{x: 0f32, y: 0f32, z: 0f32};
        unsafe { 
            (get_nav_agent_location)(std::ptr::addr_of!(self.base_controller).byte_offset(std::mem::size_of::<AActor>() as isize), std::ptr::addr_of_mut!(result));
        }
        
        result
    }
}