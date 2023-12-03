use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FRotator {
    pitch: u32le,
    yaw: u32le,
    roll: u32le
}