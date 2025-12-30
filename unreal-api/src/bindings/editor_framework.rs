#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(16))]
pub struct FDepthBarStyle {
    pub padding: FMargin,
    pub background_brush: FSlateBrush,
    pub track_brush: FSlateBrush,
    pub track_width: f32,
    pub slice_normal_brush: FSlateBrush,
    pub slice_hovered_brush: FSlateBrush,
    pub slice_width: f32,
    pub slice_top_brush: FSlateBrush,
    pub slice_top_hovered_brush: FSlateBrush,
    pub slice_bottom_brush: FSlateBrush,
    pub slice_bottom_hovered_brush: FSlateBrush,
    pub slice_handle_size: FVector2f,
    pub far_plane_button_brush: FSlateBrush,
    pub near_plane_button_brush: FSlateBrush,
    pub plane_button_size: FVector2f,
    pub far_plane_button_padding: FMargin,
    pub near_plane_button_padding: FMargin,
    pub plane_button_normal_color: FSlateColor,
    pub plane_button_hovered_color: FSlateColor,
}
#[repr(C, align(16))]
pub struct FAssetPlacementInfo {
    pub asset_to_place: FAssetData,
    pub name_override: FName,
    pub preferred_level: TWeakObjectPtr<ULevel>,
    pub finalized_transform: FTransform,
    pub factory_override: TScriptInterface<IAssetFactoryInterface>,
    pub item_guid: FGuid,
    pub settings_object: UPtr<UInstancedPlacemenClientSettings>,
}
#[repr(C, align(4))]
pub struct FPlacementOptions {
    pub instanced_placement_grid_guid: FGuid,
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
