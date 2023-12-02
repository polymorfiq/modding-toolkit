use simple_endian::u16le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TWeakPtr<T> {
    pub object: *const T,
    pub weak_ref_count: u16le
}