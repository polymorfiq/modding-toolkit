use ue_types::*;
use crate::data::game_name::VirtualGameName;

pub struct AssetData(*const FAssetData);

impl AssetData {
    pub fn new(data: *const FAssetData) -> Self { Self(data) }
    pub fn asset_data(&self) -> &FAssetData { unsafe { self.0.as_ref().unwrap() } }
}

impl std::fmt::Debug for AssetData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.asset_data();

        f.debug_struct("AssetData")
         .field("object_path", &data.object_path.to_string())
         .field("package_data", &data.package_data.to_string())
         .field("package_path", &data.package_path.to_string())
         .field("asset_name", &data.asset_name.to_string())
         .field("asset_class", &data.asset_class.to_string())
         .field("tags_and_values", &"???".to_string())
         .field("chunk_ids", &"???".to_string())
         .finish()
    }
}