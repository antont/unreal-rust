#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub class_to_spawn: TSubclassOf<AActor>,
}
pub struct UGameplayTask_TimeLimitedExecution {
    pub on_finished: FGameplayTask_TimeLimitedExecution_OnFinished,
    pub on_time_expired: FGameplayTask_TimeLimitedExecution_OnTimeExpired,
}
pub struct UGameplayTask_WaitDelay {
    pub on_finish: FGameplayTask_WaitDelay_OnFinish,
}
