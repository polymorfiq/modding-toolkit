use std::marker::PhantomData;
use crate::{AActor, FName, FURL, TArray, UObject, UnknownType, UWorld};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ULevel<'a> {
    // 0x288
    base_object: UObject,
    base_asset_user_data: IInterfaceAssetUserData,
    url: FURL,
    actors: TArray<*const AActor>,
    actors_for_gc: TArray<*const AActor>,
    owning_world: *const UWorld<'a>,
    _other_stuff: [u8; 0x1C0],
    _phantom: PhantomData<&'a u8>
}

impl<'a> ULevel<'a> {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn actors(&self) -> TArray<*const AActor> { self.actors }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IInterfaceAssetUserData {
    _vftable: *const UnknownType
}