use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct AGameMode {
    // Size: 0x438
    pub base: AGameModeBase,
    pub match_state: FName,
    pub b_delayed_start: u8,
    _unknown_a: [u8; 0x3],
    pub num_specators: u32le,
    pub num_players: u32le,
    pub num_bots: u32le,
    pub min_respawn_delay: f32le,
    pub num_travelling_players: u32le,
    pub engine_message_class: *const UClass,
    pub inactive_player_array: TArray<*const APlayerState, FDefaultAllocator>,
    pub inactive_player_state_lifespan: f32le,
    pub max_inactive_players: u32le,
    pub b_handle_dedicated_server_replays: u8,
    _padding: [u8; 0x7]
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct AGameModeBase {
    // Size: 0x3E8
    pub base_ainfo: AInfo,
    pub options_string: FString,
    pub game_session_class: TSubclassOf<AGameSession>,
    pub game_state_class: TSubclassOf<AGameStateBase>,
    pub player_controller_class: TSubclassOf<APlayerController>,
    pub player_state_class: TSubclassOf<APlayerState>,
    pub hud_class: *const UClass,
    pub default_pawn_class: TSubclassOf<APawn>,
    pub spectator_class: *const UClass,
    pub replay_spectator_player_controller_class: TSubclassOf<APlayerController>,
    pub server_stat_replicator_class: TSubclassOf<AServerStatReplicator>,
    pub game_session: *const AGameSession,
    pub game_state: *const AGameStateBase,
    pub server_stat_replicator: *const AServerStatReplicator,
    pub default_player_name: FText,
    _bf_3d0: u8,
    _padding_3d1: [u8; 7],
    pub pausers: TArray<UnknownType, FDefaultAllocator>
}