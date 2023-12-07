use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct UActorComponent {
    // Size: 0x100
    pub base_object: UObject,
    pub base_asset_data: IInterfaceAssetUserData,
    pub primary_component_tick: FActorTickFunction,
    pub component_tags: TArray<FName, FDefaultAllocator>,
    pub asset_user_data: TArray<*const UnknownType, FDefaultAllocator>,
    _unknown: [u8; 0x50]
}