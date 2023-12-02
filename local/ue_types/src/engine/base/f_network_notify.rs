use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FNetworkNotify {
    pub vf_table: *const UnknownType
}