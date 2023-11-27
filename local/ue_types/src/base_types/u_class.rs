use std::ffi::c_void;
use crate::{FName, TArray, TMap, UField, UObject, UStruct, UnknownType};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UClass {
    // Size: 0x210
    base_struct: UStruct,
    class_constructor: *const c_void,
    class_vtable_helper_ctor_caller: *const c_void,
    class_add_referenced_obj: *const c_void,
    class_unique_and_cooked: u32,
    class_flags: u32,
    class_cast_flags: u64,
    class_within: *const UClass,
    class_generated_by: *const UObject,
    pub config_name: FName, // 0xD8
    class_reps: TArray<UnknownType>, // 248
    net_fields: TArray<UField>,
    class_default_object: *const UObject, // 264
    func_map: TMap<FName, *const UnknownType>,
    super_func_map: TMap<FName, *const UnknownType>,
    _func_map_lock: u64,
    interfaces: TArray<UnknownType>,
    _ref_token_stream: [u8; 0x10],
    _ref_token_stream_critical: [u8; 0x28],
    native_func_lookup_table: TArray<UnknownType>
}

impl UClass {
    pub fn base_struct(&self) -> UStruct { self.base_struct }
    pub fn name(&self) -> FName { self.base_struct().field().object().name() }
    pub fn full_name(&self) -> String { self.base_struct().field().object().full_name() }

    pub fn unique(&self) -> u32 {
        self.class_unique_and_cooked >> 1
    }

    pub fn cooked(&self) -> bool {
        self.class_unique_and_cooked & 0x1 > 0
    }

    pub fn default_object<'b>(&self) -> Option<&'b UObject> {
        if (self.class_default_object as *const c_void) != std::ptr::null() {
            Some(unsafe { self.class_default_object.as_ref::<'b>().unwrap() })
        } else {
            None
        }
    }

    pub fn within<'b>(&self) -> Option<&'b UClass> {
        if (self.class_within as *const c_void) != std::ptr::null() {
            Some(unsafe { self.class_within.as_ref::<'b>().unwrap() })
        } else {
            None
        }
    }

    pub fn generated_by<'b>(&self) -> Option<&'b UObject> {
        if (self.class_generated_by as *const c_void) != std::ptr::null() {
            Some(unsafe { self.class_generated_by.as_ref::<'b>().unwrap() })
        } else {
            None
        }
    }
}