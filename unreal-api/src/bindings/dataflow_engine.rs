#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub time_range: crate::bindings::core_u_object::FVector2f,
    pub b_restart_simulation: bool,
    pub restart_time_range: crate::bindings::core_u_object::FVector2f,
    pub b_enable_async_caching: bool,
    pub b_async_caching: bool,
}
#[repr(C, align(8))]
pub struct FDataflowColorRamp {}
#[repr(C, align(4))]
pub struct FDataflowNodeDebugDrawSettings {
    pub render_type: EDataflowDebugDrawRenderType,
    pub b_translucent: bool,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub line_width_multiplier: f32,
}
#[repr(C, align(4))]
pub struct FDataflowNodeSphereCoveringDebugDrawSettings {
    pub b_display_sphere_covering: bool,
    pub render_type: EDataflowDebugDrawRenderType,
    pub b_translucent: bool,
    pub line_width_multiplier: f32,
    pub color_method: EDataflowSphereCoveringColorMethod,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub color_random_seed: i32,
    pub color_a: crate::bindings::core_u_object::FLinearColor,
    pub color_b: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FDataflowProxyElement {}
#[repr(C, align(8))]
pub struct FDataflowDynamicConnections {
    pub dynamic_properties: TArray<crate::bindings::dataflow_core::FDataflowAllTypes>,
}
#[repr(C, align(8))]
pub struct FDataflowVariableOverrides {
    pub variables: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub overridden_variable_guids: TArray<crate::bindings::core_u_object::FGuid>,
}
#[repr(C, align(8))]
pub struct FDataflowInstance {
    pub dataflow_asset: UPtr<UDataflow>,
    pub dataflow_terminal: FName,
    pub variable_overrides: FDataflowVariableOverrides,
    pub owner: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FDataflowSubGraphInputNode {
    pub dynamic_connections: FDataflowDynamicConnections,
    pub property_bag: crate::bindings::core_u_object::FInstancedPropertyBag,
}
#[repr(C, align(8))]
pub struct FDataflowSubGraphOutputNode {
    pub dynamic_connections: FDataflowDynamicConnections,
    pub property_bag: crate::bindings::core_u_object::FInstancedPropertyBag,
}
#[repr(C, align(8))]
pub struct FDataflowSubGraphGetCurrentIndexNode {
    pub index: i32,
}
#[repr(C, align(8))]
pub struct FDataflowCallSubGraphNode {
    pub sub_graph_guid: crate::bindings::core_u_object::FGuid,
    pub sub_graph_name: FName,
    pub dynamic_inputs: FDataflowDynamicConnections,
    pub inputs_property_bag: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub dynamic_outputs: FDataflowDynamicConnections,
    pub outputs_property_bag: crate::bindings::core_u_object::FInstancedPropertyBag,
}
#[repr(C, align(8))]
pub struct FDataflowTextureTerminalNode {
    pub image: crate::bindings::dataflow_core::FDataflowImage,
    pub texture_asset: UPtr<crate::bindings::engine::UTexture2D>,
}
#[repr(C, align(8))]
pub struct FDataflowTextureToImageNode {
    pub texture_asset: UPtr<crate::bindings::engine::UTexture2D>,
    pub image: crate::bindings::dataflow_core::FDataflowImage,
}
#[repr(C, align(8))]
pub struct FDataflowImageToTextureNode {
    pub image: crate::bindings::dataflow_core::FDataflowImage,
    pub texture_name: FName,
    pub transient_texture: UPtr<crate::bindings::engine::UTexture2D>,
}
#[repr(C, align(8))]
pub struct FGetDataflowVariableNode {
    pub value: crate::bindings::dataflow_core::FDataflowAnyType,
    pub variable_property_bag: crate::bindings::core_u_object::FInstancedPropertyBag,
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
    pub terminal_asset: UPtr<crate::bindings::core_u_object::UObject>,
    pub b_is_construction_dirty: bool,
    pub b_is_simulation_dirty: bool,
}
pub struct UDataflowSkeletalContent {
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
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
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
}
pub struct UDataflow {
    pub b_active: bool,
    pub targets: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub material: UPtr<crate::bindings::engine::UMaterial>,
    pub ty: EDataflowType,
    pub variables: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub preview_cache_params: FDataflowPreviewCacheParams,
    pub preview_cache_asset: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub preview_blueprint_class: TSubclassOf<crate::bindings::engine::AActor>,
    pub preview_blueprint_transform: crate::bindings::core_u_object::FTransform,
    pub preview_geometry_cache_asset: TSoftObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
    pub preview_embedded_skeletal_mesh: TSoftObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
    pub preview_embedded_static_mesh: TSoftObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
    pub dataflow_sub_graphs: TArray<UPtr<UDataflowSubGraph>>,
}
pub struct UDataflowSubGraph {
    pub sub_graph_guid: crate::bindings::core_u_object::FGuid,
    pub b_is_for_each: bool,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowDebugDrawRenderType(pub u8);
impl EDataflowDebugDrawRenderType {
    pub const WIREFRAME: EDataflowDebugDrawRenderType = EDataflowDebugDrawRenderType(0);
    pub const SHADED: EDataflowDebugDrawRenderType = EDataflowDebugDrawRenderType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowSphereCoveringColorMethod(pub u8);
impl EDataflowSphereCoveringColorMethod {
    pub const SINGLE: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        0,
    );
    pub const COLOR_BY_RADIUS: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        1,
    );
    pub const RANDOM: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowType(pub u8);
impl EDataflowType {
    pub const CONSTRUCTION: EDataflowType = EDataflowType(0);
    pub const SIMULATION: EDataflowType = EDataflowType(1);
}
