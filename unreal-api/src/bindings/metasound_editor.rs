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
pub static mut U_META_SOUND_PRESET_WIDGET_INTERFACE_ON_CONSTRUCTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_META_SOUND_PRESET_WIDGET_INTERFACE_ON_AUDITION_STATE_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_META_SOUND_PRESET_WIDGET_INTERFACE_GET_SUPPORTED_META_SOUNDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_META_SOUND_EDITOR_BUILDER_LISTENER_REMOVE_ALL_DELEGATES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_EDITOR_SETTINGS_GET_AUDITION_PLATFORM_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_EDITOR_SETTINGS_GET_AUDITION_PAGE_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_META_SOUND_EDITOR_SUBSYSTEM_SET_NODE_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_META_SOUND_EDITOR_SUBSYSTEM_SET_FOCUSED_PAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_META_SOUND_EDITOR_SUBSYSTEM_FIND_OR_CREATE_GRAPH_INPUT_METADATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_META_SOUND_EDITOR_SUBSYSTEM_FIND_OR_BEGIN_BUILDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_META_SOUND_EDITOR_SUBSYSTEM_BUILD_TO_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_META_SOUND_EDITOR_SUBSYSTEM_ADD_BUILDER_DELEGATE_LISTENER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundPresetWidgetInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnConstructed"),
            &raw mut U_META_SOUND_PRESET_WIDGET_INTERFACE_ON_CONSTRUCTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAuditionStateChanged"),
            &raw mut U_META_SOUND_PRESET_WIDGET_INTERFACE_ON_AUDITION_STATE_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedMetaSounds"),
            &raw mut U_META_SOUND_PRESET_WIDGET_INTERFACE_GET_SUPPORTED_META_SOUNDS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundEditorBuilderListener::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllDelegates"),
            &raw mut U_META_SOUND_EDITOR_BUILDER_LISTENER_REMOVE_ALL_DELEGATES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetasoundEditorSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAuditionPlatformNames"),
            &raw mut U_METASOUND_EDITOR_SETTINGS_GET_AUDITION_PLATFORM_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAuditionPageNames"),
            &raw mut U_METASOUND_EDITOR_SETTINGS_GET_AUDITION_PAGE_NAMES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundEditorSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeLocation"),
            &raw mut U_META_SOUND_EDITOR_SUBSYSTEM_SET_NODE_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFocusedPage"),
            &raw mut U_META_SOUND_EDITOR_SUBSYSTEM_SET_FOCUSED_PAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindOrCreateGraphInputMetadata"),
            &raw mut U_META_SOUND_EDITOR_SUBSYSTEM_FIND_OR_CREATE_GRAPH_INPUT_METADATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindOrBeginBuilding"),
            &raw mut U_META_SOUND_EDITOR_SUBSYSTEM_FIND_OR_BEGIN_BUILDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BuildToAsset"),
            &raw mut U_META_SOUND_EDITOR_SUBSYSTEM_BUILD_TO_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBuilderDelegateListener"),
            &raw mut U_META_SOUND_EDITOR_SUBSYSTEM_ADD_BUILDER_DELEGATE_LISTENER,
        );
    }
}
pub struct IMetaSoundPresetWidgetInterface {}
#[repr(C, align(8))]
pub struct UMetaSoundPresetWidgetInterface {
    __padding_end: [u8; 48],
}
impl UMetaSoundPresetWidgetInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundPresetWidgetInterface")
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
pub struct UAssetDefinition_MetaSoundPatch {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_MetaSoundPatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_MetaSoundPatch")
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
pub struct UAssetDefinition_MetaSoundSource {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_MetaSoundSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_MetaSoundSource")
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
pub struct UMetasoundEditorViewBase {
    __padding_end: [u8; 56],
}
impl UMetasoundEditorViewBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorViewBase")
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
pub struct UMetasoundInterfacesView {
    __padding_end: [u8; 56],
}
impl UMetasoundInterfacesView {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundInterfacesView")
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
pub struct UMetasoundPagesView {
    __padding_end: [u8; 56],
}
impl UMetasoundPagesView {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundPagesView")
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
pub struct UMetaSoundEditorBuilderListener {
    __padding_end: [u8; 904],
}
impl UMetaSoundEditorBuilderListener {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundEditorBuilderListener")
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
pub struct UMetasoundEditorGraphMemberDefaultLiteral {
    __padding_end: [u8; 64],
}
impl UMetasoundEditorGraphMemberDefaultLiteral {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultLiteral")
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
pub struct UMetasoundEditorGraphMember {
    __padding_end: [u8; 104],
}
impl UMetasoundEditorGraphMember {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMember")
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
pub struct UMetasoundEditorGraphVertex {
    __padding_end: [u8; 288],
}
impl UMetasoundEditorGraphVertex {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphVertex")
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
pub struct UMetasoundEditorGraphInput {
    __padding_end: [u8; 288],
}
impl UMetasoundEditorGraphInput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphInput")
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
pub struct UMetasoundEditorGraphOutput {
    __padding_end: [u8; 288],
}
impl UMetasoundEditorGraphOutput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphOutput")
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
pub struct UMetasoundEditorGraphVariable {
    __padding_end: [u8; 240],
}
impl UMetasoundEditorGraphVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphVariable")
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
pub struct UMetasoundEditorGraph {
    __padding_end: [u8; 248],
}
impl UMetasoundEditorGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraph")
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
pub struct UMetasoundEditorGraphCommentNode {
    __padding_end: [u8; 312],
}
impl UMetasoundEditorGraphCommentNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphCommentNode")
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
pub struct UMetasoundEditorGraphNode {
    __padding_end: [u8; 256],
}
impl UMetasoundEditorGraphNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphNode")
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
pub struct UMetasoundEditorGraphMemberNode {
    __padding_end: [u8; 256],
}
impl UMetasoundEditorGraphMemberNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberNode")
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
pub struct UMetasoundEditorGraphInputNode {
    __padding_end: [u8; 616],
}
impl UMetasoundEditorGraphInputNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphInputNode")
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
pub struct UMetasoundEditorGraphMemberDefaultBool {
    __padding_end: [u8; 120],
}
impl UMetasoundEditorGraphMemberDefaultBool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultBool")
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
pub struct UMetasoundEditorGraphMemberDefaultBoolArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultBoolArray {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultBoolArray")
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
pub struct UMetasoundEditorGraphMemberDefaultInt {
    __padding_end: [u8; 88],
}
impl UMetasoundEditorGraphMemberDefaultInt {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultInt")
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
pub struct UMetasoundEditorGraphMemberDefaultIntArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultIntArray {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultIntArray")
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
pub struct UMetasoundEditorGraphMemberDefaultFloat {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub range: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 120],
}
impl UMetasoundEditorGraphMemberDefaultFloat {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultFloat")
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
pub struct UMetasoundEditorGraphMemberDefaultFloatArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultFloatArray {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultFloatArray")
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
pub struct UMetasoundEditorGraphMemberDefaultString {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultString {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultString")
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
pub struct UMetasoundEditorGraphMemberDefaultStringArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultStringArray {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultStringArray")
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
pub struct UMetasoundEditorGraphMemberDefaultObject {
    __padding_end: [u8; 88],
}
impl UMetasoundEditorGraphMemberDefaultObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultObject")
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
pub struct UMetasoundEditorGraphMemberDefaultObjectArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultObjectArray {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultObjectArray")
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
pub struct UMetasoundEditorGraphOutputNode {
    __padding_end: [u8; 600],
}
impl UMetasoundEditorGraphOutputNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphOutputNode")
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
pub struct UMetasoundEditorGraphExternalNode {
    __padding_end: [u8; 416],
}
impl UMetasoundEditorGraphExternalNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphExternalNode")
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
pub struct UMetasoundEditorGraphVariableNode {
    __padding_end: [u8; 648],
}
impl UMetasoundEditorGraphVariableNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphVariableNode")
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
pub struct UMetasoundEditorGraphSchema {
    __padding_end: [u8; 48],
}
impl UMetasoundEditorGraphSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphSchema")
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
pub struct UMetasoundEditorSettings {
    __padding_end: [u8; 640],
}
impl UMetasoundEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorSettings")
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
pub struct UMetaSoundEditorSubsystem {
    __padding_end: [u8; 72],
}
impl UMetaSoundEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundEditorSubsystem")
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
pub struct UMetaSoundBaseFactory {
    __padding_end: [u8; 144],
}
impl UMetaSoundBaseFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundBaseFactory")
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
pub struct UMetaSoundFactory {
    __padding_end: [u8; 144],
}
impl UMetaSoundFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundFactory")
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
pub struct UMetaSoundSourceFactory {
    __padding_end: [u8; 144],
}
impl UMetaSoundSourceFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundSourceFactory")
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
pub struct FMetaSoundEditorBuilderListener_OnDocumentDisplayNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentDescriptionChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentAuthorChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentKeywordsChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentCategoryHierarchyChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentIsDeprecatedChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputAddedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnRemovingGraphInputDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDisplayNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDataTypeChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDescriptionChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputSortOrderIndexChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputIsConstructorPinChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputIsAdvancedDisplayChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDefaultChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputInheritsDefaultChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputAddedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnRemovingGraphOutputDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDisplayNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDataTypeChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDescriptionChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputSortOrderIndexChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputIsConstructorPinChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputIsAdvancedDisplayChangedDelegate {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EMetasoundActiveAnalyzerEnvelopeDirection(pub u8);
impl EMetasoundActiveAnalyzerEnvelopeDirection {
    pub const FROM_SOURCE_OUTPUT: EMetasoundActiveAnalyzerEnvelopeDirection = EMetasoundActiveAnalyzerEnvelopeDirection(
        0,
    );
    pub const FROM_DESTINATION_INPUT: EMetasoundActiveAnalyzerEnvelopeDirection = EMetasoundActiveAnalyzerEnvelopeDirection(
        1,
    );
}
#[repr(transparent)]
pub struct EMetasoundBoolMemberDefaultWidget(pub u8);
impl EMetasoundBoolMemberDefaultWidget {
    pub const NONE: EMetasoundBoolMemberDefaultWidget = EMetasoundBoolMemberDefaultWidget(
        0,
    );
    pub const BUTTON: EMetasoundBoolMemberDefaultWidget = EMetasoundBoolMemberDefaultWidget(
        1,
    );
}
#[repr(transparent)]
pub struct EMetasoundMemberDefaultWidget(pub u8);
impl EMetasoundMemberDefaultWidget {
    pub const NONE: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(0);
    pub const SLIDER: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(1);
    pub const RADIAL_SLIDER: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(
        2,
    );
}
#[repr(transparent)]
pub struct EMetasoundMemberDefaultWidgetValueType(pub u8);
impl EMetasoundMemberDefaultWidgetValueType {
    pub const LINEAR: EMetasoundMemberDefaultWidgetValueType = EMetasoundMemberDefaultWidgetValueType(
        0,
    );
    pub const FREQUENCY: EMetasoundMemberDefaultWidgetValueType = EMetasoundMemberDefaultWidgetValueType(
        1,
    );
    pub const VOLUME: EMetasoundMemberDefaultWidgetValueType = EMetasoundMemberDefaultWidgetValueType(
        2,
    );
}
#[repr(transparent)]
pub struct EAuditionPageMode(pub u8);
impl EAuditionPageMode {
    pub const FOCUSED: EAuditionPageMode = EAuditionPageMode(0);
    pub const USER: EAuditionPageMode = EAuditionPageMode(1);
}
#[repr(transparent)]
pub struct EMetasoundActiveDetailView(pub u8);
impl EMetasoundActiveDetailView {
    pub const METASOUND: EMetasoundActiveDetailView = EMetasoundActiveDetailView(0);
    pub const GENERAL: EMetasoundActiveDetailView = EMetasoundActiveDetailView(1);
}
