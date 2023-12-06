use simple_endian::*;
use std::marker::PhantomData;
use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TSparseArray<T, AlignedBytes, ArrayAllocator, BitArrayAllocator> {
    // Size: 0x28
    pub data: TArray<TSparseArrayElementOrFreeListLink<AlignedBytes>, ArrayAllocator>,
    pub allocation_flags: TBitArray<BitArrayAllocator>,
    pub first_free_index: u32le,
    pub num_free_indices: u32le,
    _phantom: PhantomData<T>
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TSparseArrayBaseIterator<T, AlignedBytes, ArrayAllocator, BitArrayAllocator> {
    // Size: 0x28
    pub array: *const TSparseArray<T, AlignedBytes, ArrayAllocator, BitArrayAllocator>,
    pub bit_array_it: TConstSetBitIterator<BitArrayAllocator>
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TSparseArrayElementOrFreeListLink<AlignedBytes> {
    bytes: AlignedBytes
}