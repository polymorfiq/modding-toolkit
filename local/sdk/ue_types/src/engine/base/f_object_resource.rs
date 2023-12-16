use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FObjectResource {
    pub object_name: FName,
    pub outer_index: FPackageIndex
}