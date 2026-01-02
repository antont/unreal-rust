#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FCSVImportSettings {
    pub import_row_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub import_type: ECSVImportType,
    pub import_curve_interp_mode: crate::bindings::engine::ERichCurveInterpMode,
    __padding_end: [u8; 22],
}
impl FCSVImportSettings {}
#[repr(C, align(4))]
pub struct FFbxMaterialBakeSize {
    pub size: crate::bindings::core_u_object::FIntPoint,
    pub b_auto_detect: bool,
    __padding_end: [u8; 3],
}
impl FFbxMaterialBakeSize {}
#[repr(C, align(8))]
pub struct FImportMeshLodSectionsData {
    __padding_end: [u8; 16],
}
impl FImportMeshLodSectionsData {}
#[repr(C, align(8))]
pub struct FCollectionScriptingContainerSource {
    pub name: FName,
    pub title: FText,
}
impl FCollectionScriptingContainerSource {}
#[repr(C, align(4))]
pub struct FCollectionScriptingRef {
    pub container: FName,
    pub name: FName,
    pub share_type: crate::bindings::engine::ECollectionScriptingShareType,
    __padding_end: [u8; 3],
}
impl FCollectionScriptingRef {}
#[repr(C, align(8))]
pub struct UAssetDefinitionDefault {
    __padding_end: [u8; 72],
}
impl UAssetDefinitionDefault {}
#[repr(C, align(8))]
pub struct UFactory {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub flags_48: u8,
    pub supported_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub context_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub formats: TArray<FString>,
    pub flags_88: u8,
    pub automated_import_data: UPtr<UAutomatedAssetImportData>,
    pub asset_import_task: UPtr<UAssetImportTask>,
    pub supported_workflows: u8,
    __padding_end: [u8; 23],
}
impl UFactory {}
#[repr(C, align(8))]
pub struct UEditorWorldExtension {
    __padding_end: [u8; 80],
}
impl UEditorWorldExtension {}
#[repr(C, align(8))]
pub struct UActorFactory {
    __padding_end: [u8; 144],
}
impl UActorFactory {}
#[repr(C, align(8))]
pub struct UActorFactoryVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryVolume {}
#[repr(C, align(8))]
pub struct UActorFactoryBoxVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryBoxVolume {}
#[repr(C, align(8))]
pub struct UThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UDefaultSizedThumbnailRenderer {
    __padding_end: [u8; 56],
}
impl UDefaultSizedThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UBlueprintThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UBlueprintThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UEditorState {
    __padding_end: [u8; 48],
}
impl UEditorState {}
#[repr(C, align(8))]
pub struct UWorldDependantEditorState {
    __padding_end: [u8; 48],
}
impl UWorldDependantEditorState {}
#[repr(C, align(8))]
pub struct UActorEditorContextStateCollection {
    __padding_end: [u8; 128],
}
impl UActorEditorContextStateCollection {}
#[repr(C, align(8))]
pub struct UActorEditorContextClientState {
    __padding_end: [u8; 48],
}
impl UActorEditorContextClientState {}
#[repr(C, align(8))]
pub struct UBaseWidgetBlueprint {
    __padding_end: [u8; 1440],
}
impl UBaseWidgetBlueprint {}
#[repr(C, align(16))]
pub struct UEditorInteractiveToolsContext {
    __padding_end: [u8; 944],
}
impl UEditorInteractiveToolsContext {}
#[repr(C, align(16))]
pub struct UModeManagerInteractiveToolsContext {
    __padding_end: [u8; 1296],
}
impl UModeManagerInteractiveToolsContext {}
#[repr(C, align(8))]
pub struct UActorExporterT3D {
    __padding_end: [u8; 128],
}
impl UActorExporterT3D {}
#[repr(C, align(8))]
pub struct UGroupActorExporterT3D {
    __padding_end: [u8; 128],
}
impl UGroupActorExporterT3D {}
#[repr(C, align(8))]
pub struct UPhysicsVolumeExporterT3D {
    __padding_end: [u8; 128],
}
impl UPhysicsVolumeExporterT3D {}
#[repr(C, align(8))]
pub struct UActorFactoryAmbientSound {
    __padding_end: [u8; 144],
}
impl UActorFactoryAmbientSound {}
#[repr(C, align(8))]
pub struct UActorFactorySkeletalMesh {
    __padding_end: [u8; 152],
}
impl UActorFactorySkeletalMesh {}
#[repr(C, align(8))]
pub struct UActorFactoryAnimationAsset {
    __padding_end: [u8; 152],
}
impl UActorFactoryAnimationAsset {}
#[repr(C, align(8))]
pub struct UActorFactoryStaticMesh {
    __padding_end: [u8; 144],
}
impl UActorFactoryStaticMesh {}
#[repr(C, align(8))]
pub struct UActorFactoryBasicShape {
    __padding_end: [u8; 144],
}
impl UActorFactoryBasicShape {}
#[repr(C, align(8))]
pub struct UActorFactoryBlueprint {
    __padding_end: [u8; 144],
}
impl UActorFactoryBlueprint {}
#[repr(C, align(8))]
pub struct UActorFactoryBoxReflectionCapture {
    __padding_end: [u8; 144],
}
impl UActorFactoryBoxReflectionCapture {}
#[repr(C, align(8))]
pub struct UActorFactoryCameraActor {
    __padding_end: [u8; 144],
}
impl UActorFactoryCameraActor {}
#[repr(C, align(8))]
pub struct UActorFactoryCharacter {
    __padding_end: [u8; 144],
}
impl UActorFactoryCharacter {}
#[repr(C, align(8))]
pub struct UActorFactoryClass {
    __padding_end: [u8; 144],
}
impl UActorFactoryClass {}
#[repr(C, align(8))]
pub struct UActorFactoryCylinderVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryCylinderVolume {}
#[repr(C, align(8))]
pub struct UActorFactoryDeferredDecal {
    __padding_end: [u8; 144],
}
impl UActorFactoryDeferredDecal {}
#[repr(C, align(8))]
pub struct UActorFactoryDirectionalLight {
    __padding_end: [u8; 144],
}
impl UActorFactoryDirectionalLight {}
#[repr(C, align(8))]
pub struct UActorFactoryEmitter {
    __padding_end: [u8; 144],
}
impl UActorFactoryEmitter {}
#[repr(C, align(8))]
pub struct UActorFactoryEmptyActor {
    __padding_end: [u8; 152],
}
impl UActorFactoryEmptyActor {}
#[repr(C, align(8))]
pub struct UActorFactoryExponentialHeightFog {
    __padding_end: [u8; 144],
}
impl UActorFactoryExponentialHeightFog {}
#[repr(C, align(8))]
pub struct UActorFactoryInteractiveFoliage {
    __padding_end: [u8; 144],
}
impl UActorFactoryInteractiveFoliage {}
#[repr(C, align(8))]
pub struct UActorFactoryLevelSequence {
    __padding_end: [u8; 144],
}
impl UActorFactoryLevelSequence {}
#[repr(C, align(8))]
pub struct UActorFactoryLocalFogVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryLocalFogVolume {}
#[repr(C, align(8))]
pub struct UActorFactoryNote {
    __padding_end: [u8; 144],
}
impl UActorFactoryNote {}
#[repr(C, align(8))]
pub struct UActorFactoryPawn {
    __padding_end: [u8; 152],
}
impl UActorFactoryPawn {}
#[repr(C, align(8))]
pub struct UActorFactoryPhysicsAsset {
    __padding_end: [u8; 144],
}
impl UActorFactoryPhysicsAsset {}
#[repr(C, align(8))]
pub struct UActorFactoryPlanarReflection {
    __padding_end: [u8; 144],
}
impl UActorFactoryPlanarReflection {}
#[repr(C, align(8))]
pub struct UActorFactoryPlaneReflectionCapture {
    __padding_end: [u8; 144],
}
impl UActorFactoryPlaneReflectionCapture {}
#[repr(C, align(8))]
pub struct UActorFactoryPlayerStart {
    __padding_end: [u8; 144],
}
impl UActorFactoryPlayerStart {}
#[repr(C, align(8))]
pub struct UActorFactoryRuntimeVirtualTextureVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryRuntimeVirtualTextureVolume {}
#[repr(C, align(8))]
pub struct UActorFactorySkyAtmosphere {
    __padding_end: [u8; 144],
}
impl UActorFactorySkyAtmosphere {}
#[repr(C, align(8))]
pub struct UActorFactorySkyLight {
    __padding_end: [u8; 144],
}
impl UActorFactorySkyLight {}
#[repr(C, align(8))]
pub struct UActorFactorySphereReflectionCapture {
    __padding_end: [u8; 144],
}
impl UActorFactorySphereReflectionCapture {}
#[repr(C, align(8))]
pub struct UActorFactorySphereVolume {
    __padding_end: [u8; 144],
}
impl UActorFactorySphereVolume {}
#[repr(C, align(8))]
pub struct UActorFactoryTargetPoint {
    __padding_end: [u8; 144],
}
impl UActorFactoryTargetPoint {}
#[repr(C, align(8))]
pub struct UActorFactoryTextRender {
    __padding_end: [u8; 144],
}
impl UActorFactoryTextRender {}
#[repr(C, align(8))]
pub struct UActorFactoryTriggerBox {
    __padding_end: [u8; 144],
}
impl UActorFactoryTriggerBox {}
#[repr(C, align(8))]
pub struct UActorFactoryTriggerCapsule {
    __padding_end: [u8; 144],
}
impl UActorFactoryTriggerCapsule {}
#[repr(C, align(8))]
pub struct UActorFactoryTriggerSphere {
    __padding_end: [u8; 144],
}
impl UActorFactoryTriggerSphere {}
#[repr(C, align(8))]
pub struct UActorFactoryVectorFieldVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryVectorFieldVolume {}
#[repr(C, align(8))]
pub struct UActorFactoryVolumetricCloud {
    __padding_end: [u8; 144],
}
impl UActorFactoryVolumetricCloud {}
#[repr(C, align(8))]
pub struct UBlendSpaceFactory1D {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 16],
}
impl UBlendSpaceFactory1D {}
#[repr(C, align(8))]
pub struct UAimOffsetBlendSpaceFactory1D {
    __padding_end: [u8; 168],
}
impl UAimOffsetBlendSpaceFactory1D {}
#[repr(C, align(8))]
pub struct UBlendSpaceFactoryNew {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 16],
}
impl UBlendSpaceFactoryNew {}
#[repr(C, align(8))]
pub struct UAimOffsetBlendSpaceFactoryNew {
    __padding_end: [u8; 168],
}
impl UAimOffsetBlendSpaceFactoryNew {}
#[repr(C, align(8))]
pub struct UAnimationBlueprintEditorOptions {
    __padding_end: [u8; 56],
}
impl UAnimationBlueprintEditorOptions {}
#[repr(C, align(8))]
pub struct UAnimBlueprintSettings {
    __padding_end: [u8; 72],
}
impl UAnimBlueprintSettings {}
#[repr(C, align(8))]
pub struct UExporterFBX {
    __padding_end: [u8; 128],
}
impl UExporterFBX {}
#[repr(C, align(8))]
pub struct UAnimSequenceExporterFBX {
    __padding_end: [u8; 128],
}
impl UAnimSequenceExporterFBX {}
#[repr(C, align(8))]
pub struct UAssetEditorToolkitMenuContext {
    __padding_end: [u8; 64],
}
impl UAssetEditorToolkitMenuContext {}
#[repr(C, align(8))]
pub struct UBlueprintFactory {
    __padding_end: [u8; 176],
}
impl UBlueprintFactory {}
#[repr(C, align(8))]
pub struct UBlueprintFunctionLibraryFactory {
    __padding_end: [u8; 176],
}
impl UBlueprintFunctionLibraryFactory {}
#[repr(C, align(8))]
pub struct UBlueprintInterfaceFactory {
    __padding_end: [u8; 176],
}
impl UBlueprintInterfaceFactory {}
#[repr(C, align(8))]
pub struct UBlueprintMacroFactory {
    __padding_end: [u8; 176],
}
impl UBlueprintMacroFactory {}
#[repr(C, align(8))]
pub struct UCanvasRenderTarget2DFactoryNew {
    __padding_end: [u8; 152],
}
impl UCanvasRenderTarget2DFactoryNew {}
#[repr(C, align(8))]
pub struct UCascadeOptions {
    __padding_end: [u8; 296],
}
impl UCascadeOptions {}
#[repr(C, align(8))]
pub struct UClassViewerSettings {
    __padding_end: [u8; 72],
}
impl UClassViewerSettings {}
#[repr(C, align(8))]
pub struct UCompressAnimationsCommandlet {
    __padding_end: [u8; 136],
}
impl UCompressAnimationsCommandlet {}
#[repr(C, align(8))]
pub struct UEditorBrushBuilder {
    __padding_end: [u8; 136],
}
impl UEditorBrushBuilder {}
#[repr(C, align(8))]
pub struct UConeBuilder {
    __padding_end: [u8; 176],
}
impl UConeBuilder {}
#[repr(C, align(8))]
pub struct UContentBrowserSettings {
    __padding_end: [u8; 80],
}
impl UContentBrowserSettings {}
#[repr(C, align(8))]
pub struct UCookGlobalShadersCommandlet {
    __padding_end: [u8; 224],
}
impl UCookGlobalShadersCommandlet {}
#[repr(C, align(8))]
pub struct UCookGlobalShadersDeviceHelperBase {
    __padding_end: [u8; 48],
}
impl UCookGlobalShadersDeviceHelperBase {}
#[repr(C, align(8))]
pub struct UCookGlobalShadersDeviceHelperStaged {
    __padding_end: [u8; 64],
}
impl UCookGlobalShadersDeviceHelperStaged {}
#[repr(C, align(8))]
pub struct UCrashReporterSettings {
    __padding_end: [u8; 96],
}
impl UCrashReporterSettings {}
#[repr(C, align(8))]
pub struct UCubeBuilder {
    __padding_end: [u8; 168],
}
impl UCubeBuilder {}
#[repr(C, align(8))]
pub struct UCurvedStairBuilder {
    __padding_end: [u8; 176],
}
impl UCurvedStairBuilder {}
#[repr(C, align(8))]
pub struct UCurveEdOptions {
    __padding_end: [u8; 168],
}
impl UCurveEdOptions {}
#[repr(C, align(8))]
pub struct UCurveFactory {
    __padding_end: [u8; 144],
}
impl UCurveFactory {}
#[repr(C, align(8))]
pub struct UCurveFloatFactory {
    __padding_end: [u8; 144],
}
impl UCurveFloatFactory {}
#[repr(C, align(8))]
pub struct UCurveLinearColorFactory {
    __padding_end: [u8; 144],
}
impl UCurveLinearColorFactory {}
#[repr(C, align(8))]
pub struct UCurveVectorFactory {
    __padding_end: [u8; 144],
}
impl UCurveVectorFactory {}
#[repr(C, align(8))]
pub struct UCurveImportFactory {
    __padding_end: [u8; 136],
}
impl UCurveImportFactory {}
#[repr(C, align(8))]
pub struct UCurveLinearColorAtlasFactory {
    __padding_end: [u8; 144],
}
impl UCurveLinearColorAtlasFactory {}
#[repr(C, align(8))]
pub struct UCylinderBuilder {
    __padding_end: [u8; 168],
}
impl UCylinderBuilder {}
#[repr(C, align(8))]
pub struct UDataAssetFactory {
    __padding_end: [u8; 144],
}
impl UDataAssetFactory {}
#[repr(C, align(8))]
pub struct UDEditorParameterValue {
    __padding_end: [u8; 128],
}
impl UDEditorParameterValue {}
#[repr(C, align(16))]
pub struct UDEditorDoubleVectorParameterValue {
    __padding_end: [u8; 160],
}
impl UDEditorDoubleVectorParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorFontParameterValue {
    __padding_end: [u8; 144],
}
impl UDEditorFontParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorMaterialLayersParameterValue {
    __padding_end: [u8; 408],
}
impl UDEditorMaterialLayersParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorParameterCollectionParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorParameterCollectionParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorRuntimeVirtualTextureParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorRuntimeVirtualTextureParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorScalarParameterValue {
    __padding_end: [u8; 304],
}
impl UDEditorScalarParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorSparseVolumeTextureParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorSparseVolumeTextureParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorStaticComponentMaskParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorStaticComponentMaskParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorStaticSwitchParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorStaticSwitchParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorTextureCollectionParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorTextureCollectionParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorTextureParameterValue {
    __padding_end: [u8; 200],
}
impl UDEditorTextureParameterValue {}
#[repr(C, align(8))]
pub struct UDEditorVectorParameterValue {
    __padding_end: [u8; 216],
}
impl UDEditorVectorParameterValue {}
#[repr(C, align(8))]
pub struct UEditorExperimentalSettings {
    __padding_end: [u8; 120],
}
impl UEditorExperimentalSettings {}
#[repr(C, align(8))]
pub struct UEditorLoadingSavingSettings {
    __padding_end: [u8; 200],
}
impl UEditorLoadingSavingSettings {}
#[repr(C, align(8))]
pub struct UEditorMiscSettings {
    __padding_end: [u8; 48],
}
impl UEditorMiscSettings {}
#[repr(C, align(16))]
pub struct UEditorStyleSettings {
    __padding_end: [u8; 624],
}
impl UEditorStyleSettings {}
#[repr(C, align(8))]
pub struct UEnumFactory {
    __padding_end: [u8; 136],
}
impl UEnumFactory {}
#[repr(C, align(8))]
pub struct UExportTextContainer {
    __padding_end: [u8; 64],
}
impl UExportTextContainer {}
#[repr(C, align(8))]
pub struct UFbxImportUI {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub b_is_obj_import: bool,
    pub original_import_type: EFBXImportType,
    pub mesh_type_to_import: EFBXImportType,
    #[doc(hidden)]
    __padding_60: [u8; 1],
    pub flags_60: u8,
    #[doc(hidden)]
    __padding_64: [u8; 3],
    pub b_import_as_skeletal: bool,
    pub b_import_mesh: bool,
    pub skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub flags_80: u8,
    pub physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    pub flags_96: u8,
    pub lod_distance0: f32,
    pub lod_distance1: f32,
    pub lod_distance2: f32,
    pub lod_distance3: f32,
    pub lod_distance4: f32,
    pub lod_distance5: f32,
    pub lod_distance6: f32,
    pub lod_distance7: f32,
    pub minimum_lod_number: i32,
    pub lod_number: i32,
    pub flags_140: u8,
    pub override_animation_name: FString,
    pub flags_160: u8,
    pub static_mesh_import_data: UPtr<UFbxStaticMeshImportData>,
    pub skeletal_mesh_import_data: UPtr<UFbxSkeletalMeshImportData>,
    pub anim_sequence_import_data: UPtr<UFbxAnimSequenceImportData>,
    pub texture_import_data: UPtr<UFbxTextureImportData>,
    pub b_automated_import_should_detect_type: bool,
    __padding_end: [u8; 335],
}
impl UFbxImportUI {}
#[repr(C, align(8))]
pub struct UFontFactory {
    __padding_end: [u8; 136],
}
impl UFontFactory {}
#[repr(C, align(8))]
pub struct UFontFileImportFactory {
    __padding_end: [u8; 176],
}
impl UFontFileImportFactory {}
#[repr(C, align(8))]
pub struct UForceFeedbackAttenuationFactory {
    __padding_end: [u8; 136],
}
impl UForceFeedbackAttenuationFactory {}
#[repr(C, align(8))]
pub struct UForceFeedbackEffectFactory {
    __padding_end: [u8; 136],
}
impl UForceFeedbackEffectFactory {}
#[repr(C, align(8))]
pub struct UHapticFeedbackEffectBufferFactory {
    __padding_end: [u8; 136],
}
impl UHapticFeedbackEffectBufferFactory {}
#[repr(C, align(8))]
pub struct UHapticFeedbackEffectCurveFactory {
    __padding_end: [u8; 136],
}
impl UHapticFeedbackEffectCurveFactory {}
#[repr(C, align(8))]
pub struct UHapticFeedbackEffectSoundWaveFactory {
    __padding_end: [u8; 136],
}
impl UHapticFeedbackEffectSoundWaveFactory {}
#[repr(C, align(8))]
pub struct ULandscapeGrassTypeCommandlet {
    __padding_end: [u8; 136],
}
impl ULandscapeGrassTypeCommandlet {}
pub struct ULegacyEdModeSelectInterface {}
pub struct ILegacyEdModeSelectInterface {}
pub struct ULegacyEdModeWidgetInterface {}
pub struct ILegacyEdModeWidgetInterface {}
pub struct ULegacyEdModeToolInterface {}
pub struct ILegacyEdModeToolInterface {}
pub struct ULegacyEdModeDrawHelperInterface {}
pub struct ILegacyEdModeDrawHelperInterface {}
pub struct ULegacyEdModeViewportInterface {}
pub struct ILegacyEdModeViewportInterface {}
#[repr(C, align(8))]
pub struct ULevelEditorMiscSettings {
    __padding_end: [u8; 176],
}
impl ULevelEditorMiscSettings {}
#[repr(C, align(8))]
pub struct UCommonResolutionMenuContext {
    __padding_end: [u8; 72],
}
impl UCommonResolutionMenuContext {}
#[repr(C, align(8))]
pub struct ULevelEditorPlaySettings {
    __padding_end: [u8; 576],
}
impl ULevelEditorPlaySettings {}
#[repr(C, align(8))]
pub struct ULevelEditorViewportSettings {
    __padding_end: [u8; 664],
}
impl ULevelEditorViewportSettings {}
#[repr(C, align(8))]
pub struct ULevelExporterFBX {
    __padding_end: [u8; 128],
}
impl ULevelExporterFBX {}
#[repr(C, align(8))]
pub struct ULevelExporterLOD {
    __padding_end: [u8; 128],
}
impl ULevelExporterLOD {}
#[repr(C, align(8))]
pub struct ULevelExporterOBJ {
    __padding_end: [u8; 128],
}
impl ULevelExporterOBJ {}
#[repr(C, align(8))]
pub struct ULevelExporterSTL {
    __padding_end: [u8; 128],
}
impl ULevelExporterSTL {}
#[repr(C, align(8))]
pub struct ULevelExporterT3D {
    __padding_end: [u8; 128],
}
impl ULevelExporterT3D {}
#[repr(C, align(8))]
pub struct ULevelFactory {
    __padding_end: [u8; 136],
}
impl ULevelFactory {}
#[repr(C, align(8))]
pub struct ULinearStairBuilder {
    __padding_end: [u8; 168],
}
impl ULinearStairBuilder {}
#[repr(C, align(8))]
pub struct UListMaterialsUsedWithMeshEmittersCommandlet {
    __padding_end: [u8; 136],
}
impl UListMaterialsUsedWithMeshEmittersCommandlet {}
#[repr(C, align(8))]
pub struct UListStaticMeshesImportedFromSpeedTreesCommandlet {
    __padding_end: [u8; 136],
}
impl UListStaticMeshesImportedFromSpeedTreesCommandlet {}
#[repr(C, align(8))]
pub struct ULoadPackageCommandlet {
    __padding_end: [u8; 136],
}
impl ULoadPackageCommandlet {}
#[repr(C, align(8))]
pub struct UMaterialEditorParameters {
    __padding_end: [u8; 80],
}
impl UMaterialEditorParameters {}
#[repr(C, align(8))]
pub struct UMaterialEditorInstanceConstant {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub subsurface_profile: UPtr<crate::bindings::engine::USubsurfaceProfile>,
    #[doc(hidden)]
    __padding_136: [u8; 8],
    pub specular_profile: UPtr<crate::bindings::engine::USpecularProfile>,
    __padding_end: [u8; 200],
}
impl UMaterialEditorInstanceConstant {}
#[repr(C, align(8))]
pub struct UMaterialEditorOptions {
    __padding_end: [u8; 72],
}
impl UMaterialEditorOptions {}
#[repr(C, align(8))]
pub struct UMaterialEditorPreviewParameters {
    __padding_end: [u8; 120],
}
impl UMaterialEditorPreviewParameters {}
#[repr(C, align(8))]
pub struct UMaterialFactoryNew {
    __padding_end: [u8; 144],
}
impl UMaterialFactoryNew {}
#[repr(C, align(8))]
pub struct UMaterialFunctionFactoryNew {
    __padding_end: [u8; 136],
}
impl UMaterialFunctionFactoryNew {}
#[repr(C, align(8))]
pub struct UMaterialFunctionInstanceFactory {
    __padding_end: [u8; 144],
}
impl UMaterialFunctionInstanceFactory {}
#[repr(C, align(8))]
pub struct UMaterialFunctionMaterialLayerInstanceFactory {
    __padding_end: [u8; 144],
}
impl UMaterialFunctionMaterialLayerInstanceFactory {}
#[repr(C, align(8))]
pub struct UMaterialFunctionMaterialLayerBlendInstanceFactory {
    __padding_end: [u8; 144],
}
impl UMaterialFunctionMaterialLayerBlendInstanceFactory {}
#[repr(C, align(8))]
pub struct UMaterialFunctionMaterialLayerBlendFactory {
    __padding_end: [u8; 136],
}
impl UMaterialFunctionMaterialLayerBlendFactory {}
#[repr(C, align(8))]
pub struct UMaterialFunctionMaterialLayerFactory {
    __padding_end: [u8; 136],
}
impl UMaterialFunctionMaterialLayerFactory {}
#[repr(C, align(8))]
pub struct UMaterialInstanceConstantFactoryNew {
    __padding_end: [u8; 144],
}
impl UMaterialInstanceConstantFactoryNew {}
#[repr(C, align(8))]
pub struct UMaterialParameterCollectionFactoryNew {
    __padding_end: [u8; 136],
}
impl UMaterialParameterCollectionFactoryNew {}
#[repr(C, align(8))]
pub struct UMaterialStatsOptions {
    __padding_end: [u8; 688],
}
impl UMaterialStatsOptions {}
#[repr(C, align(8))]
pub struct UModelExporterT3D {
    __padding_end: [u8; 128],
}
impl UModelExporterT3D {}
#[repr(C, align(8))]
pub struct UModelFactory {
    __padding_end: [u8; 136],
}
impl UModelFactory {}
#[repr(C, align(8))]
pub struct UNeuralProfileFactory {
    __padding_end: [u8; 136],
}
impl UNeuralProfileFactory {}
#[repr(C, align(8))]
pub struct UObjectExporterT3D {
    __padding_end: [u8; 128],
}
impl UObjectExporterT3D {}
#[repr(C, align(8))]
pub struct UObjectLibraryFactory {
    __padding_end: [u8; 136],
}
impl UObjectLibraryFactory {}
#[repr(C, align(8))]
pub struct UPackageFactory {
    __padding_end: [u8; 136],
}
impl UPackageFactory {}
#[repr(C, align(8))]
pub struct UParticleSystemFactoryNew {
    __padding_end: [u8; 136],
}
impl UParticleSystemFactoryNew {}
#[repr(C, align(8))]
pub struct UPersonaOptions {
    __padding_end: [u8; 632],
}
impl UPersonaOptions {}
#[repr(C, align(8))]
pub struct UPhysicalMaterialFactoryNew {
    __padding_end: [u8; 144],
}
impl UPhysicalMaterialFactoryNew {}
#[repr(C, align(8))]
pub struct UPhysicalMaterialMaskFactory {
    __padding_end: [u8; 176],
}
impl UPhysicalMaterialMaskFactory {}
#[repr(C, align(8))]
pub struct UPhysicsAssetEditorOptions {
    __padding_end: [u8; 144],
}
impl UPhysicsAssetEditorOptions {}
#[repr(C, align(8))]
pub struct UPhysicsAssetGenerationSettings {
    __padding_end: [u8; 88],
}
impl UPhysicsAssetGenerationSettings {}
#[repr(C, align(8))]
pub struct UPkgInfoCommandlet {
    __padding_end: [u8; 136],
}
impl UPkgInfoCommandlet {}
#[repr(C, align(8))]
pub struct UPolysExporterOBJ {
    __padding_end: [u8; 128],
}
impl UPolysExporterOBJ {}
#[repr(C, align(8))]
pub struct UPolysExporterT3D {
    __padding_end: [u8; 128],
}
impl UPolysExporterT3D {}
#[repr(C, align(8))]
pub struct UPolysFactory {
    __padding_end: [u8; 136],
}
impl UPolysFactory {}
#[repr(C, align(8))]
pub struct UPreviewMeshCollectionFactory {
    __padding_end: [u8; 144],
}
impl UPreviewMeshCollectionFactory {}
#[repr(C, align(8))]
pub struct UPropertyColorSettings {
    __padding_end: [u8; 64],
}
impl UPropertyColorSettings {}
#[repr(C, align(8))]
pub struct UCSVImportFactory {
    #[doc(hidden)]
    __padding_152: [u8; 152],
    pub automated_import_settings: FCSVImportSettings,
    __padding_end: [u8; 8],
}
impl UCSVImportFactory {}
#[repr(C, align(8))]
pub struct UReimportCurveFactory {
    __padding_end: [u8; 224],
}
impl UReimportCurveFactory {}
#[repr(C, align(8))]
pub struct UReimportCurveTableFactory {
    __padding_end: [u8; 224],
}
impl UReimportCurveTableFactory {}
#[repr(C, align(8))]
pub struct UReimportDataTableFactory {
    __padding_end: [u8; 224],
}
impl UReimportDataTableFactory {}
#[repr(C, align(8))]
pub struct UFbxFactory {
    __padding_end: [u8; 160],
}
impl UFbxFactory {}
#[repr(C, align(8))]
pub struct UReimportFbxAnimSequenceFactory {
    __padding_end: [u8; 192],
}
impl UReimportFbxAnimSequenceFactory {}
#[repr(C, align(8))]
pub struct UReimportFbxSkeletalMeshFactory {
    __padding_end: [u8; 192],
}
impl UReimportFbxSkeletalMeshFactory {}
#[repr(C, align(8))]
pub struct UReimportFbxStaticMeshFactory {
    __padding_end: [u8; 192],
}
impl UReimportFbxStaticMeshFactory {}
#[repr(C, align(16))]
pub struct UTextureFactory {
    __padding_end: [u8; 256],
}
impl UTextureFactory {}
#[repr(C, align(16))]
pub struct UReimportTextureFactory {
    __padding_end: [u8; 304],
}
impl UReimportTextureFactory {}
#[repr(C, align(8))]
pub struct UVectorFieldStaticFactory {
    __padding_end: [u8; 136],
}
impl UVectorFieldStaticFactory {}
#[repr(C, align(8))]
pub struct UReimportVectorFieldStaticFactory {
    __padding_end: [u8; 168],
}
impl UReimportVectorFieldStaticFactory {}
#[repr(C, align(8))]
pub struct URenderTargetExporterPNG {
    __padding_end: [u8; 128],
}
impl URenderTargetExporterPNG {}
#[repr(C, align(8))]
pub struct URenderTargetExporterEXR {
    __padding_end: [u8; 128],
}
impl URenderTargetExporterEXR {}
#[repr(C, align(8))]
pub struct UReplaceActorCommandlet {
    __padding_end: [u8; 136],
}
impl UReplaceActorCommandlet {}
#[repr(C, align(8))]
pub struct UResavePackagesCommandlet {
    __padding_end: [u8; 624],
}
impl UResavePackagesCommandlet {}
#[repr(C, align(8))]
pub struct USceneImportFactory {
    __padding_end: [u8; 136],
}
impl USceneImportFactory {}
#[repr(C, align(8))]
pub struct UEditorViewportViewMenuContext {
    __padding_end: [u8; 64],
}
impl UEditorViewportViewMenuContext {}
#[repr(C, align(8))]
pub struct USequenceExporterT3D {
    __padding_end: [u8; 128],
}
impl USequenceExporterT3D {}
#[repr(C, align(8))]
pub struct USheetBuilder {
    __padding_end: [u8; 168],
}
impl USheetBuilder {}
#[repr(C, align(8))]
pub struct USkeletalMeshEditorSettings {
    __padding_end: [u8; 96],
}
impl USkeletalMeshEditorSettings {}
#[repr(C, align(8))]
pub struct USkeletalMeshExporterFBX {
    __padding_end: [u8; 128],
}
impl USkeletalMeshExporterFBX {}
#[repr(C, align(8))]
pub struct USkeletalMeshToolMenuContext {
    __padding_end: [u8; 64],
}
impl USkeletalMeshToolMenuContext {}
#[repr(C, align(8))]
pub struct USoundExporterOGG {
    __padding_end: [u8; 128],
}
impl USoundExporterOGG {}
#[repr(C, align(8))]
pub struct USoundExporterWAV {
    __padding_end: [u8; 128],
}
impl USoundExporterWAV {}
#[repr(C, align(8))]
pub struct USoundSurroundExporterWAV {
    __padding_end: [u8; 128],
}
impl USoundSurroundExporterWAV {}
#[repr(C, align(8))]
pub struct USpecularProfileFactory {
    __padding_end: [u8; 136],
}
impl USpecularProfileFactory {}
#[repr(C, align(8))]
pub struct USpiralStairBuilder {
    __padding_end: [u8; 176],
}
impl USpiralStairBuilder {}
#[repr(C, align(8))]
pub struct UStaticMeshExporterFBX {
    __padding_end: [u8; 128],
}
impl UStaticMeshExporterFBX {}
#[repr(C, align(8))]
pub struct UStaticMeshExporterOBJ {
    __padding_end: [u8; 128],
}
impl UStaticMeshExporterOBJ {}
#[repr(C, align(8))]
pub struct UStaticMeshMinLodCommandlet {
    __padding_end: [u8; 136],
}
impl UStaticMeshMinLodCommandlet {}
#[repr(C, align(8))]
pub struct UStringTableFactory {
    __padding_end: [u8; 136],
}
impl UStringTableFactory {}
#[repr(C, align(8))]
pub struct UStructureFactory {
    __padding_end: [u8; 136],
}
impl UStructureFactory {}
#[repr(C, align(8))]
pub struct UStructViewerSettings {
    __padding_end: [u8; 56],
}
impl UStructViewerSettings {}
#[repr(C, align(8))]
pub struct USubsurfaceProfileFactory {
    __padding_end: [u8; 136],
}
impl USubsurfaceProfileFactory {}
#[repr(C, align(8))]
pub struct USubUVAnimationFactory {
    __padding_end: [u8; 144],
}
impl USubUVAnimationFactory {}
#[repr(C, align(8))]
pub struct UTetrahedronBuilder {
    __padding_end: [u8; 160],
}
impl UTetrahedronBuilder {}
#[repr(C, align(8))]
pub struct UTexAligner {
    __padding_end: [u8; 80],
}
impl UTexAligner {}
#[repr(C, align(8))]
pub struct UTexAlignerBox {
    __padding_end: [u8; 80],
}
impl UTexAlignerBox {}
#[repr(C, align(8))]
pub struct UTexAlignerDefault {
    __padding_end: [u8; 80],
}
impl UTexAlignerDefault {}
#[repr(C, align(8))]
pub struct UTexAlignerFit {
    __padding_end: [u8; 80],
}
impl UTexAlignerFit {}
#[repr(C, align(8))]
pub struct UTexAlignerPlanar {
    __padding_end: [u8; 80],
}
impl UTexAlignerPlanar {}
#[repr(C, align(8))]
pub struct UTextBufferExporterTXT {
    __padding_end: [u8; 128],
}
impl UTextBufferExporterTXT {}
#[repr(C, align(8))]
pub struct UTextureThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UTextureThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UTexture2DArrayThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UTexture2DArrayThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UTexture2DFactoryNew {
    __padding_end: [u8; 144],
}
impl UTexture2DFactoryNew {}
#[repr(C, align(8))]
pub struct UTextureCubeExporterHDR {
    __padding_end: [u8; 128],
}
impl UTextureCubeExporterHDR {}
#[repr(C, align(8))]
pub struct URenderTargetCubeExporterHDR {
    __padding_end: [u8; 128],
}
impl URenderTargetCubeExporterHDR {}
#[repr(C, align(8))]
pub struct UTextureExporterGeneric {
    __padding_end: [u8; 128],
}
impl UTextureExporterGeneric {}
#[repr(C, align(8))]
pub struct UTextureExporterBMP {
    __padding_end: [u8; 128],
}
impl UTextureExporterBMP {}
#[repr(C, align(8))]
pub struct UVirtualTextureBuilderExporterBMP {
    __padding_end: [u8; 128],
}
impl UVirtualTextureBuilderExporterBMP {}
#[repr(C, align(8))]
pub struct UTextureExporterDDS {
    __padding_end: [u8; 128],
}
impl UTextureExporterDDS {}
#[repr(C, align(8))]
pub struct UVirtualTextureBuilderExporterDDS {
    __padding_end: [u8; 128],
}
impl UVirtualTextureBuilderExporterDDS {}
#[repr(C, align(8))]
pub struct UTextureExporterEXR {
    __padding_end: [u8; 128],
}
impl UTextureExporterEXR {}
#[repr(C, align(8))]
pub struct UVirtualTextureBuilderExporterEXR {
    __padding_end: [u8; 128],
}
impl UVirtualTextureBuilderExporterEXR {}
#[repr(C, align(8))]
pub struct UTextureExporterHDR {
    __padding_end: [u8; 128],
}
impl UTextureExporterHDR {}
#[repr(C, align(8))]
pub struct UVirtualTextureBuilderExporterHDR {
    __padding_end: [u8; 128],
}
impl UVirtualTextureBuilderExporterHDR {}
#[repr(C, align(8))]
pub struct UTextureExporterPNG {
    __padding_end: [u8; 128],
}
impl UTextureExporterPNG {}
#[repr(C, align(8))]
pub struct UVirtualTextureBuilderExporterPNG {
    __padding_end: [u8; 128],
}
impl UVirtualTextureBuilderExporterPNG {}
#[repr(C, align(8))]
pub struct UTextureExporterJPEG {
    __padding_end: [u8; 128],
}
impl UTextureExporterJPEG {}
#[repr(C, align(8))]
pub struct UTextureExporterUEJPEG {
    __padding_end: [u8; 128],
}
impl UTextureExporterUEJPEG {}
#[repr(C, align(8))]
pub struct UTextureExporterTGA {
    __padding_end: [u8; 128],
}
impl UTextureExporterTGA {}
#[repr(C, align(8))]
pub struct UUDIMTextureFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UUDIMTextureFunctionLibrary {}
#[repr(C, align(8))]
pub struct UTextureRenderTarget2DArrayFactoryNew {
    __padding_end: [u8; 152],
}
impl UTextureRenderTarget2DArrayFactoryNew {}
#[repr(C, align(8))]
pub struct UTextureRenderTargetCubeFactoryNew {
    __padding_end: [u8; 144],
}
impl UTextureRenderTargetCubeFactoryNew {}
#[repr(C, align(8))]
pub struct UTextureRenderTargetFactoryNew {
    __padding_end: [u8; 152],
}
impl UTextureRenderTargetFactoryNew {}
#[repr(C, align(8))]
pub struct UTextureRenderTargetVolumeFactoryNew {
    __padding_end: [u8; 152],
}
impl UTextureRenderTargetVolumeFactoryNew {}
#[repr(C, align(8))]
pub struct UTouchInterfaceFactory {
    __padding_end: [u8; 136],
}
impl UTouchInterfaceFactory {}
#[repr(C, align(8))]
pub struct UTransactor {
    __padding_end: [u8; 48],
}
impl UTransactor {}
#[repr(C, align(8))]
pub struct UTransBuffer {
    __padding_end: [u8; 336],
}
impl UTransBuffer {}
#[repr(C, align(16))]
pub struct UTrueTypeFontFactory {
    __padding_end: [u8; 304],
}
impl UTrueTypeFontFactory {}
#[repr(C, align(8))]
pub struct UUnrealEdKeyBindings {
    __padding_end: [u8; 64],
}
impl UUnrealEdKeyBindings {}
#[repr(C, align(8))]
pub struct UUnrealEdOptions {
    __padding_end: [u8; 240],
}
impl UUnrealEdOptions {}
#[repr(C, align(8))]
pub struct UVectorFieldExporter {
    __padding_end: [u8; 128],
}
impl UVectorFieldExporter {}
#[repr(C, align(8))]
pub struct UViewportToolBarContext {
    __padding_end: [u8; 80],
}
impl UViewportToolBarContext {}
#[repr(C, align(8))]
pub struct UVolumetricBuilder {
    __padding_end: [u8; 160],
}
impl UVolumetricBuilder {}
#[repr(C, align(8))]
pub struct UWorldPartitionBuildNavigationOptions {
    __padding_end: [u8; 56],
}
impl UWorldPartitionBuildNavigationOptions {}
#[repr(C, align(8))]
pub struct UDataLayerConversionInfo {
    __padding_end: [u8; 96],
}
impl UDataLayerConversionInfo {}
#[repr(C, align(8))]
pub struct UDataLayerToAssetCommandletContext {
    __padding_end: [u8; 80],
}
impl UDataLayerToAssetCommandletContext {}
#[repr(C, align(8))]
pub struct UDataLayerToAssetCommandlet {
    __padding_end: [u8; 192],
}
impl UDataLayerToAssetCommandlet {}
#[repr(C, align(8))]
pub struct UWorldPartitionBuilder {
    __padding_end: [u8; 464],
}
impl UWorldPartitionBuilder {}
#[repr(C, align(8))]
pub struct UWorldPartitionLandscapeBuilder {
    __padding_end: [u8; 464],
}
impl UWorldPartitionLandscapeBuilder {}
#[repr(C, align(8))]
pub struct UWrangleContentCommandlet {
    __padding_end: [u8; 136],
}
impl UWrangleContentCommandlet {}
#[repr(C, align(8))]
pub struct UActorGroupingUtils {
    __padding_end: [u8; 48],
}
impl UActorGroupingUtils {}
#[repr(C, align(8))]
pub struct UAnalyticsPrivacySettings {
    __padding_end: [u8; 64],
}
impl UAnalyticsPrivacySettings {}
#[repr(C, align(8))]
pub struct UCrashReportsPrivacySettings {
    __padding_end: [u8; 64],
}
impl UCrashReportsPrivacySettings {}
#[repr(C, align(8))]
pub struct UAnimSeqExportOption {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_export_transforms: bool,
    pub b_export_morph_targets: bool,
    pub b_export_attribute_curves: bool,
    pub b_export_material_curves: bool,
    pub b_record_in_world_space: bool,
    pub b_evaluate_all_skeletal_mesh_components: bool,
    pub interpolation: crate::bindings::engine::EAnimInterpolationType,
    pub curve_interpolation: crate::bindings::engine::ERichCurveInterpMode,
    pub include_animation_names: TArray<FString>,
    pub exclude_animation_names: TArray<FString>,
    pub warm_up_frames: crate::bindings::core_u_object::FFrameNumber,
    pub delay_before_start: crate::bindings::core_u_object::FFrameNumber,
    pub b_transact_recording: bool,
    pub b_bake_timecode: bool,
    pub b_timecode_rate_override: bool,
    pub override_timecode_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_use_custom_time_range: bool,
    pub custom_start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub custom_end_frame: crate::bindings::core_u_object::FFrameNumber,
    pub custom_display_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_use_custom_frame_rate: bool,
    pub custom_frame_rate: crate::bindings::core_u_object::FFrameRate,
    __padding_end: [u8; 4],
}
impl UAnimSeqExportOption {}
#[repr(C, align(16))]
pub struct UDebugSkelMeshComponent {
    __padding_end: [u8; 5184],
}
impl UDebugSkelMeshComponent {}
#[repr(C, align(8))]
pub struct UEditorAnimBaseObj {
    __padding_end: [u8; 80],
}
impl UEditorAnimBaseObj {}
#[repr(C, align(8))]
pub struct UEditorAnimCompositeSegment {
    __padding_end: [u8; 128],
}
impl UEditorAnimCompositeSegment {}
#[repr(C, align(8))]
pub struct UEditorAnimCurveBoneLinks {
    __padding_end: [u8; 136],
}
impl UEditorAnimCurveBoneLinks {}
#[repr(C, align(8))]
pub struct UEditorAnimSegment {
    __padding_end: [u8; 128],
}
impl UEditorAnimSegment {}
#[repr(C, align(8))]
pub struct UEditorCompositeSection {
    __padding_end: [u8; 184],
}
impl UEditorCompositeSection {}
#[repr(C, align(8))]
pub struct UEditorNotifyObject {
    __padding_end: [u8; 320],
}
impl UEditorNotifyObject {}
#[repr(C, align(8))]
pub struct UEditorParentPlayerListObj {
    __padding_end: [u8; 152],
}
impl UEditorParentPlayerListObj {}
#[repr(C, align(8))]
pub struct UEditorSkeletonNotifyObj {
    __padding_end: [u8; 80],
}
impl UEditorSkeletonNotifyObj {}
#[repr(C, align(8))]
pub struct UAssetGuideline {
    __padding_end: [u8; 112],
}
impl UAssetGuideline {}
#[repr(C, align(8))]
pub struct UAssetImportTask {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub filename: FString,
    pub destination_path: FString,
    pub destination_name: FString,
    pub b_replace_existing: bool,
    pub b_replace_existing_settings: bool,
    pub b_automated: bool,
    pub b_save: bool,
    pub b_async: bool,
    pub factory: UPtr<UFactory>,
    pub options: UPtr<crate::bindings::core_u_object::UObject>,
    pub imported_object_paths: TArray<FString>,
    __padding_end: [u8; 32],
}
impl UAssetImportTask {}
#[repr(C, align(8))]
pub struct UAutomatedAssetImportData {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub group_name: FString,
    pub filenames: TArray<FString>,
    pub destination_path: FString,
    pub factory_name: FString,
    pub b_replace_existing: bool,
    pub b_skip_read_only: bool,
    pub factory: UPtr<UFactory>,
    pub level_to_load: FString,
    __padding_end: [u8; 16],
}
impl UAutomatedAssetImportData {}
#[repr(C, align(8))]
pub struct UAutoReimportManager {
    __padding_end: [u8; 64],
}
impl UAutoReimportManager {}
#[repr(C, align(8))]
pub struct UAssetRegUtilCommandlet {
    __padding_end: [u8; 144],
}
impl UAssetRegUtilCommandlet {}
#[repr(C, align(8))]
pub struct UAssetRegistryDumpCommandlet {
    __padding_end: [u8; 136],
}
impl UAssetRegistryDumpCommandlet {}
#[repr(C, align(8))]
pub struct UAssetSizeQueryCommandlet {
    __padding_end: [u8; 136],
}
impl UAssetSizeQueryCommandlet {}
#[repr(C, align(8))]
pub struct UAudioMixerCommandlet {
    __padding_end: [u8; 136],
}
impl UAudioMixerCommandlet {}
#[repr(C, align(8))]
pub struct UBaseIteratePackagesCommandlet {
    __padding_end: [u8; 584],
}
impl UBaseIteratePackagesCommandlet {}
#[repr(C, align(8))]
pub struct UChunkDependencyInfo {
    __padding_end: [u8; 192],
}
impl UChunkDependencyInfo {}
#[repr(C, align(8))]
pub struct UCompileAllBlueprintsCommandlet {
    __padding_end: [u8; 288],
}
impl UCompileAllBlueprintsCommandlet {}
#[repr(C, align(8))]
pub struct UCompileShadersTestBedCommandlet {
    __padding_end: [u8; 136],
}
impl UCompileShadersTestBedCommandlet {}
#[repr(C, align(8))]
pub struct UConvertLevelsToExternalActorsCommandlet {
    __padding_end: [u8; 144],
}
impl UConvertLevelsToExternalActorsCommandlet {}
#[repr(C, align(8))]
pub struct UCookCommandlet {
    __padding_end: [u8; 216],
}
impl UCookCommandlet {}
#[repr(C, align(8))]
pub struct UCookShadersCommandlet {
    __padding_end: [u8; 136],
}
impl UCookShadersCommandlet {}
#[repr(C, align(8))]
pub struct UCopyNaniteFallbackSettingsToRayTracingProxyCommandlet {
    __padding_end: [u8; 136],
}
impl UCopyNaniteFallbackSettingsToRayTracingProxyCommandlet {}
#[repr(C, align(8))]
pub struct UDDCCleanupCommandlet {
    __padding_end: [u8; 136],
}
impl UDDCCleanupCommandlet {}
#[repr(C, align(8))]
pub struct UDebugShaderCompileJobCommandlet {
    __padding_end: [u8; 136],
}
impl UDebugShaderCompileJobCommandlet {}
#[repr(C, align(8))]
pub struct UDerivedDataCacheCommandlet {
    __padding_end: [u8; 432],
}
impl UDerivedDataCacheCommandlet {}
#[repr(C, align(8))]
pub struct UDetectOrphanedLocalizedAssetsCommandlet {
    __padding_end: [u8; 136],
}
impl UDetectOrphanedLocalizedAssetsCommandlet {}
#[repr(C, align(8))]
pub struct UDiffAssetRegistriesCommandlet {
    __padding_end: [u8; 2488],
}
impl UDiffAssetRegistriesCommandlet {}
#[repr(C, align(8))]
pub struct UDiffAssetsCommandlet {
    __padding_end: [u8; 136],
}
impl UDiffAssetsCommandlet {}
#[repr(C, align(8))]
pub struct UDiffCookCommandlet {
    __padding_end: [u8; 3488],
}
impl UDiffCookCommandlet {}
#[repr(C, align(8))]
pub struct UDiffFilesCommandlet {
    __padding_end: [u8; 272],
}
impl UDiffFilesCommandlet {}
#[repr(C, align(8))]
pub struct UDumpAssetRegistryCommandlet {
    __padding_end: [u8; 192],
}
impl UDumpAssetRegistryCommandlet {}
#[repr(C, align(8))]
pub struct UDumpBlueprintsInfoCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpBlueprintsInfoCommandlet {}
#[repr(C, align(8))]
pub struct UDumpHiddenCategoriesCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpHiddenCategoriesCommandlet {}
#[repr(C, align(8))]
pub struct UDumpLightFunctionMaterialInfoCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpLightFunctionMaterialInfoCommandlet {}
#[repr(C, align(8))]
pub struct UDumpMaterialExpressionInfoCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpMaterialExpressionInfoCommandlet {}
#[repr(C, align(8))]
pub struct UDumpMaterialExpressionsCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpMaterialExpressionsCommandlet {}
#[repr(C, align(8))]
pub struct UDumpMaterialInfoCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpMaterialInfoCommandlet {}
#[repr(C, align(8))]
pub struct UDumpMaterialShaderTypesCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpMaterialShaderTypesCommandlet {}
#[repr(C, align(8))]
pub struct UGatherTextCommandletBase {
    __padding_end: [u8; 264],
}
impl UGatherTextCommandletBase {}
#[repr(C, align(8))]
pub struct UExportDialogueScriptCommandlet {
    __padding_end: [u8; 264],
}
impl UExportDialogueScriptCommandlet {}
#[repr(C, align(8))]
pub struct UExportPakDependenciesCommandlet {
    __padding_end: [u8; 136],
}
impl UExportPakDependenciesCommandlet {}
#[repr(C, align(8))]
pub struct UExternalActorsCommandlet {
    __padding_end: [u8; 176],
}
impl UExternalActorsCommandlet {}
#[repr(C, align(8))]
pub struct UExtractLocResCommandlet {
    __padding_end: [u8; 136],
}
impl UExtractLocResCommandlet {}
#[repr(C, align(8))]
pub struct UFileServerCommandlet {
    __padding_end: [u8; 152],
}
impl UFileServerCommandlet {}
#[repr(C, align(8))]
pub struct UFixConflictingLocalizationKeysCommandlet {
    __padding_end: [u8; 136],
}
impl UFixConflictingLocalizationKeysCommandlet {}
#[repr(C, align(8))]
pub struct UFixupNeedsLoadForEditorGameCommandlet {
    __padding_end: [u8; 704],
}
impl UFixupNeedsLoadForEditorGameCommandlet {}
#[repr(C, align(8))]
pub struct UGatherTextCommandlet {
    __padding_end: [u8; 1024],
}
impl UGatherTextCommandlet {}
#[repr(C, align(8))]
pub struct UGatherTextFromAssetsCommandlet {
    __padding_end: [u8; 656],
}
impl UGatherTextFromAssetsCommandlet {}
#[repr(C, align(8))]
pub struct UGatherTextFromAssetsWorkerCommandlet {
    __padding_end: [u8; 232],
}
impl UGatherTextFromAssetsWorkerCommandlet {}
#[repr(C, align(8))]
pub struct UGatherTextFromMetaDataCommandlet {
    __padding_end: [u8; 336],
}
impl UGatherTextFromMetaDataCommandlet {}
#[repr(C, align(8))]
pub struct UGatherTextFromSourceCommandlet {
    __padding_end: [u8; 264],
}
impl UGatherTextFromSourceCommandlet {}
#[repr(C, align(8))]
pub struct UGenerateAssetManifestCommandlet {
    __padding_end: [u8; 136],
}
impl UGenerateAssetManifestCommandlet {}
#[repr(C, align(8))]
pub struct UGenerateBlueprintAPICommandlet {
    __padding_end: [u8; 136],
}
impl UGenerateBlueprintAPICommandlet {}
#[repr(C, align(8))]
pub struct UGenerateDistillFileSetsCommandlet {
    __padding_end: [u8; 136],
}
impl UGenerateDistillFileSetsCommandlet {}
#[repr(C, align(8))]
pub struct UGenerateGatherArchiveCommandlet {
    __padding_end: [u8; 264],
}
impl UGenerateGatherArchiveCommandlet {}
#[repr(C, align(8))]
pub struct UGenerateGatherManifestCommandlet {
    __padding_end: [u8; 264],
}
impl UGenerateGatherManifestCommandlet {}
#[repr(C, align(8))]
pub struct UGenerateTextLocalizationReportCommandlet {
    __padding_end: [u8; 312],
}
impl UGenerateTextLocalizationReportCommandlet {}
#[repr(C, align(8))]
pub struct UGenerateTextLocalizationResourceCommandlet {
    __padding_end: [u8; 264],
}
impl UGenerateTextLocalizationResourceCommandlet {}
#[repr(C, align(8))]
pub struct UImportAssetsCommandlet {
    __padding_end: [u8; 192],
}
impl UImportAssetsCommandlet {}
#[repr(C, align(8))]
pub struct UImportDialogueScriptCommandlet {
    __padding_end: [u8; 264],
}
impl UImportDialogueScriptCommandlet {}
#[repr(C, align(8))]
pub struct UImportLocalizedDialogueCommandlet {
    __padding_end: [u8; 344],
}
impl UImportLocalizedDialogueCommandlet {}
#[repr(C, align(8))]
pub struct UInternationalizationConditioningCommandlet {
    __padding_end: [u8; 17232],
}
impl UInternationalizationConditioningCommandlet {}
#[repr(C, align(8))]
pub struct UInternationalizationExportCommandlet {
    __padding_end: [u8; 272],
}
impl UInternationalizationExportCommandlet {}
#[repr(C, align(8))]
pub struct UIoStoreCommandlet {
    __padding_end: [u8; 136],
}
impl UIoStoreCommandlet {}
#[repr(C, align(8))]
pub struct UMakeBinaryConfigCommandlet {
    __padding_end: [u8; 136],
}
impl UMakeBinaryConfigCommandlet {}
#[repr(C, align(8))]
pub struct UMaterialLayerUsageCommandlet {
    __padding_end: [u8; 136],
}
impl UMaterialLayerUsageCommandlet {}
#[repr(C, align(8))]
pub struct UMergeShaderPipelineCachesCommandlet {
    __padding_end: [u8; 136],
}
impl UMergeShaderPipelineCachesCommandlet {}
#[repr(C, align(8))]
pub struct UParticleSystemAuditCommandlet {
    __padding_end: [u8; 1232],
}
impl UParticleSystemAuditCommandlet {}
#[repr(C, align(8))]
pub struct UPopulateDialogueWaveFromCharacterSheetCommandlet {
    __padding_end: [u8; 136],
}
impl UPopulateDialogueWaveFromCharacterSheetCommandlet {}
#[repr(C, align(8))]
pub struct UReplaceAssetsCommandlet {
    __padding_end: [u8; 136],
}
impl UReplaceAssetsCommandlet {}
#[repr(C, align(8))]
pub struct USavePackageUtilitiesCommandlet {
    __padding_end: [u8; 136],
}
impl USavePackageUtilitiesCommandlet {}
#[repr(C, align(8))]
pub struct UShaderCodeLibraryToolsCommandlet {
    __padding_end: [u8; 136],
}
impl UShaderCodeLibraryToolsCommandlet {}
#[repr(C, align(8))]
pub struct UShaderPipelineCacheToolsCommandlet {
    __padding_end: [u8; 136],
}
impl UShaderPipelineCacheToolsCommandlet {}
#[repr(C, align(8))]
pub struct UStabilizeLocalizationKeysCommandlet {
    __padding_end: [u8; 136],
}
impl UStabilizeLocalizationKeysCommandlet {}
#[repr(C, align(8))]
pub struct USubstrateCommandlet {
    __padding_end: [u8; 136],
}
impl USubstrateCommandlet {}
#[repr(C, align(8))]
pub struct USummarizeTraceCommandlet {
    __padding_end: [u8; 168],
}
impl USummarizeTraceCommandlet {}
#[repr(C, align(8))]
pub struct USummarizeTraceEditorWorkflowsCommandlet {
    __padding_end: [u8; 136],
}
impl USummarizeTraceEditorWorkflowsCommandlet {}
#[repr(C, align(8))]
pub struct USwapSoundForDialogueInCuesCommandlet {
    __padding_end: [u8; 136],
}
impl USwapSoundForDialogueInCuesCommandlet {}
#[repr(C, align(8))]
pub struct UTextAssetCommandlet {
    __padding_end: [u8; 136],
}
impl UTextAssetCommandlet {}
#[repr(C, align(8))]
pub struct UUnrealPakCommandlet {
    __padding_end: [u8; 136],
}
impl UUnrealPakCommandlet {}
#[repr(C, align(8))]
pub struct UUpdateGameProjectCommandlet {
    __padding_end: [u8; 136],
}
impl UUpdateGameProjectCommandlet {}
#[repr(C, align(8))]
pub struct UWorldPartitionBuilderCommandlet {
    __padding_end: [u8; 256],
}
impl UWorldPartitionBuilderCommandlet {}
#[repr(C, align(8))]
pub struct UWorldPartitionConvertCommandlet {
    __padding_end: [u8; 944],
}
impl UWorldPartitionConvertCommandlet {}
#[repr(C, align(8))]
pub struct UCookFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UCookFunctionLibrary {}
#[repr(C, align(8))]
pub struct UEdGraphNode_Comment {
    __padding_end: [u8; 296],
}
impl UEdGraphNode_Comment {}
#[repr(C, align(8))]
pub struct UActorEditorContextActorFolderState {
    __padding_end: [u8; 64],
}
impl UActorEditorContextActorFolderState {}
#[repr(C, align(8))]
pub struct UEditorDomainSaveCommandlet {
    __padding_end: [u8; 136],
}
impl UEditorDomainSaveCommandlet {}
#[repr(C, align(16))]
pub struct UEditorEngine {
    __padding_end: [u8; 10032],
}
impl UEditorEngine {}
#[repr(C, align(8))]
pub struct UEditorLevelUtils {
    __padding_end: [u8; 48],
}
impl UEditorLevelUtils {}
#[repr(C, align(8))]
pub struct UEditorPerformanceSettings {
    __padding_end: [u8; 200],
}
impl UEditorPerformanceSettings {}
#[repr(C, align(8))]
pub struct UActorEditorContextEditorState {
    __padding_end: [u8; 64],
}
impl UActorEditorContextEditorState {}
#[repr(C, align(8))]
pub struct UEditorStateSubsystem {
    __padding_end: [u8; 80],
}
impl UEditorStateSubsystem {}
#[repr(C, align(8))]
pub struct UWorldEditorState {
    __padding_end: [u8; 96],
}
impl UWorldEditorState {}
#[repr(C, align(8))]
pub struct UEditorWorldExtensionCollection {
    __padding_end: [u8; 80],
}
impl UEditorWorldExtensionCollection {}
#[repr(C, align(8))]
pub struct UEditorWorldExtensionManager {
    __padding_end: [u8; 64],
}
impl UEditorWorldExtensionManager {}
#[repr(C, align(8))]
pub struct UActorElementDetailsInterface {
    __padding_end: [u8; 56],
}
impl UActorElementDetailsInterface {}
#[repr(C, align(8))]
pub struct UActorElementEditorAssetDataInterface {
    __padding_end: [u8; 56],
}
impl UActorElementEditorAssetDataInterface {}
#[repr(C, align(8))]
pub struct UActorElementsCopy {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub actors_to_copy: TArray<UPtr<crate::bindings::engine::AActor>>,
}
impl UActorElementsCopy {}
#[repr(C, align(8))]
pub struct UActorElementsExporterT3D {
    __padding_end: [u8; 128],
}
impl UActorElementsExporterT3D {}
#[repr(C, align(8))]
pub struct UActorElementEditorSelectionInterface {
    __padding_end: [u8; 56],
}
impl UActorElementEditorSelectionInterface {}
#[repr(C, align(8))]
pub struct UActorElementEditorWorldInterface {
    __padding_end: [u8; 56],
}
impl UActorElementEditorWorldInterface {}
#[repr(C, align(8))]
pub struct UComponentElementDetailsInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementDetailsInterface {}
#[repr(C, align(8))]
pub struct UComponentElementsCopy {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub components_to_copy: TArray<UPtr<crate::bindings::engine::UActorComponent>>,
}
impl UComponentElementsCopy {}
#[repr(C, align(8))]
pub struct UComponentElementsExporterT3D {
    __padding_end: [u8; 128],
}
impl UComponentElementsExporterT3D {}
#[repr(C, align(8))]
pub struct UComponentElementEditorSelectionInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementEditorSelectionInterface {}
#[repr(C, align(8))]
pub struct UComponentElementEditorWorldInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementEditorWorldInterface {}
#[repr(C, align(8))]
pub struct UObjectElementDetailsInterface {
    __padding_end: [u8; 56],
}
impl UObjectElementDetailsInterface {}
#[repr(C, align(8))]
pub struct UObjectElementEditorSelectionInterface {
    __padding_end: [u8; 56],
}
impl UObjectElementEditorSelectionInterface {}
#[repr(C, align(8))]
pub struct USMInstanceElementDetailsInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementDetailsInterface {}
#[repr(C, align(16))]
pub struct USMInstanceElementDetailsProxyObject {
    __padding_end: [u8; 192],
}
impl USMInstanceElementDetailsProxyObject {}
#[repr(C, align(8))]
pub struct USMInstanceElementEditorSelectionInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementEditorSelectionInterface {}
#[repr(C, align(8))]
pub struct USMInstanceElementEditorWorldInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementEditorWorldInterface {}
#[repr(C, align(8))]
pub struct UActorFactoryPointLight {
    __padding_end: [u8; 144],
}
impl UActorFactoryPointLight {}
#[repr(C, align(8))]
pub struct UActorFactoryRectLight {
    __padding_end: [u8; 144],
}
impl UActorFactoryRectLight {}
#[repr(C, align(8))]
pub struct UActorFactorySpotLight {
    __padding_end: [u8; 144],
}
impl UActorFactorySpotLight {}
#[repr(C, align(8))]
pub struct UAnimBankFactory {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 16],
}
impl UAnimBankFactory {}
#[repr(C, align(8))]
pub struct UTransformProviderDataFactory {
    __padding_end: [u8; 144],
}
impl UTransformProviderDataFactory {}
#[repr(C, align(8))]
pub struct UAnimBankDataFactory {
    __padding_end: [u8; 144],
}
impl UAnimBankDataFactory {}
#[repr(C, align(8))]
pub struct UAnimBlueprintFactory {
    __padding_end: [u8; 176],
}
impl UAnimBlueprintFactory {}
#[repr(C, align(8))]
pub struct UAnimLayerInterfaceFactory {
    __padding_end: [u8; 176],
}
impl UAnimLayerInterfaceFactory {}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionSettingsFactory {
    __padding_end: [u8; 136],
}
impl UAnimBoneCompressionSettingsFactory {}
#[repr(C, align(8))]
pub struct UAnimCompositeFactory {
    __padding_end: [u8; 176],
}
impl UAnimCompositeFactory {}
#[repr(C, align(8))]
pub struct UAnimCurveCompressionSettingsFactory {
    __padding_end: [u8; 136],
}
impl UAnimCurveCompressionSettingsFactory {}
#[repr(C, align(8))]
pub struct UAnimMontageFactory {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub source_animation: UPtr<crate::bindings::engine::UAnimSequence>,
    __padding_end: [u8; 24],
}
impl UAnimMontageFactory {}
#[repr(C, align(8))]
pub struct UAnimSequenceFactory {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 16],
}
impl UAnimSequenceFactory {}
#[repr(C, align(8))]
pub struct UAnimStreamableFactory {
    __padding_end: [u8; 152],
}
impl UAnimStreamableFactory {}
#[repr(C, align(8))]
pub struct UCompositeCurveTableFactory {
    __padding_end: [u8; 136],
}
impl UCompositeCurveTableFactory {}
#[repr(C, align(8))]
pub struct UDataTableFactory {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub _struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
}
impl UDataTableFactory {}
#[repr(C, align(8))]
pub struct UCompositeDataTableFactory {
    __padding_end: [u8; 144],
}
impl UCompositeDataTableFactory {}
#[repr(C, align(8))]
pub struct UCurveTableFactory {
    __padding_end: [u8; 144],
}
impl UCurveTableFactory {}
#[repr(C, align(8))]
pub struct UEditorStaticMeshFactory {
    __padding_end: [u8; 224],
}
impl UEditorStaticMeshFactory {}
#[repr(C, align(8))]
pub struct UInterchangeReimportHandler {
    __padding_end: [u8; 80],
}
impl UInterchangeReimportHandler {}
#[repr(C, align(8))]
pub struct ULightWeightInstanceFactory {
    __padding_end: [u8; 144],
}
impl ULightWeightInstanceFactory {}
#[repr(C, align(8))]
pub struct UMaterialImportHelpers {
    __padding_end: [u8; 48],
}
impl UMaterialImportHelpers {}
#[repr(C, align(8))]
pub struct UMeshDeformerCollectionFactory {
    __padding_end: [u8; 136],
}
impl UMeshDeformerCollectionFactory {}
#[repr(C, align(8))]
pub struct UMirrorTableFindReplaceExpressions {
    __padding_end: [u8; 64],
}
impl UMirrorTableFindReplaceExpressions {}
#[repr(C, align(8))]
pub struct UMirrorDataTableFactory {
    __padding_end: [u8; 160],
}
impl UMirrorDataTableFactory {}
#[repr(C, align(8))]
pub struct UPackFactory {
    __padding_end: [u8; 136],
}
impl UPackFactory {}
#[repr(C, align(8))]
pub struct UPhysicsAssetFactory {
    __padding_end: [u8; 160],
}
impl UPhysicsAssetFactory {}
#[repr(C, align(8))]
pub struct UPoseAssetFactory {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub source_animation: UPtr<crate::bindings::engine::UAnimSequence>,
    pub pose_names: TArray<FString>,
    __padding_end: [u8; 16],
}
impl UPoseAssetFactory {}
#[repr(C, align(8))]
pub struct USkeletonFactory {
    __padding_end: [u8; 160],
}
impl USkeletonFactory {}
#[repr(C, align(8))]
pub struct USlateBrushAssetFactory {
    __padding_end: [u8; 144],
}
impl USlateBrushAssetFactory {}
#[repr(C, align(8))]
pub struct USlateWidgetStyleAssetFactory {
    __padding_end: [u8; 144],
}
impl USlateWidgetStyleAssetFactory {}
#[repr(C, align(8))]
pub struct USparseVolumeTextureMaterialFactoryNew {
    __padding_end: [u8; 144],
}
impl USparseVolumeTextureMaterialFactoryNew {}
#[repr(C, align(8))]
pub struct USparseVolumeTextureMaterialInstanceFactoryNew {
    __padding_end: [u8; 200],
}
impl USparseVolumeTextureMaterialInstanceFactoryNew {}
#[repr(C, align(8))]
pub struct UTexture2DArrayFactory {
    __padding_end: [u8; 152],
}
impl UTexture2DArrayFactory {}
#[repr(C, align(8))]
pub struct UTextureCubeArrayFactory {
    __padding_end: [u8; 152],
}
impl UTextureCubeArrayFactory {}
#[repr(C, align(8))]
pub struct UVariableFrameStrippingSettingsFactory {
    __padding_end: [u8; 136],
}
impl UVariableFrameStrippingSettingsFactory {}
#[repr(C, align(8))]
pub struct UVolumeTextureFactory {
    __padding_end: [u8; 144],
}
impl UVolumeTextureFactory {}
#[repr(C, align(8))]
pub struct UWorldFactory {
    __padding_end: [u8; 144],
}
impl UWorldFactory {}
#[repr(C, align(8))]
pub struct UFbxAssetImportData {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub import_translation: crate::bindings::core_u_object::FVector,
    pub import_rotation: crate::bindings::core_u_object::FRotator,
    pub import_uniform_scale: f32,
    #[doc(hidden)]
    __padding_149: [u8; 1],
    pub b_convert_scene: bool,
    pub b_force_front_x_axis: bool,
    pub b_convert_scene_unit: bool,
    __padding_end: [u8; 24],
}
impl UFbxAssetImportData {}
#[repr(C, align(8))]
pub struct UFbxAnimSequenceImportData {
    __padding_end: [u8; 256],
}
impl UFbxAnimSequenceImportData {}
#[repr(C, align(8))]
pub struct UFbxExportOption {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub fbx_export_compatibility: EFbxExportCompatibility,
    #[doc(hidden)]
    __padding_52: [u8; 3],
    pub flags_52: u8,
    pub flags_53: u8,
    #[doc(hidden)]
    __padding_56: [u8; 2],
    pub bake_camera_and_light_animation: EMovieSceneBakeType,
    pub bake_actor_animation: EMovieSceneBakeType,
    pub bake_material_inputs: EFbxMaterialBakeMode,
    pub default_material_bake_size: FFbxMaterialBakeSize,
}
impl UFbxExportOption {}
#[repr(C, align(8))]
pub struct UFbxMeshImportData {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub b_transform_vertex_to_absolute: bool,
    pub b_bake_pivot_in_vertex: bool,
    #[doc(hidden)]
    __padding_180: [u8; 2],
    pub flags_180: u8,
    #[doc(hidden)]
    __padding_184: [u8; 3],
    pub normal_import_method: EFBXNormalImportMethod,
    pub normal_generation_method: EFBXNormalGenerationMethod,
    #[doc(hidden)]
    __padding_188: [u8; 2],
    pub flags_188: u8,
    #[doc(hidden)]
    __padding_192: [u8; 3],
    pub b_reorder_material_to_fbx_order: bool,
    __padding_end: [u8; 39],
}
impl UFbxMeshImportData {}
#[repr(C, align(8))]
pub struct UFbxSceneImportData {
    __padding_end: [u8; 184],
}
impl UFbxSceneImportData {}
#[repr(C, align(8))]
pub struct UFbxSceneImportFactory {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub scene_import_options: UPtr<UFbxSceneImportOptions>,
    pub scene_import_options_static_mesh: UPtr<UFbxSceneImportOptionsStaticMesh>,
    pub scene_import_options_skeletal_mesh: UPtr<UFbxSceneImportOptionsSkeletalMesh>,
    __padding_end: [u8; 240],
}
impl UFbxSceneImportFactory {}
#[repr(C, align(8))]
pub struct UFbxSceneImportOptions {
    __padding_end: [u8; 128],
}
impl UFbxSceneImportOptions {}
#[repr(C, align(8))]
pub struct UFbxSceneImportOptionsSkeletalMesh {
    __padding_end: [u8; 104],
}
impl UFbxSceneImportOptionsSkeletalMesh {}
#[repr(C, align(8))]
pub struct UFbxSceneImportOptionsStaticMesh {
    __padding_end: [u8; 80],
}
impl UFbxSceneImportOptionsStaticMesh {}
#[repr(C, align(8))]
pub struct UFbxSkeletalMeshImportData {
    #[doc(hidden)]
    __padding_234: [u8; 234],
    pub vertex_color_import_option: EVertexColorImportOption,
    pub vertex_override_color: crate::bindings::core_u_object::FColor,
    __padding_end: [u8; 24],
}
impl UFbxSkeletalMeshImportData {}
#[repr(C, align(8))]
pub struct UFbxStaticMeshImportData {
    #[doc(hidden)]
    __padding_232: [u8; 232],
    pub static_mesh_lod_group: FName,
    pub vertex_color_import_option: EVertexColorImportOption,
    pub vertex_override_color: crate::bindings::core_u_object::FColor,
    pub flags_252: u8,
    pub distance_field_resolution_scale: f32,
    __padding_end: [u8; 4],
}
impl UFbxStaticMeshImportData {}
#[repr(C, align(8))]
pub struct UFbxTextureImportData {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub flags_176: u8,
    #[doc(hidden)]
    __padding_180: [u8; 3],
    pub material_search_location: EMaterialSearchLocation,
    pub base_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    #[doc(hidden)]
    __padding_232: [u8; 8],
    pub base_color_name: FString,
    pub base_diffuse_texture_name: FString,
    pub base_normal_texture_name: FString,
    pub base_emissive_color_name: FString,
    pub base_emmisive_texture_name: FString,
    pub base_specular_texture_name: FString,
    pub base_opacity_texture_name: FString,
}
impl UFbxTextureImportData {}
#[repr(C, align(8))]
pub struct UReimportFbxSceneFactory {
    __padding_end: [u8; 464],
}
impl UReimportFbxSceneFactory {}
#[repr(C, align(8))]
pub struct UEditorLoadingAndSavingUtils {
    __padding_end: [u8; 48],
}
impl UEditorLoadingAndSavingUtils {}
#[repr(C, align(8))]
pub struct AGroupActor {
    __padding_end: [u8; 1176],
}
impl AGroupActor {}
#[repr(C, align(8))]
pub struct UHierarchicalLODSettings {
    __padding_end: [u8; 240],
}
impl UHierarchicalLODSettings {}
#[repr(C, align(8))]
pub struct AHierarchicalLODVolume {
    __padding_end: [u8; 1232],
}
impl AHierarchicalLODVolume {}
#[repr(C, align(16))]
pub struct UEditorInstancedPlacementSettings {
    __padding_end: [u8; 800],
}
impl UEditorInstancedPlacementSettings {}
#[repr(C, align(8))]
pub struct ULayersSubsystem {
    __padding_end: [u8; 136],
}
impl ULayersSubsystem {}
#[repr(C, align(8))]
pub struct ULevelEditorDragDropHandler {
    __padding_end: [u8; 104],
}
impl ULevelEditorDragDropHandler {}
#[repr(C, align(8))]
pub struct ULightmassOptionsObject {
    __padding_end: [u8; 72],
}
impl ULightmassOptionsObject {}
#[repr(C, align(16))]
pub struct UMaterialEditorMeshComponent {
    __padding_end: [u8; 1888],
}
impl UMaterialEditorMeshComponent {}
#[repr(C, align(8))]
pub struct UMaterialGraph {
    __padding_end: [u8; 328],
}
impl UMaterialGraph {}
#[repr(C, align(8))]
pub struct UMaterialGraphNode_Base {
    __padding_end: [u8; 192],
}
impl UMaterialGraphNode_Base {}
#[repr(C, align(8))]
pub struct UMaterialGraphNode {
    __padding_end: [u8; 280],
}
impl UMaterialGraphNode {}
#[repr(C, align(8))]
pub struct UMaterialGraphNode_Comment {
    __padding_end: [u8; 328],
}
impl UMaterialGraphNode_Comment {}
#[repr(C, align(8))]
pub struct UMaterialGraphNode_Composite {
    __padding_end: [u8; 288],
}
impl UMaterialGraphNode_Composite {}
#[repr(C, align(8))]
pub struct UMaterialGraphNode_Custom {
    __padding_end: [u8; 280],
}
impl UMaterialGraphNode_Custom {}
#[repr(C, align(8))]
pub struct UMaterialGraphNode_Knot {
    __padding_end: [u8; 280],
}
impl UMaterialGraphNode_Knot {}
#[repr(C, align(8))]
pub struct UMaterialGraphNode_Operator {
    __padding_end: [u8; 280],
}
impl UMaterialGraphNode_Operator {}
#[repr(C, align(8))]
pub struct UMaterialGraphNode_PinBase {
    __padding_end: [u8; 280],
}
impl UMaterialGraphNode_PinBase {}
#[repr(C, align(8))]
pub struct UMaterialGraphNode_Root {
    __padding_end: [u8; 200],
}
impl UMaterialGraphNode_Root {}
#[repr(C, align(8))]
pub struct UMaterialGraphSchema {
    __padding_end: [u8; 48],
}
impl UMaterialGraphSchema {}
#[repr(C, align(8))]
pub struct UUndoableResolveHandler {
    __padding_end: [u8; 128],
}
impl UUndoableResolveHandler {}
#[repr(C, align(8))]
pub struct UPackageTools {
    __padding_end: [u8; 48],
}
impl UPackageTools {}
#[repr(C, align(8))]
pub struct UPreviewMaterial {
    __padding_end: [u8; 3400],
}
impl UPreviewMaterial {}
#[repr(C, align(8))]
pub struct UPropertyEditorTestInstancedObject {
    __padding_end: [u8; 56],
}
impl UPropertyEditorTestInstancedObject {}
#[repr(C, align(8))]
pub struct UFirstDerivedPropertyEditorTestObject {
    __padding_end: [u8; 72],
}
impl UFirstDerivedPropertyEditorTestObject {}
#[repr(C, align(8))]
pub struct USecondDerivedPropertyEditorTestObject {
    __padding_end: [u8; 64],
}
impl USecondDerivedPropertyEditorTestObject {}
#[repr(C, align(16))]
pub struct UPropertyEditorTestObject {
    __padding_end: [u8; 5008],
}
impl UPropertyEditorTestObject {}
#[repr(C, align(8))]
pub struct UHideCategoriesBase {
    __padding_end: [u8; 56],
}
impl UHideCategoriesBase {}
#[repr(C, align(8))]
pub struct UShowCategoriesTest {
    __padding_end: [u8; 64],
}
impl UShowCategoriesTest {}
#[repr(C, align(8))]
pub struct UBlueprintPropertyTestObject {
    __padding_end: [u8; 72],
}
impl UBlueprintPropertyTestObject {}
#[repr(C, align(8))]
pub struct UBlueprintPropertyContainerTestObject {
    __padding_end: [u8; 64],
}
impl UBlueprintPropertyContainerTestObject {}
#[repr(C, align(8))]
pub struct UTestSparseClassDataBase {
    __padding_end: [u8; 48],
}
impl UTestSparseClassDataBase {}
#[repr(C, align(8))]
pub struct UTestSparseClassData {
    __padding_end: [u8; 48],
}
impl UTestSparseClassData {}
#[repr(C, align(8))]
pub struct APropertyEditorTestActor {
    __padding_end: [u8; 1264],
}
impl APropertyEditorTestActor {}
#[repr(C, align(8))]
pub struct UPropertyEditorRowGeneratorTest {
    __padding_end: [u8; 96],
}
impl UPropertyEditorRowGeneratorTest {}
#[repr(C, align(8))]
pub struct UUnrealEdViewportToolbarContext {
    __padding_end: [u8; 176],
}
impl UUnrealEdViewportToolbarContext {}
#[repr(C, align(8))]
pub struct UCommonViewportToolbarBaseMenuContext {
    __padding_end: [u8; 192],
}
impl UCommonViewportToolbarBaseMenuContext {}
#[repr(C, align(8))]
pub struct USelection {
    __padding_end: [u8; 72],
}
impl USelection {}
#[repr(C, align(8))]
pub struct UBlueprintEditorProjectSettings {
    __padding_end: [u8; 216],
}
impl UBlueprintEditorProjectSettings {}
#[repr(C, align(16))]
pub struct UEditorPerProjectUserSettings {
    __padding_end: [u8; 320],
}
impl UEditorPerProjectUserSettings {}
#[repr(C, align(8))]
pub struct UEditorProjectAppearanceSettings {
    __padding_end: [u8; 184],
}
impl UEditorProjectAppearanceSettings {}
#[repr(C, align(8))]
pub struct ULevelEditor2DSettings {
    __padding_end: [u8; 128],
}
impl ULevelEditor2DSettings {}
#[repr(C, align(8))]
pub struct ULevelEditorProjectSettings {
    __padding_end: [u8; 112],
}
impl ULevelEditorProjectSettings {}
#[repr(C, align(8))]
pub struct UEditorPerformanceProjectSettings {
    __padding_end: [u8; 136],
}
impl UEditorPerformanceProjectSettings {}
#[repr(C, align(8))]
pub struct UEditorProjectAssetSettings {
    __padding_end: [u8; 112],
}
impl UEditorProjectAssetSettings {}
#[repr(C, align(8))]
pub struct UDDCProjectSettings {
    __padding_end: [u8; 112],
}
impl UDDCProjectSettings {}
#[repr(C, align(8))]
pub struct UEditorSettings {
    __padding_end: [u8; 360],
}
impl UEditorSettings {}
#[repr(C, align(8))]
pub struct UActorEditorContextSubsystem {
    __padding_end: [u8; 128],
}
impl UActorEditorContextSubsystem {}
#[repr(C, align(8))]
pub struct UAssetEditorSubsystem {
    __padding_end: [u8; 1328],
}
impl UAssetEditorSubsystem {}
#[repr(C, align(8))]
pub struct UBrowseToAssetOverrideSubsystem {
    __padding_end: [u8; 216],
}
impl UBrowseToAssetOverrideSubsystem {}
#[repr(C, align(8))]
pub struct UBrushEditingSubsystem {
    __padding_end: [u8; 56],
}
impl UBrushEditingSubsystem {}
#[repr(C, align(8))]
pub struct UCollectionManagerScriptingSubsystem {
    __padding_end: [u8; 56],
}
impl UCollectionManagerScriptingSubsystem {}
#[repr(C, align(8))]
pub struct UEditorActorSubsystem {
    __padding_end: [u8; 368],
}
impl UEditorActorSubsystem {}
#[repr(C, align(8))]
pub struct UEditorAssetSubsystem {
    __padding_end: [u8; 104],
}
impl UEditorAssetSubsystem {}
#[repr(C, align(8))]
pub struct UEditorSubsystemBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UEditorSubsystemBlueprintLibrary {}
#[repr(C, align(16))]
pub struct UImportSubsystem {
    __padding_end: [u8; 304],
}
impl UImportSubsystem {}
#[repr(C, align(8))]
pub struct UPanelExtensionSubsystem {
    __padding_end: [u8; 296],
}
impl UPanelExtensionSubsystem {}
#[repr(C, align(8))]
pub struct UPropertyVisibilityOverrideSubsystem {
    __padding_end: [u8; 136],
}
impl UPropertyVisibilityOverrideSubsystem {}
#[repr(C, align(8))]
pub struct UUnrealEditorSubsystem {
    __padding_end: [u8; 56],
}
impl UUnrealEditorSubsystem {}
#[repr(C, align(8))]
pub struct UDEPRECATED_TemplateMapMetadata {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_TemplateMapMetadata {}
#[repr(C, align(8))]
pub struct UFbxTestPlan {
    __padding_end: [u8; 104],
}
impl UFbxTestPlan {}
#[repr(C, align(8))]
pub struct AAnimationThumbnailSkeletalMeshActor {
    __padding_end: [u8; 1264],
}
impl AAnimationThumbnailSkeletalMeshActor {}
#[repr(C, align(8))]
pub struct UThumbnailManager {
    __padding_end: [u8; 336],
}
impl UThumbnailManager {}
#[repr(C, align(8))]
pub struct UAnimBlueprintThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UAnimBlueprintThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UAnimSequenceThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UAnimSequenceThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UBlendSpaceThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UBlendSpaceThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UClassThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UClassThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UCurveFloatThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UCurveFloatThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UCurveVector3ThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UCurveVector3ThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UCurveLinearColorThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UCurveLinearColorThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UFontThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UFontThumbnailRenderer {}
#[repr(C, align(8))]
pub struct ULevelThumbnailRenderer {
    __padding_end: [u8; 56],
}
impl ULevelThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UMaterialFunctionThumbnailRenderer {
    __padding_end: [u8; 80],
}
impl UMaterialFunctionThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UMaterialInstanceThumbnailRenderer {
    __padding_end: [u8; 80],
}
impl UMaterialInstanceThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UNeuralProfileRenderer {
    __padding_end: [u8; 48],
}
impl UNeuralProfileRenderer {}
#[repr(C, align(8))]
pub struct UParticleSystemThumbnailRenderer {
    __padding_end: [u8; 72],
}
impl UParticleSystemThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UPhysicalMaterialMaskThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UPhysicalMaterialMaskThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UPhysicsAssetThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UPhysicsAssetThumbnailRenderer {}
#[repr(C, align(8))]
pub struct USceneThumbnailInfo {
    __padding_end: [u8; 64],
}
impl USceneThumbnailInfo {}
#[repr(C, align(8))]
pub struct USceneThumbnailInfoWithPrimitive {
    __padding_end: [u8; 120],
}
impl USceneThumbnailInfoWithPrimitive {}
#[repr(C, align(8))]
pub struct USkeletalMeshThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl USkeletalMeshThumbnailRenderer {}
#[repr(C, align(8))]
pub struct USkeletonThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl USkeletonThumbnailRenderer {}
#[repr(C, align(8))]
pub struct USlateBrushThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl USlateBrushThumbnailRenderer {}
#[repr(C, align(8))]
pub struct USoundWaveThumbnailRenderer {
    __padding_end: [u8; 56],
}
impl USoundWaveThumbnailRenderer {}
#[repr(C, align(8))]
pub struct USpecularProfileRenderer {
    __padding_end: [u8; 48],
}
impl USpecularProfileRenderer {}
#[repr(C, align(8))]
pub struct UStaticMeshThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UStaticMeshThumbnailRenderer {}
#[repr(C, align(8))]
pub struct USubsurfaceProfileRenderer {
    __padding_end: [u8; 48],
}
impl USubsurfaceProfileRenderer {}
#[repr(C, align(8))]
pub struct UTextureCubeArrayThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UTextureCubeArrayThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UTextureCubeThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UTextureCubeThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UVolumeTextureThumbnailRenderer {
    __padding_end: [u8; 72],
}
impl UVolumeTextureThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UWorldThumbnailInfo {
    __padding_end: [u8; 72],
}
impl UWorldThumbnailInfo {}
#[repr(C, align(8))]
pub struct UWorldThumbnailRenderer {
    __padding_end: [u8; 72],
}
impl UWorldThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UAssetEditorContextObject {
    __padding_end: [u8; 64],
}
impl UAssetEditorContextObject {}
#[repr(C, align(8))]
pub struct UEdMode {
    __padding_end: [u8; 312],
}
impl UEdMode {}
#[repr(C, align(8))]
pub struct UBaseLegacyWidgetEdMode {
    __padding_end: [u8; 344],
}
impl UBaseLegacyWidgetEdMode {}
#[repr(C, align(8))]
pub struct UEdModeDefault {
    __padding_end: [u8; 344],
}
impl UEdModeDefault {}
#[repr(C, align(8))]
pub struct UAssetEdModeDefault {
    __padding_end: [u8; 344],
}
impl UAssetEdModeDefault {}
#[repr(C, align(16))]
pub struct UEdModeInteractiveToolsContext {
    __padding_end: [u8; 960],
}
impl UEdModeInteractiveToolsContext {}
#[repr(C, align(8))]
pub struct ULegacyEdModeWrapper {
    __padding_end: [u8; 376],
}
impl ULegacyEdModeWrapper {}
#[repr(C, align(8))]
pub struct UAssetEditor {
    __padding_end: [u8; 64],
}
impl UAssetEditor {}
#[repr(C, align(16))]
pub struct UUnrealEdEngine {
    __padding_end: [u8; 10736],
}
impl UUnrealEdEngine {}
#[repr(C, align(8))]
pub struct UUnrealEdTypes {
    __padding_end: [u8; 48],
}
impl UUnrealEdTypes {}
#[repr(C, align(8))]
pub struct UUserDefinedStructEditorData {
    __padding_end: [u8; 104],
}
impl UUserDefinedStructEditorData {}
#[repr(C, align(8))]
pub struct UWorldFolders {
    __padding_end: [u8; 296],
}
impl UWorldFolders {}
#[repr(C, align(8))]
pub struct UWorldPartitionFoliageBuilder {
    __padding_end: [u8; 472],
}
impl UWorldPartitionFoliageBuilder {}
#[repr(C, align(8))]
pub struct UWorldPartitionHLODsBuilder {
    __padding_end: [u8; 888],
}
impl UWorldPartitionHLODsBuilder {}
#[repr(C, align(8))]
pub struct UWorldPartitionLandscapeSplineMeshesBuilder {
    __padding_end: [u8; 472],
}
impl UWorldPartitionLandscapeSplineMeshesBuilder {}
#[repr(C, align(16))]
pub struct UWorldPartitionMiniMapBuilder {
    __padding_end: [u8; 624],
}
impl UWorldPartitionMiniMapBuilder {}
#[repr(C, align(8))]
pub struct UWorldPartitionNavigationDataBuilder {
    __padding_end: [u8; 584],
}
impl UWorldPartitionNavigationDataBuilder {}
#[repr(C, align(8))]
pub struct UWorldPartitionRenameDuplicateBuilder {
    __padding_end: [u8; 600],
}
impl UWorldPartitionRenameDuplicateBuilder {}
#[repr(C, align(8))]
pub struct UWorldPartitionResaveActorsBuilder {
    __padding_end: [u8; 664],
}
impl UWorldPartitionResaveActorsBuilder {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeVirtualTextureBuilder {
    __padding_end: [u8; 464],
}
impl UWorldPartitionRuntimeVirtualTextureBuilder {}
#[repr(C, align(8))]
pub struct UWorldPartitionStaticLightingBuilder {
    __padding_end: [u8; 944],
}
impl UWorldPartitionStaticLightingBuilder {}
#[repr(C, align(8))]
pub struct UCookOnTheFlyServer {
    __padding_end: [u8; 1976],
}
impl UCookOnTheFlyServer {}
#[repr(C, align(8))]
pub struct FAddOnExtractAssetFromFile_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRemoveOnExtractAssetFromFile_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnNewActorsDropped {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnNewActorsPlaced {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditCutActorsBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditCutActorsEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditCopyActorsBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditCopyActorsEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditPasteActorsBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditPasteActorsEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnDuplicateActorsBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnDuplicateActorsEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnDeleteActorsBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnDeleteActorsEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnActorLabelChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FImportSubsystem_OnAssetPreImport_BP {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FImportSubsystem_OnAssetPostImport_BP {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FImportSubsystem_OnAssetReimport_BP {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FImportSubsystem_OnAssetPostLODImport_BP {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ELevelViewportType(pub u8);
impl ELevelViewportType {
    pub const LVT_ORTHO_XY: ELevelViewportType = ELevelViewportType(0);
    pub const LVT_ORTHO_XZ: ELevelViewportType = ELevelViewportType(1);
    pub const LVT_ORTHO_YZ: ELevelViewportType = ELevelViewportType(2);
    pub const LVT_PERSPECTIVE: ELevelViewportType = ELevelViewportType(3);
    pub const LVT_ORTHO_FREELOOK: ELevelViewportType = ELevelViewportType(4);
    pub const LVT_ORTHO_NEGATIVE_XY: ELevelViewportType = ELevelViewportType(5);
    pub const LVT_ORTHO_NEGATIVE_XZ: ELevelViewportType = ELevelViewportType(6);
    pub const LVT_ORTHO_NEGATIVE_YZ: ELevelViewportType = ELevelViewportType(7);
    pub const LVT_ORTHO_TOP: ELevelViewportType = ELevelViewportType(0);
    pub const LVT_ORTHO_LEFT: ELevelViewportType = ELevelViewportType(1);
    pub const LVT_ORTHO_FRONT: ELevelViewportType = ELevelViewportType(7);
    pub const LVT_ORTHO_BACK: ELevelViewportType = ELevelViewportType(2);
    pub const LVT_ORTHO_BOTTOM: ELevelViewportType = ELevelViewportType(5);
    pub const LVT_ORTHO_RIGHT: ELevelViewportType = ELevelViewportType(6);
    pub const LVT_NONE: ELevelViewportType = ELevelViewportType(255);
}
#[repr(transparent)]
pub struct EAnimationViewportCameraFollowMode(pub u8);
impl EAnimationViewportCameraFollowMode {
    pub const NONE: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        0,
    );
    pub const BOUNDS: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        1,
    );
    pub const BONE: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        2,
    );
    pub const ROOT: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        3,
    );
}
#[repr(transparent)]
pub struct ECSVImportType(pub u8);
impl ECSVImportType {
    pub const ECSV_DATA_TABLE: ECSVImportType = ECSVImportType(0);
    pub const ECSV_CURVE_TABLE: ECSVImportType = ECSVImportType(1);
    pub const ECSV_CURVE_FLOAT: ECSVImportType = ECSVImportType(2);
    pub const ECSV_CURVE_VECTOR: ECSVImportType = ECSVImportType(3);
    pub const ECSV_CURVE_LINEAR_COLOR: ECSVImportType = ECSVImportType(4);
}
#[repr(transparent)]
pub struct ETestEnumFlags(pub u8);
impl ETestEnumFlags {
    pub const NONE: ETestEnumFlags = ETestEnumFlags(0);
    pub const ONE: ETestEnumFlags = ETestEnumFlags(1);
    pub const TWO: ETestEnumFlags = ETestEnumFlags(2);
    pub const FOUR: ETestEnumFlags = ETestEnumFlags(4);
}
#[repr(transparent)]
pub struct NetworkEmulationTarget(pub i32);
impl NetworkEmulationTarget {
    pub const SERVER: NetworkEmulationTarget = NetworkEmulationTarget(0);
    pub const CLIENT: NetworkEmulationTarget = NetworkEmulationTarget(1);
    pub const ANY: NetworkEmulationTarget = NetworkEmulationTarget(2);
}
#[repr(transparent)]
pub struct EFBXExpectedResultPreset(pub u8);
impl EFBXExpectedResultPreset {
    pub const ERROR_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(0);
    pub const WARNING_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(1);
    pub const CREATED_STATICMESH_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        2,
    );
    pub const CREATED_SKELETALMESH_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        3,
    );
    pub const MATERIALS_CREATED_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        4,
    );
    pub const MATERIAL_SLOT_IMPORTED_NAME: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        5,
    );
    pub const VERTEX_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(6);
    pub const LOD_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(7);
    pub const VERTEX_NUMBER_LOD: EFBXExpectedResultPreset = EFBXExpectedResultPreset(8);
    pub const MESH_MATERIALS_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        9,
    );
    pub const MESH_LOD_SECTION_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        10,
    );
    pub const MESH_LOD_SECTION_VERTEX_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        11,
    );
    pub const MESH_LOD_SECTION_TRIANGLE_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        12,
    );
    pub const MESH_LOD_SECTION_MATERIAL_NAME: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        13,
    );
    pub const MESH_LOD_SECTION_MATERIAL_INDEX: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        14,
    );
    pub const MESH_LOD_SECTION_MATERIAL_IMPORTED_NAME: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        15,
    );
    pub const MESH_LOD_VERTEX_POSITION: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        16,
    );
    pub const MESH_LOD_VERTEX_NORMAL: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        17,
    );
    pub const LOD_UV_CHANNEL_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        18,
    );
    pub const BONE_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(19);
    pub const BONE_POSITION: EFBXExpectedResultPreset = EFBXExpectedResultPreset(20);
    pub const ANIMATION_FRAME_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        21,
    );
    pub const ANIMATION_LENGTH: EFBXExpectedResultPreset = EFBXExpectedResultPreset(22);
    pub const ANIMATION_CUSTOM_CURVE_KEY_VALUE: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        23,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_ARRIVE_TANGENT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        24,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_LEAVE_TANGENT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        25,
    );
    pub const SKIN_BY_BONE_VERTEX_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        26,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_ARRIVE_TANGENT_WEIGHT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        27,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_LEAVE_TANGENT_WEIGHT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        28,
    );
}
#[repr(transparent)]
pub struct EPhysicsAssetEditorCenterOfMassViewMode(pub u8);
impl EPhysicsAssetEditorCenterOfMassViewMode {
    pub const ALL: EPhysicsAssetEditorCenterOfMassViewMode = EPhysicsAssetEditorCenterOfMassViewMode(
        0,
    );
    pub const SELECTED: EPhysicsAssetEditorCenterOfMassViewMode = EPhysicsAssetEditorCenterOfMassViewMode(
        1,
    );
    pub const NONE: EPhysicsAssetEditorCenterOfMassViewMode = EPhysicsAssetEditorCenterOfMassViewMode(
        2,
    );
}
#[repr(transparent)]
pub struct EPhysicsAssetEditorCollisionViewMode(pub u8);
impl EPhysicsAssetEditorCollisionViewMode {
    pub const SOLID: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        0,
    );
    pub const WIREFRAME: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        1,
    );
    pub const SOLID_WIREFRAME: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        2,
    );
    pub const NONE: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        3,
    );
}
#[repr(transparent)]
pub struct EPhysicsAssetEditorConstraintViewMode(pub u8);
impl EPhysicsAssetEditorConstraintViewMode {
    pub const NONE: EPhysicsAssetEditorConstraintViewMode = EPhysicsAssetEditorConstraintViewMode(
        0,
    );
    pub const ALL_POSITIONS: EPhysicsAssetEditorConstraintViewMode = EPhysicsAssetEditorConstraintViewMode(
        1,
    );
    pub const ALL_LIMITS: EPhysicsAssetEditorConstraintViewMode = EPhysicsAssetEditorConstraintViewMode(
        2,
    );
}
#[repr(transparent)]
pub struct EEditorAssetMetaDataSortType(pub u8);
impl EEditorAssetMetaDataSortType {
    pub const STRING: EEditorAssetMetaDataSortType = EEditorAssetMetaDataSortType(0);
    pub const NUMERIC: EEditorAssetMetaDataSortType = EEditorAssetMetaDataSortType(1);
    pub const DATE_TIME: EEditorAssetMetaDataSortType = EEditorAssetMetaDataSortType(2);
}
#[repr(transparent)]
pub struct EEditorAssetSortOrder(pub u8);
impl EEditorAssetSortOrder {
    pub const ASCENDING: EEditorAssetSortOrder = EEditorAssetSortOrder(0);
    pub const DESCENDING: EEditorAssetSortOrder = EEditorAssetSortOrder(1);
}
#[repr(transparent)]
pub struct ELevelVisibilityDirtyMode(pub u8);
impl ELevelVisibilityDirtyMode {
    pub const MODIFY_ON_CHANGE: ELevelVisibilityDirtyMode = ELevelVisibilityDirtyMode(0);
    pub const DONT_MODIFY: ELevelVisibilityDirtyMode = ELevelVisibilityDirtyMode(1);
}
#[repr(transparent)]
pub struct EMaterialSearchLocation(pub u8);
impl EMaterialSearchLocation {
    pub const LOCAL: EMaterialSearchLocation = EMaterialSearchLocation(0);
    pub const UNDER_PARENT: EMaterialSearchLocation = EMaterialSearchLocation(1);
    pub const UNDER_ROOT: EMaterialSearchLocation = EMaterialSearchLocation(2);
    pub const ALL_ASSETS: EMaterialSearchLocation = EMaterialSearchLocation(3);
    pub const DO_NOT_SEARCH: EMaterialSearchLocation = EMaterialSearchLocation(4);
}
#[repr(transparent)]
pub struct EReloadPackagesInteractionMode(pub u8);
impl EReloadPackagesInteractionMode {
    pub const INTERACTIVE: EReloadPackagesInteractionMode = EReloadPackagesInteractionMode(
        0,
    );
    pub const ASSUME_POSITIVE: EReloadPackagesInteractionMode = EReloadPackagesInteractionMode(
        1,
    );
    pub const ASSUME_NEGATIVE: EReloadPackagesInteractionMode = EReloadPackagesInteractionMode(
        2,
    );
}
#[repr(transparent)]
pub struct EFbxExportCompatibility(pub u8);
impl EFbxExportCompatibility {
    pub const FBX_2011: EFbxExportCompatibility = EFbxExportCompatibility(0);
    pub const FBX_2012: EFbxExportCompatibility = EFbxExportCompatibility(1);
    pub const FBX_2013: EFbxExportCompatibility = EFbxExportCompatibility(2);
    pub const FBX_2014: EFbxExportCompatibility = EFbxExportCompatibility(3);
    pub const FBX_2016: EFbxExportCompatibility = EFbxExportCompatibility(4);
    pub const FBX_2018: EFbxExportCompatibility = EFbxExportCompatibility(5);
    pub const FBX_2019: EFbxExportCompatibility = EFbxExportCompatibility(6);
    pub const FBX_2020: EFbxExportCompatibility = EFbxExportCompatibility(7);
}
#[repr(transparent)]
pub struct EBlueprintBreakpointReloadMethod(pub i32);
impl EBlueprintBreakpointReloadMethod {
    pub const RESTORE_ALL: EBlueprintBreakpointReloadMethod = EBlueprintBreakpointReloadMethod(
        0,
    );
    pub const RESTORE_ALL_AND_DISABLE: EBlueprintBreakpointReloadMethod = EBlueprintBreakpointReloadMethod(
        1,
    );
    pub const DISCARD_ALL: EBlueprintBreakpointReloadMethod = EBlueprintBreakpointReloadMethod(
        2,
    );
}
#[repr(transparent)]
pub struct ECommentBoxMode(pub u8);
impl ECommentBoxMode {
    pub const GROUP_MOVEMENT: ECommentBoxMode = ECommentBoxMode(0);
    pub const NO_GROUP_MOVEMENT: ECommentBoxMode = ECommentBoxMode(1);
}
#[repr(transparent)]
pub struct EClassViewerDeveloperType(pub u8);
impl EClassViewerDeveloperType {
    pub const CVDT_NONE: EClassViewerDeveloperType = EClassViewerDeveloperType(0);
    pub const CVDT_CURRENT_USER: EClassViewerDeveloperType = EClassViewerDeveloperType(
        1,
    );
    pub const CVDT_ALL: EClassViewerDeveloperType = EClassViewerDeveloperType(2);
}
#[repr(transparent)]
pub struct ELoadLevelAtStartup(pub u8);
impl ELoadLevelAtStartup {
    pub const NONE: ELoadLevelAtStartup = ELoadLevelAtStartup(0);
    pub const PROJECT_DEFAULT: ELoadLevelAtStartup = ELoadLevelAtStartup(1);
    pub const LAST_OPENED: ELoadLevelAtStartup = ELoadLevelAtStartup(2);
}
#[repr(transparent)]
pub struct ERestoreOpenAssetTabsMethod(pub u8);
impl ERestoreOpenAssetTabsMethod {
    pub const ALWAYS_PROMPT: ERestoreOpenAssetTabsMethod = ERestoreOpenAssetTabsMethod(
        0,
    );
    pub const ALWAYS_RESTORE: ERestoreOpenAssetTabsMethod = ERestoreOpenAssetTabsMethod(
        1,
    );
    pub const NEVER_RESTORE: ERestoreOpenAssetTabsMethod = ERestoreOpenAssetTabsMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EAutoSaveMethod(pub u8);
impl EAutoSaveMethod {
    pub const BACKUP_AND_RESTORE: EAutoSaveMethod = EAutoSaveMethod(0);
    pub const BACKUP_AND_OVERWRITE: EAutoSaveMethod = EAutoSaveMethod(1);
}
#[repr(transparent)]
pub struct EAssetEditorOpenLocation(pub u8);
impl EAssetEditorOpenLocation {
    pub const DEFAULT: EAssetEditorOpenLocation = EAssetEditorOpenLocation(0);
    pub const NEW_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(1);
    pub const MAIN_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(2);
    pub const CONTENT_BROWSER: EAssetEditorOpenLocation = EAssetEditorOpenLocation(3);
    pub const LAST_DOCKED_WINDOW_OR_NEW_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(
        4,
    );
    pub const LAST_DOCKED_WINDOW_OR_MAIN_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(
        5,
    );
    pub const LAST_DOCKED_WINDOW_OR_CONTENT_BROWSER: EAssetEditorOpenLocation = EAssetEditorOpenLocation(
        6,
    );
}
#[repr(transparent)]
pub struct EFBXImportType(pub u8);
impl EFBXImportType {
    pub const FBXIT_STATIC_MESH: EFBXImportType = EFBXImportType(0);
    pub const FBXIT_SKELETAL_MESH: EFBXImportType = EFBXImportType(1);
    pub const FBXIT_ANIMATION: EFBXImportType = EFBXImportType(2);
}
#[repr(transparent)]
pub struct ELabelAnchorMode(pub u8);
impl ELabelAnchorMode {
    pub const LABEL_ANCHOR_MODE_TOP_LEFT: ELabelAnchorMode = ELabelAnchorMode(0);
    pub const LABEL_ANCHOR_MODE_TOP_CENTER: ELabelAnchorMode = ELabelAnchorMode(1);
    pub const LABEL_ANCHOR_MODE_TOP_RIGHT: ELabelAnchorMode = ELabelAnchorMode(2);
    pub const LABEL_ANCHOR_MODE_CENTER_LEFT: ELabelAnchorMode = ELabelAnchorMode(3);
    pub const LABEL_ANCHOR_MODE_CENTERED: ELabelAnchorMode = ELabelAnchorMode(4);
    pub const LABEL_ANCHOR_MODE_CENTER_RIGHT: ELabelAnchorMode = ELabelAnchorMode(5);
    pub const LABEL_ANCHOR_MODE_BOTTOM_LEFT: ELabelAnchorMode = ELabelAnchorMode(6);
    pub const LABEL_ANCHOR_MODE_BOTTOM_CENTER: ELabelAnchorMode = ELabelAnchorMode(7);
    pub const LABEL_ANCHOR_MODE_BOTTOM_RIGHT: ELabelAnchorMode = ELabelAnchorMode(8);
}
#[repr(transparent)]
pub struct EPlayOnBuildMode(pub u8);
impl EPlayOnBuildMode {
    pub const PLAY_ON_BUILD_ALWAYS: EPlayOnBuildMode = EPlayOnBuildMode(0);
    pub const PLAY_ON_BUILD_NEVER: EPlayOnBuildMode = EPlayOnBuildMode(1);
    pub const PLAY_ON_BUILD_DEFAULT: EPlayOnBuildMode = EPlayOnBuildMode(2);
    pub const PLAY_ON_BUILD_IF_EDITOR_BUILT_LOCALLY: EPlayOnBuildMode = EPlayOnBuildMode(
        3,
    );
}
#[repr(transparent)]
pub struct EPlayOnLaunchConfiguration(pub u8);
impl EPlayOnLaunchConfiguration {
    pub const LAUNCH_CONFIG_DEFAULT: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        0,
    );
    pub const LAUNCH_CONFIG_DEBUG: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        1,
    );
    pub const LAUNCH_CONFIG_DEVELOPMENT: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        2,
    );
    pub const LAUNCH_CONFIG_TEST: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        3,
    );
    pub const LAUNCH_CONFIG_SHIPPING: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        4,
    );
}
#[repr(transparent)]
pub struct EPlayOnPakFileMode(pub u8);
impl EPlayOnPakFileMode {
    pub const NO_PAK: EPlayOnPakFileMode = EPlayOnPakFileMode(0);
    pub const PAK_NO_COMPRESS: EPlayOnPakFileMode = EPlayOnPakFileMode(1);
    pub const PAK_COMPRESS: EPlayOnPakFileMode = EPlayOnPakFileMode(2);
}
#[repr(transparent)]
pub struct EPlayNetMode(pub u8);
impl EPlayNetMode {
    pub const PIE_STANDALONE: EPlayNetMode = EPlayNetMode(0);
    pub const PIE_LISTEN_SERVER: EPlayNetMode = EPlayNetMode(1);
    pub const PIE_CLIENT: EPlayNetMode = EPlayNetMode(2);
}
#[repr(transparent)]
pub struct ELaunchModeType(pub u8);
impl ELaunchModeType {
    pub const LAUNCH_MODE_ON_DEVICE: ELaunchModeType = ELaunchModeType(0);
}
#[repr(transparent)]
pub struct EPlayModeLocations(pub u8);
impl EPlayModeLocations {
    pub const PLAY_LOCATION_CURRENT_CAMERA_LOCATION: EPlayModeLocations = EPlayModeLocations(
        0,
    );
    pub const PLAY_LOCATION_DEFAULT_PLAYER_START: EPlayModeLocations = EPlayModeLocations(
        1,
    );
}
#[repr(transparent)]
pub struct EPlayModeType(pub u8);
impl EPlayModeType {
    pub const PLAY_MODE_IN_VIEW_PORT: EPlayModeType = EPlayModeType(0);
    pub const PLAY_MODE_IN_EDITOR_FLOATING: EPlayModeType = EPlayModeType(1);
    pub const PLAY_MODE_IN_MOBILE_PREVIEW: EPlayModeType = EPlayModeType(2);
    pub const PLAY_MODE_IN_TARGETED_MOBILE_PREVIEW: EPlayModeType = EPlayModeType(3);
    pub const PLAY_MODE_IN_VULKAN_PREVIEW: EPlayModeType = EPlayModeType(4);
    pub const PLAY_MODE_IN_NEW_PROCESS: EPlayModeType = EPlayModeType(5);
    pub const PLAY_MODE_IN_VR: EPlayModeType = EPlayModeType(6);
    pub const PLAY_MODE_SIMULATE: EPlayModeType = EPlayModeType(7);
    pub const PLAY_MODE_QUICK_LAUNCH: EPlayModeType = EPlayModeType(8);
    pub const PLAY_MODE_COUNT: EPlayModeType = EPlayModeType(9);
}
#[repr(transparent)]
pub struct EWASDType(pub u8);
impl EWASDType {
    pub const WASD_ALWAYS: EWASDType = EWASDType(0);
    pub const WASD_RMB_ONLY: EWASDType = EWASDType(1);
    pub const WASD_NEVER: EWASDType = EWASDType(2);
}
#[repr(transparent)]
pub struct ELandscapeFoliageEditorControlType(pub u8);
impl ELandscapeFoliageEditorControlType {
    pub const IGNORE_CTRL: ELandscapeFoliageEditorControlType = ELandscapeFoliageEditorControlType(
        0,
    );
    pub const REQUIRE_CTRL: ELandscapeFoliageEditorControlType = ELandscapeFoliageEditorControlType(
        1,
    );
    pub const REQUIRE_NO_CTRL: ELandscapeFoliageEditorControlType = ELandscapeFoliageEditorControlType(
        2,
    );
}
#[repr(transparent)]
pub struct EScrollGestureDirection(pub u8);
impl EScrollGestureDirection {
    pub const USE_SYSTEM_SETTING: EScrollGestureDirection = EScrollGestureDirection(0);
    pub const STANDARD: EScrollGestureDirection = EScrollGestureDirection(1);
    pub const NATURAL: EScrollGestureDirection = EScrollGestureDirection(2);
}
#[repr(transparent)]
pub struct ERotationGridMode(pub u8);
impl ERotationGridMode {
    pub const GRID_MODE_DIVISIONS_OF360: ERotationGridMode = ERotationGridMode(0);
    pub const GRID_MODE_COMMON: ERotationGridMode = ERotationGridMode(1);
}
#[repr(transparent)]
pub struct EMarqueeSelectionMode(pub u8);
impl EMarqueeSelectionMode {
    pub const CROSSING: EMarqueeSelectionMode = EMarqueeSelectionMode(0);
    pub const WINDOW: EMarqueeSelectionMode = EMarqueeSelectionMode(1);
    pub const CROSS_LEFT: EMarqueeSelectionMode = EMarqueeSelectionMode(2);
    pub const CROSS_RIGHT: EMarqueeSelectionMode = EMarqueeSelectionMode(3);
}
#[repr(transparent)]
pub struct EMeasuringToolUnits(pub u8);
impl EMeasuringToolUnits {
    pub const MEASURE_UNITS_CENTIMETERS: EMeasuringToolUnits = EMeasuringToolUnits(0);
    pub const MEASURE_UNITS_METERS: EMeasuringToolUnits = EMeasuringToolUnits(1);
    pub const MEASURE_UNITS_KILOMETERS: EMeasuringToolUnits = EMeasuringToolUnits(2);
}
#[repr(transparent)]
pub struct EMaterialKind(pub u8);
impl EMaterialKind {
    pub const UNKNOWN: EMaterialKind = EMaterialKind(0);
    pub const BASE: EMaterialKind = EMaterialKind(1);
    pub const NORMAL: EMaterialKind = EMaterialKind(2);
    pub const SPECULAR: EMaterialKind = EMaterialKind(3);
    pub const EMISSIVE: EMaterialKind = EMaterialKind(4);
}
#[repr(transparent)]
pub struct EMaterialStatsDerivedMIOption(pub u8);
impl EMaterialStatsDerivedMIOption {
    pub const IGNORE: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(0);
    pub const COMPILE_ONLY: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(
        1,
    );
    pub const SHOW_STATS: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(
        2,
    );
    pub const INVALID_OR_MAX: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(
        3,
    );
}
#[repr(transparent)]
pub struct EPhysicsAssetEditorMeshViewMode(pub u8);
impl EPhysicsAssetEditorMeshViewMode {
    pub const SOLID: EPhysicsAssetEditorMeshViewMode = EPhysicsAssetEditorMeshViewMode(
        0,
    );
    pub const WIREFRAME: EPhysicsAssetEditorMeshViewMode = EPhysicsAssetEditorMeshViewMode(
        1,
    );
    pub const NONE: EPhysicsAssetEditorMeshViewMode = EPhysicsAssetEditorMeshViewMode(2);
}
#[repr(transparent)]
pub struct ETextureSourceColorSpace(pub i32);
impl ETextureSourceColorSpace {
    pub const AUTO: ETextureSourceColorSpace = ETextureSourceColorSpace(0);
    pub const LINEAR: ETextureSourceColorSpace = ETextureSourceColorSpace(1);
    pub const SRGB: ETextureSourceColorSpace = ETextureSourceColorSpace(2);
}
#[repr(transparent)]
pub struct ESheetAxis(pub u8);
impl ESheetAxis {
    pub const AX_HORIZONTAL: ESheetAxis = ESheetAxis(0);
    pub const AX_X_AXIS: ESheetAxis = ESheetAxis(1);
    pub const AX_Y_AXIS: ESheetAxis = ESheetAxis(2);
}
#[repr(transparent)]
pub struct EStructViewerDeveloperType(pub u8);
impl EStructViewerDeveloperType {
    pub const SVDT_NONE: EStructViewerDeveloperType = EStructViewerDeveloperType(0);
    pub const SVDT_CURRENT_USER: EStructViewerDeveloperType = EStructViewerDeveloperType(
        1,
    );
    pub const SVDT_ALL: EStructViewerDeveloperType = EStructViewerDeveloperType(2);
}
#[repr(transparent)]
pub struct ETexAlign(pub u8);
impl ETexAlign {
    pub const TEXALIGN_NONE: ETexAlign = ETexAlign(0);
    pub const TEXALIGN_DEFAULT: ETexAlign = ETexAlign(1);
    pub const TEXALIGN_BOX: ETexAlign = ETexAlign(2);
    pub const TEXALIGN_PLANAR: ETexAlign = ETexAlign(3);
    pub const TEXALIGN_FIT: ETexAlign = ETexAlign(4);
    pub const TEXALIGN_PLANAR_AUTO: ETexAlign = ETexAlign(5);
    pub const TEXALIGN_PLANAR_WALL: ETexAlign = ETexAlign(6);
    pub const TEXALIGN_PLANAR_FLOOR: ETexAlign = ETexAlign(7);
}
#[repr(transparent)]
pub struct ESkeletonDrawMode(pub u8);
impl ESkeletonDrawMode {
    pub const DEFAULT: ESkeletonDrawMode = ESkeletonDrawMode(0);
    pub const HIDDEN: ESkeletonDrawMode = ESkeletonDrawMode(1);
    pub const GREYED_OUT: ESkeletonDrawMode = ESkeletonDrawMode(2);
}
#[repr(transparent)]
pub struct EProcessRootMotionMode(pub u8);
impl EProcessRootMotionMode {
    pub const IGNORE: EProcessRootMotionMode = EProcessRootMotionMode(0);
    pub const LOOP: EProcessRootMotionMode = EProcessRootMotionMode(1);
    pub const LOOP_AND_RESET: EProcessRootMotionMode = EProcessRootMotionMode(2);
}
#[repr(transparent)]
pub struct EVisualizeRootMotionMode(pub u8);
impl EVisualizeRootMotionMode {
    pub const NONE: EVisualizeRootMotionMode = EVisualizeRootMotionMode(0);
    pub const TRAJECTORY: EVisualizeRootMotionMode = EVisualizeRootMotionMode(1);
    pub const TRAJECTORY_AND_ORIENTATION: EVisualizeRootMotionMode = EVisualizeRootMotionMode(
        2,
    );
}
#[repr(transparent)]
pub struct EEditorUserScreenPercentageModeOverride(pub i32);
impl EEditorUserScreenPercentageModeOverride {
    pub const PROJECT_DEFAULT: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        0,
    );
    pub const MANUAL: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        1,
    );
    pub const BASED_ON_DISPLAY_RESOLUTION: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        2,
    );
    pub const BASED_ON_DPI_SCALE: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        3,
    );
}
#[repr(transparent)]
pub struct ECoordinateSystemPolicy(pub u8);
impl ECoordinateSystemPolicy {
    pub const MATCH_UP_FORWARD_AXES: ECoordinateSystemPolicy = ECoordinateSystemPolicy(
        0,
    );
    pub const MATCH_UP_AXIS: ECoordinateSystemPolicy = ECoordinateSystemPolicy(1);
    pub const KEEP_XYZ_AXES: ECoordinateSystemPolicy = ECoordinateSystemPolicy(2);
}
#[repr(transparent)]
pub struct EFBXAnimationLengthImportType(pub u8);
impl EFBXAnimationLengthImportType {
    pub const FBXALIT_EXPORTED_TIME: EFBXAnimationLengthImportType = EFBXAnimationLengthImportType(
        0,
    );
    pub const FBXALIT_ANIMATED_KEY: EFBXAnimationLengthImportType = EFBXAnimationLengthImportType(
        1,
    );
    pub const FBXALIT_SET_RANGE: EFBXAnimationLengthImportType = EFBXAnimationLengthImportType(
        2,
    );
}
#[repr(transparent)]
pub struct EMovieSceneBakeType(pub u8);
impl EMovieSceneBakeType {
    pub const NONE: EMovieSceneBakeType = EMovieSceneBakeType(0);
    pub const BAKE_CHANNELS: EMovieSceneBakeType = EMovieSceneBakeType(1);
    pub const BAKE_TRANSFORMS: EMovieSceneBakeType = EMovieSceneBakeType(2);
    pub const BAKE_ALL: EMovieSceneBakeType = EMovieSceneBakeType(3);
}
#[repr(transparent)]
pub struct EFbxMaterialBakeMode(pub u8);
impl EFbxMaterialBakeMode {
    pub const DISABLED: EFbxMaterialBakeMode = EFbxMaterialBakeMode(0);
    pub const SIMPLE: EFbxMaterialBakeMode = EFbxMaterialBakeMode(1);
    pub const USE_MESH_DATA: EFbxMaterialBakeMode = EFbxMaterialBakeMode(2);
}
#[repr(transparent)]
pub struct EFBXNormalImportMethod(pub u8);
impl EFBXNormalImportMethod {
    pub const FBXNIM_COMPUTE_NORMALS: EFBXNormalImportMethod = EFBXNormalImportMethod(0);
    pub const FBXNIM_IMPORT_NORMALS: EFBXNormalImportMethod = EFBXNormalImportMethod(1);
    pub const FBXNIM_IMPORT_NORMALS_AND_TANGENTS: EFBXNormalImportMethod = EFBXNormalImportMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EFBXNormalGenerationMethod(pub u8);
impl EFBXNormalGenerationMethod {
    pub const BUILT_IN: EFBXNormalGenerationMethod = EFBXNormalGenerationMethod(0);
    pub const MIKK_T_SPACE: EFBXNormalGenerationMethod = EFBXNormalGenerationMethod(1);
}
#[repr(transparent)]
pub struct EFBXSceneOptionsCreateHierarchyType(pub u8);
impl EFBXSceneOptionsCreateHierarchyType {
    pub const FBXSOCHT_CREATE_LEVEL_ACTORS: EFBXSceneOptionsCreateHierarchyType = EFBXSceneOptionsCreateHierarchyType(
        0,
    );
    pub const FBXSOCHT_CREATE_ACTOR_COMPONENTS: EFBXSceneOptionsCreateHierarchyType = EFBXSceneOptionsCreateHierarchyType(
        1,
    );
    pub const FBXSOCHT_CREATE_BLUEPRINT: EFBXSceneOptionsCreateHierarchyType = EFBXSceneOptionsCreateHierarchyType(
        2,
    );
}
#[repr(transparent)]
pub struct EFbxSceneVertexColorImportOption(pub u8);
impl EFbxSceneVertexColorImportOption {
    pub const REPLACE: EFbxSceneVertexColorImportOption = EFbxSceneVertexColorImportOption(
        0,
    );
    pub const IGNORE: EFbxSceneVertexColorImportOption = EFbxSceneVertexColorImportOption(
        1,
    );
    pub const OVERRIDE: EFbxSceneVertexColorImportOption = EFbxSceneVertexColorImportOption(
        2,
    );
}
#[repr(transparent)]
pub struct EFBXSceneNormalImportMethod(pub u8);
impl EFBXSceneNormalImportMethod {
    pub const FBX_SCENE_NIM_COMPUTE_NORMALS: EFBXSceneNormalImportMethod = EFBXSceneNormalImportMethod(
        0,
    );
    pub const FBX_SCENE_NIM_IMPORT_NORMALS: EFBXSceneNormalImportMethod = EFBXSceneNormalImportMethod(
        1,
    );
    pub const FBX_SCENE_NIM_IMPORT_NORMALS_AND_TANGENTS: EFBXSceneNormalImportMethod = EFBXSceneNormalImportMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EFBXSceneNormalGenerationMethod(pub u8);
impl EFBXSceneNormalGenerationMethod {
    pub const BUILT_IN: EFBXSceneNormalGenerationMethod = EFBXSceneNormalGenerationMethod(
        0,
    );
    pub const MIKK_T_SPACE: EFBXSceneNormalGenerationMethod = EFBXSceneNormalGenerationMethod(
        1,
    );
}
#[repr(transparent)]
pub struct EFBXImportContentType(pub u8);
impl EFBXImportContentType {
    pub const FBXICT_ALL: EFBXImportContentType = EFBXImportContentType(0);
    pub const FBXICT_GEOMETRY: EFBXImportContentType = EFBXImportContentType(1);
    pub const FBXICT_SKINNING_WEIGHTS: EFBXImportContentType = EFBXImportContentType(2);
}
#[repr(transparent)]
pub struct EVertexColorImportOption(pub u8);
impl EVertexColorImportOption {
    pub const REPLACE: EVertexColorImportOption = EVertexColorImportOption(0);
    pub const IGNORE: EVertexColorImportOption = EVertexColorImportOption(1);
    pub const OVERRIDE: EVertexColorImportOption = EVertexColorImportOption(2);
}
#[repr(transparent)]
pub struct EPropertyEditorTestEnum(pub u8);
impl EPropertyEditorTestEnum {
    pub const PROPERTY_EDITOR_TEST_ENUM1: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        0,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM2: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        1,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM3: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        2,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM4: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        3,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM5: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        4,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM6: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        5,
    );
}
#[repr(transparent)]
pub struct EPropertyEditorTestEditColor(pub u8);
impl EPropertyEditorTestEditColor {
    pub const RED: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(0);
    pub const ORANGE: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(1);
    pub const YELLOW: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(2);
    pub const GREEN: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(3);
    pub const BLUE: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(4);
    pub const INDIGO: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(5);
    pub const VIOLET: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(6);
    pub const PINK: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(7);
    pub const MAGENTA: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(8);
    pub const CYAN: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(9);
}
#[repr(transparent)]
pub struct EPropertyEditorTestUnderscores(pub u8);
impl EPropertyEditorTestUnderscores {
    pub const ONE: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(0);
    pub const TWO: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(1);
    pub const THREE: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(2);
    pub const NOT_UNDERSCORE: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(
        3,
    );
}
#[repr(transparent)]
pub struct EReferenceViewerSettingMode(pub u8);
impl EReferenceViewerSettingMode {
    pub const NO_PREFERENCE: EReferenceViewerSettingMode = EReferenceViewerSettingMode(
        0,
    );
    pub const SHOW_BY_DEFAULT: EReferenceViewerSettingMode = EReferenceViewerSettingMode(
        1,
    );
    pub const HIDE_BY_DEFAULT: EReferenceViewerSettingMode = EReferenceViewerSettingMode(
        2,
    );
}
#[repr(transparent)]
pub struct EUnitDisplay(pub u8);
impl EUnitDisplay {
    pub const NONE: EUnitDisplay = EUnitDisplay(0);
    pub const METRIC: EUnitDisplay = EUnitDisplay(1);
    pub const IMPERIAL: EUnitDisplay = EUnitDisplay(2);
    pub const INVALID: EUnitDisplay = EUnitDisplay(3);
}
#[repr(transparent)]
pub struct EDefaultLocationUnit(pub u8);
impl EDefaultLocationUnit {
    pub const MICROMETERS: EDefaultLocationUnit = EDefaultLocationUnit(0);
    pub const MILLIMETERS: EDefaultLocationUnit = EDefaultLocationUnit(1);
    pub const CENTIMETERS: EDefaultLocationUnit = EDefaultLocationUnit(2);
    pub const METERS: EDefaultLocationUnit = EDefaultLocationUnit(3);
    pub const KILOMETERS: EDefaultLocationUnit = EDefaultLocationUnit(4);
    pub const INCHES: EDefaultLocationUnit = EDefaultLocationUnit(5);
    pub const FEET: EDefaultLocationUnit = EDefaultLocationUnit(6);
    pub const YARDS: EDefaultLocationUnit = EDefaultLocationUnit(7);
    pub const MILES: EDefaultLocationUnit = EDefaultLocationUnit(8);
    pub const INVALID: EDefaultLocationUnit = EDefaultLocationUnit(9);
}
#[repr(transparent)]
pub struct ELevelEditor2DAxis(pub u8);
impl ELevelEditor2DAxis {
    pub const X: ELevelEditor2DAxis = ELevelEditor2DAxis(0);
    pub const Y: ELevelEditor2DAxis = ELevelEditor2DAxis(1);
    pub const Z: ELevelEditor2DAxis = ELevelEditor2DAxis(2);
}
#[repr(transparent)]
pub struct EFBXTestPlanActionType(pub u8);
impl EFBXTestPlanActionType {
    pub const IMPORT: EFBXTestPlanActionType = EFBXTestPlanActionType(0);
    pub const REIMPORT: EFBXTestPlanActionType = EFBXTestPlanActionType(1);
    pub const ADD_LOD: EFBXTestPlanActionType = EFBXTestPlanActionType(2);
    pub const REIMPORT_LOD: EFBXTestPlanActionType = EFBXTestPlanActionType(3);
    pub const IMPORT_RELOAD: EFBXTestPlanActionType = EFBXTestPlanActionType(4);
    pub const ADD_ALTERNATE_SKINNIG: EFBXTestPlanActionType = EFBXTestPlanActionType(5);
}
#[repr(transparent)]
pub struct EThumbnailPrimType(pub u8);
impl EThumbnailPrimType {
    pub const TPT_NONE: EThumbnailPrimType = EThumbnailPrimType(0);
    pub const TPT_SPHERE: EThumbnailPrimType = EThumbnailPrimType(1);
    pub const TPT_CUBE: EThumbnailPrimType = EThumbnailPrimType(2);
    pub const TPT_PLANE: EThumbnailPrimType = EThumbnailPrimType(3);
    pub const TPT_CYLINDER: EThumbnailPrimType = EThumbnailPrimType(4);
}
#[repr(transparent)]
pub struct EOrthoThumbnailDirection(pub u8);
impl EOrthoThumbnailDirection {
    pub const TOP: EOrthoThumbnailDirection = EOrthoThumbnailDirection(0);
    pub const BOTTOM: EOrthoThumbnailDirection = EOrthoThumbnailDirection(1);
    pub const LEFT: EOrthoThumbnailDirection = EOrthoThumbnailDirection(2);
    pub const RIGHT: EOrthoThumbnailDirection = EOrthoThumbnailDirection(3);
    pub const FRONT: EOrthoThumbnailDirection = EOrthoThumbnailDirection(4);
    pub const BACK: EOrthoThumbnailDirection = EOrthoThumbnailDirection(5);
}
