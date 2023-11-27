use crate::{APlayerController, TEnumAsByte, UnknownType, UObject};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UPlayer {
    // Size: 0x0048
    base_object: UObject,
    base_f_exec: FExec,
    player_controller: *const APlayerController,
    current_net_speed: u32,
    configured_internet_speed: u32,
    configured_lan_speed: u32
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ULocalPlayer {
    player_base: UPlayer,
    cached_unique_net_id: FUniqueNetIdRepl,
    viewport_client: *const UnknownType,
    origin: FVector2D,
    size: FVector2D,
    last_view_location: FVector,
    aspect_ratio_axis_constraint: TEnumAsByte<UnknownType>,
    // Some more stuff...
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FExec {
    _unknown: [u8; 0x8]
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FUniqueNetIdRepl {
    _unknown: [u8; 0x28]
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FVector {
    _unknown: [u8; 0xC]
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FVector2D {
    _unknown: [u8; 0x8]
}