use ue_types::*;

#[cfg(feature = "client-sdk")]

pub trait VirtualUObject<VFTable> {
    fn object(&self) -> *const UObject<VFTable>;

    fn virtual_funcs(&self) -> *const VFTable { unsafe { (*self.object()).base.vf_table } }
    
    fn name(&self) -> FName { unsafe { (*self.object()).base.name_private } }

    fn class(&self) -> Option<*const UClass> {
        let base = unsafe { &(*self.object()).base };
        if base.class_private != std::ptr::null() {
            Some(base.class_private)
        } else {
            None
        }
    }

    fn outer(&self) -> Option<UObject<UnknownType>> {
        let base = unsafe { &(*self.object()).base };
        if base.outer_private != std::ptr::null() {
            Some(unsafe { *base.outer_private })
        } else {
            None
        }
    }

    fn full_name(&self) -> String {
        let base = unsafe { &(*self.object()).base };
        let my_name = base.name_private;
        let name_str = my_name.to_string();

        match self.outer() {
            Some(outer) => [outer.full_name(), name_str].join("."),
            None => name_str
        }
    }
}

impl VirtualUObject<UnknownType> for UObject<UnknownType> {
    fn object(&self) -> *const UObject<UnknownType> { self }
}

impl VirtualUObject<UConsoleVFTable> for UConsole {
    fn object(&self) -> *const UObject<UConsoleVFTable> { std::ptr::addr_of!(self.base_object) as *const UObject<UConsoleVFTable> }
}

impl VirtualUObject<UnknownType> for UActorComponent {
    fn object(&self) -> *const UObject<UnknownType> { &self.base_object }
}

impl VirtualUObject<UnknownType> for USceneComponent {
    fn object(&self) -> *const UObject<UnknownType> { self.base_actor_component.object() }
}

impl VirtualUObject<UnknownType> for AActor {
    fn object(&self) -> *const UObject<UnknownType> { &self.base_object }
}

impl VirtualUObject<UnknownType> for ACharacter {
    fn object(&self) -> *const UObject<UnknownType> { self.base_pawn.object() }
}

impl VirtualUObject<UnknownType> for APawn {
    fn object(&self) -> *const UObject<UnknownType> { self.base_actor.object() }
}

impl VirtualUObject<UnknownType> for UClass {
    fn object(&self) -> *const UObject<UnknownType> { self.base_struct.object() }
}

impl VirtualUObject<UnknownType> for UEngine {
    fn object(&self) -> *const UObject<UnknownType> { &self.base_object }
}

impl VirtualUObject<UnknownType> for UField {
    fn object(&self) -> *const UObject<UnknownType> { &self.base_object }
}

impl VirtualUObject<UnknownType> for UGameEngine {
    fn object(&self) -> *const UObject<UnknownType> { self.base_engine.object() }
}

impl VirtualUObject<UnknownType> for UGameInstance {
    fn object(&self) -> *const UObject<UnknownType> { &self.base_object }
}

impl VirtualUObject<UnknownType> for ULevel {
    fn object(&self) -> *const UObject<UnknownType> { &self.base_object }
}

impl VirtualUObject<UnknownType> for ULocalPlayer {
    fn object(&self) -> *const UObject<UnknownType> { self.base_player.object() }
}

impl VirtualUObject<UnknownType> for UPlayer {
    fn object(&self) -> *const UObject<UnknownType> { &self.base_object }
}

impl VirtualUObject<UnknownType> for UProperty {
    fn object(&self) -> *const UObject<UnknownType> { self.base_field.object() }
}

impl VirtualUObject<UnknownType> for UStruct {
    fn object(&self) -> *const UObject<UnknownType> { self.base_field.object() }
}

impl VirtualUObject<UnknownType> for UWorld {
    fn object(&self) -> *const UObject<UnknownType> { &self.base_object }
}

impl VirtualUObject<UnknownType> for AController {
    fn object(&self) -> *const UObject<UnknownType> { self.base_actor.object() }
}

impl VirtualUObject<UnknownType> for APlayerController {
    fn object(&self) -> *const UObject<UnknownType> { self.base_controller.object() }
}