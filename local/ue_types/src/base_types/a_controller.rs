use crate::{AActor, FName, TBaseDynamicMulticastDelegate, TSubclassOf, TWeakObjectPtr, UnknownType};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AController {
    // Size: 0x03A8
    base_actor: AActor,
    base_nav_agent_interface: INavAgentInterface,
    player_state: *const UnknownType,
    start_spot: TWeakObjectPtr<AActor>,
    on_instigated_any_damage: TBaseDynamicMulticastDelegate,
    state_name: FName,
    pawn: *const UnknownType,
    character: *const UnknownType,
    transform_component: *const UnknownType,
    _unknown: [u8; 0x18],
    control_rotation: FRotator,
    flags: [u8; 5]
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct APlayerController {
    // Size: 0x0678
    base_controller: AController,
    player: *const UnknownType,
    acknowledge_pawn: *const UnknownType,
    controlling_dir_track_inst: *const UnknownType,
    my_hud: *const UnknownType,
    player_camera_manager: *const UnknownType,
    player_camera_manager_class: TSubclassOf<UnknownType>,
    b_auto_manage_active_camera_target: u8,
    // More Stuff...
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct INavAgentInterface {
    vftable: *const UnknownType
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FRotator {
    data: [u8; 0xC]
}