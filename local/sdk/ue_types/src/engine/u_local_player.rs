use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ULocalPlayer<'a> {
    pub base_player: UPlayer<'a>,
    pub cached_unique_net_id: FUniqueNetIdRepl,
    pub viewport_client: *const UnknownType,
    pub origin: FVector2D,
    pub size: FVector2D,
    pub last_view_location: FVector,
    pub aspect_ratio_axis_constraint: TEnumAsByte<UnknownType>,
    // Some more stuff...
}

impl<'a> ULocalPlayer<'a> {
    pub fn player(&self) -> UPlayer { self.base_player }
    pub fn object(&self) -> UObject { self.player().object() }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }
}