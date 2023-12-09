use crate::*;
use ue_types::*;

pub struct GObjects {}

impl GObjects {
    pub fn gobjects<'a, VFTable>() -> &'a FUObjectArray<VFTable> {
        unsafe {
            std::mem::transmute(GameBase::singleton().at_offset(crate::offsets::OFFSET_STRUCT_GOBJECTS))
        }
    }

    pub fn objects<VFTable>() -> &'static FChunkedFixedUObjectArray<VFTable> {
        unsafe {
            std::mem::transmute(std::ptr::addr_of!(Self::gobjects::<VFTable>().objects_array))
        }
    }
}