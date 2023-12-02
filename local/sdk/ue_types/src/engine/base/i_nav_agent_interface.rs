use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct INavAgentInterface {
    pub vftable: *const UnknownType
}