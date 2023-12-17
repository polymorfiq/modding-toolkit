use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct AXGameState {
    pub base_game_state: AGameStateBase,
    pub match_phases: TArray<FName, FDefaultAllocator>,
    pub on_any_player_damaged: [u8; 0x10],
    pub on_any_player_killed: [u8; 0x10],
    pub on_phase_changed: [u8; 0x10],
    pub on_countdown_freeze_time_changed: [u8; 0x10],
    pub on_match_start_time_change: [u8; 0x10],
    pub on_any_chest_open: [u8; 0x10],
    pub b_should_start: u8,
    pub _unknown: [u8; 0x3],
    pub num_bots_to_spawn: u32le,
    pub countdown_freeze_time: f32le,
    pub match_start_time: f32le,
    pub match_phase: FName
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct AGameStateBase {
    pub base_ainfo: AInfo,
    pub game_mode_class: *const UClass,
    pub authority_game_mode: *const AXGameMode,
    pub spectator_class: *const UClass,
    pub player_array: TArray<*const APlayerState, FDefaultAllocator>,
    pub b_replicated_has_begun_play: u8,
    _padding: [u8; 0x3],
    pub replicated_world_time_seconds: f32le,
    pub server_world_time_seconds_delta: f32le,
    pub server_world_time_seconds_update_frequency: f32le,
    _unknown: [u8; 0x8]
}