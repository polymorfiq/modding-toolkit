use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct INavAgentInterface<T> {
    pub vftable: INavAgentVFTable<T>
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct INavAgentVFTable<T> {
    addr: *const std::ffi::c_void,
    _phantom: std::marker::PhantomData<T>
}

impl<T> INavAgentVFTable<T> {
    pub fn get_nav_agent_location(&self) -> *const fn(*const INavAgentInterface<T>, *mut FVector) -> *const FVector {
        unsafe { std::mem::transmute(self.addr.byte_offset(0x18)) }
    }
}