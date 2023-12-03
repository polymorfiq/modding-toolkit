use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FWeakObjectPtr {
    pub obj_index: u32le,
    pub obj_serial_number: u32le
}