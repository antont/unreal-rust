#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(2))]
pub struct FGameplayResourceSet {}
pub struct UGameplayTask {
    pub instance_name: FName,
    pub resource_overlap_policy: ETaskResourceOverlapPolicy,
    pub child_task: UPtr<UGameplayTask>,
}
pub struct UGameplayTaskOwnerInterface {}
pub struct IGameplayTaskOwnerInterface {}
pub struct UGameplayTaskResource {
    pub manual_resource_id: i32,
    pub auto_resource_id: i8,
    pub flags_56: u8,
}
pub struct UGameplayTasksComponent {
    pub task_priority_queue: TArray<UPtr<UGameplayTask>>,
    pub ticking_tasks: TArray<UPtr<UGameplayTask>>,
    pub known_tasks: TArray<UPtr<UGameplayTask>>,
    pub on_claimed_resources_change: FGameplayTasksComponent_OnClaimedResourcesChange,
    pub simulated_tasks: TArray<UPtr<UGameplayTask>>,
}
pub struct UGameplayTask_ClaimResource {}
pub struct UGameplayTask_SpawnActor {
    pub success: FGameplayTask_SpawnActor_Success,
    pub did_not_spawn: FGameplayTask_SpawnActor_DidNotSpawn,
    pub class_to_spawn: TSubclassOf<crate::bindings::engine::AActor>,
}
pub struct UGameplayTask_TimeLimitedExecution {
    pub on_finished: FGameplayTask_TimeLimitedExecution_OnFinished,
    pub on_time_expired: FGameplayTask_TimeLimitedExecution_OnTimeExpired,
}
pub struct UGameplayTask_WaitDelay {
    pub on_finish: FGameplayTask_WaitDelay_OnFinish,
}
pub struct FGameplayTasksComponent_OnClaimedResourcesChange;
pub struct FGameplayTask_SpawnActor_Success;
pub struct FGameplayTask_SpawnActor_DidNotSpawn;
pub struct FGameplayTask_TimeLimitedExecution_OnFinished;
pub struct FGameplayTask_TimeLimitedExecution_OnTimeExpired;
pub struct FGameplayTask_WaitDelay_OnFinish;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGameplayTaskRunResult(pub u8);
impl EGameplayTaskRunResult {
    pub const ERROR: EGameplayTaskRunResult = EGameplayTaskRunResult(0);
    pub const FAILED: EGameplayTaskRunResult = EGameplayTaskRunResult(1);
    pub const SUCCESS_PAUSED: EGameplayTaskRunResult = EGameplayTaskRunResult(2);
    pub const SUCCESS_ACTIVE: EGameplayTaskRunResult = EGameplayTaskRunResult(3);
    pub const SUCCESS_FINISHED: EGameplayTaskRunResult = EGameplayTaskRunResult(4);
}
#[allow(non_camel_case_types)]
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
