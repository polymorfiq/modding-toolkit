#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TRefCountPtr<T> {
    reference: *const T
}
