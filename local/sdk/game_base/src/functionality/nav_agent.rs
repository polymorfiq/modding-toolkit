use ue_types::*;

#[cfg(feature = "client-sdk")]
const OFFSET_APAWN_VFTABLE: isize = 0x5a3f380;

// #[cfg(feature = "client-sdk")]
// const OFFSET_ACHARACTER_VFTABLE: isize = 0x5955a38;

// #[cfg(feature = "client-sdk")]
// const OFFSET_ACONTROLLER_VFTABLE: isize = 0x599c138;

pub trait NavAgent<T> {
    fn vftable(&self) -> *const NavAgentVFTable<T>;

    fn get_nav_agent_location(&self, this: *const INavAgentInterface<T>, result: *mut FVector) -> *const FVector {
        let virtual_func: fn(*const INavAgentInterface<T>, *mut FVector) -> *const FVector = unsafe {
            std::mem::transmute((*self.vftable()).get_nav_agent_location_fn)
        };

        (virtual_func)(this, result)
    }
}

impl NavAgent<APawn> for INavAgentInterface<APawn> {
    fn vftable(&self) -> *const NavAgentVFTable<APawn> {
        crate::GameBase::singleton().at_offset(OFFSET_APAWN_VFTABLE) as *const NavAgentVFTable<APawn>
    }
}

pub trait NavAgentLocatable<T: std::fmt::Debug + 'static> {
    fn nav_agent(&self) -> Option<&'static dyn NavAgent<T>>;
    fn nav_agent_owner(&self) -> Option<*const INavAgentInterface<T>>;

    fn get_nav_agent_location(&self) -> Option<FVector> {
        let nav_agent = self.nav_agent()?;
        println!("NAV AGENT VFTABLE {:?}", nav_agent.vftable());
        
        let nav_agent_owner = self.nav_agent_owner()?;

        let result_buf: Box<FVector> = Box::new(FVector{x: 0f32, y: 0f32, z: 0f32});
        let result = nav_agent.get_nav_agent_location(nav_agent_owner, Box::into_raw(result_buf));
        unsafe { Some(*result) }
    }
}

impl NavAgentLocatable<APawn> for *const APawn {
    fn nav_agent(&self) -> Option<&'static dyn NavAgent<APawn>> { unsafe { Some(&(**self).base_nav_agent as &'static dyn NavAgent<APawn>) } }
    fn nav_agent_owner(&self) -> Option<*const INavAgentInterface<APawn>> { unsafe { Some(self.byte_offset(std::mem::size_of::<AActor>() as isize) as *const INavAgentInterface<APawn>) } }
}