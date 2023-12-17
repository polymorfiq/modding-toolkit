use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AController {
    // Size: 0x03A8
    pub base_actor: AActor,
    pub base_nav_agent: INavAgentInterface<AController>,
    pub player_state: *const APlayerState,
    pub start_spot: TWeakObjectPtr<AActor>,
    pub on_instigated_any_damage: TBaseDynamicMulticastDelegate,
    pub state_name: FName,
    _padding_a: [u8; 4],
    pub pawn: *const APawn,
    pub old_pawn: TWeakObjectPtr<APawn>,
    pub character: *const ACharacter,
    pub transform_component: *const UnknownType,
    _on_new_pawn: [u8; 0x18],
    pub control_rotation: FRotator,
    _bf_3c4: u8,
    pub ignore_move_input: u8,
    pub ignore_look_input: u8,
    _padding_b: u8
}

impl AController {
    pub fn character(&self) -> Option<&ACharacter> {
        unsafe { self.character.as_ref() }
    }
}