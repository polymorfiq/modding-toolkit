use simple_endian::*;
use std::marker::PhantomData;

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
pub struct UnknownType {}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TEnumAsByte<T> {
    data: u8,
    _phantom_a: PhantomData<T>
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TWeakObjectPtr<T> {
    ptr: u64,
    _phantom: PhantomData<T>
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TSubclassOf<T> {
    ptr: u64,
    _phantom: PhantomData<T>
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TBaseDynamicMulticastDelegate {
    _unknown: [u8; 0x10]
}