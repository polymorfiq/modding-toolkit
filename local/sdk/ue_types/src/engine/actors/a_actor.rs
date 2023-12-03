use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AActor<'a> {
    // Size: 0x0348
    pub base_object: UObject,
    pub primary_actor_tick: FActorTickFunction<'a>,
    b_something: u32,
    pub b_hidden: u8,
    pub remote_role: TEnumAsByte<UnknownType>,
    _something_a: [u8; 2],
    pub replicated_movement: FRepMovement,
    pub initial_life_span: f32,
    pub custom_time_dilation: f32,
    pub creation_time: f32,
    pub attachment_replication: FRepAttachment,
    pub owner: *const AActor<'a>,
    pub net_driver_name: FName,
    pub role: TEnumAsByte<UnknownType>,
    pub net_dormancy: TEnumAsByte<UnknownType>,
    pub spawn_collision_handling_method: u8,
    pub auto_receive_input: TEnumAsByte<UnknownType>,
    pub input_priority: u32,
    _something_b: [u8; 4],
    pub input_component: *const UnknownType,
    pub net_cull_distance_squared: f32,
    pub net_tag: u32,
    pub net_update_frequency: f32,
    pub min_net_update_frequency: f32,
    pub net_priority: f32,
    pub cached_last_render_time: f32,
    pub instigator: *const UnknownType,
    pub children: TArray<*const AActor<'a>>,
    pub root_component: *const USceneComponent<'a>,
    pub controlling_matinee_actors: TArray<UnknownType>,
    pub timer_handle_lifespan_expired: u64,
    pub layers: TArray<FName>,
    pub parent_component: TWeakObjectPtr<UnknownType>,
    _something_c: [u8; 8],
    pub tags: TArray<FName>,
    pub on_take_any_damage: TBaseDynamicMulticastDelegate,
    pub on_take_point_damage: TBaseDynamicMulticastDelegate,
    pub on_take_radial_damage: TBaseDynamicMulticastDelegate,
    pub on_actor_begin_overlap: TBaseDynamicMulticastDelegate,
    pub on_actor_end_overlap: TBaseDynamicMulticastDelegate,
    pub on_begin_cursor_over: TBaseDynamicMulticastDelegate,
    pub on_end_cursor_over: TBaseDynamicMulticastDelegate,
    pub on_clicked: TBaseDynamicMulticastDelegate,
    pub on_released: TBaseDynamicMulticastDelegate,
    pub on_input_touch_begin: TBaseDynamicMulticastDelegate,
    pub on_input_touch_end: TBaseDynamicMulticastDelegate,
    pub on_input_touch_enter: TBaseDynamicMulticastDelegate,
    pub on_input_touch_leave: TBaseDynamicMulticastDelegate,
    pub on_actor_hit: TBaseDynamicMulticastDelegate,
    pub on_destroyed: TBaseDynamicMulticastDelegate,
    pub on_end_play: TBaseDynamicMulticastDelegate,
    pub replicated_components: TArray<*const UnknownType>,
    _owned_components: [u8; 0x50],
    pub instance_components: TArray<UnknownType>,
    pub blueprint_created_components: TArray<UnknownType>,
    _detach_fence: [u8; 0x10],
    _phantom: PhantomData<&'a u8>
}

impl<'a> AActor<'a> {
    pub fn object(&self) -> UObject { self.base_object }
    pub fn name(&self) -> FName { self.object().name() }
    pub fn full_name(&self) -> String { self.object().full_name() }
}