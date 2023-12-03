use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct ForceFeedbackValues {
    // Size: 0x10
    left_large: u32le,
    left_small: u32le,
    right_large: u32le,
    right_small: u32le
}