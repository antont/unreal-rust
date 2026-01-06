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
pub static mut U_GAMEPLAY_TASK_READY_FOR_ACTIVATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASK_GENERIC_GAMEPLAY_TASK_DELEGATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASK_END_TASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASKS_COMPONENT_ON_REP_SIMULATED_TASKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASKS_COMPONENT_K2_RUN_GAMEPLAY_TASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASK_CLAIM_RESOURCE_CLAIM_RESOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASK_CLAIM_RESOURCE_CLAIM_RESOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASK_SPAWN_ACTOR_SPAWN_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASK_SPAWN_ACTOR_FINISH_SPAWNING_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASK_SPAWN_ACTOR_BEGIN_SPAWNING_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASK_TIME_LIMITED_EXECUTION_TASK_FINISH_DELEGATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASK_WAIT_DELAY_TASK_WAIT_DELAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_TASK_WAIT_DELAY_TASK_DELAY_DELEGATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayTask::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadyForActivation"),
            &raw mut U_GAMEPLAY_TASK_READY_FOR_ACTIVATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenericGameplayTaskDelegate__DelegateSignature"),
            &raw mut U_GAMEPLAY_TASK_GENERIC_GAMEPLAY_TASK_DELEGATE_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EndTask"),
            &raw mut U_GAMEPLAY_TASK_END_TASK,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayTasksComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRep_SimulatedTasks"),
            &raw mut U_GAMEPLAY_TASKS_COMPONENT_ON_REP_SIMULATED_TASKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_RunGameplayTask"),
            &raw mut U_GAMEPLAY_TASKS_COMPONENT_K2_RUN_GAMEPLAY_TASK,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayTask_ClaimResource::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClaimResources"),
            &raw mut U_GAMEPLAY_TASK_CLAIM_RESOURCE_CLAIM_RESOURCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClaimResource"),
            &raw mut U_GAMEPLAY_TASK_CLAIM_RESOURCE_CLAIM_RESOURCE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayTask_SpawnActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnActor"),
            &raw mut U_GAMEPLAY_TASK_SPAWN_ACTOR_SPAWN_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinishSpawningActor"),
            &raw mut U_GAMEPLAY_TASK_SPAWN_ACTOR_FINISH_SPAWNING_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BeginSpawningActor"),
            &raw mut U_GAMEPLAY_TASK_SPAWN_ACTOR_BEGIN_SPAWNING_ACTOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayTask_TimeLimitedExecution::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TaskFinishDelegate__DelegateSignature"),
            &raw mut U_GAMEPLAY_TASK_TIME_LIMITED_EXECUTION_TASK_FINISH_DELEGATE_DELEGATE_SIGNATURE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayTask_WaitDelay::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TaskWaitDelay"),
            &raw mut U_GAMEPLAY_TASK_WAIT_DELAY_TASK_WAIT_DELAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TaskDelayDelegate__DelegateSignature"),
            &raw mut U_GAMEPLAY_TASK_WAIT_DELAY_TASK_DELAY_DELEGATE_DELEGATE_SIGNATURE,
        );
    }
}
#[repr(C, align(2))]
pub struct FGameplayResourceSet {
    __padding_end: [u8; 2],
}
impl FGameplayResourceSet {}
#[repr(C, align(8))]
pub struct UGameplayTask {
    __padding_end: [u8; 128],
}
impl UGameplayTask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTask")
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
pub struct IGameplayTaskOwnerInterface {}
#[repr(C, align(8))]
pub struct UGameplayTaskOwnerInterface {
    __padding_end: [u8; 48],
}
impl UGameplayTaskOwnerInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTaskOwnerInterface")
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
pub struct UGameplayTaskResource {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub manual_resource_id: i32,
    __padding_end: [u8; 12],
}
impl UGameplayTaskResource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTaskResource")
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
pub struct UGameplayTasksComponent {
    #[doc(hidden)]
    __padding_328: [u8; 328],
    pub on_claimed_resources_change: FGameplayTasksComponent_OnClaimedResourcesChange,
    __padding_end: [u8; 32],
}
impl UGameplayTasksComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTasksComponent")
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
pub struct UGameplayTask_ClaimResource {
    __padding_end: [u8; 128],
}
impl UGameplayTask_ClaimResource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTask_ClaimResource")
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
pub struct UGameplayTask_SpawnActor {
    __padding_end: [u8; 232],
}
impl UGameplayTask_SpawnActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTask_SpawnActor")
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
pub struct UGameplayTask_TimeLimitedExecution {
    __padding_end: [u8; 200],
}
impl UGameplayTask_TimeLimitedExecution {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTask_TimeLimitedExecution")
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
pub struct UGameplayTask_WaitDelay {
    __padding_end: [u8; 168],
}
impl UGameplayTask_WaitDelay {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTask_WaitDelay")
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
pub struct FGameplayTasksComponent_OnClaimedResourcesChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGameplayTask_SpawnActor_Success {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGameplayTask_SpawnActor_DidNotSpawn {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGameplayTask_TimeLimitedExecution_OnFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGameplayTask_TimeLimitedExecution_OnTimeExpired {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGameplayTask_WaitDelay_OnFinish {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EGameplayTaskRunResult(pub u8);
impl EGameplayTaskRunResult {
    pub const ERROR: EGameplayTaskRunResult = EGameplayTaskRunResult(0);
    pub const FAILED: EGameplayTaskRunResult = EGameplayTaskRunResult(1);
    pub const SUCCESS_PAUSED: EGameplayTaskRunResult = EGameplayTaskRunResult(2);
    pub const SUCCESS_ACTIVE: EGameplayTaskRunResult = EGameplayTaskRunResult(3);
    pub const SUCCESS_FINISHED: EGameplayTaskRunResult = EGameplayTaskRunResult(4);
}
#[repr(transparent)]
pub struct ETaskResourceOverlapPolicy(pub u8);
impl ETaskResourceOverlapPolicy {
    pub const START_ON_TOP: ETaskResourceOverlapPolicy = ETaskResourceOverlapPolicy(0);
    pub const START_AT_END: ETaskResourceOverlapPolicy = ETaskResourceOverlapPolicy(1);
    pub const REQUEST_CANCEL_AND_START_ON_TOP: ETaskResourceOverlapPolicy = ETaskResourceOverlapPolicy(
        2,
    );
    pub const REQUEST_CANCEL_AND_START_AT_END: ETaskResourceOverlapPolicy = ETaskResourceOverlapPolicy(
        3,
    );
}
