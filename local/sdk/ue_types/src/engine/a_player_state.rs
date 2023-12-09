use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct APlayerState {
    pub base_ainfo: AInfo,
    pub score: f32le,
    _unknown_a: [u8; 0x4],
    pub player_name: FString,
    _unknown_b: [u8; 0x10],
    pub player_id: u32le,
    pub ping: u8,
    _unknown_c: u8,
    pub b_should_update_replicated_ping: u8,
    pub b_is_spectator: u8,
    pub b_only_spectator: u8,
    pub b_is_a_bot: u8,
    _unknown_d: u8,
    pub b_is_inactive: u8,
    pub b_from_previous_level: u8,
    _unknown_e: u8,
    pub start_time: u32le,
    _unknown_f: [u8; 0x4],
    pub engine_message_class: *const UClass,
    _unknown_g: [u8; 0x8],
    pub saved_network_address: FString,
    pub unique_id: FUniqueNetIdRepl,
    _unknown_h: [u8; 0x8],
    pub pawn_private: *const APawn,
    _unknown_i: [u8; 0x78],
    pub player_name_private: FString,
    _uknown_j: [u8; 0x10]
}