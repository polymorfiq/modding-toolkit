use crate::*;
use std::marker::PhantomData;

#[derive(Debug)]
#[repr(C, align(0x4))]
pub struct TWeakObjectPtr<T> {
    pub ptr: FWeakObjectPtr,
    _phantom: PhantomData<T>
}


impl<T> Copy for TWeakObjectPtr<T> { }

impl<T> Clone for TWeakObjectPtr<T> {
    fn clone(&self) -> Self { *self }
}