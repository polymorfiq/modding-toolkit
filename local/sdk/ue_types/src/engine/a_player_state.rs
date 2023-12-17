use crate::*;
use simple_endian::*;
use bitfield_struct::bitfield;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct APlayerState {
    pub base_ainfo: AInfo,
    pub score: f32le,
    _unknown_a: [u8; 0x4],
    pub player_name: FString,
    pub old_name: FString,
    pub player_id: u32le,
    pub ping: u8,
    pub curr_ping_bucket: u8,
    pub flags: PlayerStateFlags,
    pub start_time: u32le,
    _unknown_f: [u8; 0x4],
    pub engine_message_class: TSubclassOf<UnknownType>,
    pub exact_ping: f32le,
    pub exact_ping_v2: f32le
}

#[bitfield(u8, order = Lsb)]
pub struct PlayerStateFlags {
    pub b_should_update_replicated_ping: bool,
    pub b_is_spectator: bool,
    pub b_only_spectator: bool,
    pub b_is_a_bot: bool,
    pub b_has_been_welcomed: bool,
    pub b_is_inactive: bool,
    pub b_from_previous_level: bool,
    pub b_use_custom_player_names: bool
}