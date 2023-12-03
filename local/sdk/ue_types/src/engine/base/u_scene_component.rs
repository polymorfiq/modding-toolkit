use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct USceneComponent<'a> {
    // Size: 0x270
    base_actor_component: UActorComponent<'a>,
    cached_level_collection: *const UnknownType,
    physics_volume: TWeakObjectPtr<UnknownType>,
    attach_parent: *const USceneComponent<'a>,
    attach_socket_name: FName,
    _padding_a: [u8; 4],
    attach_children: TArray<*const USceneComponent<'a>>,
    client_attached_children: TArray<*const USceneComponent<'a>>,
    net_old_attached_socket_name: FName,
    _padding_b: [u8; 4],
    net_old_attached_parent: *const UnknownType,
    bounds: FBoxSphereBounds,
    relative_location: FVector,
    relative_rotation: FRotator,
    relative_scale_3d: FVector,
    component_to_world: FTransform,
    component_velocity: FVector,
    _padding_c: [u8; 3],
    mobility: TEnumAsByte<UnknownType>,
    detail_mode: TEnumAsByte<UnknownType>,
    _padding_d: [u8; 15],
    world_rotation_cache: FRotationConversionCache,
    relative_rotation_cache: FRotationConversionCache,
    physics_volume_changed_delegate: FPhysicsVolumeChanged,
    _f_transform_updated: [u8; 0x18],
    scoped_movement_stack: TArray<UnknownType>,
    _unknown_b: [u8; 0x8]
}