use simple_endian::u16le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TSharedPtr<T> {
    pub object: *const T,
    pub shared_ref_count: u16le
}