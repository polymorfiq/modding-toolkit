use std::marker::PhantomData;
use crate::{FExec, FName, FWorldContext, TArray, ULocalPlayer, UObject, UnknownType};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UGameInstance<'a> {
    base_object: UObject,
    base_exec: FExec,
    world_context: *const FWorldContext<'a>,
    local_players: TArray<*const ULocalPlayer>,
    online_session: *const UnknownType,
    referenced_objects: TArray<*const UObject>,
    _phantom: PhantomData<&'a u8>,
    // More stuff...
}

impl<'a> UGameInstance<'a> {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }
    pub fn local_players(&self) -> TArray<*const ULocalPlayer> { self.local_players }
    pub fn world_context(&self) -> &'a FWorldContext { unsafe { self.world_context.as_ref::<'a>().unwrap() } }
}