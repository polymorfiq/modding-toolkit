use ue_types::*;
use crate::*;
use ue_types::ue_endian::u32le;

pub struct ObjectLibrary {
    pub addr: *const UnknownType
}

static mut OBJECT_LIBRARY_ADDR: Option<*const UnknownType> = None;

impl ObjectLibrary {
    pub fn get() -> Option<Self> {
        let known_addr = unsafe { OBJECT_LIBRARY_ADDR };
        
        match known_addr {
            Some(addr) => Some(Self{addr: addr}),
            None => {
                let library = GObjects::find_first(|obj| {
                    obj.full_name() == "/Script/Engine.Default__ObjectLibrary"
                });

                if library.is_none() { return None };
                unsafe {
                    OBJECT_LIBRARY_ADDR = Some(library.unwrap() as *const UnknownType);
                    Some(Self{addr: OBJECT_LIBRARY_ADDR.unwrap()})
                }
            }
        }
    }

    pub fn load_assets_from_path(&self, path: String) -> u32le {        
        let load_assets_from_path: fn(*const UnknownType, *const FString) -> u32le = unsafe {
            std::mem::transmute(GameBase::singleton().at_offset(OFFSETS.asset_funcs.object_library_load_assets_at_path))
        };

        let path_fstring: Box<FString> = Box::new(path.into());
        (load_assets_from_path)(self.addr, Box::into_raw(path_fstring))
    }
}