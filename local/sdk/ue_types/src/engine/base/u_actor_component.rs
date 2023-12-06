use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct UActorComponent<'a> {
    // Size: 0x100
    base_object: UObject,
    base_asset_data: IInterfaceAssetUserData,
    primary_component_tick: FActorTickFunction<'a>,
    component_tags: TArray<FName, FDefaultAllocator>,
    asset_user_data: TArray<*const UnknownType, FDefaultAllocator>,
    _unknown: [u8; 0x50]
}