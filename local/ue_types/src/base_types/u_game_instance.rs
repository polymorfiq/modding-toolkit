use std::marker::PhantomData;
use super::UWorld;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UGameInstance<'a> {
    world_addr: *const UWorld<'a>,
    _phantom: PhantomData<&'a u8>
}

impl<'a> UGameInstance<'a> {
    pub fn world(&self) -> &'a UWorld {
        unsafe { self.world_addr.as_ref::<'a>().unwrap() }
    }
}