#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_chaos_solver_engine_blueprint_library_convert_physics_collision_to_hit_result: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_solver_actor_set_solver_active: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_solver_actor_set_as_current_world_solver: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_chaos_solver_engine_blueprint_library_convert_physics_collision_to_hit_result: std::ptr::null_mut(),
            a_chaos_solver_actor_set_solver_active: std::ptr::null_mut(),
            a_chaos_solver_actor_set_as_current_world_solver: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UChaosSolverEngineBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertPhysicsCollisionToHitResult"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_solver_engine_blueprint_library_convert_physics_collision_to_hit_result,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AChaosSolverActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSolverActive"),
            &raw mut __FUNCTION_PTRS.a_chaos_solver_actor_set_solver_active,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAsCurrentWorldSolver"),
            &raw mut __FUNCTION_PTRS.a_chaos_solver_actor_set_as_current_world_solver,
        );
    }
}
#[repr(C, align(8))]
pub struct FChaosPhysicsCollisionInfo {
    pub component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub other_component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub location: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub accumulated_impulse: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub other_velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub other_angular_velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub other_mass: f32,
}
impl FChaosPhysicsCollisionInfo {}
#[repr(C, align(8))]
pub struct FDataflowRigidSolverProxy {
    __padding_end: [u8; 144],
}
impl FDataflowRigidSolverProxy {}
#[repr(C, align(8))]
pub struct UChaosDebugDrawComponent {
    __padding_end: [u8; 264],
}
impl UChaosDebugDrawComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosDebugDrawComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UChaosDebugDrawSubsystem {
    __padding_end: [u8; 136],
}
impl UChaosDebugDrawSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosDebugDrawSubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UChaosEventListenerComponent {
    __padding_end: [u8; 248],
}
impl UChaosEventListenerComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosEventListenerComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UChaosGameplayEventDispatcher {
    __padding_end: [u8; 776],
}
impl UChaosGameplayEventDispatcher {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosGameplayEventDispatcher")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IChaosNotifyHandlerInterface {}
#[repr(C, align(8))]
pub struct UChaosNotifyHandlerInterface {
    __padding_end: [u8; 48],
}
impl UChaosNotifyHandlerInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosNotifyHandlerInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UChaosSolverEngineBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UChaosSolverEngineBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosSolverEngineBlueprintLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn convert_physics_collision_to_hit_result(
        physics_collision: &FChaosPhysicsCollisionInfo,
    ) -> crate::bindings::engine::FHitResult {
        let mut __stack = crate::core_data::StackAlloc::<456>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_solver_engine::__FUNCTION_PTRS
                    .u_chaos_solver_engine_blueprint_library_convert_physics_collision_to_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                physics_collision,
                __buffer.add(0).cast::<FChaosPhysicsCollisionInfo>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chaos_solver_engine::UChaosSolverEngineBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_solver_engine::__FUNCTION_PTRS
                    .u_chaos_solver_engine_blueprint_library_convert_physics_collision_to_hit_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(192).cast::<crate::bindings::engine::FHitResult>().read() }
    }
}
#[repr(C, align(8))]
pub struct UChaosSolver {
    __padding_end: [u8; 48],
}
impl UChaosSolver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosSolver")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct AChaosSolverActor {
    __padding_end: [u8; 1672],
}
impl AChaosSolverActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AChaosSolverActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_solver_active(&mut self, b_active: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_solver_engine::__FUNCTION_PTRS
                    .a_chaos_solver_actor_set_solver_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_active, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_solver_engine::__FUNCTION_PTRS
                    .a_chaos_solver_actor_set_solver_active,
                __buffer,
            )
        };
    }
    pub fn set_as_current_world_solver(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_solver_engine::__FUNCTION_PTRS
                    .a_chaos_solver_actor_set_as_current_world_solver,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_solver_engine::__FUNCTION_PTRS
                    .a_chaos_solver_actor_set_as_current_world_solver,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UChaosSolverSettings {
    __padding_end: [u8; 152],
}
impl UChaosSolverSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosSolverSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(transparent)]
pub struct ERelayThrottlingState(pub u8);
impl ERelayThrottlingState {
    pub const INACTIVE: ERelayThrottlingState = ERelayThrottlingState(0);
    pub const ACTIVE: ERelayThrottlingState = ERelayThrottlingState(1);
}
#[repr(transparent)]
pub struct EClusterConnectionTypeEnum(pub u8);
impl EClusterConnectionTypeEnum {
    pub const CHAOS_POINT_IMPLICIT: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(
        0,
    );
    pub const CHAOS_DELAUNAY_TRIANGULATION: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(
        1,
    );
    pub const CHAOS_MINIMAL_SPANNING_SUBSET_DELAUNAY_TRIANGULATION: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(
        2,
    );
    pub const CHAOS_POINT_IMPLICIT_AUGMENTED_WITH_MINIMAL_DELAUNAY: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(
        3,
    );
    pub const CHAOS_BOUNDS_OVERLAP_FILTERED_DELAUNAY_TRIANGULATION: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(
        4,
    );
    pub const CHAOS_NONE: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(5);
}
