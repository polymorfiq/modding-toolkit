use crate::offsets::GameOffsets;

pub const fn get_offsets() -> GameOffsets {
    let mut offsets = GameOffsets::empty();

    // Base Game
    offsets.base_structs.gobjects = 0x645FEC8;

    offsets.base_funcs.getnames = 0xF08E80;
    offsets.base_funcs.get_display_name = 0xF08E10;

    offsets
}