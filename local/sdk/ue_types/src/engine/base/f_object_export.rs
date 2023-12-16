use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FObjectExport {
    // Size: 0x88
    pub base_resource: FObjectResource,
    pub class_index: FPackageIndex,
    pub this_index: FPackageIndex,
    pub super_index: FPackageIndex,
    pub template_index: FPackageIndex,
    pub object_flags: u32le,
    _padding: [u8; 0x4],
    pub serial_size: u64le,
    pub serial_offset: u64le,
    pub script_serialization_start_offset: u64le,
    pub script_serialization_end_offset: u64le,
    pub object: *const UObject<*const UnknownType>,
    pub hash_next: u32le,
    pub b_forced_export: u8,
    pub b_not_for_client: u8,
    pub b_not_for_server: u8,
    pub b_not_always_loaded_for_editor_game: u8,
    pub b_is_asset: u8,
    pub b_export_load_failed: u8,
    pub dynamic_type: u8,
    pub b_was_filtered: u8,
    pub package_guid: FGuid,
    pub package_flags: u32le,
    pub first_export_dependency: u32le,
    pub serialization_before_serialization_dependencies: u32le,
    pub create_before_serialization_dependencies: u32le,
    pub serialization_before_create_dependencies: u32le,
    pub create_before_create_dependencies: u32le,
    _padding_b: [u8; 0x4]
}