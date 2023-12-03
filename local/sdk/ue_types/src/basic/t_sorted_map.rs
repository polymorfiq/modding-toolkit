use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TSortedMap<K, V> {
    // Size: 0x10
    pub pairs: TArray<(K, V)>
}