use simple_endian::*;
use std::marker::PhantomData;
use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ULevel {
    // 0x2B0
    pub base_object: UObject<UnknownType>,
    pub base_asset_user_data: IInterfaceAssetUserData,
    pub url: FUrl,
    pub actors: TArray<*const AActor, FDefaultAllocator>,
    pub actors_for_gc: TArray<*const AActor, FDefaultAllocator>,
    pub owning_world: *const UWorld,
    pub model: *const UnknownType,
    pub model_components: TArray<UnknownType, FDefaultAllocator>,
    pub actor_cluster: *const UnknownType,
    pub num_texture_streaming_unbuilt_components: u32le,
    pub num_texture_streaming_dirty_resources: u32le,
    pub level_script_actor: *const UnknownType,
    pub nav_list_start: *const UnknownType,
    pub nav_list_end: *const UnknownType,
    pub nav_data_chunks: TArray<UnknownType, FDefaultAllocator>,
    pub lightmap_total_size: f32le,
    pub shadowmap_total_size: f32le,
    pub static_navigable_geometry: TArray<FVector, FDefaultAllocator>,
    pub streaming_texture_guids: TArray<FGuid, FDefaultAllocator>,
    pub tick_task_level: *const UnknownType,
    pub pre_computed_light_volume: *const UnknownType,
    pub pre_computed_volumetric_light_map: *const UnknownType,
    pub pre_computed_visibility_handler: FPrecomputedVisibilityHandler,
    pub pre_computed_volume_distance_field: FPrecomputedVolumeDistanceField,
    pub remove_from_scene_fence: FRenderCommandFence,
    pub level_build_data_id: FGuid,
    pub map_build_data: *const UnknownType,
    pub light_build_level_offset: FIntVector,
    pub b_is_lighting_scenario: u8,
    pub b_are_components_currently_registered: u8,
    pub b_geometry_dirty_for_lighting: u8,
    pub b_texture_streaming_rotation_changed: u8,
    pub b_static_components_registered_in_streaming_manager: u8,
    pub b_is_visible: u8,
    pub b_already_moved_actors: u8,
    pub b_already_shifted_actors: u8,
    pub b_already_updated_components: u8,
    pub b_already_associated_streamable_resources: u8,
    pub b_already_initialized_network_actors: u8,
    pub b_already_cleared_actors_seamless_travel_flag: u8,
    pub b_already_routed_actor_initialize: u8,
    pub b_already_sorted_actor_list: u8,
    pub b_is_associating_level: u8,
    pub b_require_full_visibility_to_render: u8,
    pub b_client_only_visible: u8,
    pub b_was_duplicate_for_pie: u8,
    pub b_is_being_removed: u8,
    pub b_has_rerun_construction_scripts: u8,
    pub b_actor_cluster_created: u8,
    pub b_has_current_actor_called_pre_register: u8,
    pub current_actor_index_for_update_components: u32le,
    pub current_actor_index_for_unregister_components: u32le,
    pub on_apply_level_transform: TMulticastDelegate<UnknownType, *const FTransform>,
    pub level_bounds_actor: TWeakObjectPtr<UnknownType>,
    pub instance_foliage_actor: TWeakObjectPtr<UnknownType>,
    pub level_bounds_updated_actor_event: TBaseMulticastDelegate<UnknownType, UnknownType>,
    pub world_settings: *const UnknownType,
    pub cached_level_collection: *const UnknownType,
    pub asset_user_data: TArray<UnknownType, FDefaultAllocator>,
    pub pending_auto_receive_input_actors: TArray<UnknownType, FDefaultAllocator>,
    pub destroyed_replicated_static_actors: TArray<UnknownType, FDefaultAllocator>,
    _phantom: PhantomData<u8>
}

impl ULevel {
    pub fn actors(&self) -> TArray<*const AActor, FDefaultAllocator> { self.actors }
    pub fn owning_world(&self) -> &UWorld { unsafe { self.owning_world.as_ref().unwrap() } }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FPrecomputedVisibilityHandler {
    pub volume_max_distance: f32le,
    pub volume_box: FBox,
    pub volume_size_x: u32le,
    pub volume_size_y: u32le,
    pub volume_size_z: u32le,
    pub data: TArray<FColor, FDefaultAllocator>
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FPrecomputedVolumeDistanceField {
    pub volume_max_distance: f32le,
    pub volume_box: FBox,
    pub volume_size_x: u32le,
    pub volume_size_y: u32le,
    pub volume_size_z: u32le,
    pub data: TArray<FColor, FDefaultAllocator>
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FRenderCommandFence {
    pub completion_event: TRefCountPtr<UnknownType>,
    pub trigger_thread_index: TEnumAsByte<UnknownType>
}

