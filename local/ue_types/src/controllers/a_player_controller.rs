use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct APlayerController {
    // Size: 0x0678
    pub base_controller: AController,
    pub player: *const UnknownType,
    pub acknowledge_pawn: *const UnknownType,
    pub controlling_dir_track_inst: *const UnknownType,
    pub my_hud: *const UnknownType,
    pub player_camera_manager: *const UnknownType,
    pub player_camera_manager_class: TSubclassOf<UnknownType>,
    pub b_auto_manage_active_camera_target: u8,
    // More Stuff...
}