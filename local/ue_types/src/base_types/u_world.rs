use std::marker::PhantomData;
use std::ffi::c_void;
use super::ULevel;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UWorld<'a> {
    level: *const ULevel<'a>,
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
    pub fn level(&self) -> &'a ULevel<'a> {
        unsafe { self.level.as_ref::<'a>().unwrap() }
    }
}