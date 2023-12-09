use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UPlayer {
    // Size: 0x0048
    pub base_object: UObject<UnknownType>,
    pub base_f_exec: FExec,
    pub player_controller: *const APlayerController,
    pub current_net_speed: u32,
    pub configured_internet_speed: u32,
    pub configured_lan_speed: u32
}

impl UPlayer {}