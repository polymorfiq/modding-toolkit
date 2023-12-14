use ue_types::*;
use crate::data::game_name::VirtualGameName;

pub struct AssetId(*const FPrimaryAssetId);

impl AssetId {
    pub fn new(data: *const FPrimaryAssetId) -> Self { Self(data) }
    pub fn asset_id(&self) -> &FPrimaryAssetId { unsafe { self.0.as_ref().unwrap() } }
}

impl std::fmt::Debug for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.asset_id();

        f.debug_struct("AssetId")
         .field("primary_asset_type", &id.primary_asset_type.name.to_string())
         .field("primary_asset_name", &id.primary_asset_name.to_string())
         .finish()
    }
}