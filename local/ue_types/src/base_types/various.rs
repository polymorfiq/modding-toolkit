use simple_endian::*;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FStructBaseChain {
    // Size: 0x4
    num_struct_bases_in_chain_minus_one: u32le
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TArray<T> {
    // Size: 0x10
    allocator_instance: *const T,
    array_num: u32le,
    array_max: u32le
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TMap<K, V> {
    // Size: 0x50
    _something: [u8; 0x50],
    _phantom_a: PhantomData<K>,
    _phantom_b: PhantomData<V>,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UProperty {}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UnknownType {}