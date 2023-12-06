use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FDelegateHandle {
    pub id: *const UnknownType
}