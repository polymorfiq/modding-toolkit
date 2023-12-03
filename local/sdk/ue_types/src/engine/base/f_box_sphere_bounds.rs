use simple_endian::*;
use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x4))]
pub struct FBoxSphereBounds {
    // Size: 0x1C
    pub origin: FVector,
    pub box_extent: FVector,
    pub sphere_radius: u32le
}