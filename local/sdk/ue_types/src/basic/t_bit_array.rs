use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TBitArray<Allocator> {
    // Size: 0x20
    pub allocator_instance: Allocator,
    pub num_bits: u32le,
    pub max_bits: u32le
}