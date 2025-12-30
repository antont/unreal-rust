#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FStringValuePair {
    pub key: FString,
    pub value: FString,
}
#[repr(C, align(4))]
pub struct FDataflowPreviewCacheParams {
    pub frame_rate: i32,
    pub subframe_rate: i32,
    pub b_can_edit_subframe_rate: bool,
    pub time_range: FVector2f,
    pub b_restart_simulation: bool,
    pub restart_time_range: FVector2f,
    pub b_enable_async_caching: bool,
    pub b_async_caching: bool,
}
#[repr(C, align(8))]
pub struct FDataflowColorRamp {}
#[repr(C, align(4))]
pub struct FDataflowNodeDebugDrawSettings {
    pub render_type: EDataflowDebugDrawRenderType,
    pub b_translucent: bool,
    pub color: FLinearColor,
    pub line_width_multiplier: f32,
}
#[repr(C, align(4))]
pub struct FDataflowNodeSphereCoveringDebugDrawSettings {
    pub b_display_sphere_covering: bool,
    pub render_type: EDataflowDebugDrawRenderType,
    pub b_translucent: bool,
    pub line_width_multiplier: f32,
    pub color_method: EDataflowSphereCoveringColorMethod,
    pub color: FLinearColor,
    pub color_random_seed: i32,
    pub color_a: FLinearColor,
    pub color_b: FLinearColor,
}
#[repr(C, align(8))]
pub struct FDataflowProxyElement {}
#[repr(C, align(8))]
pub struct FDataflowDynamicConnections {
    pub dynamic_properties: TArray<FDataflowAllTypes>,
}
#[repr(C, align(8))]
pub struct FDataflowVariableOverrides {
    pub variables: FInstancedPropertyBag,
    pub overridden_variable_guids: TArray<FGuid>,
}
#[repr(C, align(8))]
pub struct FDataflowInstance {
    pub dataflow_asset: UPtr<UDataflow>,
    pub dataflow_terminal: FName,
    pub variable_overrides: FDataflowVariableOverrides,
    pub owner: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FDataflowSubGraphInputNode {
    pub dynamic_connections: FDataflowDynamicConnections,
    pub property_bag: FInstancedPropertyBag,
}
#[repr(C, align(8))]
pub struct FDataflowSubGraphOutputNode {
    pub dynamic_connections: FDataflowDynamicConnections,
    pub property_bag: FInstancedPropertyBag,
}
#[repr(C, align(8))]
pub struct FDataflowSubGraphGetCurrentIndexNode {
    pub index: i32,
}
#[repr(C, align(8))]
pub struct FDataflowCallSubGraphNode {
    pub sub_graph_guid: FGuid,
    pub sub_graph_name: FName,
    pub dynamic_inputs: FDataflowDynamicConnections,
    pub inputs_property_bag: FInstancedPropertyBag,
    pub dynamic_outputs: FDataflowDynamicConnections,
    pub outputs_property_bag: FInstancedPropertyBag,
}
#[repr(C, align(8))]
pub struct FDataflowTextureTerminalNode {
    pub image: FDataflowImage,
    pub texture_asset: UPtr<UTexture2D>,
}
#[repr(C, align(8))]
pub struct FDataflowTextureToImageNode {
    pub texture_asset: UPtr<UTexture2D>,
    pub image: FDataflowImage,
}
#[repr(C, align(8))]
pub struct FDataflowImageToTextureNode {
    pub image: FDataflowImage,
    pub texture_name: FName,
    pub transient_texture: UPtr<UTexture2D>,
}
#[repr(C, align(8))]
pub struct FGetDataflowVariableNode {
    pub value: FDataflowAnyType,
    pub variable_property_bag: FInstancedPropertyBag,
    pub variable_name: FName,
}
pub struct UDataflowBlueprintLibrary {}
pub struct UDataflowContentOwner {}
pub struct IDataflowContentOwner {}
pub struct UDataflowContextObject {
    pub selected_node: UPtr<UDataflowEdNode>,
    pub dataflow_graph: UPtr<UDataflow>,
}
pub struct UDataflowBaseContent {
    pub dataflow_terminal: FString,
    pub terminal_asset: UPtr<UObject>,
    pub b_is_construction_dirty: bool,
    pub b_is_simulation_dirty: bool,
}
pub struct UDataflowSkeletalContent {
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub animation_asset: UPtr<UAnimationAsset>,
}
pub struct UDataflowDebugDrawComponent {}
pub struct UDataflowEdNode {
    pub b_render_in_asset_editor: bool,
    pub b_render_wireframe_in_asset_editor: bool,
    pub b_can_enable_render_wireframe: bool,
}
pub struct UDataflowInstanceInterface {}
pub struct IDataflowInstanceInterface {}
pub struct UDataflowMesh {
    pub materials: TArray<UPtr<UMaterialInterface>>,
}
pub struct UDataflow {
    pub b_active: bool,
    pub targets: TArray<UPtr<UObject>>,
    pub material: UPtr<UMaterial>,
    pub ty: EDataflowType,
    pub variables: FInstancedPropertyBag,
    pub preview_cache_params: FDataflowPreviewCacheParams,
    pub preview_cache_asset: TSoftObjectPtr<UObject>,
    pub preview_blueprint_class: TSubclassOf<AActor>,
    pub preview_blueprint_transform: FTransform,
    pub preview_geometry_cache_asset: TSoftObjectPtr<UObject>,
    pub preview_embedded_skeletal_mesh: TSoftObjectPtr<UObject>,
    pub preview_embedded_static_mesh: TSoftObjectPtr<UObject>,
    pub dataflow_sub_graphs: TArray<UPtr<UDataflowSubGraph>>,
}
pub struct UDataflowSubGraph {
    pub sub_graph_guid: FGuid,
    pub b_is_for_each: bool,
}
