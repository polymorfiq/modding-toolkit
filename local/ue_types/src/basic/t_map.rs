use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TMap<K, V> {
    // Size: 0x50
    _something: [u8; 0x50],
    _phantom_a: PhantomData<K>,
    _phantom_b: PhantomData<V>,
}