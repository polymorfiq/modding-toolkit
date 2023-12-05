use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UPlayer<'a> {
    // Size: 0x0048
    pub base_object: UObject,
    pub base_f_exec: FExec,
    pub player_controller: &'a APlayerController<'a>,
    pub current_net_speed: u32,
    pub configured_internet_speed: u32,
    pub configured_lan_speed: u32
}

impl<'a> UPlayer<'a> {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }
}