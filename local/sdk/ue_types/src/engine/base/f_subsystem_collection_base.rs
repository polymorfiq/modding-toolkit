use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FSubsystemCollectionBase {
    // Size: 0xC8
    pub base_object: FGCObject,
    pub subsystem_map: TMap<UnknownType, UnknownType>,
    pub subsystem_array_map: TMap<UnknownType, UnknownType>,
    pub base_type: TSubclassOf<UnknownType>,
    pub outer: *const UnknownType,
    pub b_populating: u8,
    _padding_a: [u8; 7]
}