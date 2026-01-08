#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_animation_sequence_browser_context_menu_context_get_selected_objects: *mut crate::ffi::UFunctionOpague,
    pub u_persona_tool_menu_context_get_skeleton: *mut crate::ffi::UFunctionOpague,
    pub u_persona_tool_menu_context_get_preview_mesh_component: *mut crate::ffi::UFunctionOpague,
    pub u_persona_tool_menu_context_get_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_persona_tool_menu_context_get_anim_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_persona_tool_menu_context_get_animation_asset: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_animation_sequence_browser_context_menu_context_get_selected_objects: std::ptr::null_mut(),
            u_persona_tool_menu_context_get_skeleton: std::ptr::null_mut(),
            u_persona_tool_menu_context_get_preview_mesh_component: std::ptr::null_mut(),
            u_persona_tool_menu_context_get_mesh: std::ptr::null_mut(),
            u_persona_tool_menu_context_get_anim_blueprint: std::ptr::null_mut(),
            u_persona_tool_menu_context_get_animation_asset: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimationSequenceBrowserContextMenuContext::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedObjects"),
            &raw mut __FUNCTION_PTRS
                .u_animation_sequence_browser_context_menu_context_get_selected_objects,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPersonaToolMenuContext::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeleton"),
            &raw mut __FUNCTION_PTRS.u_persona_tool_menu_context_get_skeleton,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviewMeshComponent"),
            &raw mut __FUNCTION_PTRS
                .u_persona_tool_menu_context_get_preview_mesh_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMesh"),
            &raw mut __FUNCTION_PTRS.u_persona_tool_menu_context_get_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimBlueprint"),
            &raw mut __FUNCTION_PTRS.u_persona_tool_menu_context_get_anim_blueprint,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationAsset"),
            &raw mut __FUNCTION_PTRS.u_persona_tool_menu_context_get_animation_asset,
        );
    }
}
#[repr(C, align(8))]
pub struct UAnimationEditorsAssetFamilyExtension {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationEditorsAssetFamilyExtension")
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
pub struct UAnimationSequenceBrowserContextMenuContext {
    __padding_end: [u8; 80],
}
impl UAnimationSequenceBrowserContextMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationSequenceBrowserContextMenuContext")
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
    pub fn get_selected_objects(
        &self,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_animation_sequence_browser_context_menu_context_get_selected_objects,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_animation_sequence_browser_context_menu_context_get_selected_objects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimNotifyPanelContextMenuContext {
    __padding_end: [u8; 224],
}
impl UAnimNotifyPanelContextMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyPanelContextMenuContext")
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
pub struct UAnimViewportContext {
    __padding_end: [u8; 80],
}
impl UAnimViewportContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimViewportContext")
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
pub struct UAnimViewportToolBarToolMenuContext {
    __padding_end: [u8; 64],
}
impl UAnimViewportToolBarToolMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimViewportToolBarToolMenuContext")
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
pub struct UCachedAnalysisProperties {
    __padding_end: [u8; 576],
}
impl UCachedAnalysisProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCachedAnalysisProperties")
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
pub struct ULinearAnalysisPropertiesBase {
    __padding_end: [u8; 416],
}
impl ULinearAnalysisPropertiesBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinearAnalysisPropertiesBase")
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
pub struct ULinearAnalysisProperties {
    __padding_end: [u8; 416],
}
impl ULinearAnalysisProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinearAnalysisProperties")
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
pub struct UEulerAnalysisProperties {
    __padding_end: [u8; 432],
}
impl UEulerAnalysisProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEulerAnalysisProperties")
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
pub struct UPersonaPreviewSceneDescription {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub preview_controller: TSubclassOf<UPersonaPreviewSceneController>,
    __padding_end: [u8; 192],
}
impl UPersonaPreviewSceneDescription {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersonaPreviewSceneDescription")
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
pub struct UAnimAssetFindReplaceContext {
    __padding_end: [u8; 64],
}
impl UAnimAssetFindReplaceContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimAssetFindReplaceContext")
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
pub struct UAnimAssetFindReplaceProcessor {
    __padding_end: [u8; 64],
}
impl UAnimAssetFindReplaceProcessor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimAssetFindReplaceProcessor")
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
pub struct UAnimAssetFindReplaceProcessor_StringBase {
    __padding_end: [u8; 320],
}
impl UAnimAssetFindReplaceProcessor_StringBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimAssetFindReplaceProcessor_StringBase")
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
pub struct UAnimAssetFindReplaceCurves {
    __padding_end: [u8; 328],
}
impl UAnimAssetFindReplaceCurves {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimAssetFindReplaceCurves")
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
pub struct UAnimAssetFindReplaceNotifies {
    __padding_end: [u8; 320],
}
impl UAnimAssetFindReplaceNotifies {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimAssetFindReplaceNotifies")
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
pub struct UAnimAssetFindReplaceSyncMarkers {
    __padding_end: [u8; 320],
}
impl UAnimAssetFindReplaceSyncMarkers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimAssetFindReplaceSyncMarkers")
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
pub struct AAnimationEditorPreviewActor {
    __padding_end: [u8; 1136],
}
impl AAnimationEditorPreviewActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAnimationEditorPreviewActor")
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
pub struct UAnimCurveBaseCopyObject {
    __padding_end: [u8; 80],
}
impl UAnimCurveBaseCopyObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimCurveBaseCopyObject")
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
pub struct UFloatCurveCopyObject {
    __padding_end: [u8; 288],
}
impl UFloatCurveCopyObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloatCurveCopyObject")
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
pub struct UTransformCurveCopyObject {
    __padding_end: [u8; 1552],
}
impl UTransformCurveCopyObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformCurveCopyObject")
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
pub struct UVectorCurveCopyObject {
    __padding_end: [u8; 544],
}
impl UVectorCurveCopyObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVectorCurveCopyObject")
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
pub struct UAnimTimelineClipboardContent {
    __padding_end: [u8; 64],
}
impl UAnimTimelineClipboardContent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimTimelineClipboardContent")
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
pub struct IPersonaManagerContext {}
#[repr(C, align(8))]
pub struct UPersonaManagerContext {
    __padding_end: [u8; 48],
}
impl UPersonaManagerContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersonaManagerContext")
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
pub struct UPersonaEditorModeManagerContext {
    __padding_end: [u8; 64],
}
impl UPersonaEditorModeManagerContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersonaEditorModeManagerContext")
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
pub struct ULODInfoUILayout {
    __padding_end: [u8; 576],
}
impl ULODInfoUILayout {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULODInfoUILayout")
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
pub struct UAnimationEditorsAssetFamilyExtension_SkeletonAsset {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension_SkeletonAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationEditorsAssetFamilyExtension_SkeletonAsset")
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
pub struct UAnimationEditorsAssetFamilyExtension_SkeletalMeshAsset {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension_SkeletalMeshAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationEditorsAssetFamilyExtension_SkeletalMeshAsset")
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
pub struct UAnimationEditorsAssetFamilyExtension_AnimationAsset {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension_AnimationAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationEditorsAssetFamilyExtension_AnimationAsset")
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
pub struct UAnimationEditorsAssetFamilyExtension_AnimBlueprintAsset {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension_AnimBlueprintAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationEditorsAssetFamilyExtension_AnimBlueprintAsset")
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
pub struct UAnimationEditorsAssetFamilyExtension_PhysicsAsset {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension_PhysicsAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationEditorsAssetFamilyExtension_PhysicsAsset")
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
pub struct UPersonaPreviewSceneController {
    __padding_end: [u8; 48],
}
impl UPersonaPreviewSceneController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersonaPreviewSceneController")
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
pub struct UPersonaPreviewSceneAnimationController {
    __padding_end: [u8; 96],
}
impl UPersonaPreviewSceneAnimationController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersonaPreviewSceneAnimationController")
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
pub struct UPersonaPreviewSceneDefaultController {
    __padding_end: [u8; 48],
}
impl UPersonaPreviewSceneDefaultController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersonaPreviewSceneDefaultController")
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
pub struct UPersonaPreviewSceneRefPoseController {
    __padding_end: [u8; 56],
}
impl UPersonaPreviewSceneRefPoseController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersonaPreviewSceneRefPoseController")
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
pub struct UPersonaPreviewSceneSkelMeshInstanceController {
    __padding_end: [u8; 56],
}
impl UPersonaPreviewSceneSkelMeshInstanceController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersonaPreviewSceneSkelMeshInstanceController")
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
pub struct UPersonaToolMenuContext {
    __padding_end: [u8; 64],
}
impl UPersonaToolMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersonaToolMenuContext")
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
    pub fn get_skeleton(&self) -> UPtr<crate::bindings::engine::USkeleton> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_persona_tool_menu_context_get_skeleton,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_persona_tool_menu_context_get_skeleton,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>().read()
        }
    }
    pub fn get_preview_mesh_component(
        &self,
    ) -> UPtr<crate::bindings::unreal_ed::UDebugSkelMeshComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_persona_tool_menu_context_get_preview_mesh_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_persona_tool_menu_context_get_preview_mesh_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::unreal_ed::UDebugSkelMeshComponent>>()
                .read()
        }
    }
    pub fn get_mesh(&self) -> UPtr<crate::bindings::engine::USkeletalMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_persona_tool_menu_context_get_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_persona_tool_menu_context_get_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>().read()
        }
    }
    pub fn get_anim_blueprint(&self) -> UPtr<crate::bindings::engine::UAnimBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_persona_tool_menu_context_get_anim_blueprint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_persona_tool_menu_context_get_anim_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UAnimBlueprint>>()
                .read()
        }
    }
    pub fn get_animation_asset(&self) -> UPtr<crate::bindings::engine::UAnimationAsset> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_persona_tool_menu_context_get_animation_asset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::persona::__FUNCTION_PTRS
                    .u_persona_tool_menu_context_get_animation_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UAnimationAsset>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UPhysicsAssetRenderUtilities {
    __padding_end: [u8; 152],
}
impl UPhysicsAssetRenderUtilities {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsAssetRenderUtilities")
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
pub struct USkinWeightImportOptions {
    __padding_end: [u8; 88],
}
impl USkinWeightImportOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkinWeightImportOptions")
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
#[repr(transparent)]
pub struct EAnalysisSpace(pub u8);
impl EAnalysisSpace {
    pub const WORLD: EAnalysisSpace = EAnalysisSpace(0);
    pub const FIXED: EAnalysisSpace = EAnalysisSpace(1);
    pub const CHANGING: EAnalysisSpace = EAnalysisSpace(2);
    pub const MOVING: EAnalysisSpace = EAnalysisSpace(3);
}
#[repr(transparent)]
pub struct EAnalysisLinearAxis(pub u8);
impl EAnalysisLinearAxis {
    pub const PLUS_X: EAnalysisLinearAxis = EAnalysisLinearAxis(0);
    pub const PLUS_Y: EAnalysisLinearAxis = EAnalysisLinearAxis(1);
    pub const PLUS_Z: EAnalysisLinearAxis = EAnalysisLinearAxis(2);
    pub const MINUS_X: EAnalysisLinearAxis = EAnalysisLinearAxis(3);
    pub const MINUS_Y: EAnalysisLinearAxis = EAnalysisLinearAxis(4);
    pub const MINUS_Z: EAnalysisLinearAxis = EAnalysisLinearAxis(5);
}
#[repr(transparent)]
pub struct EAnalysisEulerAxis(pub u8);
impl EAnalysisEulerAxis {
    pub const ROLL: EAnalysisEulerAxis = EAnalysisEulerAxis(0);
    pub const PITCH: EAnalysisEulerAxis = EAnalysisEulerAxis(1);
    pub const YAW: EAnalysisEulerAxis = EAnalysisEulerAxis(2);
}
#[repr(transparent)]
pub struct EEulerCalculationMethod(pub u8);
impl EEulerCalculationMethod {
    pub const AIM_DIRECTION: EEulerCalculationMethod = EEulerCalculationMethod(0);
    pub const POINT_DIRECTION: EEulerCalculationMethod = EEulerCalculationMethod(1);
}
