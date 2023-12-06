use std::marker::PhantomData;
use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TBaseMulticastDelegate<T, P> {
    // Size: 0x10
    pub base_delegate: FMulticastDelegateBase<P>,
    _phantom: PhantomData<T>
}