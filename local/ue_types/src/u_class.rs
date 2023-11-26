use std::ffi::c_void;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UClass {
    class_constructor: *mut c_void,
    class_vtable_helper_ctor_caller: *mut c_void,
    class_add_referenced_obj: *mut c_void,
    class_unique_and_cooked: u32,
    class_flags: u32,
    class_cast_flags: u64,
    class_within: *mut UClass,
    class_generated_by: *mut c_void,
    // Definitely more here...
}