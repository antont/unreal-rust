#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FStateTreeTestLog {}
#[repr(C, align(4))]
pub struct FTestEval_AInstanceData {
    pub float_a: f32,
    pub int_a: i32,
    pub b_bool_a: bool,
}
#[repr(C, align(8))]
pub struct FTestEval_A {}
#[repr(C, align(16))]
pub struct FTestEval_Custom {}
#[repr(C, align(4))]
pub struct FTestTask_BInstanceData {
    pub float_b: f32,
    pub int_b: i32,
    pub b_bool_b: bool,
}
#[repr(C, align(8))]
pub struct FTestTask_B {}
#[repr(C, align(8))]
pub struct FTestTask_PrintValueInstanceData {
    pub value: i32,
    pub array_value: TArray<i32>,
    pub enter_state_run_status: EStateTreeRunStatus,
    pub tick_run_status: EStateTreeRunStatus,
}
#[repr(C, align(4))]
pub struct FTestTask_PrintValueExecutionRuntimeData {
    pub value: i32,
}
#[repr(C, align(16))]
pub struct FTestTask_PrintValue {}
#[repr(C, align(16))]
pub struct FTestTask_PrintAndResetValue {
    pub reset_value: i32,
    pub reset_array_value: TArray<i32>,
}
#[repr(C, align(16))]
pub struct FTestTask_PrintValue_TransitionTick {}
#[repr(C, align(16))]
pub struct FTestTask_PrintValue_TransitionNoTick {}
#[repr(C, align(1))]
pub struct FTestTask_StopTreeInstanceData {}
#[repr(C, align(8))]
pub struct FTestTask_StopTree {
    pub phase: EStateTreeUpdatePhase,
}
#[repr(C, align(4))]
pub struct FTestTask_StandInstanceData {
    pub value: i32,
    pub current_tick: i32,
}
#[repr(C, align(8))]
pub struct FTestTask_Stand {
    pub ticks_to_completion: i32,
    pub tick_completion_result: EStateTreeRunStatus,
    pub enter_state_result: EStateTreeRunStatus,
}
#[repr(C, align(8))]
pub struct FTestTask_StandNoTick {}
#[repr(C, align(4))]
pub struct FTestTask_IntegersOutput_InstanceData {
    pub int_a: i32,
    pub int_b: i32,
}
#[repr(C, align(16))]
pub struct FTestTask_IntegersOutput {}
#[repr(C, align(2))]
pub struct FTestTask_PropertyRefOnNodeAndInstance_InstanceData {
    pub ref_on_instance: FStateTreePropertyRef,
}
#[repr(C, align(16))]
pub struct FTestTask_PropertyRefOnNodeAndInstance {
    pub ref_on_node: FStateTreePropertyRef,
}
#[repr(C, align(4))]
pub struct FStateTreeTestConditionInstanceData {
    pub count: i32,
}
#[repr(C, align(8))]
pub struct FStateTreeTestCondition {
    pub b_test_condition_result: bool,
}
#[repr(C, align(4))]
pub struct FStateTreeTest_PropertyStructA {
    pub a: i32,
}
#[repr(C, align(4))]
pub struct FStateTreeTest_PropertyStructB {
    pub b: i32,
}
#[repr(C, align(4))]
pub struct FStateTreeTest_PropertyStruct {
    pub a: i32,
    pub b: i32,
    pub struct_b: FStateTreeTest_PropertyStructB,
}
#[repr(C, align(8))]
pub struct FStateTreeTest_PropertyCopy {
    pub item: FStateTreeTest_PropertyStruct,
    pub array: TArray<FStateTreeTest_PropertyStruct>,
    pub fixed_array: TArray<FStateTreeTest_PropertyStruct>,
    pub c_array: FStateTreeTest_PropertyStruct,
}
#[repr(C, align(8))]
pub struct FStateTreeTest_PropertyRefSourceStruct {
    pub item: FStateTreeTest_PropertyStruct,
    pub output_item: FStateTreeTest_PropertyStruct,
    pub array: TArray<FStateTreeTest_PropertyStruct>,
}
#[repr(C, align(2))]
pub struct FStateTreeTest_PropertyRefTargetStruct {
    pub ref_to_struct: FStateTreePropertyRef,
    pub ref_to_int: FStateTreePropertyRef,
    pub ref_to_struct_array: FStateTreePropertyRef,
}
#[repr(C, align(8))]
pub struct FStateTreeTest_PropertyCopyObjects {
    pub object: UPtr<UObject>,
    pub class: TSubclassOf<UObject>,
    pub soft_object: TSoftObjectPtr<UObject>,
    pub soft_class: TSoftObjectPtr<UClass>,
}
#[repr(C, align(4))]
pub struct FTestPropertyFunction_InstanceData {
    pub input: i32,
    pub result: i32,
}
#[repr(C, align(8))]
pub struct FTestPropertyFunction {}
#[repr(C, align(8))]
pub struct FTestTask_PrintValue_StructRef_NoBindingUpdateInstanceData {
    pub property_struct: FStateTreeTest_PropertyStruct,
    pub struct_ref: FStateTreeStructRef,
}
#[repr(C, align(16))]
pub struct FTestTask_PrintValue_StructRef_NoBindingUpdate {}
#[repr(C, align(1))]
pub struct FStateTreeTestBooleanConditionInstanceData {
    pub b_success: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeTestBooleanCondition {}
#[repr(C, align(4))]
pub struct FTestTask_BroadcastDelegate_InstanceData {
    pub on_enter_delegate: FStateTreeDelegateDispatcher,
    pub on_tick_delegate: FStateTreeDelegateDispatcher,
    pub on_exit_delegate: FStateTreeDelegateDispatcher,
}
#[repr(C, align(8))]
pub struct FTestTask_BroadcastDelegate {}
#[repr(C, align(4))]
pub struct FTestTask_ListenDelegate_InstanceData {
    pub listener: FStateTreeDelegateListener,
}
#[repr(C, align(8))]
pub struct FTestTask_ListenDelegate {}
#[repr(C, align(4))]
pub struct FTestTask_RebroadcastDelegate_InstanceData {
    pub listener: FStateTreeDelegateListener,
    pub dispatcher: FStateTreeDelegateDispatcher,
}
#[repr(C, align(8))]
pub struct FTestTask_RebroadcastDelegate {}
#[repr(C, align(4))]
pub struct FTestTask_CustomFuncOnDelegate_InstanceData {
    pub listener: FStateTreeDelegateListener,
}
#[repr(C, align(16))]
pub struct FTestTask_CustomFuncOnDelegate {}
#[repr(C, align(4))]
pub struct FTestTask_DispatcherOnNodeAndInstance_InstanceData {
    pub dispatcher_on_instance: FStateTreeDelegateDispatcher,
}
#[repr(C, align(8))]
pub struct FTestTask_DispatcherOnNodeAndInstance {
    pub dispatcher_on_node: FStateTreeDelegateDispatcher,
}
#[repr(C, align(1))]
pub struct FTestTask_ListenerOnNode_InstanceData {}
#[repr(C, align(16))]
pub struct FTestTask_ListenerOnNode {
    pub listener_on_node: FStateTreeDelegateListener,
}
#[repr(C, align(4))]
pub struct FTestTask_ListenerOnInstance_InstanceData {
    pub listener_on_instance: FStateTreeDelegateListener,
}
#[repr(C, align(16))]
pub struct FTestTask_ListenerOnInstance {}
#[repr(C, align(4))]
pub struct FIntWrapper {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FTestTask_OutputBindingsTaskInstanceData {
    pub input_int_wrapper: FIntWrapper,
    pub input_int_wrapper_array: TArray<FIntWrapper>,
    pub input_int_a: i32,
    pub input_int_b: i32,
    pub flags_32: u8,
    pub output_int: i32,
    pub output_int_wrapper: FIntWrapper,
    pub output_int_wrapper_array: TArray<FIntWrapper>,
}
#[repr(C, align(16))]
pub struct FTestTask_OutputBindingsTask {
    pub flags_192: u8,
}
pub struct UStateTreeTest_PropertyObjectInstanced {
    pub a: i32,
    pub instanced_struct: FInstancedStruct,
    pub array_of_tags: TArray<FGameplayTag>,
}
pub struct UStateTreeTest_PropertyObjectInstancedWithB {
    pub b: i32,
}
pub struct UStateTreeTest_PropertyObject {
    pub instanced_object: UPtr<UStateTreeTest_PropertyObjectInstanced>,
    pub array_of_instanced_objects: TArray<UPtr<UStateTreeTest_PropertyObjectInstanced>>,
    pub array_of_ints: TArray<i32>,
    pub instanced_struct: FInstancedStruct,
    pub array_of_instanced_structs: TArray<FInstancedStruct>,
    pub _struct: FStateTreeTest_PropertyStruct,
    pub array_of_struct: TArray<FStateTreeTest_PropertyStruct>,
}
pub struct UStateTreeTest_PropertyObject2 {}
pub struct UStateTreeTestSchema {
    pub default_rules: EStateTreeStateSelectionRules,
}
pub struct UStateTreeTestSchema2 {}
