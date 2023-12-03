use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TBaseDynamicMulticastDelegate {
    // Size: 0x10
    pub base_delegate: TMulticastScriptDelegate<FWeakObjectPtr>
}