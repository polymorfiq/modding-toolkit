use std::marker::PhantomData;
use simple_endian::*;
use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TConstSetBitIterator<Allocator> {
    // Size: 0x20
    pub base: FRelativeBitReference,
    pub array: *const UnknownType,
    pub unvisited_bit_mask: u32,
    pub current_bit_index: u32le,
    pub base_bit_index: u32le,
    _padding: [u8; 4],
    _phantom: PhantomData<Allocator>
}