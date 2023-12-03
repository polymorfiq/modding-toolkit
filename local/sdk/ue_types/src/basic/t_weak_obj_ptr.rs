use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct TWeakObjectPtr<T> {
    pub ptr: FWeakObjectPtr,
    _phantom: PhantomData<T>
}