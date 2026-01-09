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
    pub u_asset_thumbnail_widget_set_thumbnail_settings: *mut crate::ffi::UFunctionOpague,
    pub u_asset_thumbnail_widget_set_resolution: *mut crate::ffi::UFunctionOpague,
    pub u_asset_thumbnail_widget_set_asset_by_object: *mut crate::ffi::UFunctionOpague,
    pub u_asset_thumbnail_widget_set_asset: *mut crate::ffi::UFunctionOpague,
    pub u_asset_thumbnail_widget_get_resolution: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_asset_thumbnail_widget_set_thumbnail_settings: std::ptr::null_mut(),
            u_asset_thumbnail_widget_set_resolution: std::ptr::null_mut(),
            u_asset_thumbnail_widget_set_asset_by_object: std::ptr::null_mut(),
            u_asset_thumbnail_widget_set_asset: std::ptr::null_mut(),
            u_asset_thumbnail_widget_get_resolution: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetThumbnailWidget::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetThumbnailSettings"),
            &raw mut __FUNCTION_PTRS.u_asset_thumbnail_widget_set_thumbnail_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetResolution"),
            &raw mut __FUNCTION_PTRS.u_asset_thumbnail_widget_set_resolution,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAssetByObject"),
            &raw mut __FUNCTION_PTRS.u_asset_thumbnail_widget_set_asset_by_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAsset"),
            &raw mut __FUNCTION_PTRS.u_asset_thumbnail_widget_set_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetResolution"),
            &raw mut __FUNCTION_PTRS.u_asset_thumbnail_widget_get_resolution,
        );
    }
}
#[repr(C, align(8))]
pub struct FAssetThumbnailWidgetSettings {
    pub b_force_generic_thumbnail: bool,
    pub b_allow_hint_text: bool,
    pub b_allow_real_time_on_hovered: bool,
    pub b_allow_asset_specific_thumbnail_overlay: bool,
    pub thumbnail_label: EThumbnailLabelType_BlueprintType,
    pub highlighted_text_delegate: FAssetThumbnailWidgetSettings_HighlightedTextDelegate,
    pub hint_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub b_override_asset_type_color: bool,
    pub asset_type_color_override: crate::bindings::core_u_object::FLinearColor,
    pub padding: crate::bindings::slate_core::FMargin,
    pub generic_thumbnail_size: i32,
    pub color_strip_orientation: EThumbnailColorStripOrientation_BlueprintType,
}
impl FAssetThumbnailWidgetSettings {}
#[repr(C, align(8))]
pub struct UWidgetBlueprint {
    __padding_end: [u8; 1624],
}
impl UWidgetBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetBlueprint")
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
pub struct UWidgetEditingProjectSettings {
    __padding_end: [u8; 688],
}
impl UWidgetEditingProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetEditingProjectSettings")
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
pub struct UUIComponentWidgetPair {
    __padding_end: [u8; 72],
}
impl UUIComponentWidgetPair {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUIComponentWidgetPair")
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
pub struct UWidgetBlueprintToolMenuContext {
    __padding_end: [u8; 64],
}
impl UWidgetBlueprintToolMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetBlueprintToolMenuContext")
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
pub struct UWidgetPaletteFavorites {
    __padding_end: [u8; 96],
}
impl UWidgetPaletteFavorites {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetPaletteFavorites")
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
pub struct UAssetDefinition_WidgetBlueprint {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_WidgetBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_WidgetBlueprint")
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
pub struct UAssetDefinition_WidgetBlueprintGeneratedClass {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_WidgetBlueprintGeneratedClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_WidgetBlueprintGeneratedClass")
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
pub struct UAssetThumbnailWidget {
    #[doc(hidden)]
    pub(crate) __padding_848: [u8; 848],
    pub resolution: crate::bindings::core_u_object::FIntPoint,
    pub thumbnail_settings: FAssetThumbnailWidgetSettings,
    __padding_end: [u8; 32],
}
impl UAssetThumbnailWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetThumbnailWidget")
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
    pub fn set_thumbnail_settings(
        &mut self,
        in_thumbnail_settings: &FAssetThumbnailWidgetSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::umg_editor::__FUNCTION_PTRS
                    .u_asset_thumbnail_widget_set_thumbnail_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_thumbnail_settings,
                __buffer.add(0).cast::<FAssetThumbnailWidgetSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::umg_editor::__FUNCTION_PTRS
                    .u_asset_thumbnail_widget_set_thumbnail_settings,
                __buffer,
            )
        };
    }
    pub fn set_resolution(
        &mut self,
        in_resolution: &crate::bindings::core_u_object::FIntPoint,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::umg_editor::__FUNCTION_PTRS
                    .u_asset_thumbnail_widget_set_resolution,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_resolution,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FIntPoint>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::umg_editor::__FUNCTION_PTRS
                    .u_asset_thumbnail_widget_set_resolution,
                __buffer,
            )
        };
    }
    pub fn set_asset_by_object(
        &mut self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::umg_editor::__FUNCTION_PTRS
                    .u_asset_thumbnail_widget_set_asset_by_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::umg_editor::__FUNCTION_PTRS
                    .u_asset_thumbnail_widget_set_asset_by_object,
                __buffer,
            )
        };
    }
    pub fn set_asset(
        &mut self,
        asset_data: &crate::bindings::core_u_object::FAssetData,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<152>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::umg_editor::__FUNCTION_PTRS
                    .u_asset_thumbnail_widget_set_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::umg_editor::__FUNCTION_PTRS
                    .u_asset_thumbnail_widget_set_asset,
                __buffer,
            )
        };
    }
    pub fn get_resolution(&self) -> crate::bindings::core_u_object::FIntPoint {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::umg_editor::__FUNCTION_PTRS
                    .u_asset_thumbnail_widget_get_resolution,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::umg_editor::__FUNCTION_PTRS
                    .u_asset_thumbnail_widget_get_resolution,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FIntPoint>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UK2Node_WidgetAnimationEvent {
    __padding_end: [u8; 440],
}
impl UK2Node_WidgetAnimationEvent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_WidgetAnimationEvent")
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
pub struct UK2Node_CreateDragDropOperation {
    __padding_end: [u8; 232],
}
impl UK2Node_CreateDragDropOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_CreateDragDropOperation")
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
pub struct UK2Node_CreateWidget {
    __padding_end: [u8; 232],
}
impl UK2Node_CreateWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_CreateWidget")
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
pub struct UK2Node_PlayAnimation {
    __padding_end: [u8; 240],
}
impl UK2Node_PlayAnimation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_PlayAnimation")
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
pub struct UK2Node_PlayAnimationTimeRange {
    __padding_end: [u8; 240],
}
impl UK2Node_PlayAnimationTimeRange {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_PlayAnimationTimeRange")
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
pub struct UK2Node_PlayAnimation2 {
    __padding_end: [u8; 240],
}
impl UK2Node_PlayAnimation2 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_PlayAnimation2")
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
pub struct UK2Node_PlayAnimationTimeRange2 {
    __padding_end: [u8; 240],
}
impl UK2Node_PlayAnimationTimeRange2 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_PlayAnimationTimeRange2")
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
pub struct UUMGEditorProjectSettings {
    __padding_end: [u8; 688],
}
impl UUMGEditorProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUMGEditorProjectSettings")
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
pub struct UWidgetDesignerSettings {
    __padding_end: [u8; 144],
}
impl UWidgetDesignerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetDesignerSettings")
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
pub struct USlateVectorArtDataFactory {
    __padding_end: [u8; 136],
}
impl USlateVectorArtDataFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateVectorArtDataFactory")
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
pub struct UWidgetEditorModeUISubsystem {
    __padding_end: [u8; 56],
}
impl UWidgetEditorModeUISubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetEditorModeUISubsystem")
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
pub struct UWidgetBlueprintExtension {
    __padding_end: [u8; 48],
}
impl UWidgetBlueprintExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetBlueprintExtension")
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
pub struct UUIComponentWidgetBlueprintExtension {
    __padding_end: [u8; 64],
}
impl UUIComponentWidgetBlueprintExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUIComponentWidgetBlueprintExtension")
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
pub struct UWidgetSlotPair {
    __padding_end: [u8; 96],
}
impl UWidgetSlotPair {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetSlotPair")
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
pub struct UWidgetBlueprintFactory {
    __padding_end: [u8; 160],
}
impl UWidgetBlueprintFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetBlueprintFactory")
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
pub struct UWidgetBlueprintThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UWidgetBlueprintThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetBlueprintThumbnailRenderer")
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
pub struct UWidgetCompilerRule {
    __padding_end: [u8; 48],
}
impl UWidgetCompilerRule {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetCompilerRule")
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
pub struct UWidgetGraphSchema {
    __padding_end: [u8; 152],
}
impl UWidgetGraphSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetGraphSchema")
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
pub struct FAssetThumbnailWidgetSettings_HighlightedTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetThumbnailSettings_HighlightedTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAssetThumbnailWidget_HighlightedTextDelegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EThumbnailLabelType_BlueprintType(pub u8);
impl EThumbnailLabelType_BlueprintType {
    pub const CLASS_NAME: EThumbnailLabelType_BlueprintType = EThumbnailLabelType_BlueprintType(
        0,
    );
    pub const ASSET_NAME: EThumbnailLabelType_BlueprintType = EThumbnailLabelType_BlueprintType(
        1,
    );
    pub const NO_LABEL: EThumbnailLabelType_BlueprintType = EThumbnailLabelType_BlueprintType(
        2,
    );
}
#[repr(transparent)]
pub struct EThumbnailColorStripOrientation_BlueprintType(pub u8);
impl EThumbnailColorStripOrientation_BlueprintType {
    pub const HORIZONTAL_BOTTOM_EDGE: EThumbnailColorStripOrientation_BlueprintType = EThumbnailColorStripOrientation_BlueprintType(
        0,
    );
    pub const VERTICAL_RIGHT_EDGE: EThumbnailColorStripOrientation_BlueprintType = EThumbnailColorStripOrientation_BlueprintType(
        1,
    );
}
#[repr(transparent)]
pub struct EPropertyBindingPermissionLevel(pub u8);
impl EPropertyBindingPermissionLevel {
    pub const ALLOW: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        0,
    );
    pub const PREVENT: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        1,
    );
    pub const PREVENT_AND_WARN: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        2,
    );
    pub const PREVENT_AND_ERROR: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        3,
    );
}
#[repr(transparent)]
pub struct EWidgetCompileTimeTickPrediction(pub u8);
impl EWidgetCompileTimeTickPrediction {
    pub const WONT_TICK: EWidgetCompileTimeTickPrediction = EWidgetCompileTimeTickPrediction(
        0,
    );
    pub const ON_DEMAND: EWidgetCompileTimeTickPrediction = EWidgetCompileTimeTickPrediction(
        1,
    );
    pub const WILL_TICK: EWidgetCompileTimeTickPrediction = EWidgetCompileTimeTickPrediction(
        2,
    );
}
#[repr(transparent)]
pub struct EThumbnailPreviewSizeMode(pub u8);
impl EThumbnailPreviewSizeMode {
    pub const MATCH_DESIGNER_MODE: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(
        0,
    );
    pub const FILL_SCREEN: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(1);
    pub const CUSTOM: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(2);
    pub const DESIRED: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(3);
}
#[repr(transparent)]
pub struct EDisplayOnCompile(pub u8);
impl EDisplayOnCompile {
    pub const DO_C_ERRORS_OR_WARNINGS: EDisplayOnCompile = EDisplayOnCompile(0);
    pub const DO_C_ERRORS_ONLY: EDisplayOnCompile = EDisplayOnCompile(1);
    pub const DO_C_WARNINGS_ONLY: EDisplayOnCompile = EDisplayOnCompile(2);
    pub const DO_C_NEVER: EDisplayOnCompile = EDisplayOnCompile(3);
}
