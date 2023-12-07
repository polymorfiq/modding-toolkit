use crate::*;
use simple_endian::u32le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FWorldContext {
    // Size: 0x278
    pub world_type: TEnumAsByte<UnknownType>,
    _padding_a: [u8; 7],
    pub seamless_travel_handler: FSeamlessTravelHandler,
    pub context_handle: FName,
    _padding_b: [u8; 4],
    pub travel_url: FString,
    pub travel_type: u8,
    _padding_c: [u8; 7],
    pub last_url: FUrl,
    pub last_remote_url: FUrl,
    pub pending_net_game: *const UnknownType,
    pub packages_to_fully_load: TArray<UnknownType, FDefaultAllocator>,
    pub levels_to_load_for_pending_map_change: TArray<FName, FDefaultAllocator>,
    pub loaded_levels_for_pending_map_change: TArray<*const ULevel, FDefaultAllocator>,
    pub pending_map_change_failure_description: FString,
    _b_a: u8,
    _padding_d: [u8; 7],
    pub object_referencers: TArray<UnknownType, FDefaultAllocator>,
    pub pending_level_streaming_status_updates: TArray<UnknownType, FDefaultAllocator>,
    pub game_viewport: *const UnknownType,
    pub owning_game_instance: *const UGameInstance,
    pub active_net_driver: TArray<UnknownType, FDefaultAllocator>,
    pub pie_instance: u32le,
    _padding_e: [u8; 4],
    pub pie_prefix: FString,
    pub pie_world_feature_level: u32le,
    pub run_as_dedicated: u8,
    pub b_waiting_on_online_subsytem: u8,
    _padding_f: [u8; 2],
    pub audio_device_handle: u32le,
    _padding_g: [u8; 4],
    pub external_references: TArray<*const UWorld, FDefaultAllocator>,
    pub this_current_world: *const UWorld
}

impl FWorldContext {
    pub fn world_type(&self) -> u8 { self.world_type.data() }
    pub fn context_handle(&self) -> FName { self.context_handle }

    pub fn world(&self) -> &UWorld {
        unsafe { self.this_current_world.as_ref().unwrap() }
    }
}