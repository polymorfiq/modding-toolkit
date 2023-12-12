use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FAssetData {
    // Size: 0x68
    pub object_path: FName,
    pub package_data: FName,
    pub package_path: FName,
    pub asset_name: FName,
    pub asset_class: FName,
    _padding: [u8; 0x4],
    pub tags_and_values: FAssetDataTagMapSharedView,
    pub chunk_ids: TArray<u32, FDefaultAllocator>,
    pub package_flags: u32le,
    _padding_b: [u8; 0x4]
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FAssetDataTagMapSharedView {
    pub map: TSharedPtr<TSortedMap<FName, FString, FDefaultAllocator, UnknownType>>
}