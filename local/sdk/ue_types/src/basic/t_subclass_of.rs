#[derive(Debug)]
#[repr(C, align(0x8))]
pub struct TSubclassOf<T> {
    // 0x8
    pub ptr: *const T
}

impl<T> Copy for TSubclassOf<T> { }

impl<T> Clone for TSubclassOf<T> {
    fn clone(&self) -> TSubclassOf<T> {
        TSubclassOf::<T>{ptr: self.ptr}
    }
}