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
pub static mut U_STATE_TREE_NODE_BLUEPRINT_BASE_SEND_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_NODE_BLUEPRINT_BASE_REQUEST_TRANSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_NODE_BLUEPRINT_BASE_RECEIVE_GET_DESCRIPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_NODE_BLUEPRINT_BASE_IS_PROPERTY_REF_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_NODE_BLUEPRINT_BASE_GET_PROPERTY_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_NODE_BLUEPRINT_BASE_GET_PROPERTY_DESCRIPTION_BY_PROPERTY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_CONDITION_BLUEPRINT_BASE_RECEIVE_TEST_CONDITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_CONSIDERATION_BLUEPRINT_BASE_RECEIVE_GET_SCORE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_EVALUATOR_BLUEPRINT_BASE_RECEIVE_TREE_STOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_EVALUATOR_BLUEPRINT_BASE_RECEIVE_TREE_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_EVALUATOR_BLUEPRINT_BASE_RECEIVE_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_TASK_BLUEPRINT_BASE_UNBIND_DELEGATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_STATE_COMPLETED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_LATENT_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_LATENT_ENTER_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_EXIT_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_ENTER_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_TASK_BLUEPRINT_BASE_FINISH_TASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_TASK_BLUEPRINT_BASE_BROADCAST_DELEGATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_TASK_BLUEPRINT_BASE_BIND_DELEGATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_K2_GET_EXTENSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_FUNCTION_LIBRARY_SET_STATE_TREE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_FUNCTION_LIBRARY_MAKE_STATE_TREE_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_FUNCTION_LIBRARY_K2_SET_PARAMETERS_PROPERTY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATE_TREE_FUNCTION_LIBRARY_K2_GET_PARAMETERS_PROPERTY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStateTreeNodeBlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SendEvent"),
            &raw mut U_STATE_TREE_NODE_BLUEPRINT_BASE_SEND_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestTransition"),
            &raw mut U_STATE_TREE_NODE_BLUEPRINT_BASE_REQUEST_TRANSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveGetDescription"),
            &raw mut U_STATE_TREE_NODE_BLUEPRINT_BASE_RECEIVE_GET_DESCRIPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPropertyRefValid"),
            &raw mut U_STATE_TREE_NODE_BLUEPRINT_BASE_IS_PROPERTY_REF_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPropertyReference"),
            &raw mut U_STATE_TREE_NODE_BLUEPRINT_BASE_GET_PROPERTY_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPropertyDescriptionByPropertyName"),
            &raw mut U_STATE_TREE_NODE_BLUEPRINT_BASE_GET_PROPERTY_DESCRIPTION_BY_PROPERTY_NAME,
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
            &raw mut U_STATE_TREE_CONDITION_BLUEPRINT_BASE_RECEIVE_TEST_CONDITION,
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
            &raw mut U_STATE_TREE_CONSIDERATION_BLUEPRINT_BASE_RECEIVE_GET_SCORE,
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
            &raw mut U_STATE_TREE_EVALUATOR_BLUEPRINT_BASE_RECEIVE_TREE_STOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTreeStart"),
            &raw mut U_STATE_TREE_EVALUATOR_BLUEPRINT_BASE_RECEIVE_TREE_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTick"),
            &raw mut U_STATE_TREE_EVALUATOR_BLUEPRINT_BASE_RECEIVE_TICK,
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
            &raw mut U_STATE_TREE_TASK_BLUEPRINT_BASE_UNBIND_DELEGATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTick"),
            &raw mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveStateCompleted"),
            &raw mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_STATE_COMPLETED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveLatentTick"),
            &raw mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_LATENT_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveLatentEnterState"),
            &raw mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_LATENT_ENTER_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveExitState"),
            &raw mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_EXIT_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveEnterState"),
            &raw mut U_STATE_TREE_TASK_BLUEPRINT_BASE_RECEIVE_ENTER_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinishTask"),
            &raw mut U_STATE_TREE_TASK_BLUEPRINT_BASE_FINISH_TASK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BroadcastDelegate"),
            &raw mut U_STATE_TREE_TASK_BLUEPRINT_BASE_BROADCAST_DELEGATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BindDelegate"),
            &raw mut U_STATE_TREE_TASK_BLUEPRINT_BASE_BIND_DELEGATE,
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
            &raw mut U_STATE_TREE_K2_GET_EXTENSION,
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
            &raw mut U_STATE_TREE_FUNCTION_LIBRARY_SET_STATE_TREE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeStateTreeReference"),
            &raw mut U_STATE_TREE_FUNCTION_LIBRARY_MAKE_STATE_TREE_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_SetParametersProperty"),
            &raw mut U_STATE_TREE_FUNCTION_LIBRARY_K2_SET_PARAMETERS_PROPERTY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetParametersProperty"),
            &raw mut U_STATE_TREE_FUNCTION_LIBRARY_K2_GET_PARAMETERS_PROPERTY,
        );
    }
}
#[repr(C, align(4))]
pub struct FStateTreeDelegateDispatcher {
    __padding_end: [u8; 16],
}
impl FStateTreeDelegateDispatcher {}
#[repr(C, align(4))]
pub struct FStateTreeDelegateListener {
    __padding_end: [u8; 20],
}
impl FStateTreeDelegateListener {}
#[repr(C, align(2))]
pub struct FStateTreeDataHandle {
    __padding_end: [u8; 6],
}
impl FStateTreeDataHandle {}
#[repr(C, align(2))]
pub struct FStateTreeStateHandle {
    __padding_end: [u8; 2],
}
impl FStateTreeStateHandle {}
#[repr(C, align(2))]
pub struct FStateTreeIndex16 {
    __padding_end: [u8; 2],
}
impl FStateTreeIndex16 {}
#[repr(C, align(8))]
pub struct FStateTreeBlueprintPropertyRef {
    __padding_end: [u8; 16],
}
impl FStateTreeBlueprintPropertyRef {}
#[repr(C, align(8))]
pub struct FStateTreeEvent {
    pub tag: crate::bindings::gameplay_tags::FGameplayTag,
    pub payload: crate::bindings::core_u_object::FInstancedStruct,
    pub origin: FName,
    __padding_end: [u8; 4],
}
impl FStateTreeEvent {}
#[repr(C, align(8))]
pub struct FStateTreeTransitionRequest {
    __padding_end: [u8; 32],
}
impl FStateTreeTransitionRequest {}
#[repr(C, align(4))]
pub struct FStateTreeActiveStates {
    __padding_end: [u8; 52],
}
impl FStateTreeActiveStates {}
#[repr(C, align(8))]
pub struct FStateTreeExecutionFrame {
    __padding_end: [u8; 112],
}
impl FStateTreeExecutionFrame {}
#[repr(C, align(8))]
pub struct FStateTreeTransitionResult {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub target_state: FStateTreeStateHandle,
    pub current_state: FStateTreeStateHandle,
    pub current_run_status: EStateTreeRunStatus,
    pub change_type: EStateTreeStateChangeType,
    pub priority: EStateTreeTransitionPriority,
    pub next_active_frames: TArray<FStateTreeExecutionFrame>,
    #[doc(hidden)]
    __padding_48: [u8; 16],
    pub source_state_tree: UPtr<UStateTree>,
    pub source_root_state: FStateTreeStateHandle,
    pub source_state: FStateTreeStateHandle,
    __padding_end: [u8; 4],
}
impl FStateTreeTransitionResult {}
#[repr(C, align(8))]
pub struct FStateTreeReference {
    __padding_end: [u8; 40],
}
impl FStateTreeReference {}
#[repr(C, align(8))]
pub struct FStateTreeStructRef {
    __padding_end: [u8; 16],
}
impl FStateTreeStructRef {}
#[repr(C, align(4))]
pub struct FStateTreeStateLink {
    __padding_end: [u8; 36],
}
impl FStateTreeStateLink {}
#[repr(C, align(1))]
pub struct FStateTreeIndex8 {
    __padding_end: [u8; 1],
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
