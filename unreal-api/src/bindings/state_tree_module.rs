#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_state_tree_node_blueprint_base_send_event: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_node_blueprint_base_request_transition: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_node_blueprint_base_receive_get_description: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_node_blueprint_base_is_property_ref_valid: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_node_blueprint_base_get_property_reference: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_node_blueprint_base_get_property_description_by_property_name: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_condition_blueprint_base_receive_test_condition: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_consideration_blueprint_base_receive_get_score: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_evaluator_blueprint_base_receive_tree_stop: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_evaluator_blueprint_base_receive_tree_start: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_evaluator_blueprint_base_receive_tick: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_task_blueprint_base_unbind_delegate: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_task_blueprint_base_receive_tick: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_task_blueprint_base_receive_state_completed: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_task_blueprint_base_receive_latent_tick: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_task_blueprint_base_receive_latent_enter_state: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_task_blueprint_base_receive_exit_state: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_task_blueprint_base_receive_enter_state: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_task_blueprint_base_finish_task: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_task_blueprint_base_broadcast_delegate: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_task_blueprint_base_bind_delegate: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_k2_get_extension: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_function_library_set_state_tree: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_function_library_make_state_tree_reference: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_function_library_k2_set_parameters_property: *mut crate::ffi::UFunctionOpague,
    pub u_state_tree_function_library_k2_get_parameters_property: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_state_tree_node_blueprint_base_send_event: std::ptr::null_mut(),
            u_state_tree_node_blueprint_base_request_transition: std::ptr::null_mut(),
            u_state_tree_node_blueprint_base_receive_get_description: std::ptr::null_mut(),
            u_state_tree_node_blueprint_base_is_property_ref_valid: std::ptr::null_mut(),
            u_state_tree_node_blueprint_base_get_property_reference: std::ptr::null_mut(),
            u_state_tree_node_blueprint_base_get_property_description_by_property_name: std::ptr::null_mut(),
            u_state_tree_condition_blueprint_base_receive_test_condition: std::ptr::null_mut(),
            u_state_tree_consideration_blueprint_base_receive_get_score: std::ptr::null_mut(),
            u_state_tree_evaluator_blueprint_base_receive_tree_stop: std::ptr::null_mut(),
            u_state_tree_evaluator_blueprint_base_receive_tree_start: std::ptr::null_mut(),
            u_state_tree_evaluator_blueprint_base_receive_tick: std::ptr::null_mut(),
            u_state_tree_task_blueprint_base_unbind_delegate: std::ptr::null_mut(),
            u_state_tree_task_blueprint_base_receive_tick: std::ptr::null_mut(),
            u_state_tree_task_blueprint_base_receive_state_completed: std::ptr::null_mut(),
            u_state_tree_task_blueprint_base_receive_latent_tick: std::ptr::null_mut(),
            u_state_tree_task_blueprint_base_receive_latent_enter_state: std::ptr::null_mut(),
            u_state_tree_task_blueprint_base_receive_exit_state: std::ptr::null_mut(),
            u_state_tree_task_blueprint_base_receive_enter_state: std::ptr::null_mut(),
            u_state_tree_task_blueprint_base_finish_task: std::ptr::null_mut(),
            u_state_tree_task_blueprint_base_broadcast_delegate: std::ptr::null_mut(),
            u_state_tree_task_blueprint_base_bind_delegate: std::ptr::null_mut(),
            u_state_tree_k2_get_extension: std::ptr::null_mut(),
            u_state_tree_function_library_set_state_tree: std::ptr::null_mut(),
            u_state_tree_function_library_make_state_tree_reference: std::ptr::null_mut(),
            u_state_tree_function_library_k2_set_parameters_property: std::ptr::null_mut(),
            u_state_tree_function_library_k2_get_parameters_property: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStateTreeNodeBlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SendEvent"),
            &raw mut __FUNCTION_PTRS.u_state_tree_node_blueprint_base_send_event,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestTransition"),
            &raw mut __FUNCTION_PTRS.u_state_tree_node_blueprint_base_request_transition,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveGetDescription"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_node_blueprint_base_receive_get_description,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPropertyRefValid"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_node_blueprint_base_is_property_ref_valid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPropertyReference"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_node_blueprint_base_get_property_reference,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPropertyDescriptionByPropertyName"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_node_blueprint_base_get_property_description_by_property_name,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStateTreeConditionBlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTestCondition"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_condition_blueprint_base_receive_test_condition,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStateTreeConsiderationBlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveGetScore"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_consideration_blueprint_base_receive_get_score,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStateTreeEvaluatorBlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTreeStop"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_evaluator_blueprint_base_receive_tree_stop,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTreeStart"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_evaluator_blueprint_base_receive_tree_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTick"),
            &raw mut __FUNCTION_PTRS.u_state_tree_evaluator_blueprint_base_receive_tick,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStateTreeTaskBlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnbindDelegate"),
            &raw mut __FUNCTION_PTRS.u_state_tree_task_blueprint_base_unbind_delegate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTick"),
            &raw mut __FUNCTION_PTRS.u_state_tree_task_blueprint_base_receive_tick,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveStateCompleted"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_task_blueprint_base_receive_state_completed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveLatentTick"),
            &raw mut __FUNCTION_PTRS.u_state_tree_task_blueprint_base_receive_latent_tick,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveLatentEnterState"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_task_blueprint_base_receive_latent_enter_state,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveExitState"),
            &raw mut __FUNCTION_PTRS.u_state_tree_task_blueprint_base_receive_exit_state,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveEnterState"),
            &raw mut __FUNCTION_PTRS.u_state_tree_task_blueprint_base_receive_enter_state,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinishTask"),
            &raw mut __FUNCTION_PTRS.u_state_tree_task_blueprint_base_finish_task,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BroadcastDelegate"),
            &raw mut __FUNCTION_PTRS.u_state_tree_task_blueprint_base_broadcast_delegate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BindDelegate"),
            &raw mut __FUNCTION_PTRS.u_state_tree_task_blueprint_base_bind_delegate,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStateTree::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetExtension"),
            &raw mut __FUNCTION_PTRS.u_state_tree_k2_get_extension,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStateTreeFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStateTree"),
            &raw mut __FUNCTION_PTRS.u_state_tree_function_library_set_state_tree,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeStateTreeReference"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_function_library_make_state_tree_reference,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_SetParametersProperty"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_function_library_k2_set_parameters_property,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetParametersProperty"),
            &raw mut __FUNCTION_PTRS
                .u_state_tree_function_library_k2_get_parameters_property,
        );
    }
}
#[repr(C, align(4))]
pub struct FStateTreeDelegateDispatcher {
    pub(crate) __padding_end: [u8; 16],
}
impl FStateTreeDelegateDispatcher {}
#[repr(C, align(4))]
pub struct FStateTreeDelegateListener {
    pub(crate) __padding_end: [u8; 20],
}
impl FStateTreeDelegateListener {}
#[repr(C, align(2))]
pub struct FStateTreeDataHandle {
    pub(crate) __padding_end: [u8; 6],
}
impl FStateTreeDataHandle {}
#[repr(C, align(2))]
pub struct FStateTreeStateHandle {
    pub(crate) __padding_end: [u8; 2],
}
impl FStateTreeStateHandle {}
#[repr(C, align(2))]
pub struct FStateTreeIndex16 {
    pub(crate) __padding_end: [u8; 2],
}
impl FStateTreeIndex16 {}
#[repr(C, align(8))]
pub struct FStateTreeBlueprintPropertyRef {
    pub(crate) __padding_end: [u8; 16],
}
impl FStateTreeBlueprintPropertyRef {}
#[repr(C, align(8))]
pub struct FStateTreeEvent {
    pub tag: crate::bindings::gameplay_tags::FGameplayTag,
    pub payload: crate::bindings::core_u_object::FInstancedStruct,
    pub origin: FName,
}
impl FStateTreeEvent {}
#[repr(C, align(8))]
pub struct FStateTreeTransitionRequest {
    pub(crate) __padding_end: [u8; 32],
}
impl FStateTreeTransitionRequest {}
#[repr(C, align(4))]
pub struct FStateTreeActiveStates {
    pub(crate) __padding_end: [u8; 52],
}
impl FStateTreeActiveStates {}
#[repr(C, align(8))]
pub struct FStateTreeExecutionFrame {
    pub(crate) __padding_end: [u8; 112],
}
impl FStateTreeExecutionFrame {}
#[repr(C, align(8))]
pub struct FStateTreeTransitionResult {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub target_state: FStateTreeStateHandle,
    pub current_state: FStateTreeStateHandle,
    pub current_run_status: EStateTreeRunStatus,
    pub change_type: EStateTreeStateChangeType,
    pub priority: EStateTreeTransitionPriority,
    pub next_active_frames: TArray<FStateTreeExecutionFrame>,
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 16],
    pub source_state_tree: UPtr<UStateTree>,
    pub source_root_state: FStateTreeStateHandle,
    pub source_state: FStateTreeStateHandle,
}
impl FStateTreeTransitionResult {}
#[repr(C, align(8))]
pub struct FStateTreeReference {
    pub(crate) __padding_end: [u8; 40],
}
impl FStateTreeReference {}
#[repr(C, align(8))]
pub struct FStateTreeStructRef {
    pub(crate) __padding_end: [u8; 16],
}
impl FStateTreeStructRef {}
#[repr(C, align(4))]
pub struct FStateTreeStateLink {
    pub(crate) __padding_end: [u8; 36],
}
impl FStateTreeStateLink {}
#[repr(C, align(1))]
pub struct FStateTreeIndex8 {
    pub(crate) __padding_end: [u8; 1],
}
impl FStateTreeIndex8 {}
pub struct IStateTreeSchemaProvider {}
#[repr(C, align(8))]
pub struct UStateTreeSchemaProvider {
    __padding_end: [u8; 48],
}
impl UStateTreeSchemaProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeSchemaProvider")
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
pub struct UStateTreeSettings {
    __padding_end: [u8; 112],
}
impl UStateTreeSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeSettings")
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
pub struct UStateTreeNodeBlueprintBase {
    __padding_end: [u8; 176],
}
impl UStateTreeNodeBlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeNodeBlueprintBase")
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
    pub fn send_event(&mut self, event: &FStateTreeEvent) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_node_blueprint_base_send_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                event,
                __buffer.add(0).cast::<FStateTreeEvent>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_node_blueprint_base_send_event,
                __buffer,
            )
        };
    }
    pub fn request_transition(
        &mut self,
        target_state: &FStateTreeStateLink,
        priority: EStateTreeTransitionPriority,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<37>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_node_blueprint_base_request_transition,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_state,
                __buffer.add(0).cast::<FStateTreeStateLink>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &priority,
                __buffer.add(36).cast::<EStateTreeTransitionPriority>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_node_blueprint_base_request_transition,
                __buffer,
            )
        };
    }
    pub fn receive_get_description(
        &self,
        formatting: EStateTreeNodeFormatting,
    ) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_node_blueprint_base_receive_get_description,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &formatting,
                __buffer.add(0).cast::<EStateTreeNodeFormatting>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_node_blueprint_base_receive_get_description,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FText>().read() }
    }
    pub fn get_property_description_by_property_name(
        &self,
        property_name: FName,
    ) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_node_blueprint_base_get_property_description_by_property_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_node_blueprint_base_get_property_description_by_property_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FText>().read() }
    }
}
#[repr(C, align(8))]
pub struct UStateTreeConditionBlueprintBase {
    __padding_end: [u8; 184],
}
impl UStateTreeConditionBlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeConditionBlueprintBase")
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
    pub fn receive_test_condition(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_condition_blueprint_base_receive_test_condition,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_condition_blueprint_base_receive_test_condition,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UStateTreeConsiderationBlueprintBase {
    __padding_end: [u8; 184],
}
impl UStateTreeConsiderationBlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeConsiderationBlueprintBase")
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
    pub fn receive_get_score(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_consideration_blueprint_base_receive_get_score,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_consideration_blueprint_base_receive_get_score,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UStateTreeEvaluatorBlueprintBase {
    __padding_end: [u8; 184],
}
impl UStateTreeEvaluatorBlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeEvaluatorBlueprintBase")
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
    pub fn receive_tree_stop(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_evaluator_blueprint_base_receive_tree_stop,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_evaluator_blueprint_base_receive_tree_stop,
                __buffer,
            )
        };
    }
    pub fn receive_tree_start(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_evaluator_blueprint_base_receive_tree_start,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_evaluator_blueprint_base_receive_tree_start,
                __buffer,
            )
        };
    }
    pub fn receive_tick(&mut self, delta_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_evaluator_blueprint_base_receive_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_time, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_evaluator_blueprint_base_receive_tick,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UStateTreeTaskBlueprintBase {
    __padding_end: [u8; 208],
}
impl UStateTreeTaskBlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeTaskBlueprintBase")
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
    pub fn unbind_delegate(&mut self, listener: &FStateTreeDelegateListener) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_unbind_delegate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                listener,
                __buffer.add(0).cast::<FStateTreeDelegateListener>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_unbind_delegate,
                __buffer,
            )
        };
    }
    pub fn receive_tick(&mut self, delta_time: f32) -> EStateTreeRunStatus {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_time, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_tick,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<EStateTreeRunStatus>().read() }
    }
    pub fn receive_state_completed(
        &mut self,
        completion_status: EStateTreeRunStatus,
        completed_active_states: FStateTreeActiveStates,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_state_completed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &completion_status,
                __buffer.add(0).cast::<EStateTreeRunStatus>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &completed_active_states,
                __buffer.add(4).cast::<FStateTreeActiveStates>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_state_completed,
                __buffer,
            )
        };
    }
    pub fn receive_latent_tick(&mut self, delta_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_latent_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_time, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_latent_tick,
                __buffer,
            )
        };
    }
    pub fn receive_latent_enter_state(
        &mut self,
        transition: &FStateTreeTransitionResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_latent_enter_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                transition,
                __buffer.add(0).cast::<FStateTreeTransitionResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_latent_enter_state,
                __buffer,
            )
        };
    }
    pub fn receive_exit_state(&mut self, transition: &FStateTreeTransitionResult) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_exit_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                transition,
                __buffer.add(0).cast::<FStateTreeTransitionResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_exit_state,
                __buffer,
            )
        };
    }
    pub fn receive_enter_state(
        &mut self,
        transition: &FStateTreeTransitionResult,
    ) -> EStateTreeRunStatus {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_enter_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                transition,
                __buffer.add(0).cast::<FStateTreeTransitionResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_receive_enter_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<EStateTreeRunStatus>().read() }
    }
    pub fn finish_task(&mut self, b_succeeded: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_finish_task,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_succeeded,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_finish_task,
                __buffer,
            )
        };
    }
    pub fn broadcast_delegate(&mut self, dispatcher: FStateTreeDelegateDispatcher) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_broadcast_delegate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dispatcher,
                __buffer.add(0).cast::<FStateTreeDelegateDispatcher>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_broadcast_delegate,
                __buffer,
            )
        };
    }
    pub fn bind_delegate(
        &mut self,
        listener: &FStateTreeDelegateListener,
        delegate: &FBindDelegate_Delegate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_bind_delegate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                listener,
                __buffer.add(0).cast::<FStateTreeDelegateListener>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delegate,
                __buffer.add(24).cast::<FBindDelegate_Delegate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_task_blueprint_base_bind_delegate,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UStateTree {
    __padding_end: [u8; 816],
}
impl UStateTree {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTree")
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
    pub fn k2_get_extension(
        &self,
        extension_type: TSubclassOf<UStateTreeExtension>,
    ) -> UPtr<UStateTreeExtension> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_k2_get_extension,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &extension_type,
                __buffer.add(0).cast::<TSubclassOf<UStateTreeExtension>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_k2_get_extension,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UStateTreeExtension>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UStateTreeExtension {
    __padding_end: [u8; 48],
}
impl UStateTreeExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeExtension")
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
pub struct UStateTreeFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UStateTreeFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeFunctionLibrary")
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
    pub fn set_state_tree(
        reference: &mut FStateTreeReference,
        state_tree: UPtr<UStateTree>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_function_library_set_state_tree,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                reference,
                __buffer.add(0).cast::<FStateTreeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &state_tree,
                __buffer.add(40).cast::<UPtr<UStateTree>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::state_tree_module::UStateTreeFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_function_library_set_state_tree,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FStateTreeReference>().swap(reference);
        }
    }
    pub fn make_state_tree_reference(
        state_tree: UPtr<UStateTree>,
    ) -> FStateTreeReference {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_function_library_make_state_tree_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &state_tree,
                __buffer.add(0).cast::<UPtr<UStateTree>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::state_tree_module::UStateTreeFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_function_library_make_state_tree_reference,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FStateTreeReference>().read() }
    }
    pub fn k2_set_parameters_property(
        reference: &mut FStateTreeReference,
        property_id: crate::bindings::core_u_object::FGuid,
        new_value: &i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_function_library_k2_set_parameters_property,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                reference,
                __buffer.add(0).cast::<FStateTreeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_id,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(new_value, __buffer.add(56).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::state_tree_module::UStateTreeFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_function_library_k2_set_parameters_property,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FStateTreeReference>().swap(reference);
        }
    }
    pub fn k2_get_parameters_property(
        reference: &FStateTreeReference,
        property_id: crate::bindings::core_u_object::FGuid,
        return_value: &mut i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_function_library_k2_get_parameters_property,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                reference,
                __buffer.add(0).cast::<FStateTreeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_id,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                return_value,
                __buffer.add(56).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::state_tree_module::UStateTreeFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::state_tree_module::__FUNCTION_PTRS
                    .u_state_tree_function_library_k2_get_parameters_property,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UStateTreeSchema {
    __padding_end: [u8; 48],
}
impl UStateTreeSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeSchema")
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
pub struct FBindDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EStateTreeDataSourceType(pub u8);
impl EStateTreeDataSourceType {
    pub const NONE: EStateTreeDataSourceType = EStateTreeDataSourceType(0);
    pub const GLOBAL_INSTANCE_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        1,
    );
    pub const GLOBAL_INSTANCE_DATA_OBJECT: EStateTreeDataSourceType = EStateTreeDataSourceType(
        2,
    );
    pub const ACTIVE_INSTANCE_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        3,
    );
    pub const ACTIVE_INSTANCE_DATA_OBJECT: EStateTreeDataSourceType = EStateTreeDataSourceType(
        4,
    );
    pub const SHARED_INSTANCE_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        5,
    );
    pub const SHARED_INSTANCE_DATA_OBJECT: EStateTreeDataSourceType = EStateTreeDataSourceType(
        6,
    );
    pub const EVALUATION_SCOPE_INSTANCE_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        7,
    );
    pub const EVALUATION_SCOPE_INSTANCE_DATA_OBJECT: EStateTreeDataSourceType = EStateTreeDataSourceType(
        8,
    );
    pub const EXECUTION_RUNTIME_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        9,
    );
    pub const EXECUTION_RUNTIME_DATA_OBJECT: EStateTreeDataSourceType = EStateTreeDataSourceType(
        10,
    );
    pub const CONTEXT_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(11);
    pub const EXTERNAL_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(12);
    pub const GLOBAL_PARAMETER_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        13,
    );
    pub const SUBTREE_PARAMETER_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        14,
    );
    pub const STATE_PARAMETER_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        15,
    );
    pub const TRANSITION_EVENT: EStateTreeDataSourceType = EStateTreeDataSourceType(16);
    pub const STATE_EVENT: EStateTreeDataSourceType = EStateTreeDataSourceType(17);
    pub const EXTERNAL_GLOBAL_PARAMETER_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        18,
    );
}
#[repr(transparent)]
pub struct EStateTreePropertyRefType(pub u8);
impl EStateTreePropertyRefType {
    pub const NONE: EStateTreePropertyRefType = EStateTreePropertyRefType(0);
    pub const BOOL: EStateTreePropertyRefType = EStateTreePropertyRefType(1);
    pub const BYTE: EStateTreePropertyRefType = EStateTreePropertyRefType(2);
    pub const INT32: EStateTreePropertyRefType = EStateTreePropertyRefType(3);
    pub const INT64: EStateTreePropertyRefType = EStateTreePropertyRefType(4);
    pub const FLOAT: EStateTreePropertyRefType = EStateTreePropertyRefType(5);
    pub const DOUBLE: EStateTreePropertyRefType = EStateTreePropertyRefType(6);
    pub const NAME: EStateTreePropertyRefType = EStateTreePropertyRefType(7);
    pub const STRING: EStateTreePropertyRefType = EStateTreePropertyRefType(8);
    pub const TEXT: EStateTreePropertyRefType = EStateTreePropertyRefType(9);
    pub const ENUM: EStateTreePropertyRefType = EStateTreePropertyRefType(10);
    pub const STRUCT: EStateTreePropertyRefType = EStateTreePropertyRefType(11);
    pub const OBJECT: EStateTreePropertyRefType = EStateTreePropertyRefType(12);
    pub const SOFT_OBJECT: EStateTreePropertyRefType = EStateTreePropertyRefType(13);
    pub const CLASS: EStateTreePropertyRefType = EStateTreePropertyRefType(14);
    pub const SOFT_CLASS: EStateTreePropertyRefType = EStateTreePropertyRefType(15);
}
#[repr(transparent)]
pub struct EStateTreeExpressionOperand(pub u8);
impl EStateTreeExpressionOperand {
    pub const COPY: EStateTreeExpressionOperand = EStateTreeExpressionOperand(0);
    pub const AND: EStateTreeExpressionOperand = EStateTreeExpressionOperand(1);
    pub const OR: EStateTreeExpressionOperand = EStateTreeExpressionOperand(2);
    pub const MULTIPLY: EStateTreeExpressionOperand = EStateTreeExpressionOperand(3);
}
#[repr(transparent)]
pub struct EStateTreeConditionEvaluationMode(pub u8);
impl EStateTreeConditionEvaluationMode {
    pub const EVALUATED: EStateTreeConditionEvaluationMode = EStateTreeConditionEvaluationMode(
        0,
    );
    pub const FORCED_TRUE: EStateTreeConditionEvaluationMode = EStateTreeConditionEvaluationMode(
        1,
    );
    pub const FORCED_FALSE: EStateTreeConditionEvaluationMode = EStateTreeConditionEvaluationMode(
        2,
    );
}
#[repr(transparent)]
pub struct EStateTreeTransitionPriority(pub u8);
impl EStateTreeTransitionPriority {
    pub const NONE: EStateTreeTransitionPriority = EStateTreeTransitionPriority(0);
    pub const LOW: EStateTreeTransitionPriority = EStateTreeTransitionPriority(1);
    pub const NORMAL: EStateTreeTransitionPriority = EStateTreeTransitionPriority(2);
    pub const MEDIUM: EStateTreeTransitionPriority = EStateTreeTransitionPriority(3);
    pub const HIGH: EStateTreeTransitionPriority = EStateTreeTransitionPriority(4);
    pub const CRITICAL: EStateTreeTransitionPriority = EStateTreeTransitionPriority(5);
}
#[repr(transparent)]
pub struct EStateTreeExternalDataRequirement(pub u8);
impl EStateTreeExternalDataRequirement {
    pub const REQUIRED: EStateTreeExternalDataRequirement = EStateTreeExternalDataRequirement(
        0,
    );
    pub const OPTIONAL: EStateTreeExternalDataRequirement = EStateTreeExternalDataRequirement(
        1,
    );
}
#[repr(transparent)]
pub struct EStateTreeSelectionFallback(pub u8);
impl EStateTreeSelectionFallback {
    pub const NONE: EStateTreeSelectionFallback = EStateTreeSelectionFallback(0);
    pub const NEXT_SELECTABLE_SIBLING: EStateTreeSelectionFallback = EStateTreeSelectionFallback(
        1,
    );
}
#[repr(transparent)]
pub struct EStateTreeRunStatus(pub u8);
impl EStateTreeRunStatus {
    pub const RUNNING: EStateTreeRunStatus = EStateTreeRunStatus(0);
    pub const STOPPED: EStateTreeRunStatus = EStateTreeRunStatus(1);
    pub const SUCCEEDED: EStateTreeRunStatus = EStateTreeRunStatus(2);
    pub const FAILED: EStateTreeRunStatus = EStateTreeRunStatus(3);
    pub const UNSET: EStateTreeRunStatus = EStateTreeRunStatus(4);
}
#[repr(transparent)]
pub struct EStateTreeUpdatePhase(pub u8);
impl EStateTreeUpdatePhase {
    pub const UNSET: EStateTreeUpdatePhase = EStateTreeUpdatePhase(0);
    pub const START_TREE: EStateTreeUpdatePhase = EStateTreeUpdatePhase(1);
    pub const STOP_TREE: EStateTreeUpdatePhase = EStateTreeUpdatePhase(2);
    pub const START_GLOBAL_TASKS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(3);
    pub const START_GLOBAL_TASKS_FOR_SELECTION: EStateTreeUpdatePhase = EStateTreeUpdatePhase(
        4,
    );
    pub const STOP_GLOBAL_TASKS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(5);
    pub const STOP_GLOBAL_TASKS_FOR_SELECTION: EStateTreeUpdatePhase = EStateTreeUpdatePhase(
        6,
    );
    pub const TICK_STATE_TREE: EStateTreeUpdatePhase = EStateTreeUpdatePhase(7);
    pub const APPLY_TRANSITIONS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(8);
    pub const TICK_TRANSITIONS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(9);
    pub const TRIGGER_TRANSITIONS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(10);
    pub const TICKING_GLOBAL_TASKS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(11);
    pub const TICKING_TASKS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(12);
    pub const TRANSITION_CONDITIONS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(13);
    pub const STATE_SELECTION: EStateTreeUpdatePhase = EStateTreeUpdatePhase(14);
    pub const TRY_SELECT_BEHAVIOR: EStateTreeUpdatePhase = EStateTreeUpdatePhase(15);
    pub const ENTER_CONDITIONS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(16);
    pub const ENTER_STATES: EStateTreeUpdatePhase = EStateTreeUpdatePhase(17);
    pub const EXIT_STATES: EStateTreeUpdatePhase = EStateTreeUpdatePhase(18);
    pub const STATE_COMPLETED: EStateTreeUpdatePhase = EStateTreeUpdatePhase(19);
}
#[repr(transparent)]
pub struct EStateTreeStateChangeType(pub u8);
impl EStateTreeStateChangeType {
    pub const NONE: EStateTreeStateChangeType = EStateTreeStateChangeType(0);
    pub const CHANGED: EStateTreeStateChangeType = EStateTreeStateChangeType(1);
    pub const SUSTAINED: EStateTreeStateChangeType = EStateTreeStateChangeType(2);
}
#[repr(transparent)]
pub struct EStateTreeBindableStructSource(pub u8);
impl EStateTreeBindableStructSource {
    pub const CONTEXT: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        0,
    );
    pub const PARAMETER: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        1,
    );
    pub const EVALUATOR: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        2,
    );
    pub const GLOBAL_TASK: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        3,
    );
    pub const STATE_PARAMETER: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        4,
    );
    pub const TASK: EStateTreeBindableStructSource = EStateTreeBindableStructSource(5);
    pub const CONDITION: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        6,
    );
    pub const CONSIDERATION: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        7,
    );
    pub const TRANSITION_EVENT: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        8,
    );
    pub const STATE_EVENT: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        9,
    );
    pub const PROPERTY_FUNCTION: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        10,
    );
    pub const TRANSITION: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        11,
    );
}
#[repr(transparent)]
pub struct EStateTreeTransitionTrigger(pub u8);
impl EStateTreeTransitionTrigger {
    pub const NONE: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(0);
    pub const ON_STATE_COMPLETED: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(
        3,
    );
    pub const ON_STATE_SUCCEEDED: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(
        1,
    );
    pub const ON_STATE_FAILED: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(
        2,
    );
    pub const ON_TICK: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(4);
    pub const ON_EVENT: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(8);
    pub const ON_DELEGATE: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(16);
    pub const MAX: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(17);
}
#[repr(transparent)]
pub struct EStateTreeTaskCompletionType(pub u8);
impl EStateTreeTaskCompletionType {
    pub const ALL: EStateTreeTaskCompletionType = EStateTreeTaskCompletionType(0);
    pub const ANY: EStateTreeTaskCompletionType = EStateTreeTaskCompletionType(1);
}
#[repr(transparent)]
pub struct EStateTreeStateType(pub u8);
impl EStateTreeStateType {
    pub const STATE: EStateTreeStateType = EStateTreeStateType(0);
    pub const GROUP: EStateTreeStateType = EStateTreeStateType(1);
    pub const LINKED: EStateTreeStateType = EStateTreeStateType(2);
    pub const LINKED_ASSET: EStateTreeStateType = EStateTreeStateType(3);
    pub const SUBTREE: EStateTreeStateType = EStateTreeStateType(4);
}
#[repr(transparent)]
pub struct EStateTreeStateSelectionBehavior(pub u8);
impl EStateTreeStateSelectionBehavior {
    pub const NONE: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        0,
    );
    pub const TRY_ENTER_STATE: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        1,
    );
    pub const TRY_SELECT_CHILDREN_IN_ORDER: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        2,
    );
    pub const TRY_SELECT_CHILDREN_AT_RANDOM: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        3,
    );
    pub const TRY_SELECT_CHILDREN_WITH_HIGHEST_UTILITY: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        4,
    );
    pub const TRY_SELECT_CHILDREN_AT_RANDOM_WEIGHTED_BY_UTILITY: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        5,
    );
    pub const TRY_FOLLOW_TRANSITIONS: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        6,
    );
    pub const TRY_SELECT_CHILDREN_AT_UNIFORM_RANDOM: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        3,
    );
    pub const TRY_SELECT_CHILDREN_BASED_ON_RELATIVE_UTILITY: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        5,
    );
}
#[repr(transparent)]
pub struct EStateTreeTransitionType(pub u8);
impl EStateTreeTransitionType {
    pub const NONE: EStateTreeTransitionType = EStateTreeTransitionType(0);
    pub const SUCCEEDED: EStateTreeTransitionType = EStateTreeTransitionType(1);
    pub const FAILED: EStateTreeTransitionType = EStateTreeTransitionType(2);
    pub const GOTO_STATE: EStateTreeTransitionType = EStateTreeTransitionType(3);
    pub const NEXT_STATE: EStateTreeTransitionType = EStateTreeTransitionType(4);
    pub const NEXT_SELECTABLE_STATE: EStateTreeTransitionType = EStateTreeTransitionType(
        5,
    );
    pub const NOT_SET: EStateTreeTransitionType = EStateTreeTransitionType(6);
}
#[repr(transparent)]
pub struct EStateTreeBreakpointType(pub u8);
impl EStateTreeBreakpointType {
    pub const UNSET: EStateTreeBreakpointType = EStateTreeBreakpointType(0);
    pub const ON_ENTER: EStateTreeBreakpointType = EStateTreeBreakpointType(1);
    pub const ON_EXIT: EStateTreeBreakpointType = EStateTreeBreakpointType(2);
    pub const ON_TRANSITION: EStateTreeBreakpointType = EStateTreeBreakpointType(3);
}
#[repr(transparent)]
pub struct EStateTreeNodeFormatting(pub u8);
impl EStateTreeNodeFormatting {
    pub const RICH_TEXT: EStateTreeNodeFormatting = EStateTreeNodeFormatting(0);
    pub const TEXT: EStateTreeNodeFormatting = EStateTreeNodeFormatting(1);
}
#[repr(transparent)]
pub struct EStateTreeParameterDataType(pub u8);
impl EStateTreeParameterDataType {
    pub const GLOBAL_PARAMETER_DATA: EStateTreeParameterDataType = EStateTreeParameterDataType(
        0,
    );
    pub const EXTERNAL_GLOBAL_PARAMETER_DATA: EStateTreeParameterDataType = EStateTreeParameterDataType(
        1,
    );
}
#[repr(transparent)]
pub struct EStateTreeStateSelectionRules(pub u32);
impl EStateTreeStateSelectionRules {
    pub const NONE: EStateTreeStateSelectionRules = EStateTreeStateSelectionRules(0);
    pub const COMPLETED_TRANSITION_STATES_CREATE_NEW_STATES: EStateTreeStateSelectionRules = EStateTreeStateSelectionRules(
        1,
    );
    pub const COMPLETED_STATE_BEFORE_TRANSITION_SOURCE_FAILS_TRANSITION: EStateTreeStateSelectionRules = EStateTreeStateSelectionRules(
        2,
    );
    pub const RESELECTED_STATE_CREATES_NEW_STATES: EStateTreeStateSelectionRules = EStateTreeStateSelectionRules(
        4,
    );
    pub const DEFAULT: EStateTreeStateSelectionRules = EStateTreeStateSelectionRules(3);
}
