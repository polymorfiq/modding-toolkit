use ue_types::*;

#[derive(Debug, Copy, Clone)]
pub struct Object<VFTable>(*const UObject<VFTable>);

impl<VFTable> Object<VFTable> {
    pub fn new(uobject: *const UObject<VFTable>) -> Self {
        Self(uobject)
    }
}

pub trait VirtualObject<VFTable: Copy + std::fmt::Debug> {
    fn object(&self) -> Object<VFTable>;

    fn object_virtual_funcs(&self) -> VFTable { unsafe {
        (*self.object().0).base.vftable
    } }
    
    fn name(&self) -> FName { unsafe { (*self.object().0).base.name_private } }

    fn class(&self) -> Option<*const UClass> {
        let base = unsafe { &(*self.object().0).base };
        if base.class_private != std::ptr::null() {
            Some(base.class_private)
        } else {
            None
        }
    }

    fn outer(&self) -> Option<Object<*const UnknownType>> {
        let base = unsafe { &(*self.object().0).base };
        if base.outer_private != std::ptr::null() {
            Some(Object(base.outer_private))
        } else {
            None
        }
    }

    fn full_name(&self) -> String {
        let base = unsafe { &(*self.object().0).base };
        let my_name = base.name_private;
        let name_str = my_name.to_string();

        match self.outer() {
            Some(outer) => [outer.full_name(), name_str].join("."),
            None => name_str
        }
    }
}

impl<VFTable: Copy + std::fmt::Debug> VirtualObject<VFTable> for Object<VFTable> {
    fn object(&self) -> Object<VFTable> { *self }
}

impl VirtualObject<*const UnknownType> for UObject<*const UnknownType> {
    fn object(&self) -> Object<*const UnknownType> { Object(std::ptr::addr_of!(*self)) }
}

impl VirtualObject<*const UnknownType> for UActorComponent {
    fn object(&self) -> Object<*const UnknownType> { Object(std::ptr::addr_of!(self.base_object)) }
}

impl VirtualObject<*const UnknownType> for USceneComponent {
    fn object(&self) -> Object<*const UnknownType> { self.base_actor_component.object() }
}

impl VirtualObject<*const UnknownType> for AActor {
    fn object(&self) -> Object<*const UnknownType> { Object(std::ptr::addr_of!(self.base_object)) }
}

impl VirtualObject<*const UnknownType> for ACharacter {
    fn object(&self) -> Object<*const UnknownType> { self.base_pawn.object() }
}

impl VirtualObject<*const UnknownType> for APawn {
    fn object(&self) -> Object<*const UnknownType> { self.base_actor.object() }
}

impl VirtualObject<*const UnknownType> for UClass {
    fn object(&self) -> Object<*const UnknownType> { self.base_struct.object() }
}

impl VirtualObject<*const UnknownType> for UEngine {
    fn object(&self) -> Object<*const UnknownType> { Object(std::ptr::addr_of!(self.base_object)) }
}

impl VirtualObject<*const UnknownType> for UField {
    fn object(&self) -> Object<*const UnknownType> { Object(std::ptr::addr_of!(self.base_object)) }
}

impl VirtualObject<*const UnknownType> for UGameEngine {
    fn object(&self) -> Object<*const UnknownType> { self.base_engine.object() }
}

impl VirtualObject<*const UnknownType> for UGameInstance {
    fn object(&self) -> Object<*const UnknownType> { Object(std::ptr::addr_of!(self.base_object)) }
}

impl VirtualObject<*const UnknownType> for ULevel {
    fn object(&self) -> Object<*const UnknownType> { Object(std::ptr::addr_of!(self.base_object)) }
}

impl VirtualObject<*const UnknownType> for ULocalPlayer {
    fn object(&self) -> Object<*const UnknownType> { self.base_player.object() }
}

impl VirtualObject<*const UnknownType> for UPlayer {
    fn object(&self) -> Object<*const UnknownType> { Object(std::ptr::addr_of!(self.base_object)) }
}

impl VirtualObject<*const UnknownType> for UProperty {
    fn object(&self) -> Object<*const UnknownType> { self.base_field.object() }
}

impl VirtualObject<*const UnknownType> for UStruct {
    fn object(&self) -> Object<*const UnknownType> { self.base_field.object() }
}

impl VirtualObject<*const UnknownType> for UWorld {
    fn object(&self) -> Object<*const UnknownType> { Object(std::ptr::addr_of!(self.base_object)) }
}

impl VirtualObject<*const UnknownType> for AController {
    fn object(&self) -> Object<*const UnknownType> { self.base_actor.object() }
}

impl VirtualObject<*const UnknownType> for APlayerController {
    fn object(&self) -> Object<*const UnknownType> { self.base_controller.object() }
}