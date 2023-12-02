use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IInterfaceAssetUserData {
    pub vftable: *const UnknownType
}