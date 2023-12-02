use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AController {
    // Size: 0x03A8
    pub base_actor: AActor,
    pub base_nav_agent_interface: INavAgentInterface,
    pub player_state: *const UnknownType,
    pub start_spot: TWeakObjectPtr<AActor>,
    pub on_instigated_any_damage: TBaseDynamicMulticastDelegate,
    pub state_name: FName,
    pub pawn: *const UnknownType,
    pub character: *const UnknownType,
    pub transform_component: *const UnknownType,
    _unknown: [u8; 0x18],
    pub control_rotation: FRotator,
    pub flags: [u8; 5]
}