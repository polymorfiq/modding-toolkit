#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct FGuid {
    // Size: 0x10
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32
}