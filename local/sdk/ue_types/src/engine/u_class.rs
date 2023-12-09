use crate::*;
use std::ffi::c_void;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UClass {
    // Size: 0x210
    pub base_struct: UStruct,
    pub class_constructor: *const c_void,
    pub class_vtable_helper_ctor_caller: *const c_void,
    pub class_add_referenced_obj: *const c_void,
    pub class_unique_and_cooked: u32,
    pub class_flags: u32,
    pub class_cast_flags: u64,
    pub class_within: *const UClass,
    pub class_generated_by: *const UObject<UnknownType>,
    pub config_name: FName, // 0xD8
    pub class_reps: TArray<UnknownType, FDefaultAllocator>, // 248
    pub net_fields: TArray<UField, FDefaultAllocator>,
    pub class_default_object: *const UObject<UnknownType>, // 264
    pub func_map: TMap<FName, *const UnknownType>,
    pub super_func_map: TMap<FName, *const UnknownType>,
    _func_map_lock: u64,
    pub interfaces: TArray<UnknownType, FDefaultAllocator>,
    _ref_token_stream: [u8; 0x10],
    _ref_token_stream_critical: [u8; 0x28],
    pub native_func_lookup_table: TArray<UnknownType, FDefaultAllocator>
}

impl UClass {
    pub fn base_struct(&self) -> &UStruct { &self.base_struct }

    pub fn unique(&self) -> u32 {
        self.class_unique_and_cooked >> 1
    }

    pub fn cooked(&self) -> bool {
        self.class_unique_and_cooked & 0x1 > 0
    }

    pub fn default_object<'b>(&self) -> Option<&'b UObject<UnknownType>> {
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

    pub fn generated_by<'b>(&self) -> Option<&'b UObject<UnknownType>> {
        if (self.class_generated_by as *const c_void) != std::ptr::null() {
            Some(unsafe { self.class_generated_by.as_ref::<'b>().unwrap() })
        } else {
            None
        }
    }
}