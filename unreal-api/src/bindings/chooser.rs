#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_chooser_table_result_asset_filter: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_make_evaluate_chooser: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_make_chooser_evaluation_context: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_get_chooser_struct_output: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_get_chooser_object_input: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_evaluate_object_chooser_base_soft: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_evaluate_object_chooser_base_multi_soft: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_evaluate_object_chooser_base_multi: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_evaluate_object_chooser_base: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_evaluate_chooser_multi: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_evaluate_chooser: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_add_chooser_struct_input: *mut crate::ffi::UFunctionOpague,
    pub u_chooser_function_library_add_chooser_object_input: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_chooser_table_result_asset_filter: std::ptr::null_mut(),
            u_chooser_function_library_make_evaluate_chooser: std::ptr::null_mut(),
            u_chooser_function_library_make_chooser_evaluation_context: std::ptr::null_mut(),
            u_chooser_function_library_get_chooser_struct_output: std::ptr::null_mut(),
            u_chooser_function_library_get_chooser_object_input: std::ptr::null_mut(),
            u_chooser_function_library_evaluate_object_chooser_base_soft: std::ptr::null_mut(),
            u_chooser_function_library_evaluate_object_chooser_base_multi_soft: std::ptr::null_mut(),
            u_chooser_function_library_evaluate_object_chooser_base_multi: std::ptr::null_mut(),
            u_chooser_function_library_evaluate_object_chooser_base: std::ptr::null_mut(),
            u_chooser_function_library_evaluate_chooser_multi: std::ptr::null_mut(),
            u_chooser_function_library_evaluate_chooser: std::ptr::null_mut(),
            u_chooser_function_library_add_chooser_struct_input: std::ptr::null_mut(),
            u_chooser_function_library_add_chooser_object_input: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChooserTable::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResultAssetFilter"),
                &raw mut __FUNCTION_PTRS.u_chooser_table_result_asset_filter,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UChooserFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeEvaluateChooser"),
                &raw mut __FUNCTION_PTRS.u_chooser_function_library_make_evaluate_chooser,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeChooserEvaluationContext"),
                &raw mut __FUNCTION_PTRS
                    .u_chooser_function_library_make_chooser_evaluation_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetChooserStructOutput"),
                &raw mut __FUNCTION_PTRS
                    .u_chooser_function_library_get_chooser_struct_output,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetChooserObjectInput"),
                &raw mut __FUNCTION_PTRS
                    .u_chooser_function_library_get_chooser_object_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateObjectChooserBaseSoft"),
                &raw mut __FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base_soft,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateObjectChooserBaseMultiSoft"),
                &raw mut __FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base_multi_soft,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateObjectChooserBaseMulti"),
                &raw mut __FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base_multi,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateObjectChooserBase"),
                &raw mut __FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateChooserMulti"),
                &raw mut __FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_chooser_multi,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateChooser"),
                &raw mut __FUNCTION_PTRS.u_chooser_function_library_evaluate_chooser,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddChooserStructInput"),
                &raw mut __FUNCTION_PTRS
                    .u_chooser_function_library_add_chooser_struct_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddChooserObjectInput"),
                &raw mut __FUNCTION_PTRS
                    .u_chooser_function_library_add_chooser_object_input,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FAnimCurveOverride {
    pub curve_name: FName,
    pub curve_value: f32,
}
impl FAnimCurveOverride {}
#[repr(C, align(8))]
pub struct FAnimCurveOverrideList {
    pub values: TArray<FAnimCurveOverride>,
    pub(crate) __padding_end: [u8; 8],
}
impl FAnimCurveOverrideList {}
#[repr(C, align(8))]
pub struct FChooserPlayerSettings {
    pub b_mirror: bool,
    pub start_time: f32,
    pub b_force_looping: bool,
    pub playback_rate: f32,
    pub curve_overrides: FAnimCurveOverrideList,
    pub blend_time: f32,
    pub blend_profile: UPtr<crate::bindings::engine::UBlendProfile>,
    pub blend_option: crate::bindings::engine::EAlphaBlendOption,
    pub b_use_inertial_blend: bool,
    pub b_force_blend_to: bool,
    pub min_delta_time_to_force_blend_to: f32,
    pub(crate) __padding_end: [u8; 48],
}
impl FChooserPlayerSettings {}
#[repr(C, align(8))]
pub struct FChooserRandomizationContext {
    pub(crate) __padding_end: [u8; 80],
}
impl FChooserRandomizationContext {}
#[repr(C, align(8))]
pub struct FChooserEvaluationContext {
    pub(crate) __padding_end: [u8; 136],
}
impl FChooserEvaluationContext {}
#[repr(C, align(8))]
pub struct FAnimNode_ChooserPlayer {
    #[doc(hidden)]
    pub(crate) __padding_392: [u8; 392],
    pub blend_space_x: f32,
    pub blend_space_y: f32,
    #[doc(hidden)]
    pub(crate) __padding_408: [u8; 8],
    pub default_settings: FChooserPlayerSettings,
    pub(crate) __padding_end: [u8; 192],
}
impl FAnimNode_ChooserPlayer {}
pub struct IChooserColumn {}
#[repr(C, align(8))]
pub struct UChooserColumn {
    __padding_end: [u8; 48],
}
impl UChooserColumn {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserColumn")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserColumn")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IChooserParameterBool {}
#[repr(C, align(8))]
pub struct UChooserParameterBool {
    __padding_end: [u8; 48],
}
impl UChooserParameterBool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserParameterBool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserParameterBool")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IChooserParameterEnum {}
#[repr(C, align(8))]
pub struct UChooserParameterEnum {
    __padding_end: [u8; 48],
}
impl UChooserParameterEnum {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserParameterEnum")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserParameterEnum")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IChooserParameterFloat {}
#[repr(C, align(8))]
pub struct UChooserParameterFloat {
    __padding_end: [u8; 48],
}
impl UChooserParameterFloat {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserParameterFloat")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserParameterFloat")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IChooserParameterGameplayTag {}
#[repr(C, align(8))]
pub struct UChooserParameterGameplayTag {
    __padding_end: [u8; 48],
}
impl UChooserParameterGameplayTag {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserParameterGameplayTag")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserParameterGameplayTag")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IHasContextClass {}
#[repr(C, align(8))]
pub struct UHasContextClass {
    __padding_end: [u8; 48],
}
impl UHasContextClass {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHasContextClass")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHasContextClass")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IObjectChooser {}
#[repr(C, align(8))]
pub struct UObjectChooser {
    __padding_end: [u8; 48],
}
impl UObjectChooser {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectChooser")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectChooser")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_ChooserParameterBool_ContextProperty {
    __padding_end: [u8; 72],
}
impl UDEPRECATED_ChooserParameterBool_ContextProperty {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserParameterBool_ContextProperty")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserParameterBool_ContextProperty")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_ChooserColumnBool {
    __padding_end: [u8; 88],
}
impl UDEPRECATED_ChooserColumnBool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserColumnBool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserColumnBool")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UChooserSignature {
    __padding_end: [u8; 120],
}
impl UChooserSignature {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserSignature")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserSignature")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UChooserTable {
    __padding_end: [u8; 520],
}
impl UChooserTable {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserTable")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserTable")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_ObjectChooser_EvaluateChooser {
    __padding_end: [u8; 64],
}
impl UDEPRECATED_ObjectChooser_EvaluateChooser {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ObjectChooser_EvaluateChooser")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ObjectChooser_EvaluateChooser")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UChooserColumnMenuContext {
    __padding_end: [u8; 72],
}
impl UChooserColumnMenuContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserColumnMenuContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserColumnMenuContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UChooserFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UChooserFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserFunctionLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn make_evaluate_chooser(
        chooser: UPtr<UChooserTable>,
    ) -> crate::bindings::core_u_object::FInstancedStruct {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_make_evaluate_chooser,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chooser,
                __buffer.add(0).cast::<UPtr<UChooserTable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_make_evaluate_chooser,
                __buffer,
            )
        };
        std::mem::forget(chooser);
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::core_u_object::FInstancedStruct>()
                .read()
        }
    }
    pub fn make_chooser_evaluation_context() -> FChooserEvaluationContext {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_make_chooser_evaluation_context,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_make_chooser_evaluation_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FChooserEvaluationContext>().read() }
    }
    pub fn get_chooser_struct_output(
        context: &mut FChooserEvaluationContext,
        index: i32,
        value: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_get_chooser_struct_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FChooserEvaluationContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(136).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(140).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_get_chooser_struct_output,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FChooserEvaluationContext>().swap(context);
        }
        unsafe {
            __buffer.add(140).cast::<i32>().swap(value);
        }
        std::mem::forget(index);
    }
    pub fn get_chooser_object_input(
        context: &mut FChooserEvaluationContext,
        index: i32,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_get_chooser_object_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FChooserEvaluationContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(136).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(144)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_get_chooser_object_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FChooserEvaluationContext>().swap(context);
        }
        std::mem::forget(index);
        std::mem::forget(object_class);
        unsafe {
            __buffer
                .add(152)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn evaluate_object_chooser_base_soft(
        context: &mut FChooserEvaluationContext,
        object_chooser: &crate::bindings::core_u_object::FInstancedStruct,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        b_result_is_class: bool,
    ) -> TSoftObjectPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<216>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base_soft,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FChooserEvaluationContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_chooser,
                __buffer
                    .add(136)
                    .cast::<crate::bindings::core_u_object::FInstancedStruct>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(152)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_result_is_class,
                __buffer.add(160).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base_soft,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FChooserEvaluationContext>().swap(context);
        }
        std::mem::forget(object_class);
        std::mem::forget(b_result_is_class);
        unsafe {
            __buffer
                .add(168)
                .cast::<TSoftObjectPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn evaluate_object_chooser_base_multi_soft(
        context: &mut FChooserEvaluationContext,
        object_chooser: &crate::bindings::core_u_object::FInstancedStruct,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        b_result_is_class: bool,
    ) -> TArray<TSoftObjectPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base_multi_soft,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FChooserEvaluationContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_chooser,
                __buffer
                    .add(136)
                    .cast::<crate::bindings::core_u_object::FInstancedStruct>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(152)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_result_is_class,
                __buffer.add(160).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base_multi_soft,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FChooserEvaluationContext>().swap(context);
        }
        std::mem::forget(object_class);
        std::mem::forget(b_result_is_class);
        unsafe {
            __buffer
                .add(168)
                .cast::<
                    TArray<TSoftObjectPtr<crate::bindings::core_u_object::UObject>>,
                >()
                .read()
        }
    }
    pub fn evaluate_object_chooser_base_multi(
        context: &mut FChooserEvaluationContext,
        object_chooser: &crate::bindings::core_u_object::FInstancedStruct,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        b_result_is_class: bool,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base_multi,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FChooserEvaluationContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_chooser,
                __buffer
                    .add(136)
                    .cast::<crate::bindings::core_u_object::FInstancedStruct>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(152)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_result_is_class,
                __buffer.add(160).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base_multi,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FChooserEvaluationContext>().swap(context);
        }
        std::mem::forget(object_class);
        std::mem::forget(b_result_is_class);
        unsafe {
            __buffer
                .add(168)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn evaluate_object_chooser_base(
        context: &mut FChooserEvaluationContext,
        object_chooser: &crate::bindings::core_u_object::FInstancedStruct,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        b_result_is_class: bool,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<176>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FChooserEvaluationContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_chooser,
                __buffer
                    .add(136)
                    .cast::<crate::bindings::core_u_object::FInstancedStruct>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(152)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_result_is_class,
                __buffer.add(160).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_object_chooser_base,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FChooserEvaluationContext>().swap(context);
        }
        std::mem::forget(object_class);
        std::mem::forget(b_result_is_class);
        unsafe {
            __buffer
                .add(168)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn evaluate_chooser_multi(
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
        chooser_table: UPtr<UChooserTable>,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_chooser_multi,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chooser_table,
                __buffer.add(8).cast::<UPtr<UChooserTable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_chooser_multi,
                __buffer,
            )
        };
        std::mem::forget(context_object);
        std::mem::forget(chooser_table);
        std::mem::forget(object_class);
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn evaluate_chooser(
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
        chooser_table: UPtr<UChooserTable>,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_chooser,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chooser_table,
                __buffer.add(8).cast::<UPtr<UChooserTable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_evaluate_chooser,
                __buffer,
            )
        };
        std::mem::forget(context_object);
        std::mem::forget(chooser_table);
        std::mem::forget(object_class);
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn add_chooser_struct_input(
        context: &mut FChooserEvaluationContext,
        value: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<140>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_add_chooser_struct_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FChooserEvaluationContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(136).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_add_chooser_struct_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FChooserEvaluationContext>().swap(context);
        }
        std::mem::forget(value);
    }
    pub fn add_chooser_object_input(
        context: &mut FChooserEvaluationContext,
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_add_chooser_object_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FChooserEvaluationContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer
                    .add(136)
                    .cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chooser::UChooserFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chooser::__FUNCTION_PTRS
                    .u_chooser_function_library_add_chooser_object_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FChooserEvaluationContext>().swap(context);
        }
        std::mem::forget(object);
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_ChooserParameterEnum_ContextProperty {
    __padding_end: [u8; 72],
}
impl UDEPRECATED_ChooserParameterEnum_ContextProperty {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserParameterEnum_ContextProperty")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserParameterEnum_ContextProperty")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_ChooserColumnEnum {
    __padding_end: [u8; 88],
}
impl UDEPRECATED_ChooserColumnEnum {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserColumnEnum")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserColumnEnum")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFloatAutoPopulator {
    __padding_end: [u8; 48],
}
impl UFloatAutoPopulator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloatAutoPopulator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloatAutoPopulator")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_ChooserParameterFloat_ContextProperty {
    __padding_end: [u8; 72],
}
impl UDEPRECATED_ChooserParameterFloat_ContextProperty {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserParameterFloat_ContextProperty")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserParameterFloat_ContextProperty")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_ChooserColumnFloatRange {
    __padding_end: [u8; 88],
}
impl UDEPRECATED_ChooserColumnFloatRange {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserColumnFloatRange")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserColumnFloatRange")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_ChooserParameterGameplayTag_ContextProperty {
    __padding_end: [u8; 72],
}
impl UDEPRECATED_ChooserParameterGameplayTag_ContextProperty {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserParameterGameplayTag_ContextProperty")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserParameterGameplayTag_ContextProperty")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_ChooserColumnGameplayTag {
    __padding_end: [u8; 96],
}
impl UDEPRECATED_ChooserColumnGameplayTag {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserColumnGameplayTag")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ChooserColumnGameplayTag")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_ObjectChooser_Asset {
    __padding_end: [u8; 64],
}
impl UDEPRECATED_ObjectChooser_Asset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ObjectChooser_Asset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ObjectChooser_Asset")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(transparent)]
pub struct EChooserEvaluationFrequency(pub i32);
impl EChooserEvaluationFrequency {
    pub const ON_INITIAL_UPDATE: EChooserEvaluationFrequency = EChooserEvaluationFrequency(
        0,
    );
    pub const ON_BECOME_RELEVANT: EChooserEvaluationFrequency = EChooserEvaluationFrequency(
        1,
    );
    pub const ON_LOOP: EChooserEvaluationFrequency = EChooserEvaluationFrequency(2);
    pub const ON_UPDATE: EChooserEvaluationFrequency = EChooserEvaluationFrequency(3);
}
#[repr(transparent)]
pub struct EBoolColumnCellValue(pub u8);
impl EBoolColumnCellValue {
    pub const MATCH_FALSE: EBoolColumnCellValue = EBoolColumnCellValue(0);
    pub const MATCH_TRUE: EBoolColumnCellValue = EBoolColumnCellValue(1);
    pub const MATCH_ANY: EBoolColumnCellValue = EBoolColumnCellValue(2);
}
#[repr(transparent)]
pub struct EContextObjectDirection(pub i32);
impl EContextObjectDirection {
    pub const READ: EContextObjectDirection = EContextObjectDirection(0);
    pub const WRITE: EContextObjectDirection = EContextObjectDirection(1);
    pub const READ_WRITE: EContextObjectDirection = EContextObjectDirection(2);
}
#[repr(transparent)]
pub struct EEnumColumnCellValueComparison(pub i32);
impl EEnumColumnCellValueComparison {
    pub const MATCH_EQUAL: EEnumColumnCellValueComparison = EEnumColumnCellValueComparison(
        0,
    );
    pub const MATCH_NOT_EQUAL: EEnumColumnCellValueComparison = EEnumColumnCellValueComparison(
        1,
    );
    pub const MATCH_ANY: EEnumColumnCellValueComparison = EEnumColumnCellValueComparison(
        2,
    );
    pub const MODULUS: EEnumColumnCellValueComparison = EEnumColumnCellValueComparison(
        3,
    );
}
#[repr(transparent)]
pub struct EGameplayTagMatchDirection(pub u8);
impl EGameplayTagMatchDirection {
    pub const ROW_VALUE_IN_INPUT: EGameplayTagMatchDirection = EGameplayTagMatchDirection(
        0,
    );
    pub const INPUT_IN_ROW_VALUE: EGameplayTagMatchDirection = EGameplayTagMatchDirection(
        1,
    );
}
#[repr(transparent)]
pub struct EObjectClassColumnCellValueComparison(pub i32);
impl EObjectClassColumnCellValueComparison {
    pub const EQUAL: EObjectClassColumnCellValueComparison = EObjectClassColumnCellValueComparison(
        0,
    );
    pub const NOT_EQUAL: EObjectClassColumnCellValueComparison = EObjectClassColumnCellValueComparison(
        1,
    );
    pub const SUB_CLASS_OF: EObjectClassColumnCellValueComparison = EObjectClassColumnCellValueComparison(
        2,
    );
    pub const NOT_SUB_CLASS_OF: EObjectClassColumnCellValueComparison = EObjectClassColumnCellValueComparison(
        3,
    );
    pub const ANY: EObjectClassColumnCellValueComparison = EObjectClassColumnCellValueComparison(
        4,
    );
}
#[repr(transparent)]
pub struct EObjectColumnCellValueComparison(pub i32);
impl EObjectColumnCellValueComparison {
    pub const MATCH_EQUAL: EObjectColumnCellValueComparison = EObjectColumnCellValueComparison(
        0,
    );
    pub const MATCH_NOT_EQUAL: EObjectColumnCellValueComparison = EObjectColumnCellValueComparison(
        1,
    );
    pub const MATCH_ANY: EObjectColumnCellValueComparison = EObjectColumnCellValueComparison(
        2,
    );
    pub const MODULUS: EObjectColumnCellValueComparison = EObjectColumnCellValueComparison(
        3,
    );
}
#[repr(transparent)]
pub struct EObjectChooserResultType(pub i32);
impl EObjectChooserResultType {
    pub const OBJECT_RESULT: EObjectChooserResultType = EObjectChooserResultType(0);
    pub const CLASS_RESULT: EObjectChooserResultType = EObjectChooserResultType(1);
    pub const NO_PRIMARY_RESULT: EObjectChooserResultType = EObjectChooserResultType(2);
}
