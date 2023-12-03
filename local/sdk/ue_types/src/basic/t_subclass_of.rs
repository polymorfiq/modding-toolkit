use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TSubclassOf<T> {
    // 0x8
    pub ptr: u64,
    _phantom: PhantomData<T>
}