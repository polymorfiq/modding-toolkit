use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct INavAgentInterface<T> {
    pub vftable: *const NavAgentVFTable<T>,
    _phantom: PhantomData<T>
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct NavAgentVFTable<T> {
    _unknown_a: *const UnknownType,
    _unknown_b: *const UnknownType,
    _unknown_c: *const UnknownType,
    pub get_nav_agent_location_fn: *const fn(*const INavAgentInterface<T>, *mut FVector) -> *const FVector
}