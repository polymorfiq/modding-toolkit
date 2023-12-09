use ue_types::*;

#[derive(Debug, Copy, Clone)]
pub struct NavAgent<T>(*const INavAgentInterface<T>);

impl<T> NavAgent<T> {
    pub fn new(agent: *const INavAgentInterface<T>) -> Self {
        Self(agent)
    }
}

pub trait VirtualNavAgent<T> {
    fn nav_agent(&self) -> NavAgent<T>;

    fn nav_virtual_funcs(&self) -> &INavAgentVFTable<T> { unsafe { &(*self.nav_agent().0).vftable } }

    fn get_world_location(&self) -> FVector {
        let result_buf: Box<FVector> = Box::new(FVector{x: 0f32, y: 0f32, z: 0f32});
        println!("self.nav_virtual_funcs().get_nav_agent_location() - {:p}", self.nav_virtual_funcs().get_nav_agent_location());

        unsafe {
            let pos = (*self.nav_virtual_funcs().get_nav_agent_location())(self.nav_agent().0, Box::into_raw(result_buf));
            *pos
        }
    }
}

impl VirtualNavAgent<APawn> for APawn {
    fn nav_agent(&self) -> NavAgent<APawn> {
        NavAgent::new(std::ptr::addr_of!(self.base_nav_agent))
    }
}