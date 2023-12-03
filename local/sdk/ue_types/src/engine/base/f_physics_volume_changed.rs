use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FPhysicsVolumeChanged {
    // Size: 0x10
    base_delegate: TBaseDynamicMulticastDelegate
}