#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TSharedRef<T, const SOMETHING: usize> {
    pub object: *const T,
    pub shared_ref_count: FSharedReferencer<SOMETHING>
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FSharedReferencer<const SOMETHING: usize> {
    _unknown: [u8; 0x8]
}