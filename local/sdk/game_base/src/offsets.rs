#[derive(Copy, Clone, Debug, Default)]
pub struct GameOffsets {
    pub base_structs: BaseStructOffsets,
    pub base_funcs: BaseFuncOffsets,
    pub vf_tables: VFTableOffsets,
    pub asset_funcs: AssetFuncOffsets
}

#[derive(Copy, Clone, Debug, Default)]
pub struct BaseStructOffsets {
    pub gobjects: isize,
    pub uworld_proxy: isize,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct BaseFuncOffsets {
    pub getnames: isize,
    pub get_display_name: isize,
    pub fname_to_string: isize,
    pub fname_str_constructor: isize,
    pub get_game_mode: isize
}

#[derive(Copy, Clone, Debug, Default)]
pub struct VFTableOffsets {
    pub console_uobject: isize,
    pub apawn_inavagent: isize,
    pub acharacter_inavagent: isize,
    pub acontroller_inavagent: isize
}

#[derive(Copy, Clone, Debug, Default)]
pub struct AssetFuncOffsets {
    pub asset_registry_add_asset_data: isize,
    pub asset_manager_is_non_pak_filename_allowed: isize,
    pub asset_manager_scan_path_for_primary_assets: isize,
    pub object_library_load_assets_at_path: isize,
    pub fpakfile_find: isize,
    pub fpakfile_setup_signed_pak_reader: isize,
    pub fpakplatformfile_mount: isize,
    pub fpackagename_register_mount_point: isize,
}

impl GameOffsets {
    pub const fn empty() -> Self {
        Self {
            base_structs: BaseStructOffsets {
                gobjects: 0x0,
                uworld_proxy: 0x0
            },
            base_funcs: BaseFuncOffsets {
                getnames: 0x0,
                get_display_name: 0x0,
                fname_to_string: 0x0,
                fname_str_constructor: 0x0,
                get_game_mode: 0x0
            },
            vf_tables: VFTableOffsets {
                console_uobject: 0x0,
                apawn_inavagent: 0x0,
                acharacter_inavagent: 0x0,
                acontroller_inavagent: 0x0
            },
            asset_funcs: AssetFuncOffsets {
                asset_registry_add_asset_data: 0x0,
                asset_manager_is_non_pak_filename_allowed: 0x0,
                asset_manager_scan_path_for_primary_assets: 0x0,
                object_library_load_assets_at_path: 0x0,
                fpakfile_find: 0x0,
                fpakfile_setup_signed_pak_reader: 0x0,
                fpakplatformfile_mount: 0x0,
                fpackagename_register_mount_point: 0x0
            },
        }
    }
}