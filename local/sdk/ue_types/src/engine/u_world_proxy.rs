use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UWorldProxy {
    pub world: *const UWorld
}