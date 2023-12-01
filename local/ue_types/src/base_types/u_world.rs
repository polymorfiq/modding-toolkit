use simple_endian::*;
use super::{
    AController,
    APlayerController,
    FGuid,
    FIntVector,
    FName,
    FString,
    FURL,
    TArray,
    TEnumAsByte,
    TWeakObjectPtr,
    UGameInstance,
    ULevel,
    UObject,
    UnknownType
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UWorld<'a> {
    base_object: UObject,
    base_network_notify: FNetworkNotify,
    persistent_level: *const ULevel<'a>,
    net_driver: *const UnknownType,
    line_batch_comp: *const UnknownType,
    line_batch_comp_persistent: *const UnknownType,
    line_batch_comp_foreground: *const UnknownType,
    network_manager: *const UnknownType,
    physical_collision_handler: *const UnknownType,
    extra_referenced_objs: TArray<*const UObject>,
    per_module_data_objs: TArray<*const UObject>,
    streaming_levels: TArray<*const UnknownType>,
    streaming_levels_to_consider: [u8; 0x28],
    streaming_levels_prefix: FString,
    current_levels_pending_visibility: TArray<UnknownType>,
    current_level_pending_visibility: *const UnknownType,
    demo_net_driver: *const UnknownType,
    my_particle_event_manager: *const UnknownType,
    default_physics_volume: *const UnknownType,
    view_locations_rendered_last_frame: TArray<UnknownType>,
    feature_level: TEnumAsByte<UnknownType>,
    tick_group: TEnumAsByte<UnknownType>,
    world_type: TEnumAsByte<UnknownType>,
    _b_something: u8,
    _b_something_b: u32le,
    navigation_system: *const UnknownType,
    authority_game_mode: *const UnknownType,
    game_state: *const UnknownType,
    ai_system: *const UnknownType,
    avoidance_manager: *const UnknownType,
    levels: TArray<*const ULevel<'a>>,
    level_collections: TArray<UnknownType>,
    active_level_collection_idx: u32le,
    audio_device_handle: u32le,
    owning_game_instance: *const UGameInstance<'a>,
    parameter_collection_instances: TArray<UnknownType>,
    canvas_for_rendering_to_target: *const UnknownType,
    canvas_for_draw_material_to_render_target: *const UnknownType,
    scene: *const UnknownType,
    controller_list: TArray<TWeakObjectPtr<AController>>,
    player_controller_list: TArray<TWeakObjectPtr<APlayerController>>,
    pawn_list: TArray<UnknownType>,
    auto_camera_actor_list: TArray<UnknownType>,
    non_default_physics_volume_list: TArray<UnknownType>,
    physics_scene: *const UnknownType,
    components_that_need_end_of_frame_update: TArray<UnknownType>,
    components_that_need_end_of_frame_update_on_game_thread: TArray<UnknownType>,
    async_trace_state: [u8; 0xC8],
    on_actor_spawned: [u8; 0x18],
    timer_manager: *const UnknownType,
    latent_action_manager: [u8; 0x60],
    build_streaming_data_timer: *const UnknownType,
    tick_dispatch_event: [u8; 0x18],
    post_tick_dispatch_event: [u8; 0x18],
    tick_flush_event: [u8; 0x18],
    post_tick_flush_event: [u8; 0x18],
    levels_changed_event: [u8; 0x18],
    url: FURL,
    fx_tick_system: *const UnknownType,
    tick_last_event: *const UnknownType,
    start_physics_tick_function: [u8; 0x58],
    end_physics_tick_function: [u8; 0x58],
    player_num: u32le,
    streaming_volume_update_delay: u32le,
    post_process_volumes: TArray<UnknownType>,
    exponential_fog_volumes: TArray<UnknownType>,
    audio_volumes: TArray<UnknownType>,
    last_time_unbuilt_light_was_encountered: *const UnknownType,
    time_seconds: u32le,
    unpaused_time_seconds: u32le,
    real_time_seconds: u32le,
    audio_time_seconds: u32le,
    delta_time_seconds: u32le,
    pause_delay: u32le,
    origin_location: FIntVector,
    requested_origin_location: FIntVector,
    origin_offset_this_frame: [u8; 0x3],
    next_switch_countdown: u32le,
    world_composition: *const UnknownType,
    flush_level_streaming_type: *const UnknownType,
    next_travel_type: TEnumAsByte<UnknownType>,
    _unknown_a: [u8; 6],
    next_url: FString,
    preparing_level_names: TArray<FName>,
    committed_persistent_level_name: FName,
    num_lighting_unbuilt_objects: u32le,
    num_unbuilt_reflection_captures: u32le,
    num_texture_streaming_unbuilt_components: u32le,
    num_texture_streaming_dirty_resources: u32le,
    _unknown_b: [u8; 4],
    perf_trackers: *const UnknownType,
    on_actors_initialized: [u8; 0x18],
    game_state_set_event: [u8; 0x18],
    psc_pool: [u8; 0x58],
    subsystem_collection: [u8; 0xC8]
}

