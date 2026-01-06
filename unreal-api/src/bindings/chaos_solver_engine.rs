#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_CHAOS_SOLVER_ENGINE_BLUEPRINT_LIBRARY_CONVERT_PHYSICS_COLLISION_TO_HIT_RESULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_SOLVER_ACTOR_SET_SOLVER_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_SOLVER_ACTOR_SET_AS_CURRENT_WORLD_SOLVER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UChaosSolverEngineBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertPhysicsCollisionToHitResult"),
            &raw mut U_CHAOS_SOLVER_ENGINE_BLUEPRINT_LIBRARY_CONVERT_PHYSICS_COLLISION_TO_HIT_RESULT,
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
            &raw mut A_CHAOS_SOLVER_ACTOR_SET_SOLVER_ACTIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAsCurrentWorldSolver"),
            &raw mut A_CHAOS_SOLVER_ACTOR_SET_AS_CURRENT_WORLD_SOLVER,
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
