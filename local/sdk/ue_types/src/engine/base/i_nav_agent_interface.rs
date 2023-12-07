use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct INavAgentInterface<T> {
    pub vftable: *const NavAgentVFTable<T>
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct NavAgentVFTable<T> {
    _unknown_a: *const UnknownType,
    _unknown_b: *const UnknownType,
    _unknown_c: *const UnknownType,
    pub get_nav_agent_location_fn: *const fn(*const T, *mut FVector) -> *const FVector
}