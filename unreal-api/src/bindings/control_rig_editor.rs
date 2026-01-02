#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMultiControlRigElementSelection {
    __padding_end: [u8; 32],
}
impl FMultiControlRigElementSelection {}
#[repr(C, align(8))]
pub struct FControlRigInteractionTransformContext {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub drag: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_40: [u8; 8],
    pub rot: crate::bindings::core_u_object::FRotator,
    #[doc(hidden)]
    __padding_72: [u8; 8],
    pub scale: crate::bindings::core_u_object::FVector,
    pub space: EControlRigInteractionTransformSpace,
    __padding_end: [u8; 4],
}
impl FControlRigInteractionTransformContext {}
#[repr(C, align(4))]
pub struct FRigSpacePickerBakeSettings {
    pub target_space: crate::bindings::control_rig::FRigElementKey,
    pub settings: crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
    __padding_end: [u8; 16],
}
impl FRigSpacePickerBakeSettings {}
#[repr(C, align(8))]
pub struct FAIESelectionSetItemName {
    pub name: FString,
    pub mirror_name: FString,
    pub ty: i32,
    pub owner_actor_name: FString,
}
impl FAIESelectionSetItemName {}
#[repr(C, align(4))]
pub struct FAIESelectionSetItemViewData {
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub row: i32,
    __padding_end: [u8; 4],
}
impl FAIESelectionSetItemViewData {}
#[repr(C, align(8))]
pub struct FAIESelectionSetItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item_name: FText,
    pub names: TArray<FAIESelectionSetItemName>,
    pub view_data: FAIESelectionSetItemViewData,
    __padding_end: [u8; 32],
}
impl FAIESelectionSetItem {}
#[repr(C, align(1))]
pub struct FAnimDetailsBool {
    pub bool: bool,
}
impl FAnimDetailsBool {}
#[repr(C, align(8))]
pub struct FAnimDetailsEnum {
    __padding_end: [u8; 16],
}
impl FAnimDetailsEnum {}
#[repr(C, align(8))]
pub struct FAnimDetailsFloat {
    pub float: f64,
}
impl FAnimDetailsFloat {}
#[repr(C, align(8))]
pub struct FAnimDetailsInteger {
    pub integer: i64,
}
impl FAnimDetailsInteger {}
#[repr(C, align(8))]
pub struct FAnimDetailsLocation {
    pub lx: f64,
    pub ly: f64,
    pub lz: f64,
}
impl FAnimDetailsLocation {}
#[repr(C, align(8))]
pub struct FAnimDetailsRotation {
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
}
impl FAnimDetailsRotation {}
#[repr(C, align(8))]
pub struct FAnimDetailsScale {
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}
impl FAnimDetailsScale {}
#[repr(C, align(8))]
pub struct FAnimDetailsVector2D {
    pub x: f64,
    pub y: f64,
}
impl FAnimDetailsVector2D {}
#[repr(C, align(8))]
pub struct FControlRigSequencerBindingProxy {
    pub proxy: crate::bindings::movie_scene::FMovieSceneBindingProxy,
    pub control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
    pub track: UPtr<crate::bindings::control_rig::UMovieSceneControlRigParameterTrack>,
}
impl FControlRigSequencerBindingProxy {}
#[repr(C, align(8))]
pub struct FControlRigRigHierarchyDragAndDropContext {
    pub dragged_hierarchy_keys: TArray<crate::bindings::control_rig::FRigHierarchyKey>,
    pub target_hierarchy_key: crate::bindings::control_rig::FRigHierarchyKey,
    __padding_end: [u8; 4],
}
impl FControlRigRigHierarchyDragAndDropContext {}
#[repr(C, align(8))]
pub struct FControlRigGraphNodeContextMenuContext {
    pub graph: UPtr<crate::bindings::rig_vm_developer::URigVMGraph>,
    pub node: UPtr<crate::bindings::rig_vm_developer::URigVMNode>,
    pub pin: UPtr<crate::bindings::rig_vm_developer::URigVMPin>,
}
impl FControlRigGraphNodeContextMenuContext {}
#[repr(C, align(8))]
pub struct FControlRigRigHierarchyToGraphDragAndDropContext {
    pub dragged_hierarchy_keys: TArray<crate::bindings::control_rig::FRigHierarchyKey>,
    __padding_end: [u8; 24],
}
impl FControlRigRigHierarchyToGraphDragAndDropContext {}
#[repr(C, align(4))]
pub struct FMergeAnimLayerSettings {
    pub baking_key_settings: crate::bindings::movie_scene_tools::EBakingKeySettings,
    pub frame_increment: i32,
    pub b_reduce_keys: bool,
    pub tolerance_percentage: f32,
}
impl FMergeAnimLayerSettings {}
#[repr(C, align(4))]
pub struct FAnimLayerPropertyAndChannels {
    pub name: FName,
    __padding_end: [u8; 4],
}
impl FAnimLayerPropertyAndChannels {}
#[repr(C, align(8))]
pub struct FAnimLayerSelectionSet {
    pub bound_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub names: TMap<FName, FAnimLayerPropertyAndChannels>,
}
impl FAnimLayerSelectionSet {}
#[repr(C, align(8))]
pub struct FAnimLayerSectionItem {
    pub anim_layer_set: FAnimLayerSelectionSet,
    pub section: TWeakObjectPtr<crate::bindings::movie_scene::UMovieSceneSection>,
}
impl FAnimLayerSectionItem {}
#[repr(C, align(8))]
pub struct FAnimLayerItem {
    pub section_items: TArray<FAnimLayerSectionItem>,
    __padding_end: [u8; 16],
}
impl FAnimLayerItem {}
#[repr(C, align(8))]
pub struct FAnimLayerState {
    pub b_keyed: crate::bindings::slate_core::ECheckBoxState,
    pub b_selected: crate::bindings::slate_core::ECheckBoxState,
    pub b_active: bool,
    pub b_lock: bool,
    pub name: FText,
    pub weight: f64,
    pub ty: i32,
    __padding_end: [u8; 4],
}
impl FAnimLayerState {}
#[repr(C, align(8))]
pub struct FAnimLayerControlRigObject {
    pub control_rig: TWeakObjectPtr<crate::bindings::control_rig::UControlRig>,
    pub control_names: TArray<FName>,
}
impl FAnimLayerControlRigObject {}
#[repr(C, align(4))]
pub struct FAnimLayerSceneObject {
    pub scene_object_or_component: TWeakObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
}
impl FAnimLayerSceneObject {}
#[repr(C, align(8))]
pub struct FAnimLayerObjects {
    pub control_rig_objects: TArray<FAnimLayerControlRigObject>,
    pub scene_objects: TArray<FAnimLayerSceneObject>,
}
impl FAnimLayerObjects {}
#[repr(C, align(8))]
pub struct FControlRigForWorldTransforms {
    pub control_rig: TWeakObjectPtr<crate::bindings::control_rig::UControlRig>,
    pub control_names: TArray<FName>,
}
impl FControlRigForWorldTransforms {}
#[repr(C, align(8))]
pub struct FControlRigSnapperSelection {
    pub actors: TArray<crate::bindings::movie_scene::FActorForWorldTransforms>,
    pub control_rigs: TArray<FControlRigForWorldTransforms>,
}
impl FControlRigSnapperSelection {}
#[repr(C, align(8))]
pub struct UAnimDetailsOptionsMenuContext {
    __padding_end: [u8; 64],
}
impl UAnimDetailsOptionsMenuContext {}
#[repr(C, align(8))]
pub struct UAnimDetailsSettings {
    __padding_end: [u8; 56],
}
impl UAnimDetailsSettings {}
#[repr(C, align(8))]
pub struct UAnimSequenceConverterFactory {
    __padding_end: [u8; 168],
}
impl UAnimSequenceConverterFactory {}
#[repr(C, align(8))]
pub struct UConstraintCreationOptions {
    __padding_end: [u8; 56],
}
impl UConstraintCreationOptions {}
#[repr(C, align(8))]
pub struct UAIESelectionSets {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub selection_sets: TMap<
        crate::bindings::core_u_object::FGuid,
        FAIESelectionSetItem,
    >,
    __padding_end: [u8; 200],
}
impl UAIESelectionSets {}
#[repr(C, align(8))]
pub struct UAnimDetailsProxyManager {
    __padding_end: [u8; 336],
}
impl UAnimDetailsProxyManager {}
#[repr(C, align(8))]
pub struct UAnimDetailsSelection {
    __padding_end: [u8; 160],
}
impl UAnimDetailsSelection {}
#[repr(C, align(16))]
pub struct UAnimDetailsProxyBase {
    __padding_end: [u8; 368],
}
impl UAnimDetailsProxyBase {}
#[repr(C, align(16))]
pub struct UAnimDetailsProxyBool {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub bool: FAnimDetailsBool,
    __padding_end: [u8; 7],
}
impl UAnimDetailsProxyBool {}
#[repr(C, align(16))]
pub struct UAnimDetailsProxyEnum {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub enum_: FAnimDetailsEnum,
    __padding_end: [u8; 8],
}
impl UAnimDetailsProxyEnum {}
#[repr(C, align(16))]
pub struct UAnimDetailsProxyFloat {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub float: FAnimDetailsFloat,
}
impl UAnimDetailsProxyFloat {}
#[repr(C, align(16))]
pub struct UAnimDetailsProxyInteger {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub integer: FAnimDetailsInteger,
}
impl UAnimDetailsProxyInteger {}
#[repr(C, align(16))]
pub struct UAnimDetailsProxyLocation {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub location: FAnimDetailsLocation,
}
impl UAnimDetailsProxyLocation {}
#[repr(C, align(16))]
pub struct UAnimDetailsProxyRotation {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub rotation: FAnimDetailsRotation,
}
impl UAnimDetailsProxyRotation {}
#[repr(C, align(16))]
pub struct UAnimDetailsProxyScale {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub scale: FAnimDetailsScale,
}
impl UAnimDetailsProxyScale {}
#[repr(C, align(16))]
pub struct UAnimDetailsProxyTransform {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub location: FAnimDetailsLocation,
    pub rotation: FAnimDetailsRotation,
    pub scale: FAnimDetailsScale,
}
impl UAnimDetailsProxyTransform {}
#[repr(C, align(16))]
pub struct UAnimDetailsProxyVector2D {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub vector2_d: FAnimDetailsVector2D,
    __padding_end: [u8; 8],
}
impl UAnimDetailsProxyVector2D {}
#[repr(C, align(8))]
pub struct UBakeToControlRigSettings {
    __padding_end: [u8; 72],
}
impl UBakeToControlRigSettings {}
#[repr(C, align(8))]
pub struct UControlRigBlueprintEditorLibrary {
    __padding_end: [u8; 48],
}
impl UControlRigBlueprintEditorLibrary {}
#[repr(C, align(8))]
pub struct UControlRigBlueprintFactory {
    __padding_end: [u8; 144],
}
impl UControlRigBlueprintFactory {}
#[repr(C, align(8))]
pub struct UControlRigShapeLibraryFactory {
    __padding_end: [u8; 136],
}
impl UControlRigShapeLibraryFactory {}
#[repr(C, align(8))]
pub struct UControlRigSequencerEditorLibrary {
    __padding_end: [u8; 48],
}
impl UControlRigSequencerEditorLibrary {}
#[repr(C, align(8))]
pub struct UControlRigThumbnailRenderer {
    __padding_end: [u8; 232],
}
impl UControlRigThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UControlRigEditModeDelegateHelper {
    __padding_end: [u8; 72],
}
impl UControlRigEditModeDelegateHelper {}
#[repr(C, align(8))]
pub struct UControlRigEditModeSettings {
    __padding_end: [u8; 240],
}
impl UControlRigEditModeSettings {}
#[repr(C, align(8))]
pub struct UControlRigContextMenuContext {
    __padding_end: [u8; 232],
}
impl UControlRigContextMenuContext {}
#[repr(C, align(16))]
pub struct UControlRigSkeletalMeshComponent {
    __padding_end: [u8; 5504],
}
impl UControlRigSkeletalMeshComponent {}
#[repr(C, align(8))]
pub struct UControlRigWrapperObject {
    __padding_end: [u8; 152],
}
impl UControlRigWrapperObject {}
#[repr(C, align(8))]
pub struct URigConnectorTargetsDetailWrapper {
    __padding_end: [u8; 88],
}
impl URigConnectorTargetsDetailWrapper {}
#[repr(C, align(8))]
pub struct URigDependencyGraph {
    __padding_end: [u8; 848],
}
impl URigDependencyGraph {}
#[repr(C, align(8))]
pub struct URigDependencyGraphNode {
    __padding_end: [u8; 456],
}
impl URigDependencyGraphNode {}
#[repr(C, align(8))]
pub struct URigDependencyGraphSchema {
    __padding_end: [u8; 48],
}
impl URigDependencyGraphSchema {}
#[repr(C, align(8))]
pub struct UAnimationAuthoringSettings {
    __padding_end: [u8; 112],
}
impl UAnimationAuthoringSettings {}
#[repr(C, align(8))]
pub struct UAnimLayerSequencerFilter {
    __padding_end: [u8; 48],
}
impl UAnimLayerSequencerFilter {}
#[repr(C, align(8))]
pub struct UAnimLayerWeightProxy {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub weight: f64,
}
impl UAnimLayerWeightProxy {}
#[repr(C, align(8))]
pub struct UAnimLayer {
    __padding_end: [u8; 184],
}
impl UAnimLayer {}
#[repr(C, align(8))]
pub struct UAnimLayers {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub anim_layers: TArray<UPtr<UAnimLayer>>,
    __padding_end: [u8; 72],
}
impl UAnimLayers {}
#[repr(C, align(8))]
pub struct UControlRigTrackFilter {
    __padding_end: [u8; 48],
}
impl UControlRigTrackFilter {}
#[repr(C, align(8))]
pub struct ULoadAnimToControlRigSettings {
    __padding_end: [u8; 80],
}
impl ULoadAnimToControlRigSettings {}
#[repr(C, align(8))]
pub struct USelectionSetsSettings {
    __padding_end: [u8; 120],
}
impl USelectionSetsSettings {}
#[repr(C, align(8))]
pub struct UAssetDefinition_ControlRigPose {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_ControlRigPose {}
#[repr(C, align(8))]
pub struct UControlRigPoseThumbnailRenderer {
    __padding_end: [u8; 72],
}
impl UControlRigPoseThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UControlRigSnapSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_keep_offset: bool,
    pub b_snap_position: bool,
    pub b_snap_rotation: bool,
    pub b_snap_scale: bool,
    pub baking_key_settings: crate::bindings::movie_scene_tools::EBakingKeySettings,
    pub frame_increment: i32,
    pub b_reduce_keys: bool,
    pub tolerance: f32,
    __padding_end: [u8; 4],
}
impl UControlRigSnapSettings {}
#[repr(C, align(8))]
pub struct UCreateControlPoseAssetRigSettings {
    __padding_end: [u8; 64],
}
impl UCreateControlPoseAssetRigSettings {}
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
#[repr(transparent)]
pub struct EControlRigConstrainTab(pub u8);
impl EControlRigConstrainTab {
    pub const SPACES: EControlRigConstrainTab = EControlRigConstrainTab(0);
    pub const CONSTRAINTS: EControlRigConstrainTab = EControlRigConstrainTab(1);
    pub const SNAPPER: EControlRigConstrainTab = EControlRigConstrainTab(2);
}
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
#[repr(transparent)]
pub struct EAnimLayerType(pub u8);
impl EAnimLayerType {
    pub const BASE: EAnimLayerType = EAnimLayerType(0);
    pub const ADDITIVE: EAnimLayerType = EAnimLayerType(1);
    pub const OVERRIDE: EAnimLayerType = EAnimLayerType(2);
}
