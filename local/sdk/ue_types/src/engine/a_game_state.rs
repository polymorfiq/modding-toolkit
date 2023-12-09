use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct AGameStateBase {
    pub base_ainfo: AInfo,
    pub game_mode_class: *const UClass,
    pub authority_game_mode: *const AGameModeBase,
    pub spectator_class: *const UClass,
    pub player_array: TArray<*const APlayerState, FDefaultAllocator>,
    pub b_replicated_has_begun_play: u8,
    _padding: [u8; 0x3],
    pub replicated_world_time_seconds: f32le,
    pub server_world_time_seconds_delta: f32le,
    pub server_world_time_seconds_update_frequency: f32le,
    _unknown: [u8; 0x8]
}