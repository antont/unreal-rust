#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMultiControlRigElementSelection {
    pub rigs: TArray<TWeakObjectPtr<crate::bindings::control_rig::UControlRig>>,
    pub keys_per_rig: TArray<crate::bindings::control_rig::FRigElementKeyCollection>,
}
#[repr(C, align(8))]
pub struct FControlRigInteractionTransformContext {
    pub drag: crate::bindings::core_u_object::FVector,
    pub rot: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
    pub space: EControlRigInteractionTransformSpace,
}
#[repr(C, align(1))]
pub struct FControlRigConstraintsTabRestoreState {
    pub b_was_open: bool,
    pub last_open_inline_tab: EControlRigConstrainTab,
}
#[repr(C, align(4))]
pub struct FControlRigUIRestoreStates {
    pub b_motion_trails_on: bool,
    pub b_anim_layer_tab_open: bool,
    pub b_pose_tab_open: bool,
    pub b_snapper_tab_open: bool,
    pub constraints_tab_state: FControlRigConstraintsTabRestoreState,
    pub b_selection_sets_open: bool,
    pub selection_set_overlay_state: FToolWidget_FlyoutSavedState,
    pub tween_overlay_state: FToolWidget_FlyoutSavedState,
}
#[repr(C, align(4))]
pub struct FToolWidget_FlyoutSavedState {
    pub position: crate::bindings::tool_widgets::FToolWidget_DragBoxPosition,
    pub b_was_visible: bool,
}
#[repr(C, align(4))]
pub struct FRigSpacePickerBakeSettings {
    pub target_space: crate::bindings::control_rig::FRigElementKey,
    pub settings: crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
    pub start_frame_deprecated: crate::bindings::core_u_object::FFrameNumber,
    pub end_frame_deprecated: crate::bindings::core_u_object::FFrameNumber,
    pub b_reduce_keys_deprecated: bool,
    pub tolerance_deprecated: f32,
}
#[repr(C, align(8))]
pub struct FAIESelectionSetItemName {
    pub name: FString,
    pub mirror_name: FString,
    pub ty: i32,
    pub duplicates: i32,
    pub owner_actor_name: FString,
}
#[repr(C, align(4))]
pub struct FAIESelectionSetItemViewData {
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub row: i32,
    pub column: i32,
}
#[repr(C, align(8))]
pub struct FAIESelectionSetItem {
    pub guid: crate::bindings::core_u_object::FGuid,
    pub item_name: FText,
    pub names: TArray<FAIESelectionSetItemName>,
    pub view_data: FAIESelectionSetItemViewData,
    pub parent: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FAnimDetailsSelectionPropertyData {
    pub weak_proxies: TArray<TWeakObjectPtr<UAnimDetailsProxyBase>>,
    pub b_is_selected: bool,
    pub property_name: FName,
}
#[repr(C, align(1))]
pub struct FAnimDetailsBool {
    pub bool: bool,
}
#[repr(C, align(8))]
pub struct FAnimDetailsEnum {
    pub enum_type: UPtr<crate::bindings::core_u_object::UEnum>,
    pub enum_index: i32,
}
#[repr(C, align(8))]
pub struct FAnimDetailsFloat {
    pub float: f64,
}
#[repr(C, align(8))]
pub struct FAnimDetailsInteger {
    pub integer: i64,
}
#[repr(C, align(8))]
pub struct FAnimDetailsLocation {
    pub lx: f64,
    pub ly: f64,
    pub lz: f64,
}
#[repr(C, align(8))]
pub struct FAnimDetailsRotation {
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
}
#[repr(C, align(8))]
pub struct FAnimDetailsScale {
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}
#[repr(C, align(8))]
pub struct FAnimDetailsVector2D {
    pub x: f64,
    pub y: f64,
}
#[repr(C, align(4))]
pub struct FControlRigDrawContainerImportFbxSettings {
    pub scale: f32,
    pub detail: i32,
    pub b_merge_curves: bool,
}
#[repr(C, align(8))]
pub struct FControlRigSequencerBindingProxy {
    pub proxy: crate::bindings::movie_scene::FMovieSceneBindingProxy,
    pub control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
    pub track: UPtr<crate::bindings::control_rig::UMovieSceneControlRigParameterTrack>,
}
#[repr(C, align(8))]
pub struct FControlRigRigHierarchyDragAndDropContext {
    pub dragged_hierarchy_keys: TArray<crate::bindings::control_rig::FRigHierarchyKey>,
    pub target_hierarchy_key: crate::bindings::control_rig::FRigHierarchyKey,
}
#[repr(C, align(8))]
pub struct FControlRigGraphNodeContextMenuContext {
    pub graph: UPtr<crate::bindings::rig_vm_developer::URigVMGraph>,
    pub node: UPtr<crate::bindings::rig_vm_developer::URigVMNode>,
    pub pin: UPtr<crate::bindings::rig_vm_developer::URigVMPin>,
}
#[repr(C, align(8))]
pub struct FControlRigRigHierarchyToGraphDragAndDropContext {
    pub dragged_hierarchy_keys: TArray<crate::bindings::control_rig::FRigHierarchyKey>,
}
#[repr(C, align(8))]
pub struct FRigHierarchyImportSettings {
    pub mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
#[repr(C, align(4))]
pub struct FMergeAnimLayerSettings {
    pub baking_key_settings: crate::bindings::movie_scene_tools::EBakingKeySettings,
    pub frame_increment: i32,
    pub b_reduce_keys: bool,
    pub tolerance_percentage: f32,
}
#[repr(C, align(4))]
pub struct FAnimLayerPropertyAndChannels {
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FAnimLayerSelectionSet {
    pub bound_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub names: TMap<FName, FAnimLayerPropertyAndChannels>,
}
#[repr(C, align(8))]
pub struct FAnimLayerSectionItem {
    pub anim_layer_set: FAnimLayerSelectionSet,
    pub section: TWeakObjectPtr<crate::bindings::movie_scene::UMovieSceneSection>,
}
#[repr(C, align(8))]
pub struct FAnimLayerItem {
    pub section_items: TArray<FAnimLayerSectionItem>,
    pub sequencer_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FAnimLayerState {
    pub b_keyed: crate::bindings::slate_core::ECheckBoxState,
    pub b_selected: crate::bindings::slate_core::ECheckBoxState,
    pub b_active: bool,
    pub b_lock: bool,
    pub name: FText,
    pub weight: f64,
    pub ty: i32,
}
#[repr(C, align(8))]
pub struct FAnimLayerControlRigObject {
    pub control_rig: TWeakObjectPtr<crate::bindings::control_rig::UControlRig>,
    pub control_names: TArray<FName>,
}
#[repr(C, align(4))]
pub struct FAnimLayerSceneObject {
    pub scene_object_or_component: TWeakObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
}
#[repr(C, align(8))]
pub struct FAnimLayerObjects {
    pub control_rig_objects: TArray<FAnimLayerControlRigObject>,
    pub scene_objects: TArray<FAnimLayerSceneObject>,
}
#[repr(C, align(8))]
pub struct FControlRigForWorldTransforms {
    pub control_rig: TWeakObjectPtr<crate::bindings::control_rig::UControlRig>,
    pub control_names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FControlRigSnapperSelection {
    pub actors: TArray<crate::bindings::movie_scene::FActorForWorldTransforms>,
    pub control_rigs: TArray<FControlRigForWorldTransforms>,
}
pub struct UAnimDetailsOptionsMenuContext {}
pub struct UAnimDetailsSettings {
    pub num_fractional_digits: u8,
    pub b_lmb_selects_range: bool,
}
pub struct UAnimSequenceConverterFactory {}
pub struct UConstraintCreationOptions {
    pub sequencer_options: crate::bindings::movie_scene_tools::FSequencerCreationOptions,
}
pub struct UAIESelectionSets {
    pub selection_sets: TMap<
        crate::bindings::core_u_object::FGuid,
        FAIESelectionSetItem,
    >,
    pub b_show_selected_only: bool,
}
pub struct UAnimDetailsProxyManager {
    pub proxies: TArray<UPtr<UAnimDetailsProxyBase>>,
    pub external_selection: TArray<UPtr<UAnimDetailsProxyBase>>,
    pub anim_details_selection: UPtr<UAnimDetailsSelection>,
}
pub struct UAnimDetailsSelection {
    pub property_id_to_property_data_map: TMap<FName, FAnimDetailsSelectionPropertyData>,
}
pub struct UAnimDetailsProxyBase {
    pub display_name: FString,
    pub shape: crate::bindings::control_rig::FRigUnit_HierarchyAddControl_ShapeSettings,
}
pub struct UAnimDetailsProxyBool {
    pub bool: FAnimDetailsBool,
}
pub struct UAnimDetailsProxyEnum {
    pub enum_: FAnimDetailsEnum,
}
pub struct UAnimDetailsProxyFloat {
    pub float: FAnimDetailsFloat,
}
pub struct UAnimDetailsProxyInteger {
    pub integer: FAnimDetailsInteger,
}
pub struct UAnimDetailsProxyLocation {
    pub location: FAnimDetailsLocation,
}
pub struct UAnimDetailsProxyRotation {
    pub rotation: FAnimDetailsRotation,
}
pub struct UAnimDetailsProxyScale {
    pub scale: FAnimDetailsScale,
}
pub struct UAnimDetailsProxyTransform {
    pub location: FAnimDetailsLocation,
    pub rotation: FAnimDetailsRotation,
    pub scale: FAnimDetailsScale,
}
pub struct UAnimDetailsProxyVector2D {
    pub vector2_d: FAnimDetailsVector2D,
}
pub struct UBakeToControlRigSettings {
    pub b_reduce_keys: bool,
    pub tolerance: f32,
    pub smart_reduce: crate::bindings::curve_editor::FSmartReduceParams,
    pub b_reset_controls: bool,
}
pub struct UControlRigBlueprintEditorLibrary {}
pub struct UControlRigBlueprintFactory {
    pub parent_class: TSubclassOf<crate::bindings::control_rig::UControlRig>,
}
pub struct UControlRigShapeLibraryFactory {}
pub struct UControlRigSequencerEditorLibrary {}
pub struct UControlRigThumbnailRenderer {}
pub struct UControlRigEditModeDelegateHelper {}
pub struct UControlRigEditModeSettings {
    pub b_display_hierarchy: bool,
    pub b_display_nulls: bool,
    pub b_display_sockets: bool,
    pub b_hide_control_shapes: bool,
    pub b_show_all_proxy_controls: bool,
    pub b_show_controls_as_overlay: bool,
    pub driven_control_color: crate::bindings::core_u_object::FLinearColor,
    pub b_display_axes_on_selection: bool,
    pub axis_scale: f32,
    pub b_coord_system_per_widget_mode: bool,
    pub b_only_select_rig_controls: bool,
    pub b_local_transforms_in_each_local_space: bool,
    pub gizmo_scale: f32,
    pub last_ui_states: FControlRigUIRestoreStates,
    pub tween_out_of_focus_tint: crate::bindings::core_u_object::FLinearColor,
    pub b_indirect_slider_movement_should_snap_slider_to_mouse: bool,
}
pub struct UControlRigContextMenuContext {}
pub struct UControlRigSkeletalMeshComponent {}
pub struct UControlRigWrapperObject {}
pub struct URigConnectorTargetsDetailWrapper {
    pub connector: crate::bindings::control_rig::FRigElementKey,
    pub target_array: TArray<crate::bindings::control_rig::FRigElementKey>,
}
pub struct URigDependencyGraph {
    pub dependency_graph_nodes: TArray<UPtr<URigDependencyGraphNode>>,
}
pub struct URigDependencyGraphNode {}
pub struct URigDependencyGraphSchema {}
pub struct UAnimationAuthoringSettings {
    pub b_auto_key_on_release: bool,
}
pub struct UAnimLayerSequencerFilter {}
pub struct UAnimLayerWeightProxy {
    pub weight: f64,
}
pub struct UAnimLayer {
    pub anim_layer_items: TMap<
        TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
        FAnimLayerItem,
    >,
    pub state: FAnimLayerState,
    pub weight_proxy: UPtr<UAnimLayerWeightProxy>,
}
pub struct UAnimLayers {
    pub anim_layers: TArray<UPtr<UAnimLayer>>,
    pub b_open_ui_on_open: bool,
}
pub struct UControlRigTrackFilter {}
pub struct ULoadAnimToControlRigSettings {
    pub b_onto_selected_controls: bool,
    pub b_reduce_keys: bool,
    pub smart_reduce: crate::bindings::curve_editor::FSmartReduceParams,
    pub b_use_custom_time_range: bool,
    pub start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub end_frame: crate::bindings::core_u_object::FFrameNumber,
    pub b_reset_controls: bool,
}
pub struct USelectionSetsSettings {
    pub custom_colors: TArray<crate::bindings::core_u_object::FLinearColor>,
}
pub struct UAssetDefinition_ControlRigPose {}
pub struct UControlRigPoseThumbnailRenderer {}
pub struct UControlRigSnapSettings {
    pub b_keep_offset: bool,
    pub b_snap_position: bool,
    pub b_snap_rotation: bool,
    pub b_snap_scale: bool,
    pub baking_key_settings: crate::bindings::movie_scene_tools::EBakingKeySettings,
    pub frame_increment: i32,
    pub b_reduce_keys: bool,
    pub tolerance: f32,
}
pub struct UCreateControlPoseAssetRigSettings {
    pub asset_name: FString,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EControlRigInteractionTransformSpace(pub i32);
impl EControlRigInteractionTransformSpace {
    pub const WORLD: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        0,
    );
    pub const LOCAL: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        1,
    );
    pub const PARENT: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        2,
    );
    pub const EXPLICIT: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EControlRigConstrainTab(pub u8);
impl EControlRigConstrainTab {
    pub const SPACES: EControlRigConstrainTab = EControlRigConstrainTab(0);
    pub const CONSTRAINTS: EControlRigConstrainTab = EControlRigConstrainTab(1);
    pub const SNAPPER: EControlRigConstrainTab = EControlRigConstrainTab(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECastToControlRigBlueprintCases(pub u8);
impl ECastToControlRigBlueprintCases {
    pub const CAST_SUCCEEDED: ECastToControlRigBlueprintCases = ECastToControlRigBlueprintCases(
        0,
    );
    pub const CAST_FAILED: ECastToControlRigBlueprintCases = ECastToControlRigBlueprintCases(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimToolBlendOperation(pub u8);
impl EAnimToolBlendOperation {
    pub const TWEEN: EAnimToolBlendOperation = EAnimToolBlendOperation(0);
    pub const BLEND_TO_NEIGHBOR: EAnimToolBlendOperation = EAnimToolBlendOperation(1);
    pub const PUSH_PULL: EAnimToolBlendOperation = EAnimToolBlendOperation(2);
    pub const BLEND_RELATIVE: EAnimToolBlendOperation = EAnimToolBlendOperation(3);
    pub const BLEND_TO_EASE: EAnimToolBlendOperation = EAnimToolBlendOperation(4);
    pub const SMOOTH_ROUGH: EAnimToolBlendOperation = EAnimToolBlendOperation(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimLayerType(pub u8);
impl EAnimLayerType {
    pub const BASE: EAnimLayerType = EAnimLayerType(0);
    pub const ADDITIVE: EAnimLayerType = EAnimLayerType(1);
    pub const OVERRIDE: EAnimLayerType = EAnimLayerType(2);
}
