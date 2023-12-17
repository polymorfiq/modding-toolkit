use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct AGameMode {
    // Size: 0x438
    pub base: AGameModeBase,
    pub match_state: FName,
    pub b_delayed_start: u8,
    _padding_a: [u8; 0x3],
    pub num_specators: u32le,
    pub num_players: u32le,
    pub num_bots: u32le,
    pub min_respawn_delay: f32le,
    pub num_travelling_players: u32le,
    _padding_b: [u8; 0x4],
    pub engine_message_class: TSubclassOf<UnknownType>,
    pub inactive_player_array: TArray<*const APlayerState, FDefaultAllocator>,
    pub inactive_player_state_lifespan: f32le,
    pub max_inactive_players: u32le,
    pub b_handle_dedicated_server_replays: u8,
    _padding: [u8; 0x7]
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct AXGameMode {
    pub base_game_mode: AGameModeBase,
    pub mode_tag: FName,
    pub b_start_after_min_players_join: u8,
    _unknown: [u8; 0x7],
    pub match_start_conditions: TArray<UnknownType, FDefaultAllocator>,
    pub max_human_players: u32le,
    pub waiting_to_start_duration: f32le,
    _unknown_b: [u8; 0x4],
    pub num_bots_to_spawn: u32le,
    pub min_players_include_bots: u8,
    pub b_spawn_with_weapon_if_unarmed: u8,
    _unknown_c: [u8; 0x6],
    pub unarmed_loot_row: FDataTableRowHandle,
    pub loot_respawn_seconds_override: f32le,
    pub loot_respawn_jitter_override: f32le,
    pub bot_controller_class: *const UClass,
    pub fall_damage_type_class: *const UClass,
    pub cheat_component_class: *const UClass,
    pub bot_manager: *const UnknownType,
    pub game_time_map: TMap<UnknownType, f32le>,
    _unknown_d: [u8; 0x8]
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
    pub hud_class: TSubclassOf<UnknownType>,
    pub default_pawn_class: TSubclassOf<APawn>,
    pub spectator_class: TSubclassOf<UnknownType>,
    pub replay_spectator_player_controller_class: TSubclassOf<APlayerController>,
    pub server_stat_replicator_class: TSubclassOf<AServerStatReplicator>,
    pub game_session: *const AGameSession,
    pub game_state: *const AXGameState,
    pub server_stat_replicator: *const AServerStatReplicator,
    pub default_player_name: FText,
    _bf_3d0: u8,
    _padding_3d1: [u8; 7],
    pub pausers: TArray<UnknownType, FDefaultAllocator>
}