impl<'a> UWorld<'a> {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }
    pub fn url(&self) -> FURL { self.url }
    pub fn owning_game_instance(&self) -> Option<&UGameInstance> {
        if self.owning_game_instance != std::ptr::null() {
            Some(unsafe { self.owning_game_instance.as_ref::<'a>().unwrap() })
        } else {
            None
        }
    }

    pub fn persistent_level(&self) -> &'a ULevel<'a> {
        unsafe { self.persistent_level.as_ref::<'a>().unwrap() }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FNetworkNotify {
    _vf_table: *const UnknownType
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FWorldContext<'a> {
    // Size: 0x278
    world_type: TEnumAsByte<UnknownType>,
    _padding_a: [u8; 7],
    seamless_travel_handler: FSeamlessTravelHandler<'a>,
    context_handle: FName,
    _padding_b: [u8; 4],
    travel_url: FString,
    travel_type: u8,
    _padding_c: [u8; 7],
    last_url: FURL,
    last_remote_url: FURL,
    pending_net_game: *const UnknownType,
    packages_to_fully_load: TArray<UnknownType>,
    levels_to_load_for_pending_map_change: TArray<FName>,
    loaded_levels_for_pending_map_change: TArray<*const ULevel<'a>>,
    pending_map_change_failure_description: FString,
    _b_a: u8,
    _padding_d: [u8; 7],
    object_referencers: TArray<UnknownType>,
    pending_level_streaming_status_updates: TArray<UnknownType>,
    game_viewport: *const UnknownType,
    owning_game_instance: *const UGameInstance<'a>,
    active_net_driver: TArray<UnknownType>,
    pie_instance: u32le,
    _padding_e: [u8; 4],
    pie_prefix: FString,
    pie_world_feature_level: u32le,
    run_as_dedicated: u8,
    b_waiting_on_online_subsytem: u8,
    _padding_f: [u8; 2],
    audio_device_handle: u32le,
    _padding_g: [u8; 4],
    external_references: TArray<*const UWorld<'a>>,
    this_current_world: *const UWorld<'a>
}

impl<'a> FWorldContext<'a> {
    pub fn world_type(&self) -> u8 { self.world_type.data() }

    pub fn world(&self) -> &'a UWorld<'a> {
        unsafe { self.this_current_world.as_ref::<'a>().unwrap() }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FSeamlessTravelHandler<'a> {
    // Size: 0xA8
    pending_travel_url: FURL,
    pending_travel_guid: FGuid,
    loaded_package: *const UnknownType,
    current_world: *const UWorld<'a>,
    loaded_world: *const UWorld<'a>,
    b_transition_in_progress: u8,
    b_switched_to_default_map: u8,
    b_pause_at_midpoint: u8,
    b_need_cancel_cleanup: u8,
    world_context_handle: FName,
    seamless_travel_start_time: *const UnknownType
}