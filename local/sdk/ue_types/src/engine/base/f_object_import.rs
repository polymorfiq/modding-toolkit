use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FObjectImport {
    // Size: 0x40
    pub base_resource: FObjectResource,
    pub class_package: FName,
    pub class_name: FName,
    pub x_object: *const UObject<*const UnknownType>,
    pub source_linker: *const UnknownType,
    pub source_index: u32le,
    pub b_import_package_handled: u8,
    pub b_import_searched_for: u8,
    pub b_import_failed: u8,
    _padding: [u8; 0x1]
}