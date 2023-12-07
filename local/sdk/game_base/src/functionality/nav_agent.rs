use ue_types::*;

#[cfg(feature = "client-sdk")]
const OFFSET_APAWN_VFTABLE: isize = 0x5a3f380;

pub trait NavAgent<T> {
    fn vftable(&self) -> &NavAgentVFTable<T>;

    fn get_nav_agent_location(&self, this: *const T, result: *mut FVector) -> *const FVector {
        unsafe { (*self.vftable().get_nav_agent_location_fn)(this, result) }
    }
}

pub trait NavAgentLocatable<T: std::fmt::Debug> {
    fn nav_agent(&self) -> Option<&dyn NavAgent<T>>;
    fn nav_agent_owner(&self) -> Option<*const T>;

    fn get_nav_agent_location(&self) -> Option<FVector> {
        println!("NAV AGENT VFTABLE {:?}", self.nav_agent()?.vftable());
        
        let nav_agent = self.nav_agent()?;
        let nav_agent_owner = self.nav_agent_owner()?;

        let mut result_buf: FVector = FVector{x: 0f32, y: 0f32, z: 0f32};
        println!("Owner {:?}", nav_agent_owner);
        println!("Owner {:p}", nav_agent_owner);
        println!("result_buf {:p}", std::ptr::addr_of_mut!(result_buf));
        nav_agent.get_nav_agent_location(nav_agent_owner, std::ptr::addr_of_mut!(result_buf));
        Some(result_buf)
    }
}

impl NavAgent<APawn> for INavAgentInterface<APawn> {
    fn vftable(&self) -> &NavAgentVFTable<APawn> {
        let vftable = crate::GameBase::singleton().at_offset(OFFSET_APAWN_VFTABLE) as *const NavAgentVFTable<APawn>;
        unsafe { vftable.as_ref().unwrap() }
    }
}

impl NavAgentLocatable<APawn> for APawn {
    fn nav_agent(&self) -> Option<&dyn NavAgent<APawn>> { Some(&self.base_nav_agent as &dyn NavAgent<APawn>) }
    fn nav_agent_owner(&self) -> Option<*const APawn> { Some(std::ptr::addr_of!(*self)) }
}