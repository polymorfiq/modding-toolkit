use std::marker::PhantomData;
use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ULevel<'a> {
    // 0x288
    pub base_object: UObject,
    pub base_asset_user_data: IInterfaceAssetUserData,
    pub url: FUrl,
    pub actors: TArray<*const AActor>,
    pub actors_for_gc: TArray<*const AActor>,
    pub owning_world: *const UWorld<'a>,
    _other_stuff: [u8; 0x1C0],
    _phantom: PhantomData<&'a u8>
}

impl<'a> ULevel<'a> {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn actors(&self) -> TArray<*const AActor> { self.actors }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }
    pub fn owning_world(&self) -> &UWorld { unsafe { self.owning_world.as_ref::<'a>().unwrap() } }
}