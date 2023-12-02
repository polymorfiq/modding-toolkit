use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FUObjectItem {
    // Size: 0x14
    pub object_addr: *const UObject,
    pub flags: u32,
    pub cluster_root_idx: u32,
    pub serial_number: u32
}

impl FUObjectItem {
    pub fn object<'a, T>(&self) -> Option<&'a T> {
        unsafe { (self.object_addr as *const T).as_ref::<'a>() }
    }

    pub fn is_root_set(&self) -> bool {
        self.flags & object_flags::ROOT_SET > 0
    }
}