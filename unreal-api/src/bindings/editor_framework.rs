#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FDepthBarStyle {
    pub padding: crate::bindings::slate_core::FMargin,
    pub background_brush: crate::bindings::slate_core::FSlateBrush,
    pub track_brush: crate::bindings::slate_core::FSlateBrush,
    pub track_width: f32,
    pub slice_normal_brush: crate::bindings::slate_core::FSlateBrush,
    pub slice_hovered_brush: crate::bindings::slate_core::FSlateBrush,
    pub slice_width: f32,
    pub slice_top_brush: crate::bindings::slate_core::FSlateBrush,
    pub slice_top_hovered_brush: crate::bindings::slate_core::FSlateBrush,
    pub slice_bottom_brush: crate::bindings::slate_core::FSlateBrush,
    pub slice_bottom_hovered_brush: crate::bindings::slate_core::FSlateBrush,
    pub slice_handle_size: crate::bindings::core_u_object::FVector2f,
    pub far_plane_button_brush: crate::bindings::slate_core::FSlateBrush,
    pub near_plane_button_brush: crate::bindings::slate_core::FSlateBrush,
    pub plane_button_size: crate::bindings::core_u_object::FVector2f,
    pub far_plane_button_padding: crate::bindings::slate_core::FMargin,
    pub near_plane_button_padding: crate::bindings::slate_core::FMargin,
    pub plane_button_normal_color: crate::bindings::slate_core::FSlateColor,
    pub plane_button_hovered_color: crate::bindings::slate_core::FSlateColor,
}
#[repr(C, align(16))]
pub struct FAssetPlacementInfo {
    pub asset_to_place: crate::bindings::core_u_object::FAssetData,
    pub name_override: FName,
    pub preferred_level: TWeakObjectPtr<crate::bindings::engine::ULevel>,
    pub finalized_transform: crate::bindings::core_u_object::FTransform,
    pub factory_override: TScriptInterface<IAssetFactoryInterface>,
    pub item_guid: crate::bindings::core_u_object::FGuid,
    pub settings_object: UPtr<crate::bindings::engine::UInstancedPlacemenClientSettings>,
}
#[repr(C, align(4))]
pub struct FPlacementOptions {
    pub instanced_placement_grid_guid: crate::bindings::core_u_object::FGuid,
    pub b_prefer_batch_placement: bool,
    pub b_is_creating_preview_elements: bool,
}
pub struct UAssetEditorUISubsystem {}
pub struct UAssetEditorContextInterface {}
pub struct IAssetEditorContextInterface {}
pub struct UTypedElementDetailsInterface {}
pub struct ITypedElementDetailsInterface {}
pub struct UTypedElementViewportInteraction {}
pub struct UAssetFactoryInterface {}
pub struct IAssetFactoryInterface {}
pub struct UEditorElementSubsystem {}
pub struct UPlacementSubsystem {
    pub asset_factories: TArray<TScriptInterface<IAssetFactoryInterface>>,
}
