use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ULocalPlayer {
    pub base_player: UPlayer,
    pub cached_unique_net_id: FUniqueNetIdRepl,
    pub viewport_client: *const UnknownType,
    pub origin: FVector2D,
    pub size: FVector2D,
    pub last_view_location: FVector,
    pub aspect_ratio_axis_constraint: TEnumAsByte<UnknownType>,
    // Some more stuff...
}

impl ULocalPlayer {}