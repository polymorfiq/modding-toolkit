use crate::*;

#[derive(Debug, Clone)]
#[repr(C, align(0x8))]
pub struct UGameInstance {
    pub base_object: UObject<*const UnknownType>,
    pub base_exec: FExec,
    pub world_context: *const FWorldContext,
    pub local_players: TArray<*const ULocalPlayer, FDefaultAllocator>,
    pub online_session: *const UnknownType,
    pub referenced_objects: TArray<*const UObject<*const UnknownType>, FDefaultAllocator>,
    pub notify_pre_client_travel_delegates: TMulticastDelegate<UnknownType, UnknownType>,
    pub on_play_together_event_received_delegate_handle: FDelegateHandle,
    pub pie_map_name: FString,
    pub timer_manager: *const UnknownType,
    pub latent_action_manager: *const UnknownType,
    pub subsystem_collection: FSubsystemCollection<UnknownType>
}

impl UGameInstance {
    pub fn world_context(&self) -> &FWorldContext { unsafe { self.world_context.as_ref().unwrap() } }
}