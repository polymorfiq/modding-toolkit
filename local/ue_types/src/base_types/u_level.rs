use std::marker::PhantomData;
use crate::AActor;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ULevel<'a> {
    _something: [u8; 8],
    actors: *const AActor,
    _phantom: PhantomData<&'a u8>
}

impl<'a> ULevel<'a> {
    pub fn actors(&self) -> &AActor {
        unsafe { self.actors.as_ref::<'a>().unwrap() }
    }
}