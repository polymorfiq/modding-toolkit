use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FOutputDevice {
    // Size: 0x10
    pub __vf_table: *const UnknownType,
    pub b_suppress_event_tags: u8,
    pub b_auto_emit_line_terminator: u8,
    _padding: [u8; 6]
}