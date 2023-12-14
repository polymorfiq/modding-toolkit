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

    pub fn filter(test: impl Fn(&UObject<*const UnknownType>) -> bool) -> Vec<*const UObject<*const UnknownType>> {
        let g_objects = Self::objects::<UnknownType>();
        
        let mut filtered: Vec<*const UObject<*const UnknownType>> = vec![];
        for i in 0..(g_objects.num_elements.to_native()-1) {
            let item = g_objects.item_at_idx(i as usize);
            let object = if item.is_some() { unsafe { (*item.unwrap()).object::<UObject<*const UnknownType>>() } } else { None };
            
            if object.is_some() && (test)(object.unwrap()) {
                filtered.push(std::ptr::addr_of!(*object.unwrap()))
            }
        }

        return filtered
    }

    pub fn find_first(test: impl Fn(&UObject<*const UnknownType>) -> bool) -> Option<*const UObject<*const UnknownType>> {
        let g_objects = Self::objects::<UnknownType>();
        
        let mut found: Option<*const UObject<*const UnknownType>> = None;
        for i in 0..(g_objects.num_elements.to_native()-1) {
            let item = g_objects.item_at_idx(i as usize);
            let object = if item.is_some() { unsafe { (*item.unwrap()).object::<UObject<*const UnknownType>>() } } else { None };
            
            if object.is_some() && (test)(object.unwrap()) {
                found = Some(std::ptr::addr_of!(*object.unwrap()));
                break;
            }
        }

        return found
    }
}