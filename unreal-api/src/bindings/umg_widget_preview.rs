#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FPreviewableWidgetVariant {
    pub object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub cached_widget_cdo: UPtr<crate::bindings::umg::UUserWidget>,
    pub cached_widget_preview: TWeakObjectPtr<UWidgetPreview>,
}
pub struct UAssetDefinition_WidgetPreview {}
pub struct UWidgetPreview {
    pub widget_type: FPreviewableWidgetVariant,
    pub slot_widget_types: TMap<FName, FPreviewableWidgetVariant>,
    pub b_should_override_widget_size: bool,
    pub overridden_widget_size: crate::bindings::core_u_object::FVector2D,
    pub widget_instance: UPtr<crate::bindings::umg::UUserWidget>,
    pub slot_name_cache: TArray<FName>,
    pub widget_reference_cache: TArray<
        TWeakObjectPtr<crate::bindings::umg::UUserWidget>,
    >,
}
pub struct UWidgetPreviewEditor {}
pub struct UWidgetPreviewFactory {}
