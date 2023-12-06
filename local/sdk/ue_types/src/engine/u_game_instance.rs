use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UGameInstance<'a> {
    pub base_object: UObject,
    pub base_exec: FExec,
    pub world_context: *const FWorldContext<'a>,
    pub local_players: TArray<*const ULocalPlayer<'a>, FDefaultAllocator>,
    pub online_session: *const UnknownType,
    pub referenced_objects: TArray<*const UObject, FDefaultAllocator>,
    pub notify_pre_client_travel_delegates: TMulticastDelegate<UnknownType, UnknownType>,
    pub on_play_together_event_received_delegate_handle: FDelegateHandle,
    pub pie_map_name: FString,
    pub timer_manager: *const UnknownType,
    pub latent_action_manager: *const UnknownType,
    pub subsystem_collection: FSubsystemCollection<UnknownType>
}

impl<'a> UGameInstance<'a> {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }
    pub fn world_context(&self) -> &'a FWorldContext { unsafe { self.world_context.as_ref::<'a>().unwrap() } }
}