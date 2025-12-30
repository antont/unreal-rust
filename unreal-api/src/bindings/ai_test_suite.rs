#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UTestBTDecorator_Blackboard {
    pub log_index_become_relevant: i32,
    pub log_index_cease_relevant: i32,
    pub log_index_calculate: i32,
}
pub struct UTestBTDecorator_Blueprint {
    pub bp_condition_type: EBPConditionType,
    pub log_index_become_relevant: i32,
    pub log_index_cease_relevant: i32,
    pub log_index_calculate: i32,
    pub observing_key_name: FName,
}
pub struct UTestBTDecorator_CantExecute {}
pub struct UTestBTDecorator_DelayedAbort {
    pub delay_ticks: i32,
    pub b_only_once: bool,
}
pub struct UTestBTDecorator_LoopUntil {}
pub struct UTestBTDecorator_NodeMemoryValidator {}
pub struct UTestBTService_BTStopAction {
    pub log_index: i32,
    pub stop_timing: EBTTestServiceStopTiming,
    pub stop_action: EBTTestStopAction,
}
pub struct UTestBTService_Log {
    pub log_activation: i32,
    pub log_deactivation: i32,
    pub key_name_tick: FName,
    pub key_name_become_relevant: FName,
    pub key_name_cease_relevant: FName,
    pub log_tick: i32,
    pub ticks_delay_set_key_name_tick: i32,
    pub num_ticks: i32,
    pub b_toggle_value: bool,
}
pub struct UTestBTTask_BTStopAction {
    pub stop_timing: EBTTestTaskStopTiming,
    pub stop_action: EBTTestStopAction,
    pub log_index: i32,
    pub log_result: EBTNodeResult,
}
pub struct UTestBTTask_LatentWithFlags {
    pub log_index_execute_start: i32,
    pub log_index_executing: i32,
    pub log_index_execute_finish: i32,
    pub log_index_abort_start: i32,
    pub log_index_aborting: i32,
    pub log_index_abort_finish: i32,
    pub execute_half_ticks: i32,
    pub abort_half_ticks: i32,
    pub key_name_execute: FName,
    pub key_name_abort: FName,
    pub change_flag_behavior: EBTTestChangeFlagBehavior,
    pub log_result: EBTNodeResult,
}
pub struct UTestBTTask_Log {
    pub log_index: i32,
    pub log_finished: i32,
    pub execution_ticks: i32,
    pub log_tick_index: i32,
    pub log_result: EBTNodeResult,
}
pub struct UTestBTTask_RunBehavior {}
pub struct UTestBTTask_SetFlag {
    pub key_name: FName,
    pub b_value: bool,
    pub on_abort_key_name: FName,
    pub b_on_abort_value: bool,
    pub task_result: EBTNodeResult,
}
pub struct UTestBTTask_SetValue {
    pub key_name: FName,
    pub value: i32,
    pub on_abort_key_name: FName,
    pub on_abort_value: i32,
    pub task_result: EBTNodeResult,
}
pub struct UTestBTTask_SetValuesWithLogs {
    pub log_index: i32,
    pub log_finished: i32,
    pub execution_ticks1: i32,
    pub execution_ticks2: i32,
    pub log_tick_index: i32,
    pub key_name1: FName,
    pub value1: i32,
    pub key_name2: FName,
    pub value2: i32,
    pub on_abort_key_name: FName,
    pub on_abort_value: i32,
    pub task_result: EBTNodeResult,
}
pub struct UTestBTTask_TimerBasedLatent {
    pub log_index_execute_start: i32,
    pub log_index_execute_finish: i32,
    pub log_index_abort_start: i32,
    pub log_index_abort_finish: i32,
    pub num_ticks_executing: i32,
    pub num_ticks_aborting: i32,
    pub log_result: EBTNodeResult,
}
pub struct UTestBTTask_ToggleFlag {
    pub log_index: i32,
    pub key_name: FName,
    pub num_toggles: i32,
    pub task_result: EBTNodeResult,
}
pub struct UMockAI {
    pub actor: UPtr<AActor>,
    pub bb_comp: UPtr<UBlackboardComponent>,
    pub brain_comp: UPtr<UBrainComponent>,
    pub perception_comp: UPtr<UAIPerceptionComponent>,
}
pub struct UMockAI_BT {
    pub bt_comp: UPtr<UBehaviorTreeComponent>,
}
pub struct UMockTask_Log {}
pub struct UMockGameplayTasksComponent {}
pub struct UMockGameplayTaskOwner {
    pub gt_component: UPtr<UGameplayTasksComponent>,
}
