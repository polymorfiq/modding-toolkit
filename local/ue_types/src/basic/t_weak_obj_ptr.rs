use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TWeakObjectPtr<T> {
    pub ptr: u64,
    _phantom: PhantomData<T>
}