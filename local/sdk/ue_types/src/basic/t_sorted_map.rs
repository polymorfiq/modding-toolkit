use std::marker::PhantomData;
use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TSortedMap<K, V, Allocator, Indexes> {
    // Size: 0x10
    pub pairs: TArray<(K, V), Allocator>,
    _phantom: PhantomData<Indexes>
}