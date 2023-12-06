use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IInterfaceAssetUserData {
    // Size: 0x8
    pub vftable: *const UnknownType
}