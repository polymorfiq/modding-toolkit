use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AController<'a> {
    // Size: 0x03A8
    pub base_actor: AActor<'a>,
    pub base_nav_agent_interface: INavAgentInterface,
    pub player_state: *const UnknownType,
    pub start_spot: TWeakObjectPtr<AActor<'a>>,
    pub on_instigated_any_damage: TBaseDynamicMulticastDelegate,
    pub state_name: FName,
    _padding_a: [u8; 4],
    pub pawn: Option<&'a APawn<'a>>,
    pub old_pawn: TWeakObjectPtr<APawn<'a>>,
    pub character: *const UnknownType,
    pub transform_component: *const UnknownType,
    _on_new_pawn: [u8; 0x18],
    pub control_rotation: FRotator,
    _bf_3c4: u8,
    pub ignore_move_input: u8,
    pub ignore_look_input: u8,
    _padding_b: u8
}