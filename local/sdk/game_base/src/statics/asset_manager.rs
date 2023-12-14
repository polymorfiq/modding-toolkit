use ue_types::*;
use crate::*;

pub struct AssetManager {
    pub addr: *const UnknownType
}

static mut MANAGER_ADDR: Option<*const UnknownType> = None;

impl AssetManager {
    pub fn get() -> Option<Self> {
        let known_addr = unsafe { MANAGER_ADDR };
        
        match known_addr {
            Some(addr) => Some(Self{addr: addr}),
            None => {
                let manager = GObjects::find_first(|obj| {
                    let class = obj.class();
                    class.is_some() && class.unwrap().full_name() == "/Script/g3.GAssetManager"
                });

                if manager.is_none() { return None };
                unsafe {
                    MANAGER_ADDR = Some(manager.unwrap() as *const UnknownType);
                    Some(Self{addr: MANAGER_ADDR.unwrap()})
                }
            }
        }
    }

    pub fn scan_path_for_primary_assets(&self, primary_asset_type: FPrimaryAssetType, path: *const FString, base_class: *const UClass, has_blueprint_classes: bool, b_is_editor_only: bool, b_force_synchronous_scan: bool) -> u32 {
        let scan_fn: fn(*const UnknownType, FPrimaryAssetType, *const FString, *const UClass, bool, bool, bool) -> u32 = unsafe {
            std::mem::transmute(GameBase::singleton().at_offset(offsets::OFFSET_FUNC_ASSET_MANAGER_SCAN_PATH_FOR_PRIMARY_ASSETS))
        };

        (scan_fn)(self.addr, primary_asset_type, path, base_class, has_blueprint_classes, b_is_editor_only, b_force_synchronous_scan)
    }
}