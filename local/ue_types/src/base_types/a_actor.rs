use crate::{FName, TArray, TBaseDynamicMulticastDelegate, TEnumAsByte, TWeakObjectPtr, UObject, UnknownType};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct AActor {
    // Size: 0x0330
    base_object: UObject,
    primary_actor_tick: FActorTickFunction,
    b_hidden: u8,
    flags: [u8; 31],
    remote_role: TEnumAsByte<UnknownType>,
    replicated_movement: FRepMovement,
    initial_life_span: f64,
    custom_time_dilation: f64,
    creation_time: f64,
    attachment_replication: FRepAttachment,
    owner: *const AActor,
    net_driver_name: FName,
    role: TEnumAsByte<UnknownType>,
    net_dormancy: TEnumAsByte<UnknownType>,
    spawn_collision_handling_method: u8,
    auto_receive_input: TEnumAsByte<UnknownType>,
    input_priority: u32,
    input_component: *const UnknownType,
    net_cull_distance_squared: f64,
    net_tag: u32,
    net_update_frequency: f64,
    min_net_update_frequency: f64,
    net_priority: f64,
    cached_last_render_time: f64,
    instigator: *const UnknownType,
    children: TArray<*const AActor>,
    root_component: *const UnknownType,
    controlling_matinee_actors: TArray<UnknownType>,
    timer_handle_lifespan_expired: u64,
    layers: TArray<FName>,
    parent_component: TWeakObjectPtr<UnknownType>,
    tags: TArray<FName>,
    on_take_any_damage: TBaseDynamicMulticastDelegate,
    on_take_point_damage: TBaseDynamicMulticastDelegate,
    on_take_radial_damage: TBaseDynamicMulticastDelegate,
    on_actor_begin_overlap: TBaseDynamicMulticastDelegate,
    on_actor_end_overlap: TBaseDynamicMulticastDelegate,
    on_begin_cursor_over: TBaseDynamicMulticastDelegate,
    on_end_cursor_over: TBaseDynamicMulticastDelegate,
    on_clicked: TBaseDynamicMulticastDelegate,
    on_released: TBaseDynamicMulticastDelegate,
    on_input_touch_begin: TBaseDynamicMulticastDelegate,
    on_input_touch_end: TBaseDynamicMulticastDelegate,
    on_input_touch_enter: TBaseDynamicMulticastDelegate,
    on_input_touch_leave: TBaseDynamicMulticastDelegate,
    on_actor_hit: TBaseDynamicMulticastDelegate,
    on_destroyed: TBaseDynamicMulticastDelegate,
    on_end_play: TBaseDynamicMulticastDelegate,
    _unknown: [u8; 0x60],
    instance_components: TArray<UnknownType>,
    blueprint_created_components: TArray<UnknownType>,
    _unknown_b: [u8; 0x10]
}


#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FActorTickFunction {
    _unknown: [u8; 0x58]
}


#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FRepMovement {
    _unknown: [u8; 0x34]
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FRepAttachment {
    _unknown: [u8; 0x40]
}