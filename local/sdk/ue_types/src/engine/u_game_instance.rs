use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UGameInstance<'a> {
    pub base_object: UObject,
    pub base_exec: FExec,
    pub world_context: *const FWorldContext<'a>,
    pub local_players: TArray<*const ULocalPlayer<'a>>,
    pub online_session: *const UnknownType,
    pub referenced_objects: TArray<*const UObject>,
    _phantom: PhantomData<&'a u8>,
    // More stuff...
}

impl<'a> UGameInstance<'a> {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }
    pub fn world_context(&self) -> &'a FWorldContext { unsafe { self.world_context.as_ref::<'a>().unwrap() } }
}