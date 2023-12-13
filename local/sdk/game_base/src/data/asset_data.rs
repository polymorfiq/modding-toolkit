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

        let tag_value_pairs = unsafe {
            (*data.tags_and_values.map.object).pairs
        };

        let mut tag_values: Vec<(String, String)> = vec![];
        for i in 0..tag_value_pairs.len() {
            let (k, v) = tag_value_pairs.at_index(i).unwrap();
            tag_values.push((k.to_string(), v.to_string()));
        }

        let mut chunk_ids: Vec<u32> = vec![];
        for i in 0..data.chunk_ids.len() {
            let chunk_id = data.chunk_ids.at_index(i).unwrap();
            chunk_ids.push(*chunk_id);
        }


        f.debug_struct("AssetData")
         .field("object_path", &data.object_path.to_string())
         .field("package_data", &data.package_data.to_string())
         .field("package_path", &data.package_path.to_string())
         .field("asset_name", &data.asset_name.to_string())
         .field("asset_class", &data.asset_class.to_string())
         .field("tags_and_values", &tag_values)
         .field("chunk_ids", &chunk_ids)
         .finish()
    }
}