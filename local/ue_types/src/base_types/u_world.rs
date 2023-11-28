use std::marker::PhantomData;
use std::ffi::c_void;
use super::{TEnumAsByte, ULevel, UObject, UnknownType};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UWorld<'a> {
    base_object: UObject,
    base_network_notify: FNetworkNotify,
    persistent_level: *const ULevel<'a>,
    net_driver: *const c_void,
    line_batch_comp: *const c_void,
    line_batch_comp_persistent: *const c_void,
    line_batch_comp_foreground: *const c_void,
    network_manager: *const c_void,
    physical_collision_handler: *const c_void,
    extra_referenced_objs: *const c_void,
    streaming_levels: *const c_void,
    streaming_levels_to_consider: *const c_void,
    // Might have these wrong...
    _phantom: PhantomData<&'a u8>
}

impl<'a> UWorld<'a> {
    pub fn persistent_level(&self) -> &'a ULevel<'a> {
        unsafe { self.persistent_level.as_ref::<'a>().unwrap() }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FNetworkNotify {
    _vf_table: *const UnknownType
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FWorldContext<'a> {
    world_type: TEnumAsByte<UnknownType>,
    _unknown: [u8; 0x26F],
    this_current_world: *const UWorld<'a>
}

impl<'a> FWorldContext<'a> {
    pub fn world_type(&self) -> u8 { self.world_type.data() }

    pub fn world(&self) -> &'a UWorld<'a> {
        unsafe { self.this_current_world.as_ref::<'a>().unwrap() }
    }
}