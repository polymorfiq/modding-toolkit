use crate::*;
use simple_endian::u32le;

#[derive(Debug, Clone)]
#[repr(C, align(0x8))]
pub struct UWorld<'a> {
    pub base_object: UObject,
    pub base_network_notify: FNetworkNotify,
    pub persistent_level: *const ULevel<'a>,
    pub net_driver: *const UnknownType,
    pub line_batch_comp: *const UnknownType,
    pub line_batch_comp_persistent: *const UnknownType,
    pub line_batch_comp_foreground: *const UnknownType,
    pub network_manager: *const UnknownType,
    pub physical_collision_handler: *const UnknownType,
    pub extra_referenced_objs: TArray<*const UObject, FDefaultAllocator>,
    pub per_module_data_objs: TArray<*const UObject, FDefaultAllocator>,
    pub streaming_levels: TArray<*const UnknownType, FDefaultAllocator>,
    pub streaming_levels_to_consider: [u8; 0x28],
    pub streaming_levels_prefix: FString,
    pub current_levels_pending_visibility: TArray<UnknownType, FDefaultAllocator>,
    pub current_level_pending_visibility: *const UnknownType,
    pub demo_net_driver: *const UnknownType,
    pub my_particle_event_manager: *const UnknownType,
    pub default_physics_volume: *const UnknownType,
    pub view_locations_rendered_last_frame: TArray<UnknownType, FDefaultAllocator>,
    pub feature_level: TEnumAsByte<UnknownType>,
    pub tick_group: TEnumAsByte<UnknownType>,
    pub world_type: TEnumAsByte<UnknownType>,
    _b_something: u8,
    _b_something_b: u32le,
    pub navigation_system: *const UnknownType,
    pub authority_game_mode: *const UnknownType,
    pub game_state: *const UnknownType,
    pub ai_system: *const UnknownType,
    pub avoidance_manager: *const UnknownType,
    pub levels: TArray<*const ULevel<'a>, FDefaultAllocator>,
    pub level_collections: TArray<UnknownType, FDefaultAllocator>,
    pub active_level_collection_idx: u32le,
    pub audio_device_handle: u32le,
    pub owning_game_instance: *const UGameInstance<'a>,
    pub parameter_collection_instances: TArray<UnknownType, FDefaultAllocator>,
    pub canvas_for_rendering_to_target: *const UnknownType,
    pub canvas_for_draw_material_to_render_target: *const UnknownType,
    pub scene: *const UnknownType,
    pub controller_list: TArray<TWeakObjectPtr<AController<'a>>, FDefaultAllocator>,
    pub player_controller_list: TArray<TWeakObjectPtr<APlayerController<'a>>, FDefaultAllocator>,
    pub pawn_list: TArray<UnknownType, FDefaultAllocator>,
    pub auto_camera_actor_list: TArray<UnknownType, FDefaultAllocator>,
    pub non_default_physics_volume_list: TArray<UnknownType, FDefaultAllocator>,
    pub physics_scene: *const UnknownType,
    pub components_that_need_end_of_frame_update: TArray<UnknownType, FDefaultAllocator>,
    pub components_that_need_end_of_frame_update_on_game_thread: TArray<UnknownType, FDefaultAllocator>,
    pub async_trace_state: [u8; 0xC8],
    pub on_actor_spawned: [u8; 0x18],
    pub timer_manager: *const UnknownType,
    pub latent_action_manager: [u8; 0x60],
    pub build_streaming_data_timer: *const UnknownType,
    pub tick_dispatch_event: [u8; 0x18],
    pub post_tick_dispatch_event: [u8; 0x18],
    pub tick_flush_event: [u8; 0x18],
    pub post_tick_flush_event: [u8; 0x18],
    pub levels_changed_event: [u8; 0x18],
    pub url: FUrl,
    pub fx_tick_system: *const UnknownType,
    pub tick_last_event: *const UnknownType,
    pub start_physics_tick_function: [u8; 0x58],
    pub end_physics_tick_function: [u8; 0x58],
    pub player_num: u32le,
    pub streaming_volume_update_delay: u32le,
    pub post_process_volumes: TArray<UnknownType, FDefaultAllocator>,
    pub exponential_fog_volumes: TArray<UnknownType, FDefaultAllocator>,
    pub audio_volumes: TArray<UnknownType, FDefaultAllocator>,
    pub last_time_unbuilt_light_was_encountered: *const UnknownType,
    pub time_seconds: u32le,
    pub unpaused_time_seconds: u32le,
    pub real_time_seconds: u32le,
    pub audio_time_seconds: u32le,
    pub delta_time_seconds: u32le,
    pub pause_delay: u32le,
    pub origin_location: FIntVector,
    pub requested_origin_location: FIntVector,
    pub origin_offset_this_frame: [u8; 0x3],
    pub next_switch_countdown: u32le,
    pub world_composition: *const UnknownType,
    pub flush_level_streaming_type: *const UnknownType,
    pub next_travel_type: TEnumAsByte<UnknownType>,
    _unknown_a: [u8; 6],
    pub next_url: FString,
    pub preparing_level_names: TArray<FName, FDefaultAllocator>,
    pub committed_persistent_level_name: FName,
    pub num_lighting_unbuilt_objects: u32le,
    pub num_unbuilt_reflection_captures: u32le,
    pub num_texture_streaming_unbuilt_components: u32le,
    pub num_texture_streaming_dirty_resources: u32le,
    _unknown_b: [u8; 4],
    pub perf_trackers: *const UnknownType,
    pub on_actors_initialized: TMulticastDelegate<UnknownType, UnknownType>,
    pub game_state_set_event: FOnGameStateSetEvent,
    pub psc_pool: FWorldPSCPool,
    pub subsystem_collection: FSubsystemCollection<UnknownType>
}

impl<'a> UWorld<'a> {
    pub fn level(&self) -> Option<&'a ULevel<'a>> {
        if self.persistent_level != std::ptr::null() {
            Some(unsafe { self.persistent_level.as_ref::<'a>().unwrap() })
        } else {
            None
        }
    }
    
    pub fn object(&self) -> UObject { self.base_object }
    // pub fn name(&self) -> FName { self.object().name() }
    // pub fn full_name(&self) -> String { self.object().full_name() }
    // pub fn url(&self) -> FUrl { self.url }
    // pub fn owning_game_instance(&self) -> Option<&UGameInstance> {
    //     if self.owning_game_instance != std::ptr::null() {
    //         Some(unsafe { self.owning_game_instance.as_ref::<'a>().unwrap() })
    //     } else {
    //         None
    //     }
    // }
}

#[derive(Debug, Clone)]
#[repr(C, align(0x8))]
pub struct FOnGameStateSetEvent {
    base: TBaseMulticastDelegate<UnknownType, UnknownType>
}

#[derive(Debug, Clone)]
#[repr(C, align(0x8))]
pub struct FWorldPSCPool {
    // Size: 0x50
    pub world_particle_system_pools: TMap<UnknownType, UnknownType>,
    pub last_particle_system_pool_clean_time: u32le,
    pub cached_world_time: u32le
}