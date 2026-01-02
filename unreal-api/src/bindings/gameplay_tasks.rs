#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(2))]
pub struct FGameplayResourceSet {
    __padding_end: [u8; 2],
}
impl FGameplayResourceSet {}
#[repr(C, align(8))]
pub struct UGameplayTask {
    __padding_end: [u8; 128],
}
impl UGameplayTask {}
pub struct UGameplayTaskOwnerInterface {}
pub struct IGameplayTaskOwnerInterface {}
#[repr(C, align(8))]
pub struct UGameplayTaskResource {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub manual_resource_id: i32,
    __padding_end: [u8; 12],
}
impl UGameplayTaskResource {}
#[repr(C, align(8))]
pub struct UGameplayTasksComponent {
    #[doc(hidden)]
    __padding_328: [u8; 328],
    pub on_claimed_resources_change: FGameplayTasksComponent_OnClaimedResourcesChange,
    __padding_end: [u8; 32],
}
impl UGameplayTasksComponent {}
#[repr(C, align(8))]
pub struct UGameplayTask_ClaimResource {
    __padding_end: [u8; 128],
}
impl UGameplayTask_ClaimResource {}
#[repr(C, align(8))]
pub struct UGameplayTask_SpawnActor {
    __padding_end: [u8; 232],
}
impl UGameplayTask_SpawnActor {}
#[repr(C, align(8))]
pub struct UGameplayTask_TimeLimitedExecution {
    __padding_end: [u8; 200],
}
impl UGameplayTask_TimeLimitedExecution {}
#[repr(C, align(8))]
pub struct UGameplayTask_WaitDelay {
    __padding_end: [u8; 168],
}
impl UGameplayTask_WaitDelay {}
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
