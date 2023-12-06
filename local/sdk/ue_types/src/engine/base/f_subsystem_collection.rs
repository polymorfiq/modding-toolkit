use std::marker::PhantomData;
use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FSubsystemCollection<T> {
    // Size: 0xC8
    base_collection: FSubsystemCollectionBase,
    _phantom: PhantomData<T>
}