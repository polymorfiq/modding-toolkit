use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FPackageFileSummary {
    // Size: 0x118
    pub tag: u32le,
    pub file_version_ue4: u32le,
    pub file_version_licensee_ue4: u32le,
    _padding: [u8; 0x4],
    pub custom_version_container: FCustomVersionContainer,
    pub total_header_size: u32le,
    pub package_flags: u32le,
    pub folder_name: FString,
    pub name_count: u32le,
    pub name_offset: u32le,
    pub localization_id: FString,
    pub gatherable_text_data_count: u32le,
    pub gatherable_text_data_offset: u32le,
    pub export_count: u32le,
    pub export_offset: u32le,
    pub import_count: u32le,
    pub import_offset: u32le,
    pub depends_offset: u32le,
    pub soft_package_references_count: u32le,
    pub soft_package_references_offset: u32le,
    pub searchable_names_offset: u32le,
    pub thumbnail_table_offset: u32le,
    pub guid: FGuid,
    _padding_b: [u8; 0x4],
    pub generations: TArray<FGenerationInfo, FDefaultAllocator>,
    pub saved_by_engine_version: FEngineVersion,
    pub compatible_with_engine_version: FEngineVersion,
    pub compression_flags: u32le,
    pub package_source: u32le,
    pub b_unversioned: u8,
    _padding_c: [u8; 0x3],
    pub asset_registry_data_offset: u32le,
    pub bulk_data_start_offset: u64le,
    pub world_tile_info_data_offset: u32le,
    _padding_d: [u8; 0x4],
    pub chunk_ids: TArray<u32, FDefaultAllocator>,
    pub preload_dependency_count: u32le,
    pub preload_dependency_offset: u32le
}