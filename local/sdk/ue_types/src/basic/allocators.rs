use elain::{Align, Alignment};
use std::marker::PhantomData;
use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FHeapAllocatorForAnyElementType {
    // Size: 0x8
    pub data: *const UnknownType
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TInlineAllocator<const NUM: usize, T, const MIN_ALIGNMENT: usize, SecondaryAllocator>
where
    Align<MIN_ALIGNMENT>: Alignment
{
    // Size: sizeof::<T>() * NUM + sizeof::<SecondaryAllocator>()
    pub inline_data: [T; NUM],
    pub secondary_data: SecondaryAllocator,
    _align: Align<MIN_ALIGNMENT>
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TSparseArrayAllocator<Allocator, BitAllocator> {
    _phantom: PhantomData<(Allocator, BitAllocator)>
}

pub type FDefaultAllocator = FHeapAllocatorForAnyElementType;
pub type FDefaultBitArrayAllocator = TInlineAllocator<4, u32, 0x8, FDefaultAllocator>;
pub type FDefaultSparseArrayAllocator = TSparseArrayAllocator<FDefaultAllocator, FDefaultBitArrayAllocator>;