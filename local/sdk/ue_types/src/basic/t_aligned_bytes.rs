#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x1))]
pub struct TAlignedBytes1<const BYTES: usize> {
    // Size: BYTES
    bytes: [u8; BYTES]
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct TAlignedBytes4<const BYTES: usize> {
    // Size: BYTES
    bytes: [u8; BYTES]
}

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TAlignedBytes8<const BYTES: usize> {
    // Size: BYTES
    bytes: [u8; BYTES]
}