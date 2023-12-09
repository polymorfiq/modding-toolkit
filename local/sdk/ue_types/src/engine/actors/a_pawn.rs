use simple_endian::*;
use crate::*;

#[derive(Debug, Clone)]
#[repr(C, align(0x8))]
pub struct APawn {
    // Size: 0x3A8
    pub base_actor: AActor,
    pub base_nav_agent: INavAgentInterface<APawn>,
    _bf_350: u8,
    _padding_a: [u8; 3],
    pub base_eye_height: u32le,
    pub auto_assess_player: TEnumAsByte<UnknownType>,
    pub auto_possess_ai: TEnumAsByte<UnknownType>,
    pub remote_view_pitch: u8,
    _padding_b: [u8; 5],
    pub ai_controller_class: TSubclassOf<UnknownType>,
    pub player_state: *const UnknownType,
    pub blended_replay_view_pitch: u32le,
    _padding_c: [u8; 4],
    pub last_hit_by: *const UnknownType,
    pub controller: *const AController,
    pub allowed_yaw_error: u32le,
    pub control_input_vector: FVector,
    pub last_control_input_vector: FVector,
    _padding_d: [u8; 4]
}