use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FExec {
    pub vf_table: *const UnknownType
}