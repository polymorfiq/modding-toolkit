use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct USceneComponent {
    // Size: 0x270
    pub base_actor_component: UActorComponent,
    pub cached_level_collection: *const UnknownType,
    pub physics_volume: TWeakObjectPtr<UnknownType>,
    pub attach_parent: *const USceneComponent,
    pub attach_socket_name: FName,
    _padding_a: [u8; 4],
    pub attach_children: TArray<*const USceneComponent, FDefaultAllocator>,
    pub client_attached_children: TArray<*const USceneComponent, FDefaultAllocator>,
    pub net_old_attached_socket_name: FName,
    _padding_b: [u8; 4],
    pub net_old_attached_parent: *const UnknownType,
    pub bounds: FBoxSphereBounds,
    pub relative_location: FVector,
    pub relative_rotation: FRotator,
    pub relative_scale_3d: FVector,
    pub component_to_world: FTransform,
    pub component_velocity: FVector,
    _padding_c: [u8; 3],
    pub mobility: TEnumAsByte<UnknownType>,
    pub detail_mode: TEnumAsByte<UnknownType>,
    _padding_d: [u8; 15],
    pub world_rotation_cache: FRotationConversionCache,
    pub relative_rotation_cache: FRotationConversionCache,
    pub physics_volume_changed_delegate: FPhysicsVolumeChanged,
    _f_transform_updated: [u8; 0x18],
    pub scoped_movement_stack: TArray<UnknownType, FDefaultAllocator>,
    _unknown_b: [u8; 0x8]
}