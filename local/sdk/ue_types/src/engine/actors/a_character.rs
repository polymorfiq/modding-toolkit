use crate::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ACharacter<'a> {
    base_actor: AActor<'a>
}

impl ACharacter<'_> {
    pub fn get_nav_agent_location(character: *const ACharacter) -> FVector {
        let get_nav_agent_location: fn(*const ACharacter, *mut FVector) = unsafe {
            std::mem::transmute(0x00007FF74CDC76A0 as *const fn(*const ACharacter, *mut FVector))
        };

        let mut result: FVector = FVector{x: 0f32, y: 0f32, z: 0f32};
        (get_nav_agent_location)(character, &mut result);
        
        result
    }
}