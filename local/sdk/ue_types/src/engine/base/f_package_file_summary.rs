use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C, align(0x8))]
pub struct FPackageFileSummary {
    // Size: 0x118
    pub tag: u32,
    pub file_version_ue4: u32,
    pub file_version_licensee_ue4: u32,
    _padding: [u8; 0x4],
    pub custom_version_container: FCustomVersionContainer,
    pub total_header_size: u32,
    pub package_flags: u32,
    pub folder_name: FString,
    pub name_count: u32,
    pub name_offset: u32,
    pub localization_id: FString,
    pub gatherable_text_data_count: u32,
    pub gatherable_text_data_offset: u32,
    pub export_count: u32,
    pub export_offset: u32,
    pub import_count: u32,
    pub import_offset: u32,
    pub depends_offset: u32,
    pub soft_package_references_count: u32,
    pub soft_package_references_offset: u32,
    pub searchable_names_offset: u32,
    pub thumbnail_table_offset: u32,
    pub guid: FGuid,
    _padding_b: [u8; 0x4],
    pub generations: TArray<FGenerationInfo, FDefaultAllocator>,
    pub saved_by_engine_version: FEngineVersion,
    pub compatible_with_engine_version: FEngineVersion,
    pub compression_flags: u32,
    pub package_source: u32,
    pub b_unversioned: u8,
    _padding_c: [u8; 0x3],
    pub asset_registry_data_offset: u32,
    pub bulk_data_start_offset: u64le,
    pub world_tile_info_data_offset: u32,
    _padding_d: [u8; 0x4],
    pub chunk_ids: TArray<u32, FDefaultAllocator>,
    pub preload_dependency_count: u32,
    pub preload_dependency_offset: u32
}