use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FSeamlessTravelHandler {
    // Size: 0xA8
    pub pending_travel_url: FUrl,
    pub pending_travel_guid: FGuid,
    pub loaded_package: *const UnknownType,
    pub current_world: *const UWorld,
    pub loaded_world: *const UWorld,
    pub b_transition_in_progress: u8,
    pub b_switched_to_default_map: u8,
    pub b_pause_at_midpoint: u8,
    pub b_need_cancel_cleanup: u8,
    pub world_context_handle: FName,
    pub seamless_travel_start_time: *const UnknownType
}