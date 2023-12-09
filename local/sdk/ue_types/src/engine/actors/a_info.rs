use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct AInfo {
    // Size: 0x0348
    pub base_actor: AActor
}