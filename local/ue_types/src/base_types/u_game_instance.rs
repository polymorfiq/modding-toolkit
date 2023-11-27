use std::marker::PhantomData;
use crate::{TArray, ULocalPlayer, UObject, UnknownType};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UGameInstance<'a> {
    base_object: UObject,
    world_context: *const UnknownType,
    local_players: TArray<*const ULocalPlayer>,
    online_session: *const UnknownType,
    referenced_objects: TArray<*const UObject>,
    _phantom: PhantomData<&'a u8>,
    // More stuff...
}

impl<'a> UGameInstance<'a> {
}