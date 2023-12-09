use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct AGameSession {
    base_ainfo: AInfo,
    max_spectators: u32le,
    max_players: u32le,
    max_party_size: u32le,
    max_split_screens_per_connection: u32le,
    b_required_push_to_talk: u8,
    _padding: [u8; 0x2],
    session_name: FName
}