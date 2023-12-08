use ue_types::*;

#[cfg(feature = "client-sdk")]

pub trait VirtualUObject<T> {
    fn vftable(&self) -> *const UObjectVFTable<T>;

    fn get_nav_agent_location(&self, this: *const INavAgentInterface<T>, result: *mut FVector) -> *const FVector {
        let virtual_func: fn(*const INavAgentInterface<T>, *mut FVector) -> *const FVector = unsafe {
            std::mem::transmute((*self.vftable()).get_nav_agent_location_fn)
        };

        (virtual_func)(this, result)
    }
}