use crate::offsets::GameOffsets;

pub const fn get_offsets() -> GameOffsets {
    let mut offsets = GameOffsets::empty();

    // Base Game
    offsets.base_structs.gobjects = 0x753EC50;
    offsets.base_structs.uworld_proxy = 0x7678520;

    offsets.base_funcs.getnames = 0x10E94B0;
    offsets.base_funcs.get_display_name = 0x10E9440;
    offsets.base_funcs.fname_to_string = 0x10EE300;
    offsets.base_funcs.fname_str_constructor = 0x10E6C00;

    // VF Tables
    offsets.vf_tables.console_uobject = 0xBF21450;
    offsets.vf_tables.apawn_inavagent = 0x5a3f380;
    offsets.vf_tables.acharacter_inavagent = 0x5955a38;
    offsets.vf_tables.acontroller_inavagent = 0x599c138;

    // Asset management
    offsets.asset_funcs.asset_registry_add_asset_data = 0x1598CE0;
    offsets.asset_funcs.asset_manager_is_non_pak_filename_allowed = 0x1FFD9A0;
    offsets.asset_funcs.asset_manager_scan_path_for_primary_assets = 0x213F600;
    offsets.asset_funcs.object_library_load_assets_at_path = 0x2479280;
    offsets.asset_funcs.fpakfile_find = 0x1ff9800;

    offsets
}