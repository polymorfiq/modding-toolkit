use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FText {
    // Size: 0x18
    pub text_data: TSharedRef<UnknownType, 1>,
    pub flags: u32,
    _padding: [u8; 0x4]
}