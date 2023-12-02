use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TEnumAsByte<T> {
    pub data: u8,
    _phantom_a: PhantomData<T>
}

impl<T> TEnumAsByte<T> {
    pub fn data(&self) -> u8 { self.data }
}