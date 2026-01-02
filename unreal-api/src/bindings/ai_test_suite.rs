#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UTestBTDecorator_Blackboard {
    __padding_end: [u8; 232],
}
impl UTestBTDecorator_Blackboard {}
#[repr(C, align(8))]
pub struct UTestBTDecorator_Blueprint {
    __padding_end: [u8; 224],
}
impl UTestBTDecorator_Blueprint {}
#[repr(C, align(8))]
pub struct UTestBTDecorator_CantExecute {
    __padding_end: [u8; 120],
}
impl UTestBTDecorator_CantExecute {}
#[repr(C, align(8))]
pub struct UTestBTDecorator_DelayedAbort {
    __padding_end: [u8; 128],
}
impl UTestBTDecorator_DelayedAbort {}
#[repr(C, align(8))]
pub struct UTestBTDecorator_LoopUntil {
    __padding_end: [u8; 176],
}
impl UTestBTDecorator_LoopUntil {}
#[repr(C, align(8))]
pub struct UTestBTDecorator_NodeMemoryValidator {
    __padding_end: [u8; 120],
}
impl UTestBTDecorator_NodeMemoryValidator {}
#[repr(C, align(8))]
pub struct UTestBTService_BTStopAction {
    __padding_end: [u8; 136],
}
impl UTestBTService_BTStopAction {}
#[repr(C, align(8))]
pub struct UTestBTService_Log {
    __padding_end: [u8; 192],
}
impl UTestBTService_Log {}
#[repr(C, align(8))]
pub struct UTestBTTask_BTStopAction {
    __padding_end: [u8; 144],
}
impl UTestBTTask_BTStopAction {}
#[repr(C, align(8))]
pub struct UTestBTTask_LatentWithFlags {
    __padding_end: [u8; 192],
}
impl UTestBTTask_LatentWithFlags {}
#[repr(C, align(8))]
pub struct UTestBTTask_Log {
    __padding_end: [u8; 152],
}
impl UTestBTTask_Log {}
#[repr(C, align(8))]
pub struct UTestBTTask_RunBehavior {
    __padding_end: [u8; 136],
}
impl UTestBTTask_RunBehavior {}
#[repr(C, align(8))]
pub struct UTestBTTask_SetFlag {
    __padding_end: [u8; 160],
}
impl UTestBTTask_SetFlag {}
#[repr(C, align(8))]
pub struct UTestBTTask_SetValue {
    __padding_end: [u8; 168],
}
impl UTestBTTask_SetValue {}
#[repr(C, align(8))]
pub struct UTestBTTask_SetValuesWithLogs {
    __padding_end: [u8; 200],
}
impl UTestBTTask_SetValuesWithLogs {}
#[repr(C, align(8))]
pub struct UTestBTTask_TimerBasedLatent {
    __padding_end: [u8; 160],
}
impl UTestBTTask_TimerBasedLatent {}
#[repr(C, align(8))]
pub struct UTestBTTask_ToggleFlag {
    __padding_end: [u8; 152],
}
impl UTestBTTask_ToggleFlag {}
#[repr(C, align(8))]
pub struct UMockAI {
    __padding_end: [u8; 96],
}
impl UMockAI {}
#[repr(C, align(8))]
pub struct UMockAI_BT {
    __padding_end: [u8; 120],
}
impl UMockAI_BT {}
#[repr(C, align(8))]
pub struct UMockTask_Log {
    __padding_end: [u8; 144],
}
impl UMockTask_Log {}
#[repr(C, align(8))]
pub struct UMockGameplayTasksComponent {
    __padding_end: [u8; 384],
}
impl UMockGameplayTasksComponent {}
#[repr(C, align(8))]
pub struct UMockGameplayTaskOwner {
    __padding_end: [u8; 64],
}
impl UMockGameplayTaskOwner {}
#[repr(transparent)]
pub struct EBPConditionType(pub i32);
impl EBPConditionType {
    pub const NO_CONDITION: EBPConditionType = EBPConditionType(0);
    pub const TRUE_CONDITION: EBPConditionType = EBPConditionType(1);
    pub const FALSE_CONDITION: EBPConditionType = EBPConditionType(2);
}
#[repr(transparent)]
pub struct EBTTestServiceStopTiming(pub u8);
impl EBTTestServiceStopTiming {
    pub const DURING_BECOME_RELEVANT: EBTTestServiceStopTiming = EBTTestServiceStopTiming(
        0,
    );
    pub const DURING_TICK: EBTTestServiceStopTiming = EBTTestServiceStopTiming(1);
    pub const DURING_CEASE_RELEVANT: EBTTestServiceStopTiming = EBTTestServiceStopTiming(
        2,
    );
}
#[repr(transparent)]
pub struct EBTTestStopAction(pub u8);
impl EBTTestStopAction {
    pub const STOP_TREE: EBTTestStopAction = EBTTestStopAction(0);
    pub const UN_INITIALIZE: EBTTestStopAction = EBTTestStopAction(1);
    pub const CLEANUP: EBTTestStopAction = EBTTestStopAction(2);
    pub const RESTART_FORCE_REEVALUATE_ROOT_NODE: EBTTestStopAction = EBTTestStopAction(
        3,
    );
    pub const RESTART_COMPLETE: EBTTestStopAction = EBTTestStopAction(4);
    pub const START_TREE: EBTTestStopAction = EBTTestStopAction(5);
}
#[repr(transparent)]
pub struct EBTTestTaskStopTiming(pub u8);
impl EBTTestTaskStopTiming {
    pub const DURING_EXECUTE: EBTTestTaskStopTiming = EBTTestTaskStopTiming(0);
    pub const DURING_TICK: EBTTestTaskStopTiming = EBTTestTaskStopTiming(1);
    pub const DURING_ABORT: EBTTestTaskStopTiming = EBTTestTaskStopTiming(2);
    pub const DURING_FINISH: EBTTestTaskStopTiming = EBTTestTaskStopTiming(3);
}
#[repr(transparent)]
pub struct EBTTestChangeFlagBehavior(pub u8);
impl EBTTestChangeFlagBehavior {
    pub const SET: EBTTestChangeFlagBehavior = EBTTestChangeFlagBehavior(0);
    pub const TOGGLE: EBTTestChangeFlagBehavior = EBTTestChangeFlagBehavior(1);
}
