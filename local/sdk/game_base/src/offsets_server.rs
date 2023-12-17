use crate::offsets::GameOffsets;

// Base Address: 0x7FF6ED770000

pub const fn get_offsets() -> GameOffsets {
    let mut offsets = GameOffsets::empty();

    // Base Game
    offsets.base_structs.gobjects = 0x645FEC8;
    offsets.base_structs.uworld_proxy = 0x6564F28;

    offsets.base_funcs.getnames = 0xF08E80;
    offsets.base_funcs.get_display_name = 0xF08E10;
    offsets.base_funcs.fname_to_string = 0xF0D950;
    offsets.base_funcs.fname_str_constructor = 0xF06740;
    offsets.base_funcs.get_game_mode = 0x1CE9930;

    offsets
}