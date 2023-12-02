use crate::*;
use simple_endian::u32le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FUObjectArray {
    // Size: 0x130
    pub obj_first_gc_idx: u32le,
    pub obj_last_non_gc_idx: u32le,
    pub max_objects_not_considered_by_gc: u32le,
    pub open_for_disregard_for_gc: u8,
    _padding: [u8; 3],
    pub objects_array: FChunkedFixedUObjectArray,
    various_data: [u8; 0x100] // 0x30
}