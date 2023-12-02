use crate::*;
use simple_endian::u32le;

const GOBJECTS_NUM_ELEMS_PER_CHUNK: usize = 64 * 1024;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FChunkedFixedUObjectArray {
    // Size: 0x20
    pub objects: *const *const FUObjectItem,
    pub pre_allocated_objects: *const FUObjectItem,
    pub max_elements: u32le,
    pub num_elements: u32le,
    pub max_chunks: u32le,
    pub num_chunks: u32le,
}

impl FChunkedFixedUObjectArray {
    pub fn item_at_idx(&self, idx: usize) -> Option<FUObjectItem> {
		let chunk_idx = idx / GOBJECTS_NUM_ELEMS_PER_CHUNK;
        let elem_idx = idx % (GOBJECTS_NUM_ELEMS_PER_CHUNK - 1);

        if idx >= self.max_elements.to_native() as usize { return None; }
        if chunk_idx >= (self.num_chunks.to_native() as usize) { return None; }
        if elem_idx >= (self.num_elements.to_native() as usize) { return None; }

        let chunk_ptr = unsafe { self.objects.offset(chunk_idx as isize) };
        if (chunk_ptr as *const c_void) == std::ptr::null() { return None; }
        let chunk = unsafe { *chunk_ptr };

        let elem_ptr = unsafe { chunk.offset(elem_idx as isize) };
        if (elem_ptr as *const c_void) == std::ptr::null() { return None; }

        unsafe { Some(*elem_ptr) }
    }
}