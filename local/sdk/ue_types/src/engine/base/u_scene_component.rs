use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x08))]
pub struct USceneComponent {
    // Size: 0x270
    base_actor_component: UActorComponent,
    _unknown: [u8; 0x7C],
    relative_location: FVector,
    relative_rotation: FRotator,
    relative_scale_3d: FVector,
    component_to_world: FTransform,
    component_velocity: FVector,
    _unknown_b: [u8; 0x94]
}