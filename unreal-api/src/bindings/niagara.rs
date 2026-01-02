#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FNiagaraDataChannelUpdateContext {
    pub reader: UPtr<UNiagaraDataChannelReader>,
    pub first_new_data_index: i32,
    pub last_new_data_index: i32,
    pub new_element_count: i32,
    __padding_end: [u8; 4],
}
impl FNiagaraDataChannelUpdateContext {}
#[repr(C, align(4))]
pub struct FNiagaraSimCacheCaptureParameters {
    pub flags_0: u8,
    pub num_frames: i32,
    pub capture_rate: i32,
    pub flags_12: u8,
    pub timeout_frame_count: i32,
    pub flags_20: u8,
    pub immediate_capture_delta_time: f32,
}
impl FNiagaraSimCacheCaptureParameters {}
#[repr(C, align(8))]
pub struct FNiagaraSimCacheCreateParameters {
    pub attribute_capture_mode: ENiagaraSimCacheAttributeCaptureMode,
    #[doc(hidden)]
    __padding_4: [u8; 3],
    pub flags_4: u8,
    pub rebase_include_attributes: TArray<FName>,
    pub rebase_exclude_attributes: TArray<FName>,
    pub interpolation_include_attributes: TArray<FName>,
    pub interpolation_exclude_attributes: TArray<FName>,
    pub explicit_capture_attributes: TArray<FName>,
}
impl FNiagaraSimCacheCreateParameters {}
#[repr(C, align(8))]
pub struct FNiagaraMeshRendererMeshPropertiesBase {
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub scale: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub pivot_offset: crate::bindings::core_u_object::FVector,
    pub pivot_offset_space: ENiagaraMeshPivotOffsetSpace,
    __padding_end: [u8; 7],
}
impl FNiagaraMeshRendererMeshPropertiesBase {}
#[repr(C, align(8))]
pub struct FNiagaraMeshRendererMeshProperties {
    __padding_end: [u8; 1232],
}
impl FNiagaraMeshRendererMeshProperties {}
#[repr(C, align(8))]
pub struct FNiagaraVariable {
    __padding_end: [u8; 72],
}
impl FNiagaraVariable {}
#[repr(C, align(4))]
pub struct FNDIDistributionIntArrayEntry {
    __padding_end: [u8; 8],
}
impl FNDIDistributionIntArrayEntry {}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelSearchParameters {
    pub owning_component: UPtr<crate::bindings::engine::USceneComponent>,
    pub location: crate::bindings::core_u_object::FVector,
    pub flags_32: u8,
    __padding_end: [u8; 7],
}
impl FNiagaraDataChannelSearchParameters {}
#[repr(C, align(8))]
pub struct FNDCAccessContextBase {
    pub owning_component: UPtr<crate::bindings::engine::USceneComponent>,
    __padding_end: [u8; 8],
}
impl FNDCAccessContextBase {}
#[repr(C, align(4))]
pub struct FNDCSpawnedSystemRef {
    pub spawned_system: TWeakObjectPtr<UNiagaraComponent>,
}
impl FNDCSpawnedSystemRef {}
#[repr(C, align(8))]
pub struct FNDCAccessContext {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub location: crate::bindings::core_u_object::FVector,
    pub flags_40: u8,
    pub system_to_spawn: UPtr<crate::bindings::core_u_object::UObject>,
    pub spawned_systems: TArray<FNDCSpawnedSystemRef>,
}
impl FNDCAccessContext {}
#[repr(C, align(8))]
pub struct FNDCAccessContextLegacy {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub location: crate::bindings::core_u_object::FVector,
    pub flags_40: u8,
    __padding_end: [u8; 7],
}
impl FNDCAccessContextLegacy {}
#[repr(C, align(8))]
pub struct FNDCAccessContextInst {
    pub access_context: crate::bindings::core_u_object::FInstancedStruct,
}
impl FNDCAccessContextInst {}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelReference {
    pub data_channel: UPtr<UNiagaraDataChannelAsset>,
    pub access_context: FNDCAccessContextInst,
    pub b_custom_access_context: bool,
    __padding_end: [u8; 7],
}
impl FNiagaraDataChannelReference {}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelVariable {
    __padding_end: [u8; 72],
}
impl FNiagaraDataChannelVariable {}
#[repr(C, align(8))]
pub struct FNDCAccessContext_MapBase {
    __padding_end: [u8; 72],
}
impl FNDCAccessContext_MapBase {}
#[repr(C, align(8))]
pub struct FNDCAccessContext_GameplayBurst {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub flags_72: u8,
    pub cell_size_override: crate::bindings::core_u_object::FVector,
    pub system_bounds_padding: crate::bindings::core_u_object::FVector,
    pub gameplay_tag: crate::bindings::gameplay_tags::FGameplayTag,
    __padding_end: [u8; 4],
}
impl FNDCAccessContext_GameplayBurst {}
#[repr(C, align(8))]
pub struct FBasicParticleData {
    pub position: crate::bindings::core_u_object::FVector,
    pub size: f32,
    pub velocity: crate::bindings::core_u_object::FVector,
}
impl FBasicParticleData {}
#[repr(C, align(8))]
pub struct FVersionedNiagaraEmitterData {
    #[doc(hidden)]
    __padding_112: [u8; 112],
    pub b_local_space: bool,
    pub b_determinism: bool,
    pub random_seed: i32,
    pub interpolated_spawn_mode: ENiagaraInterpolatedSpawnMode,
    #[doc(hidden)]
    __padding_128: [u8; 7],
    pub sim_target: ENiagaraSimTarget,
    pub fixed_bounds: crate::bindings::core_u_object::FBox,
    pub flags_192: u8,
    #[doc(hidden)]
    __padding_328: [u8; 132],
    pub max_gpu_particles_spawn_per_frame: i32,
    pub allocation_mode: EParticleAllocationMode,
    __padding_end: [u8; 1307],
}
impl FVersionedNiagaraEmitterData {}
#[repr(C, align(4))]
pub struct FNiagaraPerfBaselineStats {
    pub per_instance_avg_gt: f32,
    pub per_instance_avg_rt: f32,
    pub per_instance_max_gt: f32,
    pub per_instance_max_rt: f32,
}
impl FNiagaraPerfBaselineStats {}
#[repr(C, align(4))]
pub struct FNiagaraRendererReadbackParameters {
    pub b_export_position: bool,
    pub b_export_tangent_basis: bool,
    pub b_export_color: bool,
    pub export_num_tex_coords: i32,
    pub b_export_materials: bool,
    pub b_apply_wpo: bool,
    pub view_index_to_capture: TOptional<i32>,
}
impl FNiagaraRendererReadbackParameters {}
#[repr(C, align(8))]
pub struct FNiagaraSystemCollectionData {
    __padding_end: [u8; 48],
}
impl FNiagaraSystemCollectionData {}
#[repr(C, align(4))]
pub struct FNiagaraPosition {
    __padding_end: [u8; 12],
}
impl FNiagaraPosition {}
#[repr(C, align(4))]
pub struct FNiagaraSpawnInfo {
    pub count: i32,
    pub interp_start_dt: f32,
    pub interval_dt: f32,
    pub spawn_group: i32,
}
impl FNiagaraSpawnInfo {}
#[repr(C, align(4))]
pub struct FNiagaraID {
    pub index: i32,
    pub acquire_tag: i32,
}
impl FNiagaraID {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterface {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterface {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfacePlatformSet {
    __padding_end: [u8; 248],
}
impl UNiagaraDataInterfacePlatformSet {}
#[repr(C, align(8))]
pub struct UNiagaraSystem {
    #[doc(hidden)]
    __padding_857: [u8; 857],
    pub flags_857: u8,
    #[doc(hidden)]
    __padding_860: [u8; 2],
    pub custom_depth_stencil_write_mask: crate::bindings::engine::ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub translucency_sort_priority: i32,
    pub translucency_sort_distance_offset: f32,
    __padding_end: [u8; 2428],
}
impl UNiagaraSystem {}
#[repr(C, align(8))]
pub struct UNiagaraConvertInPlaceUtilityBase {
    __padding_end: [u8; 48],
}
impl UNiagaraConvertInPlaceUtilityBase {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDynamicMeshSimCacheData {
    __padding_end: [u8; 96],
}
impl UNiagaraDataInterfaceDynamicMeshSimCacheData {}
#[repr(C, align(8))]
pub struct UNiagaraParameterDefinitionsBase {
    __padding_end: [u8; 96],
}
impl UNiagaraParameterDefinitionsBase {}
pub struct UNiagaraRenderableMeshArrayInterface {}
pub struct INiagaraRenderableMeshArrayInterface {}
pub struct UNiagaraRenderableMeshInterface {}
pub struct INiagaraRenderableMeshInterface {}
#[repr(C, align(8))]
pub struct UNiagaraScriptSourceBase {
    __padding_end: [u8; 112],
}
impl UNiagaraScriptSourceBase {}
pub struct UNiagaraSimCacheCustomStorageInterface {}
pub struct INiagaraSimCacheCustomStorageInterface {}
#[repr(C, align(8))]
pub struct UNiagaraValidationRuleSet {
    __padding_end: [u8; 64],
}
impl UNiagaraValidationRuleSet {}
#[repr(C, align(8))]
pub struct UNDIRenderTargetSimCacheData {
    __padding_end: [u8; 96],
}
impl UNDIRenderTargetSimCacheData {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceActorComponent {
    __padding_end: [u8; 264],
}
impl UNiagaraDataInterfaceActorComponent {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayDistributionInt {
    __padding_end: [u8; 192],
}
impl UNiagaraDataInterfaceArrayDistributionInt {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRWBase {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterfaceRWBase {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArray {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub gpu_sync_mode: ENiagaraGpuSyncMode,
    pub max_elements: i32,
}
impl UNiagaraDataInterfaceArray {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayMesh {
    __padding_end: [u8; 200],
}
impl UNiagaraDataInterfaceArrayMesh {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAsyncGpuTrace {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceAsyncGpuTrace {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceConsoleVariable {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterfaceConsoleVariable {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDataChannelRead {
    __padding_end: [u8; 296],
}
impl UNiagaraDataInterfaceDataChannelRead {}
#[repr(C, align(8))]
pub struct UNDIDataChannelWriteSimCacheData {
    __padding_end: [u8; 120],
}
impl UNDIDataChannelWriteSimCacheData {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDataChannelWrite {
    __padding_end: [u8; 384],
}
impl UNiagaraDataInterfaceDataChannelWrite {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDataTable {
    __padding_end: [u8; 256],
}
impl UNiagaraDataInterfaceDataTable {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDebugDraw {
    __padding_end: [u8; 160],
}
impl UNiagaraDataInterfaceDebugDraw {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDynamicMesh {
    __padding_end: [u8; 272],
}
impl UNiagaraDataInterfaceDynamicMesh {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceEmitterProperties {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceEmitterProperties {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGBuffer {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterfaceGBuffer {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceMemoryBuffer {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceMemoryBuffer {}
#[repr(C, align(8))]
pub struct UNDIMemoryBufferSimCacheData {
    __padding_end: [u8; 80],
}
impl UNDIMemoryBufferSimCacheData {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfacePhysicsAsset {
    __padding_end: [u8; 320],
}
impl UNiagaraDataInterfacePhysicsAsset {}
pub struct UNiagaraPhysicsAssetDICollectorInterface {}
pub struct INiagaraPhysicsAssetDICollectorInterface {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSceneCapture2D {
    __padding_end: [u8; 448],
}
impl UNiagaraDataInterfaceSceneCapture2D {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSimCacheReader {
    __padding_end: [u8; 248],
}
impl UNiagaraDataInterfaceSimCacheReader {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSimpleCounter {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceSimpleCounter {}
#[repr(C, align(8))]
pub struct UNDISimpleCounterSimCacheData {
    __padding_end: [u8; 64],
}
impl UNDISimpleCounterSimCacheData {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSocketReader {
    __padding_end: [u8; 360],
}
impl UNiagaraDataInterfaceSocketReader {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceStaticMesh {
    __padding_end: [u8; 480],
}
impl UNiagaraDataInterfaceStaticMesh {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceUObjectPropertyReader {
    __padding_end: [u8; 312],
}
impl UNiagaraDataInterfaceUObjectPropertyReader {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVirtualTexture {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceVirtualTexture {}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraSystemSpawnSection {
    __padding_end: [u8; 368],
}
impl UMovieSceneNiagaraSystemSpawnSection {}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraTrack {
    __padding_end: [u8; 400],
}
impl UMovieSceneNiagaraTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraSystemTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneNiagaraSystemTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraParameterTrack {
    __padding_end: [u8; 472],
}
impl UMovieSceneNiagaraParameterTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraBoolParameterTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneNiagaraBoolParameterTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraColorParameterTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneNiagaraColorParameterTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraFloatParameterTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneNiagaraFloatParameterTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraIntegerParameterTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneNiagaraIntegerParameterTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraVectorParameterTrack {
    __padding_end: [u8; 488],
}
impl UMovieSceneNiagaraVectorParameterTrack {}
#[repr(C, align(8))]
pub struct ANiagaraActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub niagara_component: UPtr<UNiagaraComponent>,
    __padding_end: [u8; 24],
}
impl ANiagaraActor {}
#[repr(C, align(8))]
pub struct UDEPRECATED_NiagaraAssetTagDefinitions {
    __padding_end: [u8; 104],
}
impl UDEPRECATED_NiagaraAssetTagDefinitions {}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutput {
    __padding_end: [u8; 64],
}
impl UNiagaraBakerOutput {}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutputSimCache {
    __padding_end: [u8; 168],
}
impl UNiagaraBakerOutputSimCache {}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutputSparseVolumeTexture {
    __padding_end: [u8; 480],
}
impl UNiagaraBakerOutputSparseVolumeTexture {}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutputStaticMesh {
    __padding_end: [u8; 104],
}
impl UNiagaraBakerOutputStaticMesh {}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutputTexture2D {
    __padding_end: [u8; 176],
}
impl UNiagaraBakerOutputTexture2D {}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutputVolumeTexture {
    __padding_end: [u8; 152],
}
impl UNiagaraBakerOutputVolumeTexture {}
#[repr(C, align(8))]
pub struct UNiagaraBakerSettings {
    __padding_end: [u8; 512],
}
impl UNiagaraBakerSettings {}
#[repr(C, align(16))]
pub struct UNiagaraComponent {
    #[doc(hidden)]
    __padding_2552: [u8; 2552],
    pub flags_2552: u8,
    #[doc(hidden)]
    __padding_2561: [u8; 8],
    pub occlusion_query_mode: ENiagaraOcclusionQueryMode,
    #[doc(hidden)]
    __padding_2592: [u8; 28],
    pub auto_attach_parent: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
    pub auto_attach_socket_name: FName,
    pub auto_attach_location_rule: crate::bindings::engine::EAttachmentRule,
    pub auto_attach_rotation_rule: crate::bindings::engine::EAttachmentRule,
    pub auto_attach_scale_rule: crate::bindings::engine::EAttachmentRule,
    #[doc(hidden)]
    __padding_2648: [u8; 33],
    pub flags_2648: u8,
    __padding_end: [u8; 311],
}
impl UNiagaraComponent {}
#[repr(C, align(8))]
pub struct UNiagaraComponentPool {
    __padding_end: [u8; 176],
}
impl UNiagaraComponentPool {}
#[repr(C, align(8))]
pub struct UNiagaraRendererProperties {
    __padding_end: [u8; 672],
}
impl UNiagaraRendererProperties {}
#[repr(C, align(8))]
pub struct UNiagaraComponentRendererProperties {
    __padding_end: [u8; 1600],
}
impl UNiagaraComponentRendererProperties {}
#[repr(C, align(16))]
pub struct UNiagaraCullProxyComponent {
    __padding_end: [u8; 2992],
}
impl UNiagaraCullProxyComponent {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannel {
    __padding_end: [u8; 152],
}
impl UNiagaraDataChannel {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelReader {
    __padding_end: [u8; 80],
}
impl UNiagaraDataChannelReader {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelWriter {
    __padding_end: [u8; 80],
}
impl UNiagaraDataChannelWriter {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelAsset {
    __padding_end: [u8; 64],
}
impl UNiagaraDataChannelAsset {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraDataChannelLibrary {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelHandler {
    __padding_end: [u8; 160],
}
impl UNiagaraDataChannelHandler {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannel_MapBase {
    __padding_end: [u8; 160],
}
impl UNiagaraDataChannel_MapBase {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannel_GameplayBurst {
    __padding_end: [u8; 264],
}
impl UNiagaraDataChannel_GameplayBurst {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelHandler_MapBase {
    __padding_end: [u8; 376],
}
impl UNiagaraDataChannelHandler_MapBase {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelHandler_GameplayBurst {
    __padding_end: [u8; 480],
}
impl UNiagaraDataChannelHandler_GameplayBurst {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannel_Global {
    __padding_end: [u8; 152],
}
impl UNiagaraDataChannel_Global {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelHandler_Global {
    __padding_end: [u8; 176],
}
impl UNiagaraDataChannelHandler_Global {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannel_Islands {
    __padding_end: [u8; 288],
}
impl UNiagaraDataChannel_Islands {}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelHandler_Islands {
    __padding_end: [u8; 208],
}
impl UNiagaraDataChannelHandler_Islands {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterface2DArrayTexture {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterface2DArrayTexture {}
#[repr(C, align(8))]
pub struct UNDIArraySimCacheData {
    __padding_end: [u8; 104],
}
impl UNDIArraySimCacheData {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayFloat {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub float_data: TArray<f32>,
}
impl UNiagaraDataInterfaceArrayFloat {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayFloat2 {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub float_data: TArray<crate::bindings::core_u_object::FVector2D>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayFloat2 {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayFloat3 {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub float_data: TArray<crate::bindings::core_u_object::FVector>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayFloat3 {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayPosition {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub position_data: TArray<FNiagaraPosition>,
}
impl UNiagaraDataInterfaceArrayPosition {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayFloat4 {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub float_data: TArray<crate::bindings::core_u_object::FVector4>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayFloat4 {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayColor {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub color_data: TArray<crate::bindings::core_u_object::FLinearColor>,
}
impl UNiagaraDataInterfaceArrayColor {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayQuat {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub quat_data: TArray<crate::bindings::core_u_object::FQuat>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayQuat {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayMatrix {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub matrix_data: TArray<crate::bindings::core_u_object::FMatrix>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayMatrix {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraDataInterfaceArrayFunctionLibrary {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayInt32 {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub int_data: TArray<i32>,
}
impl UNiagaraDataInterfaceArrayInt32 {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayUInt8 {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub int_data: TArray<i32>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayUInt8 {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayBool {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub bool_data: TArray<bool>,
}
impl UNiagaraDataInterfaceArrayBool {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayNiagaraID {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub int_data: TArray<FNiagaraID>,
}
impl UNiagaraDataInterfaceArrayNiagaraID {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAudioSubmix {
    __padding_end: [u8; 160],
}
impl UNiagaraDataInterfaceAudioSubmix {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAudioOscilloscope {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceAudioOscilloscope {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAudioPlayerSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_override_concurrency: bool,
    pub concurrency: UPtr<crate::bindings::engine::USoundConcurrency>,
    pub b_override_attenuation_settings: bool,
    pub attenuation_settings: crate::bindings::engine::FSoundAttenuationSettings,
}
impl UNiagaraDataInterfaceAudioPlayerSettings {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAudioPlayer {
    __padding_end: [u8; 280],
}
impl UNiagaraDataInterfaceAudioPlayer {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAudioSpectrum {
    __padding_end: [u8; 176],
}
impl UNiagaraDataInterfaceAudioSpectrum {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCamera {
    __padding_end: [u8; 160],
}
impl UNiagaraDataInterfaceCamera {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCollisionQuery {
    __padding_end: [u8; 160],
}
impl UNiagaraDataInterfaceCollisionQuery {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCurveBase {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceCurveBase {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceColorCurve {
    __padding_end: [u8; 1256],
}
impl UNiagaraDataInterfaceColorCurve {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCubeTexture {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceCubeTexture {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCurlNoise {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceCurlNoise {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCurve {
    __padding_end: [u8; 488],
}
impl UNiagaraDataInterfaceCurve {}
pub struct UNiagaraParticleCallbackHandler {}
pub struct INiagaraParticleCallbackHandler {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceExport {
    __padding_end: [u8; 240],
}
impl UNiagaraDataInterfaceExport {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid2D {
    __padding_end: [u8; 192],
}
impl UNiagaraDataInterfaceGrid2D {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid2DCollection {
    __padding_end: [u8; 448],
}
impl UNiagaraDataInterfaceGrid2DCollection {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid2DCollectionReader {
    __padding_end: [u8; 480],
}
impl UNiagaraDataInterfaceGrid2DCollectionReader {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid3D {
    __padding_end: [u8; 208],
}
impl UNiagaraDataInterfaceGrid3D {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid3DCollection {
    __padding_end: [u8; 384],
}
impl UNiagaraDataInterfaceGrid3DCollection {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid3DCollectionReader {
    __padding_end: [u8; 416],
}
impl UNiagaraDataInterfaceGrid3DCollectionReader {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceIntRenderTarget2D {
    __padding_end: [u8; 256],
}
impl UNiagaraDataInterfaceIntRenderTarget2D {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceLandscape {
    __padding_end: [u8; 192],
}
impl UNiagaraDataInterfaceLandscape {}
#[repr(C, align(8))]
pub struct UNDILandscapeSimCacheData {
    __padding_end: [u8; 64],
}
impl UNDILandscapeSimCacheData {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceMaterialInstanceDynamic {
    __padding_end: [u8; 496],
}
impl UNiagaraDataInterfaceMaterialInstanceDynamic {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceMaterialParameterCollection {
    __padding_end: [u8; 496],
}
impl UNiagaraDataInterfaceMaterialParameterCollection {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceMeshRendererInfo {
    __padding_end: [u8; 184],
}
impl UNiagaraDataInterfaceMeshRendererInfo {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceNeighborGrid3D {
    __padding_end: [u8; 216],
}
impl UNiagaraDataInterfaceNeighborGrid3D {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceOcclusion {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterfaceOcclusion {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceParticleRead {
    __padding_end: [u8; 184],
}
impl UNiagaraDataInterfaceParticleRead {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRasterizationGrid3D {
    __padding_end: [u8; 224],
}
impl UNiagaraDataInterfaceRasterizationGrid3D {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRenderTarget2D {
    __padding_end: [u8; 328],
}
impl UNiagaraDataInterfaceRenderTarget2D {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRenderTarget2DArray {
    __padding_end: [u8; 240],
}
impl UNiagaraDataInterfaceRenderTarget2DArray {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRenderTargetCube {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceRenderTargetCube {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRenderTargetVolume {
    __padding_end: [u8; 248],
}
impl UNiagaraDataInterfaceRenderTargetVolume {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRigidMeshCollisionQuery {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceRigidMeshCollisionQuery {}
#[repr(C, align(8))]
pub struct UNiagaraDIRigidMeshCollisionFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraDIRigidMeshCollisionFunctionLibrary {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSkeletalMesh {
    __padding_end: [u8; 464],
}
impl UNiagaraDataInterfaceSkeletalMesh {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSparseVolumeTexture {
    __padding_end: [u8; 240],
}
impl UNiagaraDataInterfaceSparseVolumeTexture {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSpline {
    __padding_end: [u8; 368],
}
impl UNiagaraDataInterfaceSpline {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSpriteRendererInfo {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceSpriteRendererInfo {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceTexture {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceTexture {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVector2DCurve {
    __padding_end: [u8; 744],
}
impl UNiagaraDataInterfaceVector2DCurve {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVector4Curve {
    __padding_end: [u8; 1256],
}
impl UNiagaraDataInterfaceVector4Curve {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVectorCurve {
    __padding_end: [u8; 1000],
}
impl UNiagaraDataInterfaceVectorCurve {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVectorField {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceVectorField {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVirtualTextureSample {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceVirtualTextureSample {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVolumeCache {
    __padding_end: [u8; 240],
}
impl UNiagaraDataInterfaceVolumeCache {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVolumeTexture {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceVolumeTexture {}
#[repr(C, align(8))]
pub struct UNiagaraDebugHUDSettings {
    __padding_end: [u8; 664],
}
impl UNiagaraDebugHUDSettings {}
#[repr(C, align(8))]
pub struct UNiagaraDecalRendererProperties {
    __padding_end: [u8; 4312],
}
impl UNiagaraDecalRendererProperties {}
#[repr(C, align(8))]
pub struct UNiagaraEditorDataBase {
    __padding_end: [u8; 80],
}
impl UNiagaraEditorDataBase {}
#[repr(C, align(8))]
pub struct UNiagaraEditorParametersAdapterBase {
    __padding_end: [u8; 48],
}
impl UNiagaraEditorParametersAdapterBase {}
#[repr(C, align(8))]
pub struct ANiagaraEditorPreviewActor {
    #[doc(hidden)]
    __padding_1288: [u8; 1288],
    pub niagara_component: UPtr<UNiagaraComponent>,
    __padding_end: [u8; 8],
}
impl ANiagaraEditorPreviewActor {}
#[repr(C, align(8))]
pub struct UNiagaraSignificanceHandler {
    __padding_end: [u8; 48],
}
impl UNiagaraSignificanceHandler {}
#[repr(C, align(8))]
pub struct UNiagaraSignificanceHandlerDistance {
    __padding_end: [u8; 48],
}
impl UNiagaraSignificanceHandlerDistance {}
#[repr(C, align(8))]
pub struct UNiagaraSignificanceHandlerAge {
    __padding_end: [u8; 48],
}
impl UNiagaraSignificanceHandlerAge {}
#[repr(C, align(8))]
pub struct UNiagaraEffectType {
    __padding_end: [u8; 264],
}
impl UNiagaraEffectType {}
#[repr(C, align(8))]
pub struct UNiagaraEmitterBase {
    __padding_end: [u8; 96],
}
impl UNiagaraEmitterBase {}
#[repr(C, align(8))]
pub struct UNiagaraEmitter {
    __padding_end: [u8; 1720],
}
impl UNiagaraEmitter {}
#[repr(C, align(8))]
pub struct UNiagaraEventReceiverEmitterAction {
    __padding_end: [u8; 48],
}
impl UNiagaraEventReceiverEmitterAction {}
#[repr(C, align(8))]
pub struct UNiagaraEventReceiverEmitterAction_SpawnParticles {
    __padding_end: [u8; 56],
}
impl UNiagaraEventReceiverEmitterAction_SpawnParticles {}
#[repr(C, align(8))]
pub struct UNiagaraFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraFunctionLibrary {}
#[repr(C, align(16))]
pub struct ANiagaraLensEffectBase {
    __padding_end: [u8; 1312],
}
impl ANiagaraLensEffectBase {}
#[repr(C, align(8))]
pub struct UNiagaraLightRendererProperties {
    #[doc(hidden)]
    __padding_708: [u8; 708],
    pub inverse_exposure_blend: f32,
    __padding_end: [u8; 3608],
}
impl UNiagaraLightRendererProperties {}
#[repr(C, align(8))]
pub struct UNiagaraMeshRendererProperties {
    __padding_end: [u8; 9648],
}
impl UNiagaraMeshRendererProperties {}
#[repr(C, align(8))]
pub struct UNiagaraMessageDataBase {
    __padding_end: [u8; 48],
}
impl UNiagaraMessageDataBase {}
#[repr(C, align(8))]
pub struct UNiagaraParameterCollectionInstance {
    __padding_end: [u8; 624],
}
impl UNiagaraParameterCollectionInstance {}
#[repr(C, align(8))]
pub struct UNiagaraParameterCollection {
    __padding_end: [u8; 312],
}
impl UNiagaraParameterCollection {}
#[repr(C, align(8))]
pub struct UNiagaraBaselineController {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub test_duration: f32,
    pub effect_type: UPtr<UNiagaraEffectType>,
    pub owner: UPtr<ANiagaraPerfBaselineActor>,
    __padding_end: [u8; 48],
}
impl UNiagaraBaselineController {}
#[repr(C, align(8))]
pub struct UNiagaraBaselineController_Basic {
    __padding_end: [u8; 144],
}
impl UNiagaraBaselineController_Basic {}
#[repr(C, align(8))]
pub struct ANiagaraPerfBaselineActor {
    __padding_end: [u8; 1152],
}
impl ANiagaraPerfBaselineActor {}
#[repr(C, align(8))]
pub struct UNiagaraPrecompileContainer {
    __padding_end: [u8; 72],
}
impl UNiagaraPrecompileContainer {}
#[repr(C, align(8))]
pub struct ANiagaraPreviewBase {
    __padding_end: [u8; 1136],
}
impl ANiagaraPreviewBase {}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis {
    __padding_end: [u8; 48],
}
impl UNiagaraPreviewAxis {}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamBase {
    __padding_end: [u8; 64],
}
impl UNiagaraPreviewAxis_InterpParamBase {}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamInt32 {
    __padding_end: [u8; 72],
}
impl UNiagaraPreviewAxis_InterpParamInt32 {}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamFloat {
    __padding_end: [u8; 72],
}
impl UNiagaraPreviewAxis_InterpParamFloat {}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamVector2D {
    __padding_end: [u8; 96],
}
impl UNiagaraPreviewAxis_InterpParamVector2D {}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamVector {
    __padding_end: [u8; 112],
}
impl UNiagaraPreviewAxis_InterpParamVector {}
#[repr(C, align(16))]
pub struct UNiagaraPreviewAxis_InterpParamVector4 {
    __padding_end: [u8; 128],
}
impl UNiagaraPreviewAxis_InterpParamVector4 {}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamLinearColor {
    __padding_end: [u8; 96],
}
impl UNiagaraPreviewAxis_InterpParamLinearColor {}
#[repr(C, align(8))]
pub struct ANiagaraPreviewGrid {
    __padding_end: [u8; 1232],
}
impl ANiagaraPreviewGrid {}
#[repr(C, align(8))]
pub struct UNiagaraRibbonRendererProperties {
    __padding_end: [u8; 10248],
}
impl UNiagaraRibbonRendererProperties {}
#[repr(C, align(8))]
pub struct UNiagaraScratchPadContainer {
    __padding_end: [u8; 64],
}
impl UNiagaraScratchPadContainer {}
#[repr(C, align(8))]
pub struct UNiagaraScript {
    __padding_end: [u8; 4112],
}
impl UNiagaraScript {}
#[repr(C, align(8))]
pub struct UNiagaraSettings {
    #[doc(hidden)]
    __padding_171: [u8; 171],
    pub b_limit_delta_time: bool,
    pub max_delta_time_per_tick: f32,
    __padding_end: [u8; 256],
}
impl UNiagaraSettings {}
#[repr(C, align(8))]
pub struct UNiagaraSimCache {
    __padding_end: [u8; 568],
}
impl UNiagaraSimCache {}
#[repr(C, align(8))]
pub struct UNiagaraSimCacheDebugData {
    __padding_end: [u8; 64],
}
impl UNiagaraSimCacheDebugData {}
#[repr(C, align(8))]
pub struct UAsyncNiagaraCaptureSimCache {
    __padding_end: [u8; 328],
}
impl UAsyncNiagaraCaptureSimCache {}
#[repr(C, align(8))]
pub struct UNiagaraSimCacheFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraSimCacheFunctionLibrary {}
#[repr(C, align(8))]
pub struct UNiagaraSimulationStageBase {
    __padding_end: [u8; 136],
}
impl UNiagaraSimulationStageBase {}
#[repr(C, align(8))]
pub struct UNiagaraSimulationStageGeneric {
    __padding_end: [u8; 5496],
}
impl UNiagaraSimulationStageGeneric {}
#[repr(C, align(8))]
pub struct UNiagaraSpriteRendererProperties {
    #[doc(hidden)]
    __padding_11512: [u8; 11512],
    pub b_use_material_cutout_texture: bool,
    __padding_end: [u8; 127],
}
impl UNiagaraSpriteRendererProperties {}
#[repr(C, align(8))]
pub struct UNiagaraSystemCollection {
    __padding_end: [u8; 112],
}
impl UNiagaraSystemCollection {}
#[repr(C, align(8))]
pub struct UNiagaraValidationRule {
    __padding_end: [u8; 56],
}
impl UNiagaraValidationRule {}
#[repr(C, align(8))]
pub struct UNiagaraVolumeRendererProperties {
    __padding_end: [u8; 3520],
}
impl UNiagaraVolumeRendererProperties {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule {
    __padding_end: [u8; 104],
}
impl UNiagaraStatelessModule {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_AccelerationForce {
    __padding_end: [u8; 264],
}
impl UNiagaraStatelessModule_AccelerationForce {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_AddVelocity {
    __padding_end: [u8; 728],
}
impl UNiagaraStatelessModule_AddVelocity {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_CalculateAccurateVelocity {
    __padding_end: [u8; 104],
}
impl UNiagaraStatelessModule_CalculateAccurateVelocity {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_CameraOffset {
    __padding_end: [u8; 256],
}
impl UNiagaraStatelessModule_CameraOffset {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_CurlNoiseForce {
    __padding_end: [u8; 112],
}
impl UNiagaraStatelessModule_CurlNoiseForce {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_DecalAttributes {
    __padding_end: [u8; 576],
}
impl UNiagaraStatelessModule_DecalAttributes {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_Drag {
    __padding_end: [u8; 240],
}
impl UNiagaraStatelessModule_Drag {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_DynamicMaterialParameters {
    __padding_end: [u8; 2576],
}
impl UNiagaraStatelessModule_DynamicMaterialParameters {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_GravityForce {
    __padding_end: [u8; 256],
}
impl UNiagaraStatelessModule_GravityForce {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_InitializeParticle {
    __padding_end: [u8; 1256],
}
impl UNiagaraStatelessModule_InitializeParticle {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_InitialMeshOrientation {
    __padding_end: [u8; 584],
}
impl UNiagaraStatelessModule_InitialMeshOrientation {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_LightAttributes {
    __padding_end: [u8; 872],
}
impl UNiagaraStatelessModule_LightAttributes {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_MeshIndex {
    __padding_end: [u8; 208],
}
impl UNiagaraStatelessModule_MeshIndex {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_MeshRotationRate {
    __padding_end: [u8; 424],
}
impl UNiagaraStatelessModule_MeshRotationRate {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_RotateAroundPoint {
    __padding_end: [u8; 832],
}
impl UNiagaraStatelessModule_RotateAroundPoint {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleColor {
    __padding_end: [u8; 256],
}
impl UNiagaraStatelessModule_ScaleColor {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleMeshSize {
    __padding_end: [u8; 608],
}
impl UNiagaraStatelessModule_ScaleMeshSize {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleMeshSizeBySpeed {
    __padding_end: [u8; 704],
}
impl UNiagaraStatelessModule_ScaleMeshSizeBySpeed {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleRibbonWidth {
    __padding_end: [u8; 608],
}
impl UNiagaraStatelessModule_ScaleRibbonWidth {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleSpriteSize {
    __padding_end: [u8; 608],
}
impl UNiagaraStatelessModule_ScaleSpriteSize {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleSpriteSizeBySpeed {
    __padding_end: [u8; 688],
}
impl UNiagaraStatelessModule_ScaleSpriteSizeBySpeed {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ShapeLocation {
    __padding_end: [u8; 2112],
}
impl UNiagaraStatelessModule_ShapeLocation {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_SolveVelocitiesAndForces {
    __padding_end: [u8; 104],
}
impl UNiagaraStatelessModule_SolveVelocitiesAndForces {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_SpriteFacingAndAlignment {
    __padding_end: [u8; 416],
}
impl UNiagaraStatelessModule_SpriteFacingAndAlignment {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_SpriteRotationRate {
    __padding_end: [u8; 408],
}
impl UNiagaraStatelessModule_SpriteRotationRate {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_SubUVAnimation {
    __padding_end: [u8; 224],
}
impl UNiagaraStatelessModule_SubUVAnimation {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessEmitter {
    __padding_end: [u8; 816],
}
impl UNiagaraStatelessEmitter {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessEmitterTemplate {
    __padding_end: [u8; 136],
}
impl UNiagaraStatelessEmitterTemplate {}
#[repr(C, align(8))]
pub struct UNiagaraStatelessEmitterDefault {
    __padding_end: [u8; 136],
}
impl UNiagaraStatelessEmitterDefault {}
#[repr(C, align(8))]
pub struct UVolumeCache {
    __padding_end: [u8; 104],
}
impl UVolumeCache {}
#[repr(C, align(8))]
pub struct FSubscribeToNiagaraDataChannel_UpdateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSubscribeToNiagaraDataChannel_WithContext_UpdateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSubscribeToDataChannelUpdates_UpdateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSubscribeToDataChannelUpdates_WithContext_UpdateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FNiagaraComponent_OnSystemFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncNiagaraCaptureSimCache_CaptureComplete {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ENiagaraSimCacheAttributeCaptureMode(pub u8);
impl ENiagaraSimCacheAttributeCaptureMode {
    pub const ALL: ENiagaraSimCacheAttributeCaptureMode = ENiagaraSimCacheAttributeCaptureMode(
        0,
    );
    pub const RENDERING_ONLY: ENiagaraSimCacheAttributeCaptureMode = ENiagaraSimCacheAttributeCaptureMode(
        1,
    );
    pub const EXPLICIT_ATTRIBUTES: ENiagaraSimCacheAttributeCaptureMode = ENiagaraSimCacheAttributeCaptureMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraDataSetType(pub u8);
impl ENiagaraDataSetType {
    pub const PARTICLE_DATA: ENiagaraDataSetType = ENiagaraDataSetType(0);
    pub const SHARED: ENiagaraDataSetType = ENiagaraDataSetType(1);
    pub const EVENT: ENiagaraDataSetType = ENiagaraDataSetType(2);
}
#[repr(transparent)]
pub struct ENiagaraSimTarget(pub u8);
impl ENiagaraSimTarget {
    pub const CPU_SIM: ENiagaraSimTarget = ENiagaraSimTarget(0);
    pub const GPU_COMPUTE_SIM: ENiagaraSimTarget = ENiagaraSimTarget(1);
}
#[repr(transparent)]
pub struct ENiagaraMeshPivotOffsetSpace(pub u8);
impl ENiagaraMeshPivotOffsetSpace {
    pub const MESH: ENiagaraMeshPivotOffsetSpace = ENiagaraMeshPivotOffsetSpace(0);
    pub const SIMULATION: ENiagaraMeshPivotOffsetSpace = ENiagaraMeshPivotOffsetSpace(1);
    pub const WORLD: ENiagaraMeshPivotOffsetSpace = ENiagaraMeshPivotOffsetSpace(2);
    pub const LOCAL: ENiagaraMeshPivotOffsetSpace = ENiagaraMeshPivotOffsetSpace(3);
}
#[repr(transparent)]
pub struct ENiagaraMeshLODMode(pub u8);
impl ENiagaraMeshLODMode {
    pub const LOD_LEVEL: ENiagaraMeshLODMode = ENiagaraMeshLODMode(0);
    pub const LOD_BIAS: ENiagaraMeshLODMode = ENiagaraMeshLODMode(1);
    pub const BY_COMPONENT_BOUNDS: ENiagaraMeshLODMode = ENiagaraMeshLODMode(2);
    pub const COMPONENT_ORIGIN: ENiagaraMeshLODMode = ENiagaraMeshLODMode(3);
    pub const PER_PARTICLE: ENiagaraMeshLODMode = ENiagaraMeshLODMode(4);
}
#[repr(transparent)]
pub struct ENiagaraSystemSpawnSectionStartBehavior(pub u8);
impl ENiagaraSystemSpawnSectionStartBehavior {
    pub const ACTIVATE: ENiagaraSystemSpawnSectionStartBehavior = ENiagaraSystemSpawnSectionStartBehavior(
        0,
    );
}
#[repr(transparent)]
pub struct ENiagaraSystemSpawnSectionEvaluateBehavior(pub u8);
impl ENiagaraSystemSpawnSectionEvaluateBehavior {
    pub const ACTIVATE_IF_INACTIVE: ENiagaraSystemSpawnSectionEvaluateBehavior = ENiagaraSystemSpawnSectionEvaluateBehavior(
        0,
    );
    pub const NONE: ENiagaraSystemSpawnSectionEvaluateBehavior = ENiagaraSystemSpawnSectionEvaluateBehavior(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraSystemSpawnSectionEndBehavior(pub u8);
impl ENiagaraSystemSpawnSectionEndBehavior {
    pub const SET_SYSTEM_INACTIVE: ENiagaraSystemSpawnSectionEndBehavior = ENiagaraSystemSpawnSectionEndBehavior(
        0,
    );
    pub const DEACTIVATE: ENiagaraSystemSpawnSectionEndBehavior = ENiagaraSystemSpawnSectionEndBehavior(
        1,
    );
    pub const NONE: ENiagaraSystemSpawnSectionEndBehavior = ENiagaraSystemSpawnSectionEndBehavior(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraAgeUpdateMode(pub u8);
impl ENiagaraAgeUpdateMode {
    pub const TICK_DELTA_TIME: ENiagaraAgeUpdateMode = ENiagaraAgeUpdateMode(0);
    pub const DESIRED_AGE: ENiagaraAgeUpdateMode = ENiagaraAgeUpdateMode(1);
    pub const DESIRED_AGE_NO_SEEK: ENiagaraAgeUpdateMode = ENiagaraAgeUpdateMode(2);
}
#[repr(transparent)]
pub struct ENiagaraAssetTagDefinitionImportance(pub u8);
impl ENiagaraAssetTagDefinitionImportance {
    pub const PRIMARY: ENiagaraAssetTagDefinitionImportance = ENiagaraAssetTagDefinitionImportance(
        0,
    );
    pub const SECONDARY: ENiagaraAssetTagDefinitionImportance = ENiagaraAssetTagDefinitionImportance(
        1,
    );
    pub const INTERNAL: ENiagaraAssetTagDefinitionImportance = ENiagaraAssetTagDefinitionImportance(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraBakerViewMode(pub i32);
impl ENiagaraBakerViewMode {
    pub const PERSPECTIVE: ENiagaraBakerViewMode = ENiagaraBakerViewMode(0);
    pub const ORTHO_FRONT: ENiagaraBakerViewMode = ENiagaraBakerViewMode(1);
    pub const ORTHO_BACK: ENiagaraBakerViewMode = ENiagaraBakerViewMode(2);
    pub const ORTHO_LEFT: ENiagaraBakerViewMode = ENiagaraBakerViewMode(3);
    pub const ORTHO_RIGHT: ENiagaraBakerViewMode = ENiagaraBakerViewMode(4);
    pub const ORTHO_TOP: ENiagaraBakerViewMode = ENiagaraBakerViewMode(5);
    pub const ORTHO_BOTTOM: ENiagaraBakerViewMode = ENiagaraBakerViewMode(6);
    pub const NUM: ENiagaraBakerViewMode = ENiagaraBakerViewMode(7);
}
#[repr(transparent)]
pub struct ENiagaraBindingSource(pub u8);
impl ENiagaraBindingSource {
    pub const IMPLICIT_FROM_SOURCE: ENiagaraBindingSource = ENiagaraBindingSource(0);
    pub const EXPLICIT_PARTICLES: ENiagaraBindingSource = ENiagaraBindingSource(1);
    pub const EXPLICIT_EMITTER: ENiagaraBindingSource = ENiagaraBindingSource(2);
    pub const EXPLICIT_SYSTEM: ENiagaraBindingSource = ENiagaraBindingSource(3);
    pub const EXPLICIT_USER: ENiagaraBindingSource = ENiagaraBindingSource(4);
    pub const MAX_BINDING_SOURCE: ENiagaraBindingSource = ENiagaraBindingSource(5);
}
#[repr(transparent)]
pub struct ENiagaraStructConversionType(pub u8);
impl ENiagaraStructConversionType {
    pub const COPY_ONLY: ENiagaraStructConversionType = ENiagaraStructConversionType(0);
    pub const DOUBLE_TO_FLOAT: ENiagaraStructConversionType = ENiagaraStructConversionType(
        1,
    );
    pub const VECTOR2: ENiagaraStructConversionType = ENiagaraStructConversionType(2);
    pub const VECTOR3: ENiagaraStructConversionType = ENiagaraStructConversionType(3);
    pub const VECTOR4: ENiagaraStructConversionType = ENiagaraStructConversionType(4);
    pub const QUAT: ENiagaraStructConversionType = ENiagaraStructConversionType(5);
}
#[repr(transparent)]
pub struct ENiagaraDataInterfaceEmitterBindingMode(pub i32);
impl ENiagaraDataInterfaceEmitterBindingMode {
    pub const SELF: ENiagaraDataInterfaceEmitterBindingMode = ENiagaraDataInterfaceEmitterBindingMode(
        0,
    );
    pub const OTHER: ENiagaraDataInterfaceEmitterBindingMode = ENiagaraDataInterfaceEmitterBindingMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraExecutionState(pub u32);
impl ENiagaraExecutionState {
    pub const ACTIVE: ENiagaraExecutionState = ENiagaraExecutionState(0);
    pub const INACTIVE: ENiagaraExecutionState = ENiagaraExecutionState(1);
    pub const INACTIVE_CLEAR: ENiagaraExecutionState = ENiagaraExecutionState(2);
    pub const COMPLETE: ENiagaraExecutionState = ENiagaraExecutionState(3);
    pub const DISABLED: ENiagaraExecutionState = ENiagaraExecutionState(4);
    pub const NUM: ENiagaraExecutionState = ENiagaraExecutionState(5);
}
#[repr(transparent)]
pub struct ENCPoolMethod(pub u8);
impl ENCPoolMethod {
    pub const NONE: ENCPoolMethod = ENCPoolMethod(0);
    pub const AUTO_RELEASE: ENCPoolMethod = ENCPoolMethod(1);
    pub const MANUAL_RELEASE: ENCPoolMethod = ENCPoolMethod(2);
    pub const MANUAL_RELEASE_ON_COMPLETE: ENCPoolMethod = ENCPoolMethod(3);
    pub const FREE_IN_POOL: ENCPoolMethod = ENCPoolMethod(4);
}
#[repr(transparent)]
pub struct ENiagaraGpuComputeTickStage(pub u8);
impl ENiagaraGpuComputeTickStage {
    pub const PRE_INIT_VIEWS: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(
        0,
    );
    pub const POST_INIT_VIEWS: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(
        1,
    );
    pub const POST_OPAQUE_RENDER: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(
        2,
    );
    pub const MAX: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(3);
    pub const FIRST: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(0);
    pub const LAST: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(2);
}
#[repr(transparent)]
pub struct ENiagaraDebugHudFont(pub i32);
impl ENiagaraDebugHudFont {
    pub const SMALL: ENiagaraDebugHudFont = ENiagaraDebugHudFont(0);
    pub const NORMAL: ENiagaraDebugHudFont = ENiagaraDebugHudFont(1);
}
#[repr(transparent)]
pub struct ENiagaraDebugHudHAlign(pub u8);
impl ENiagaraDebugHudHAlign {
    pub const LEFT: ENiagaraDebugHudHAlign = ENiagaraDebugHudHAlign(0);
    pub const CENTER: ENiagaraDebugHudHAlign = ENiagaraDebugHudHAlign(1);
    pub const RIGHT: ENiagaraDebugHudHAlign = ENiagaraDebugHudHAlign(2);
}
#[repr(transparent)]
pub struct ENiagaraDebugHudVAlign(pub u8);
impl ENiagaraDebugHudVAlign {
    pub const TOP: ENiagaraDebugHudVAlign = ENiagaraDebugHudVAlign(0);
    pub const CENTER: ENiagaraDebugHudVAlign = ENiagaraDebugHudVAlign(1);
    pub const BOTTOM: ENiagaraDebugHudVAlign = ENiagaraDebugHudVAlign(2);
}
#[repr(transparent)]
pub struct ENiagaraDebugHUDOverviewMode(pub i32);
impl ENiagaraDebugHUDOverviewMode {
    pub const OVERVIEW: ENiagaraDebugHUDOverviewMode = ENiagaraDebugHUDOverviewMode(0);
    pub const SCALABILITY: ENiagaraDebugHUDOverviewMode = ENiagaraDebugHUDOverviewMode(
        1,
    );
    pub const PERFORMANCE: ENiagaraDebugHUDOverviewMode = ENiagaraDebugHUDOverviewMode(
        2,
    );
    pub const PERFORMANCE_GRAPH: ENiagaraDebugHUDOverviewMode = ENiagaraDebugHUDOverviewMode(
        3,
    );
    pub const GPU_COMPUTE_PERFORMANCE: ENiagaraDebugHUDOverviewMode = ENiagaraDebugHUDOverviewMode(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraDebugHUDDOverviewSort(pub i32);
impl ENiagaraDebugHUDDOverviewSort {
    pub const NAME: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(0);
    pub const NUMBER_REGISTERED: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(
        1,
    );
    pub const NUMBER_ACTIVE: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(
        2,
    );
    pub const NUMBER_SCALABILITY: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(
        3,
    );
    pub const MEMORY_USAGE: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(
        4,
    );
    pub const RECENTLY_VISIBILTY: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(
        5,
    );
}
#[repr(transparent)]
pub struct ENiagaraDebugHudVerbosity(pub i32);
impl ENiagaraDebugHudVerbosity {
    pub const NONE: ENiagaraDebugHudVerbosity = ENiagaraDebugHudVerbosity(0);
    pub const BASIC: ENiagaraDebugHudVerbosity = ENiagaraDebugHudVerbosity(1);
    pub const VERBOSE: ENiagaraDebugHudVerbosity = ENiagaraDebugHudVerbosity(2);
}
#[repr(transparent)]
pub struct ENiagaraDebugHUDPerfSampleMode(pub i32);
impl ENiagaraDebugHUDPerfSampleMode {
    pub const FRAME_TOTAL: ENiagaraDebugHUDPerfSampleMode = ENiagaraDebugHUDPerfSampleMode(
        0,
    );
    pub const PER_INSTANCE_AVERAGE: ENiagaraDebugHUDPerfSampleMode = ENiagaraDebugHUDPerfSampleMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraDebugHUDPerfUnits(pub i32);
impl ENiagaraDebugHUDPerfUnits {
    pub const MICROSECONDS: ENiagaraDebugHUDPerfUnits = ENiagaraDebugHUDPerfUnits(0);
    pub const MILLISECONDS: ENiagaraDebugHUDPerfUnits = ENiagaraDebugHUDPerfUnits(1);
}
#[repr(transparent)]
pub struct ENiagaraDebugHUDPerfGraphMode(pub i32);
impl ENiagaraDebugHUDPerfGraphMode {
    pub const GAME_THREAD: ENiagaraDebugHUDPerfGraphMode = ENiagaraDebugHUDPerfGraphMode(
        0,
    );
    pub const RENDER_THREAD: ENiagaraDebugHUDPerfGraphMode = ENiagaraDebugHUDPerfGraphMode(
        1,
    );
    pub const GPU: ENiagaraDebugHUDPerfGraphMode = ENiagaraDebugHUDPerfGraphMode(2);
}
#[repr(transparent)]
pub struct ENiagaraDebugPlaybackMode(pub u8);
impl ENiagaraDebugPlaybackMode {
    pub const PLAY: ENiagaraDebugPlaybackMode = ENiagaraDebugPlaybackMode(0);
    pub const LOOP: ENiagaraDebugPlaybackMode = ENiagaraDebugPlaybackMode(1);
    pub const PAUSED: ENiagaraDebugPlaybackMode = ENiagaraDebugPlaybackMode(2);
    pub const STEP: ENiagaraDebugPlaybackMode = ENiagaraDebugPlaybackMode(3);
}
#[repr(transparent)]
pub struct ENiagaraCVarConditionResponse(pub u8);
impl ENiagaraCVarConditionResponse {
    pub const NONE: ENiagaraCVarConditionResponse = ENiagaraCVarConditionResponse(0);
    pub const ENABLE: ENiagaraCVarConditionResponse = ENiagaraCVarConditionResponse(1);
    pub const DISABLE: ENiagaraCVarConditionResponse = ENiagaraCVarConditionResponse(2);
}
#[repr(transparent)]
pub struct ENiagaraCullProxyMode(pub u32);
impl ENiagaraCullProxyMode {
    pub const NONE: ENiagaraCullProxyMode = ENiagaraCullProxyMode(0);
    pub const INSTANCED_RENDERED: ENiagaraCullProxyMode = ENiagaraCullProxyMode(1);
}
#[repr(transparent)]
pub struct EScriptExecutionMode(pub u8);
impl EScriptExecutionMode {
    pub const EVERY_PARTICLE: EScriptExecutionMode = EScriptExecutionMode(0);
    pub const SPAWNED_PARTICLES: EScriptExecutionMode = EScriptExecutionMode(1);
    pub const SINGLE_PARTICLE: EScriptExecutionMode = EScriptExecutionMode(2);
}
#[repr(transparent)]
pub struct ENiagaraPythonUpdateScriptReference(pub u8);
impl ENiagaraPythonUpdateScriptReference {
    pub const NONE: ENiagaraPythonUpdateScriptReference = ENiagaraPythonUpdateScriptReference(
        0,
    );
    pub const SCRIPT_ASSET: ENiagaraPythonUpdateScriptReference = ENiagaraPythonUpdateScriptReference(
        1,
    );
    pub const DIRECT_TEXT_ENTRY: ENiagaraPythonUpdateScriptReference = ENiagaraPythonUpdateScriptReference(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraInterpolatedSpawnMode(pub u8);
impl ENiagaraInterpolatedSpawnMode {
    pub const NO_INTERPOLATION: ENiagaraInterpolatedSpawnMode = ENiagaraInterpolatedSpawnMode(
        0,
    );
    pub const RUN_UPDATE_SCRIPT: ENiagaraInterpolatedSpawnMode = ENiagaraInterpolatedSpawnMode(
        1,
    );
    pub const INTERPOLATION: ENiagaraInterpolatedSpawnMode = ENiagaraInterpolatedSpawnMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraEmitterCalculateBoundMode(pub u8);
impl ENiagaraEmitterCalculateBoundMode {
    pub const DYNAMIC: ENiagaraEmitterCalculateBoundMode = ENiagaraEmitterCalculateBoundMode(
        0,
    );
    pub const FIXED: ENiagaraEmitterCalculateBoundMode = ENiagaraEmitterCalculateBoundMode(
        1,
    );
    pub const PROGRAMMABLE: ENiagaraEmitterCalculateBoundMode = ENiagaraEmitterCalculateBoundMode(
        2,
    );
}
#[repr(transparent)]
pub struct EParticleAllocationMode(pub u8);
impl EParticleAllocationMode {
    pub const AUTOMATIC_ESTIMATE: EParticleAllocationMode = EParticleAllocationMode(0);
    pub const MANUAL_ESTIMATE: EParticleAllocationMode = EParticleAllocationMode(1);
    pub const FIXED_COUNT: EParticleAllocationMode = EParticleAllocationMode(2);
}
#[repr(transparent)]
pub struct ENiagaraEmitterDefaultSummaryState(pub u8);
impl ENiagaraEmitterDefaultSummaryState {
    pub const DEFAULT: ENiagaraEmitterDefaultSummaryState = ENiagaraEmitterDefaultSummaryState(
        0,
    );
    pub const SUMMARY: ENiagaraEmitterDefaultSummaryState = ENiagaraEmitterDefaultSummaryState(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraEmitterMode(pub u8);
impl ENiagaraEmitterMode {
    pub const STANDARD: ENiagaraEmitterMode = ENiagaraEmitterMode(0);
    pub const STATELESS: ENiagaraEmitterMode = ENiagaraEmitterMode(1);
}
#[repr(transparent)]
pub struct ENiagaraDeviceProfileRedirectMode(pub u8);
impl ENiagaraDeviceProfileRedirectMode {
    pub const C_VAR: ENiagaraDeviceProfileRedirectMode = ENiagaraDeviceProfileRedirectMode(
        0,
    );
    pub const DEVICE_PROFILE: ENiagaraDeviceProfileRedirectMode = ENiagaraDeviceProfileRedirectMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraRibbonUVDistributionMode(pub u8);
impl ENiagaraRibbonUVDistributionMode {
    pub const SCALED_UNIFORMLY: ENiagaraRibbonUVDistributionMode = ENiagaraRibbonUVDistributionMode(
        0,
    );
    pub const SCALED_USING_RIBBON_SEGMENT_LENGTH: ENiagaraRibbonUVDistributionMode = ENiagaraRibbonUVDistributionMode(
        1,
    );
    pub const TILED_OVER_RIBBON_LENGTH: ENiagaraRibbonUVDistributionMode = ENiagaraRibbonUVDistributionMode(
        2,
    );
    pub const TILED_FROM_START_OVER_RIBBON_LENGTH: ENiagaraRibbonUVDistributionMode = ENiagaraRibbonUVDistributionMode(
        3,
    );
}
#[repr(transparent)]
pub struct ENiagaraRibbonUVEdgeMode(pub u8);
impl ENiagaraRibbonUVEdgeMode {
    pub const SMOOTH_TRANSITION: ENiagaraRibbonUVEdgeMode = ENiagaraRibbonUVEdgeMode(0);
    pub const LOCKED: ENiagaraRibbonUVEdgeMode = ENiagaraRibbonUVEdgeMode(1);
}
#[repr(transparent)]
pub struct ENiagaraModuleDependencyType(pub u8);
impl ENiagaraModuleDependencyType {
    pub const PRE_DEPENDENCY: ENiagaraModuleDependencyType = ENiagaraModuleDependencyType(
        0,
    );
    pub const POST_DEPENDENCY: ENiagaraModuleDependencyType = ENiagaraModuleDependencyType(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraModuleDependencyScriptConstraint(pub u8);
impl ENiagaraModuleDependencyScriptConstraint {
    pub const SAME_SCRIPT: ENiagaraModuleDependencyScriptConstraint = ENiagaraModuleDependencyScriptConstraint(
        0,
    );
    pub const ALL_SCRIPTS: ENiagaraModuleDependencyScriptConstraint = ENiagaraModuleDependencyScriptConstraint(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraScriptUsage(pub u8);
impl ENiagaraScriptUsage {
    pub const FUNCTION: ENiagaraScriptUsage = ENiagaraScriptUsage(0);
    pub const MODULE: ENiagaraScriptUsage = ENiagaraScriptUsage(1);
    pub const DYNAMIC_INPUT: ENiagaraScriptUsage = ENiagaraScriptUsage(2);
    pub const PARTICLE_SPAWN_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(3);
    pub const PARTICLE_SPAWN_SCRIPT_INTERPOLATED: ENiagaraScriptUsage = ENiagaraScriptUsage(
        4,
    );
    pub const PARTICLE_UPDATE_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(5);
    pub const PARTICLE_EVENT_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(6);
    pub const PARTICLE_SIMULATION_STAGE_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(
        7,
    );
    pub const PARTICLE_GPU_COMPUTE_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(8);
    pub const EMITTER_SPAWN_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(9);
    pub const EMITTER_UPDATE_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(10);
    pub const SYSTEM_SPAWN_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(11);
    pub const SYSTEM_UPDATE_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(12);
}
#[repr(transparent)]
pub struct ENiagaraScriptCompileStatus(pub u8);
impl ENiagaraScriptCompileStatus {
    pub const NCS_UNKNOWN: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(0);
    pub const NCS_DIRTY: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(1);
    pub const NCS_ERROR: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(2);
    pub const NCS_UP_TO_DATE: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(
        3,
    );
    pub const NCS_BEING_CREATED: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(
        4,
    );
    pub const NCS_UP_TO_DATE_WITH_WARNINGS: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(
        5,
    );
    pub const NCS_COMPUTE_UP_TO_DATE_WITH_WARNINGS: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(
        6,
    );
}
#[repr(transparent)]
pub struct ENiagaraInlineDynamicInputFormatTokenUsage(pub i32);
impl ENiagaraInlineDynamicInputFormatTokenUsage {
    pub const INPUT: ENiagaraInlineDynamicInputFormatTokenUsage = ENiagaraInlineDynamicInputFormatTokenUsage(
        0,
    );
    pub const DECORATOR: ENiagaraInlineDynamicInputFormatTokenUsage = ENiagaraInlineDynamicInputFormatTokenUsage(
        1,
    );
    pub const LINE_BREAK: ENiagaraInlineDynamicInputFormatTokenUsage = ENiagaraInlineDynamicInputFormatTokenUsage(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraScriptLibraryVisibility(pub u8);
impl ENiagaraScriptLibraryVisibility {
    pub const INVALID: ENiagaraScriptLibraryVisibility = ENiagaraScriptLibraryVisibility(
        0,
    );
    pub const UNEXPOSED: ENiagaraScriptLibraryVisibility = ENiagaraScriptLibraryVisibility(
        1,
    );
    pub const LIBRARY: ENiagaraScriptLibraryVisibility = ENiagaraScriptLibraryVisibility(
        2,
    );
    pub const HIDDEN: ENiagaraScriptLibraryVisibility = ENiagaraScriptLibraryVisibility(
        3,
    );
}
#[repr(transparent)]
pub struct ENiagaraNumericOutputTypeSelectionMode(pub u8);
impl ENiagaraNumericOutputTypeSelectionMode {
    pub const NONE: ENiagaraNumericOutputTypeSelectionMode = ENiagaraNumericOutputTypeSelectionMode(
        0,
    );
    pub const LARGEST: ENiagaraNumericOutputTypeSelectionMode = ENiagaraNumericOutputTypeSelectionMode(
        1,
    );
    pub const SMALLEST: ENiagaraNumericOutputTypeSelectionMode = ENiagaraNumericOutputTypeSelectionMode(
        2,
    );
    pub const SCALAR: ENiagaraNumericOutputTypeSelectionMode = ENiagaraNumericOutputTypeSelectionMode(
        3,
    );
    pub const CUSTOM: ENiagaraNumericOutputTypeSelectionMode = ENiagaraNumericOutputTypeSelectionMode(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraSystemInactiveResponse(pub u8);
impl ENiagaraSystemInactiveResponse {
    pub const COMPLETE: ENiagaraSystemInactiveResponse = ENiagaraSystemInactiveResponse(
        0,
    );
    pub const KILL: ENiagaraSystemInactiveResponse = ENiagaraSystemInactiveResponse(1);
}
#[repr(transparent)]
pub struct ENiagaraLoopBehavior(pub u8);
impl ENiagaraLoopBehavior {
    pub const INFINITE: ENiagaraLoopBehavior = ENiagaraLoopBehavior(0);
    pub const MULTIPLE: ENiagaraLoopBehavior = ENiagaraLoopBehavior(1);
    pub const ONCE: ENiagaraLoopBehavior = ENiagaraLoopBehavior(2);
}
#[repr(transparent)]
pub struct ENiagaraDistributionMode(pub u8);
impl ENiagaraDistributionMode {
    pub const ARRAY: ENiagaraDistributionMode = ENiagaraDistributionMode(0);
    pub const BINDING: ENiagaraDistributionMode = ENiagaraDistributionMode(1);
    pub const EXPRESSION: ENiagaraDistributionMode = ENiagaraDistributionMode(2);
    pub const UNIFORM_CONSTANT: ENiagaraDistributionMode = ENiagaraDistributionMode(3);
    pub const NON_UNIFORM_CONSTANT: ENiagaraDistributionMode = ENiagaraDistributionMode(
        4,
    );
    pub const UNIFORM_RANGE: ENiagaraDistributionMode = ENiagaraDistributionMode(5);
    pub const NON_UNIFORM_RANGE: ENiagaraDistributionMode = ENiagaraDistributionMode(6);
    pub const UNIFORM_CURVE: ENiagaraDistributionMode = ENiagaraDistributionMode(7);
    pub const NON_UNIFORM_CURVE: ENiagaraDistributionMode = ENiagaraDistributionMode(8);
    pub const COLOR_GRADIENT: ENiagaraDistributionMode = ENiagaraDistributionMode(9);
}
#[repr(transparent)]
pub struct ENiagaraEmitterInactiveResponse(pub u8);
impl ENiagaraEmitterInactiveResponse {
    pub const COMPLETE: ENiagaraEmitterInactiveResponse = ENiagaraEmitterInactiveResponse(
        0,
    );
    pub const KILL: ENiagaraEmitterInactiveResponse = ENiagaraEmitterInactiveResponse(1);
}
#[repr(transparent)]
pub struct ENiagaraLoopDurationMode(pub u8);
impl ENiagaraLoopDurationMode {
    pub const FIXED: ENiagaraLoopDurationMode = ENiagaraLoopDurationMode(0);
    pub const INFINITE: ENiagaraLoopDurationMode = ENiagaraLoopDurationMode(1);
}
#[repr(transparent)]
pub struct ENiagaraExecutionStateManagement(pub u32);
impl ENiagaraExecutionStateManagement {
    pub const AWAKEN: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        0,
    );
    pub const SLEEP_AND_LET_PARTICLES_FINISH: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        1,
    );
    pub const SLEEP_AND_CLEAR_PARTICLES: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        2,
    );
    pub const KILL_IMMEDIATELY: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        3,
    );
    pub const KILL_AFTER_PARTICLES_FINISH: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        4,
    );
    pub const NUM: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        5,
    );
}
#[repr(transparent)]
pub struct ENiagaraInputWidgetType(pub u8);
impl ENiagaraInputWidgetType {
    pub const DEFAULT: ENiagaraInputWidgetType = ENiagaraInputWidgetType(0);
    pub const SLIDER: ENiagaraInputWidgetType = ENiagaraInputWidgetType(1);
    pub const VOLUME: ENiagaraInputWidgetType = ENiagaraInputWidgetType(2);
    pub const NUMERIC_DROPDOWN: ENiagaraInputWidgetType = ENiagaraInputWidgetType(3);
    pub const ENUM_STYLE: ENiagaraInputWidgetType = ENiagaraInputWidgetType(4);
    pub const SEGMENTED_BUTTONS: ENiagaraInputWidgetType = ENiagaraInputWidgetType(5);
}
#[repr(transparent)]
pub struct ENiagaraBoolDisplayMode(pub u8);
impl ENiagaraBoolDisplayMode {
    pub const DISPLAY_ALWAYS: ENiagaraBoolDisplayMode = ENiagaraBoolDisplayMode(0);
    pub const DISPLAY_IF_TRUE: ENiagaraBoolDisplayMode = ENiagaraBoolDisplayMode(1);
    pub const DISPLAY_IF_FALSE: ENiagaraBoolDisplayMode = ENiagaraBoolDisplayMode(2);
}
#[repr(transparent)]
pub struct ENiagaraVariantMode(pub i32);
impl ENiagaraVariantMode {
    pub const NONE: ENiagaraVariantMode = ENiagaraVariantMode(0);
    pub const OBJECT: ENiagaraVariantMode = ENiagaraVariantMode(1);
    pub const DATA_INTERFACE: ENiagaraVariantMode = ENiagaraVariantMode(2);
    pub const BYTES: ENiagaraVariantMode = ENiagaraVariantMode(3);
}
#[repr(transparent)]
pub struct ENiagaraStatelessSpawnInfoType(pub i32);
impl ENiagaraStatelessSpawnInfoType {
    pub const BURST: ENiagaraStatelessSpawnInfoType = ENiagaraStatelessSpawnInfoType(0);
    pub const RATE: ENiagaraStatelessSpawnInfoType = ENiagaraStatelessSpawnInfoType(1);
}
#[repr(transparent)]
pub struct ENiagaraDefaultMode(pub u8);
impl ENiagaraDefaultMode {
    pub const VALUE: ENiagaraDefaultMode = ENiagaraDefaultMode(0);
    pub const BINDING: ENiagaraDefaultMode = ENiagaraDefaultMode(1);
    pub const CUSTOM: ENiagaraDefaultMode = ENiagaraDefaultMode(2);
    pub const FAIL_IF_PREVIOUSLY_NOT_SET: ENiagaraDefaultMode = ENiagaraDefaultMode(3);
}
#[repr(transparent)]
pub struct ENiagaraOcclusionQueryMode(pub u8);
impl ENiagaraOcclusionQueryMode {
    pub const DEFAULT: ENiagaraOcclusionQueryMode = ENiagaraOcclusionQueryMode(0);
    pub const ALWAYS_ENABLED: ENiagaraOcclusionQueryMode = ENiagaraOcclusionQueryMode(1);
    pub const ALWAYS_DISABLED: ENiagaraOcclusionQueryMode = ENiagaraOcclusionQueryMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraTickBehavior(pub u8);
impl ENiagaraTickBehavior {
    pub const USE_PREREQS: ENiagaraTickBehavior = ENiagaraTickBehavior(0);
    pub const USE_COMPONENT_TICK_GROUP: ENiagaraTickBehavior = ENiagaraTickBehavior(1);
    pub const FORCE_TICK_FIRST: ENiagaraTickBehavior = ENiagaraTickBehavior(2);
    pub const FORCE_TICK_LAST: ENiagaraTickBehavior = ENiagaraTickBehavior(3);
}
#[repr(transparent)]
pub struct ENiagartaDataChannelReadResult(pub u8);
impl ENiagartaDataChannelReadResult {
    pub const SUCCESS: ENiagartaDataChannelReadResult = ENiagartaDataChannelReadResult(
        0,
    );
    pub const FAILURE: ENiagartaDataChannelReadResult = ENiagartaDataChannelReadResult(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraScriptTemplateSpecification(pub u8);
impl ENiagaraScriptTemplateSpecification {
    pub const NONE: ENiagaraScriptTemplateSpecification = ENiagaraScriptTemplateSpecification(
        0,
    );
    pub const TEMPLATE: ENiagaraScriptTemplateSpecification = ENiagaraScriptTemplateSpecification(
        1,
    );
    pub const BEHAVIOR: ENiagaraScriptTemplateSpecification = ENiagaraScriptTemplateSpecification(
        2,
    );
}
#[repr(transparent)]
pub struct ENDIActorComponentSourceMode(pub u8);
impl ENDIActorComponentSourceMode {
    pub const DEFAULT: ENDIActorComponentSourceMode = ENDIActorComponentSourceMode(0);
    pub const ATTACH_PARENT: ENDIActorComponentSourceMode = ENDIActorComponentSourceMode(
        1,
    );
    pub const LOCAL_PLAYER: ENDIActorComponentSourceMode = ENDIActorComponentSourceMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraGpuSyncMode(pub u8);
impl ENiagaraGpuSyncMode {
    pub const NONE: ENiagaraGpuSyncMode = ENiagaraGpuSyncMode(0);
    pub const SYNC_CPU_TO_GPU: ENiagaraGpuSyncMode = ENiagaraGpuSyncMode(1);
    pub const SYNC_GPU_TO_CPU: ENiagaraGpuSyncMode = ENiagaraGpuSyncMode(2);
    pub const SYNC_BOTH: ENiagaraGpuSyncMode = ENiagaraGpuSyncMode(3);
}
#[repr(transparent)]
pub struct ENDICollisionQuery_AsyncGpuTraceProvider(pub u8);
impl ENDICollisionQuery_AsyncGpuTraceProvider {
    pub const DEFAULT: ENDICollisionQuery_AsyncGpuTraceProvider = ENDICollisionQuery_AsyncGpuTraceProvider(
        0,
    );
    pub const HWRT: ENDICollisionQuery_AsyncGpuTraceProvider = ENDICollisionQuery_AsyncGpuTraceProvider(
        1,
    );
    pub const GSDF: ENDICollisionQuery_AsyncGpuTraceProvider = ENDICollisionQuery_AsyncGpuTraceProvider(
        2,
    );
    pub const NONE: ENDICollisionQuery_AsyncGpuTraceProvider = ENDICollisionQuery_AsyncGpuTraceProvider(
        3,
    );
}
#[repr(transparent)]
pub struct ENiagaraDataChannelAllocationMode(pub u8);
impl ENiagaraDataChannelAllocationMode {
    pub const STATIC: ENiagaraDataChannelAllocationMode = ENiagaraDataChannelAllocationMode(
        0,
    );
    pub const DYNAMIC: ENiagaraDataChannelAllocationMode = ENiagaraDataChannelAllocationMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENDISceneCapture2DSourceMode(pub u8);
impl ENDISceneCapture2DSourceMode {
    pub const USER_PARAMETER_THEN_ATTACH_PARENT: ENDISceneCapture2DSourceMode = ENDISceneCapture2DSourceMode(
        0,
    );
    pub const USER_PARAMETER_ONLY: ENDISceneCapture2DSourceMode = ENDISceneCapture2DSourceMode(
        1,
    );
    pub const ATTACH_PARENT_ONLY: ENDISceneCapture2DSourceMode = ENDISceneCapture2DSourceMode(
        2,
    );
    pub const MANAGED: ENDISceneCapture2DSourceMode = ENDISceneCapture2DSourceMode(3);
}
#[repr(transparent)]
pub struct ENDISceneCapture2DOffsetMode(pub u8);
impl ENDISceneCapture2DOffsetMode {
    pub const DISABLED: ENDISceneCapture2DOffsetMode = ENDISceneCapture2DOffsetMode(0);
    pub const RELATIVE_LOCAL: ENDISceneCapture2DOffsetMode = ENDISceneCapture2DOffsetMode(
        1,
    );
    pub const RELATIVE_WORLD: ENDISceneCapture2DOffsetMode = ENDISceneCapture2DOffsetMode(
        2,
    );
    pub const ABSOLUTE_WORLD: ENDISceneCapture2DOffsetMode = ENDISceneCapture2DOffsetMode(
        3,
    );
}
#[repr(transparent)]
pub struct ENDISocketReaderSourceMode(pub u8);
impl ENDISocketReaderSourceMode {
    pub const DEFAULT: ENDISocketReaderSourceMode = ENDISocketReaderSourceMode(0);
    pub const PARAMETER_BINDING_ONLY: ENDISocketReaderSourceMode = ENDISocketReaderSourceMode(
        1,
    );
    pub const ATTACHED_PARENT_ONLY: ENDISocketReaderSourceMode = ENDISocketReaderSourceMode(
        2,
    );
    pub const SOURCE_ONLY: ENDISocketReaderSourceMode = ENDISocketReaderSourceMode(3);
}
#[repr(transparent)]
pub struct ENDIStaticMesh_SourceMode(pub u8);
impl ENDIStaticMesh_SourceMode {
    pub const DEFAULT: ENDIStaticMesh_SourceMode = ENDIStaticMesh_SourceMode(0);
    pub const SOURCE: ENDIStaticMesh_SourceMode = ENDIStaticMesh_SourceMode(1);
    pub const ATTACH_PARENT: ENDIStaticMesh_SourceMode = ENDIStaticMesh_SourceMode(2);
    pub const DEFAULT_MESH_ONLY: ENDIStaticMesh_SourceMode = ENDIStaticMesh_SourceMode(
        3,
    );
    pub const MESH_PARAMETER_BINDING: ENDIStaticMesh_SourceMode = ENDIStaticMesh_SourceMode(
        4,
    );
}
#[repr(transparent)]
pub struct ENDIObjectPropertyReaderSourceMode(pub u8);
impl ENDIObjectPropertyReaderSourceMode {
    pub const BINDING: ENDIObjectPropertyReaderSourceMode = ENDIObjectPropertyReaderSourceMode(
        0,
    );
    pub const ATTACH_PARENT_ACTOR: ENDIObjectPropertyReaderSourceMode = ENDIObjectPropertyReaderSourceMode(
        1,
    );
    pub const BINDING_THEN_ATTACH_PARENT_ACTOR: ENDIObjectPropertyReaderSourceMode = ENDIObjectPropertyReaderSourceMode(
        2,
    );
    pub const ATTACH_PARENT: ENDIObjectPropertyReaderSourceMode = ENDIObjectPropertyReaderSourceMode(
        3,
    );
    pub const BINDING_THEN_ATTACH_PARENT: ENDIObjectPropertyReaderSourceMode = ENDIObjectPropertyReaderSourceMode(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraRendererMotionVectorSetting(pub u8);
impl ENiagaraRendererMotionVectorSetting {
    pub const AUTO_DETECT: ENiagaraRendererMotionVectorSetting = ENiagaraRendererMotionVectorSetting(
        0,
    );
    pub const PRECISE: ENiagaraRendererMotionVectorSetting = ENiagaraRendererMotionVectorSetting(
        1,
    );
    pub const APPROXIMATE: ENiagaraRendererMotionVectorSetting = ENiagaraRendererMotionVectorSetting(
        2,
    );
    pub const DISABLE: ENiagaraRendererMotionVectorSetting = ENiagaraRendererMotionVectorSetting(
        3,
    );
}
#[repr(transparent)]
pub struct ENiagraDataChannel_IslandMode(pub u8);
impl ENiagraDataChannel_IslandMode {
    pub const ALIGNED_STATIC: ENiagraDataChannel_IslandMode = ENiagraDataChannel_IslandMode(
        0,
    );
    pub const DYNAMIC: ENiagraDataChannel_IslandMode = ENiagraDataChannel_IslandMode(1);
}
#[repr(transparent)]
pub struct ENDIExport_GPUAllocationMode(pub u8);
impl ENDIExport_GPUAllocationMode {
    pub const FIXED_SIZE: ENDIExport_GPUAllocationMode = ENDIExport_GPUAllocationMode(0);
    pub const PER_PARTICLE: ENDIExport_GPUAllocationMode = ENDIExport_GPUAllocationMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraGpuBufferFormat(pub u8);
impl ENiagaraGpuBufferFormat {
    pub const FLOAT: ENiagaraGpuBufferFormat = ENiagaraGpuBufferFormat(0);
    pub const HALF_FLOAT: ENiagaraGpuBufferFormat = ENiagaraGpuBufferFormat(1);
    pub const UNSIGNED_NORMALIZED_BYTE: ENiagaraGpuBufferFormat = ENiagaraGpuBufferFormat(
        2,
    );
    pub const MAX: ENiagaraGpuBufferFormat = ENiagaraGpuBufferFormat(3);
}
#[repr(transparent)]
pub struct ESetResolutionMethod(pub i32);
impl ESetResolutionMethod {
    pub const INDEPENDENT: ESetResolutionMethod = ESetResolutionMethod(0);
    pub const MAX_AXIS: ESetResolutionMethod = ESetResolutionMethod(1);
    pub const CELL_SIZE: ESetResolutionMethod = ESetResolutionMethod(2);
}
#[repr(transparent)]
pub struct ENDILandscape_SourceMode(pub u8);
impl ENDILandscape_SourceMode {
    pub const DEFAULT: ENDILandscape_SourceMode = ENDILandscape_SourceMode(0);
    pub const SOURCE: ENDILandscape_SourceMode = ENDILandscape_SourceMode(1);
    pub const ATTACH_PARENT: ENDILandscape_SourceMode = ENDILandscape_SourceMode(2);
}
#[repr(transparent)]
pub struct ENiagaraMipMapGeneration(pub u8);
impl ENiagaraMipMapGeneration {
    pub const DISABLED: ENiagaraMipMapGeneration = ENiagaraMipMapGeneration(0);
    pub const POST_STAGE: ENiagaraMipMapGeneration = ENiagaraMipMapGeneration(1);
    pub const POST_SIMULATE: ENiagaraMipMapGeneration = ENiagaraMipMapGeneration(2);
}
#[repr(transparent)]
pub struct ENDISkeletalMesh_SourceMode(pub u8);
impl ENDISkeletalMesh_SourceMode {
    pub const DEFAULT: ENDISkeletalMesh_SourceMode = ENDISkeletalMesh_SourceMode(0);
    pub const SOURCE: ENDISkeletalMesh_SourceMode = ENDISkeletalMesh_SourceMode(1);
    pub const ATTACH_PARENT: ENDISkeletalMesh_SourceMode = ENDISkeletalMesh_SourceMode(
        2,
    );
    pub const DEFAULT_MESH_ONLY: ENDISkeletalMesh_SourceMode = ENDISkeletalMesh_SourceMode(
        3,
    );
}
#[repr(transparent)]
pub struct ENDISkeletalMesh_SkinningMode(pub u8);
impl ENDISkeletalMesh_SkinningMode {
    pub const INVALID: ENDISkeletalMesh_SkinningMode = ENDISkeletalMesh_SkinningMode(
        255,
    );
    pub const NONE: ENDISkeletalMesh_SkinningMode = ENDISkeletalMesh_SkinningMode(0);
    pub const SKIN_ON_THE_FLY: ENDISkeletalMesh_SkinningMode = ENDISkeletalMesh_SkinningMode(
        1,
    );
    pub const PRE_SKIN: ENDISkeletalMesh_SkinningMode = ENDISkeletalMesh_SkinningMode(2);
}
#[repr(transparent)]
pub struct ENiagaraRendererSourceDataMode(pub u8);
impl ENiagaraRendererSourceDataMode {
    pub const PARTICLES: ENiagaraRendererSourceDataMode = ENiagaraRendererSourceDataMode(
        0,
    );
    pub const EMITTER: ENiagaraRendererSourceDataMode = ENiagaraRendererSourceDataMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraEditorPreviewActorPlaybackType(pub i32);
impl ENiagaraEditorPreviewActorPlaybackType {
    pub const ONCE: ENiagaraEditorPreviewActorPlaybackType = ENiagaraEditorPreviewActorPlaybackType(
        0,
    );
    pub const LOOPING: ENiagaraEditorPreviewActorPlaybackType = ENiagaraEditorPreviewActorPlaybackType(
        1,
    );
    pub const PING_PONG: ENiagaraEditorPreviewActorPlaybackType = ENiagaraEditorPreviewActorPlaybackType(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraEditorPreviewActorShapeType(pub i32);
impl ENiagaraEditorPreviewActorShapeType {
    pub const CIRCLE: ENiagaraEditorPreviewActorShapeType = ENiagaraEditorPreviewActorShapeType(
        0,
    );
    pub const SQUARE: ENiagaraEditorPreviewActorShapeType = ENiagaraEditorPreviewActorShapeType(
        1,
    );
    pub const TRIANGLE: ENiagaraEditorPreviewActorShapeType = ENiagaraEditorPreviewActorShapeType(
        2,
    );
    pub const CUSTOM: ENiagaraEditorPreviewActorShapeType = ENiagaraEditorPreviewActorShapeType(
        3,
    );
    pub const BLUEPRINT: ENiagaraEditorPreviewActorShapeType = ENiagaraEditorPreviewActorShapeType(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraEditorPreviewActorRotationMode(pub i32);
impl ENiagaraEditorPreviewActorRotationMode {
    pub const NONE: ENiagaraEditorPreviewActorRotationMode = ENiagaraEditorPreviewActorRotationMode(
        0,
    );
    pub const DIRECTION_OF_TRAVEL: ENiagaraEditorPreviewActorRotationMode = ENiagaraEditorPreviewActorRotationMode(
        1,
    );
    pub const BLUEPRINT: ENiagaraEditorPreviewActorRotationMode = ENiagaraEditorPreviewActorRotationMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraScalabilityUpdateFrequency(pub i32);
impl ENiagaraScalabilityUpdateFrequency {
    pub const SPAWN_ONLY: ENiagaraScalabilityUpdateFrequency = ENiagaraScalabilityUpdateFrequency(
        0,
    );
    pub const LOW: ENiagaraScalabilityUpdateFrequency = ENiagaraScalabilityUpdateFrequency(
        1,
    );
    pub const MEDIUM: ENiagaraScalabilityUpdateFrequency = ENiagaraScalabilityUpdateFrequency(
        2,
    );
    pub const HIGH: ENiagaraScalabilityUpdateFrequency = ENiagaraScalabilityUpdateFrequency(
        3,
    );
    pub const CONTINUOUS: ENiagaraScalabilityUpdateFrequency = ENiagaraScalabilityUpdateFrequency(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraCullReaction(pub i32);
impl ENiagaraCullReaction {
    pub const DEACTIVATE: ENiagaraCullReaction = ENiagaraCullReaction(0);
    pub const DEACTIVATE_IMMEDIATE: ENiagaraCullReaction = ENiagaraCullReaction(1);
    pub const DEACTIVATE_RESUME: ENiagaraCullReaction = ENiagaraCullReaction(2);
    pub const DEACTIVATE_IMMEDIATE_RESUME: ENiagaraCullReaction = ENiagaraCullReaction(
        3,
    );
    pub const PAUSE_RESUME: ENiagaraCullReaction = ENiagaraCullReaction(4);
}
#[repr(transparent)]
pub struct ENiagaraSortMode(pub u8);
impl ENiagaraSortMode {
    pub const NONE: ENiagaraSortMode = ENiagaraSortMode(0);
    pub const VIEW_DEPTH: ENiagaraSortMode = ENiagaraSortMode(1);
    pub const VIEW_DISTANCE: ENiagaraSortMode = ENiagaraSortMode(2);
    pub const CUSTOM_ASCENDING: ENiagaraSortMode = ENiagaraSortMode(3);
    pub const CUSTOM_DECENDING: ENiagaraSortMode = ENiagaraSortMode(4);
}
#[repr(transparent)]
pub struct ENiagaraRendererSortPrecision(pub u8);
impl ENiagaraRendererSortPrecision {
    pub const DEFAULT: ENiagaraRendererSortPrecision = ENiagaraRendererSortPrecision(0);
    pub const LOW: ENiagaraRendererSortPrecision = ENiagaraRendererSortPrecision(1);
    pub const HIGH: ENiagaraRendererSortPrecision = ENiagaraRendererSortPrecision(2);
}
#[repr(transparent)]
pub struct ENiagaraRendererGpuTranslucentLatency(pub u8);
impl ENiagaraRendererGpuTranslucentLatency {
    pub const PROJECT_DEFAULT: ENiagaraRendererGpuTranslucentLatency = ENiagaraRendererGpuTranslucentLatency(
        0,
    );
    pub const IMMEDIATE: ENiagaraRendererGpuTranslucentLatency = ENiagaraRendererGpuTranslucentLatency(
        1,
    );
    pub const LATENT: ENiagaraRendererGpuTranslucentLatency = ENiagaraRendererGpuTranslucentLatency(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraMeshFacingMode(pub u8);
impl ENiagaraMeshFacingMode {
    pub const DEFAULT: ENiagaraMeshFacingMode = ENiagaraMeshFacingMode(0);
    pub const VELOCITY: ENiagaraMeshFacingMode = ENiagaraMeshFacingMode(1);
    pub const CAMERA_POSITION: ENiagaraMeshFacingMode = ENiagaraMeshFacingMode(2);
    pub const CAMERA_PLANE: ENiagaraMeshFacingMode = ENiagaraMeshFacingMode(3);
}
#[repr(transparent)]
pub struct ENiagaraMeshLockedAxisSpace(pub u8);
impl ENiagaraMeshLockedAxisSpace {
    pub const SIMULATION: ENiagaraMeshLockedAxisSpace = ENiagaraMeshLockedAxisSpace(0);
    pub const WORLD: ENiagaraMeshLockedAxisSpace = ENiagaraMeshLockedAxisSpace(1);
    pub const LOCAL: ENiagaraMeshLockedAxisSpace = ENiagaraMeshLockedAxisSpace(2);
}
#[repr(transparent)]
pub struct ENiagaraPreviewGridResetMode(pub u8);
impl ENiagaraPreviewGridResetMode {
    pub const NEVER: ENiagaraPreviewGridResetMode = ENiagaraPreviewGridResetMode(0);
    pub const INDIVIDUAL: ENiagaraPreviewGridResetMode = ENiagaraPreviewGridResetMode(1);
    pub const ALL: ENiagaraPreviewGridResetMode = ENiagaraPreviewGridResetMode(2);
}
#[repr(transparent)]
pub struct ENiagaraRibbonFacingMode(pub u8);
impl ENiagaraRibbonFacingMode {
    pub const SCREEN: ENiagaraRibbonFacingMode = ENiagaraRibbonFacingMode(0);
    pub const CUSTOM: ENiagaraRibbonFacingMode = ENiagaraRibbonFacingMode(1);
    pub const CUSTOM_SIDE_VECTOR: ENiagaraRibbonFacingMode = ENiagaraRibbonFacingMode(2);
}
#[repr(transparent)]
pub struct ENiagaraRibbonAgeOffsetMode(pub u8);
impl ENiagaraRibbonAgeOffsetMode {
    pub const SCALE: ENiagaraRibbonAgeOffsetMode = ENiagaraRibbonAgeOffsetMode(0);
    pub const CLIP: ENiagaraRibbonAgeOffsetMode = ENiagaraRibbonAgeOffsetMode(1);
}
#[repr(transparent)]
pub struct ENiagaraRibbonDrawDirection(pub u8);
impl ENiagaraRibbonDrawDirection {
    pub const FRONT_TO_BACK: ENiagaraRibbonDrawDirection = ENiagaraRibbonDrawDirection(
        0,
    );
    pub const BACK_TO_FRONT: ENiagaraRibbonDrawDirection = ENiagaraRibbonDrawDirection(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraRibbonShapeMode(pub u8);
impl ENiagaraRibbonShapeMode {
    pub const PLANE: ENiagaraRibbonShapeMode = ENiagaraRibbonShapeMode(0);
    pub const MULTI_PLANE: ENiagaraRibbonShapeMode = ENiagaraRibbonShapeMode(1);
    pub const TUBE: ENiagaraRibbonShapeMode = ENiagaraRibbonShapeMode(2);
    pub const CUSTOM: ENiagaraRibbonShapeMode = ENiagaraRibbonShapeMode(3);
}
#[repr(transparent)]
pub struct ENiagaraRibbonTessellationMode(pub u8);
impl ENiagaraRibbonTessellationMode {
    pub const AUTOMATIC: ENiagaraRibbonTessellationMode = ENiagaraRibbonTessellationMode(
        0,
    );
    pub const CUSTOM: ENiagaraRibbonTessellationMode = ENiagaraRibbonTessellationMode(1);
    pub const DISABLED: ENiagaraRibbonTessellationMode = ENiagaraRibbonTessellationMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraLwcTileUpdateMode(pub u8);
impl ENiagaraLwcTileUpdateMode {
    pub const RESET_SIMULATION: ENiagaraLwcTileUpdateMode = ENiagaraLwcTileUpdateMode(0);
    pub const REBASE: ENiagaraLwcTileUpdateMode = ENiagaraLwcTileUpdateMode(1);
    pub const REBASE_OR_RESET_SIMULATION: ENiagaraLwcTileUpdateMode = ENiagaraLwcTileUpdateMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraCompileErrorSeverity(pub u8);
impl ENiagaraCompileErrorSeverity {
    pub const IGNORE: ENiagaraCompileErrorSeverity = ENiagaraCompileErrorSeverity(0);
    pub const LOG_ONLY: ENiagaraCompileErrorSeverity = ENiagaraCompileErrorSeverity(1);
    pub const WARNING: ENiagaraCompileErrorSeverity = ENiagaraCompileErrorSeverity(2);
    pub const ERROR: ENiagaraCompileErrorSeverity = ENiagaraCompileErrorSeverity(3);
}
#[repr(transparent)]
pub struct ENiagaraStripScriptByteCodeOption(pub u8);
impl ENiagaraStripScriptByteCodeOption {
    pub const DEFAULT: ENiagaraStripScriptByteCodeOption = ENiagaraStripScriptByteCodeOption(
        0,
    );
    pub const STRIP_ORIGINAL: ENiagaraStripScriptByteCodeOption = ENiagaraStripScriptByteCodeOption(
        1,
    );
    pub const STRIP_EXPERIMENTAL: ENiagaraStripScriptByteCodeOption = ENiagaraStripScriptByteCodeOption(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraCompilationMode(pub i32);
impl ENiagaraCompilationMode {
    pub const ORIGINAL: ENiagaraCompilationMode = ENiagaraCompilationMode(0);
    pub const ASYNC_TASKS: ENiagaraCompilationMode = ENiagaraCompilationMode(1);
    pub const VERIFY: ENiagaraCompilationMode = ENiagaraCompilationMode(2);
}
#[repr(transparent)]
pub struct ENiagaraDefaultRendererMotionVectorSetting(pub u8);
impl ENiagaraDefaultRendererMotionVectorSetting {
    pub const PRECISE: ENiagaraDefaultRendererMotionVectorSetting = ENiagaraDefaultRendererMotionVectorSetting(
        0,
    );
    pub const APPROXIMATE: ENiagaraDefaultRendererMotionVectorSetting = ENiagaraDefaultRendererMotionVectorSetting(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraDefaultRendererPixelCoverageMode(pub u8);
impl ENiagaraDefaultRendererPixelCoverageMode {
    pub const ENABLED: ENiagaraDefaultRendererPixelCoverageMode = ENiagaraDefaultRendererPixelCoverageMode(
        0,
    );
    pub const DISABLED: ENiagaraDefaultRendererPixelCoverageMode = ENiagaraDefaultRendererPixelCoverageMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraDefaultSortPrecision(pub u8);
impl ENiagaraDefaultSortPrecision {
    pub const LOW: ENiagaraDefaultSortPrecision = ENiagaraDefaultSortPrecision(0);
    pub const HIGH: ENiagaraDefaultSortPrecision = ENiagaraDefaultSortPrecision(1);
}
#[repr(transparent)]
pub struct ENiagaraDefaultGpuTranslucentLatency(pub u8);
impl ENiagaraDefaultGpuTranslucentLatency {
    pub const IMMEDIATE: ENiagaraDefaultGpuTranslucentLatency = ENiagaraDefaultGpuTranslucentLatency(
        0,
    );
    pub const LATENT: ENiagaraDefaultGpuTranslucentLatency = ENiagaraDefaultGpuTranslucentLatency(
        1,
    );
}
#[repr(transparent)]
pub struct ENDISkelMesh_GpuMaxInfluences(pub u8);
impl ENDISkelMesh_GpuMaxInfluences {
    pub const ALLOW_MAX4: ENDISkelMesh_GpuMaxInfluences = ENDISkelMesh_GpuMaxInfluences(
        0,
    );
    pub const ALLOW_MAX8: ENDISkelMesh_GpuMaxInfluences = ENDISkelMesh_GpuMaxInfluences(
        1,
    );
    pub const UNLIMITED: ENDISkelMesh_GpuMaxInfluences = ENDISkelMesh_GpuMaxInfluences(
        2,
    );
}
#[repr(transparent)]
pub struct ENDISkelMesh_GpuUniformSamplingFormat(pub u8);
impl ENDISkelMesh_GpuUniformSamplingFormat {
    pub const FULL: ENDISkelMesh_GpuUniformSamplingFormat = ENDISkelMesh_GpuUniformSamplingFormat(
        0,
    );
    pub const LIMITED_24_8: ENDISkelMesh_GpuUniformSamplingFormat = ENDISkelMesh_GpuUniformSamplingFormat(
        1,
    );
    pub const LIMITED_23_9: ENDISkelMesh_GpuUniformSamplingFormat = ENDISkelMesh_GpuUniformSamplingFormat(
        2,
    );
}
#[repr(transparent)]
pub struct ENDISkelMesh_AdjacencyTriangleIndexFormat(pub u8);
impl ENDISkelMesh_AdjacencyTriangleIndexFormat {
    pub const FULL: ENDISkelMesh_AdjacencyTriangleIndexFormat = ENDISkelMesh_AdjacencyTriangleIndexFormat(
        0,
    );
    pub const HALF: ENDISkelMesh_AdjacencyTriangleIndexFormat = ENDISkelMesh_AdjacencyTriangleIndexFormat(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraSpriteAlignment(pub u8);
impl ENiagaraSpriteAlignment {
    pub const UNALIGNED: ENiagaraSpriteAlignment = ENiagaraSpriteAlignment(0);
    pub const VELOCITY_ALIGNED: ENiagaraSpriteAlignment = ENiagaraSpriteAlignment(1);
    pub const CUSTOM_ALIGNMENT: ENiagaraSpriteAlignment = ENiagaraSpriteAlignment(2);
    pub const AUTOMATIC: ENiagaraSpriteAlignment = ENiagaraSpriteAlignment(3);
}
#[repr(transparent)]
pub struct ENiagaraSpriteFacingMode(pub u8);
impl ENiagaraSpriteFacingMode {
    pub const FACE_CAMERA: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(0);
    pub const FACE_CAMERA_PLANE: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(1);
    pub const CUSTOM_FACING_VECTOR: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(
        2,
    );
    pub const FACE_CAMERA_POSITION: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(
        3,
    );
    pub const FACE_CAMERA_DISTANCE_BLEND: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(
        4,
    );
    pub const AUTOMATIC: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(5);
}
#[repr(transparent)]
pub struct ENiagaraRendererPixelCoverageMode(pub u8);
impl ENiagaraRendererPixelCoverageMode {
    pub const AUTOMATIC: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        0,
    );
    pub const DISABLED: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        1,
    );
    pub const ENABLED: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        2,
    );
    pub const ENABLED_RGBA: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        3,
    );
    pub const ENABLED_RGB: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        4,
    );
    pub const ENABLED_A: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        5,
    );
}
#[repr(transparent)]
pub struct ENiagaraCoordinateSpace(pub u32);
impl ENiagaraCoordinateSpace {
    pub const SIMULATION: ENiagaraCoordinateSpace = ENiagaraCoordinateSpace(0);
    pub const WORLD: ENiagaraCoordinateSpace = ENiagaraCoordinateSpace(1);
    pub const LOCAL: ENiagaraCoordinateSpace = ENiagaraCoordinateSpace(2);
}
#[repr(transparent)]
pub struct ENSM_VelocityType(pub i32);
impl ENSM_VelocityType {
    pub const LINEAR: ENSM_VelocityType = ENSM_VelocityType(0);
    pub const FROM_POINT: ENSM_VelocityType = ENSM_VelocityType(1);
    pub const IN_CONE: ENSM_VelocityType = ENSM_VelocityType(2);
}
#[repr(transparent)]
pub struct ENSMInitialMeshOrientationMode(pub i32);
impl ENSMInitialMeshOrientationMode {
    pub const NONE: ENSMInitialMeshOrientationMode = ENSMInitialMeshOrientationMode(0);
    pub const RANDOM: ENSMInitialMeshOrientationMode = ENSMInitialMeshOrientationMode(1);
    pub const ORIENT_TO_AXIS: ENSMInitialMeshOrientationMode = ENSMInitialMeshOrientationMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENSM_ShapePrimitive(pub u8);
impl ENSM_ShapePrimitive {
    pub const BOX: ENSM_ShapePrimitive = ENSM_ShapePrimitive(0);
    pub const CYLINDER: ENSM_ShapePrimitive = ENSM_ShapePrimitive(1);
    pub const PLANE: ENSM_ShapePrimitive = ENSM_ShapePrimitive(2);
    pub const RING: ENSM_ShapePrimitive = ENSM_ShapePrimitive(3);
    pub const SPHERE: ENSM_ShapePrimitive = ENSM_ShapePrimitive(4);
    pub const MAX: ENSM_ShapePrimitive = ENSM_ShapePrimitive(5);
}
#[repr(transparent)]
pub struct ENSM_SurfaceExpansionMode(pub u8);
impl ENSM_SurfaceExpansionMode {
    pub const INNER: ENSM_SurfaceExpansionMode = ENSM_SurfaceExpansionMode(0);
    pub const CENTERED: ENSM_SurfaceExpansionMode = ENSM_SurfaceExpansionMode(1);
    pub const OUTSIDE: ENSM_SurfaceExpansionMode = ENSM_SurfaceExpansionMode(2);
}
#[repr(transparent)]
pub struct ENSMSubUVAnimation_Mode(pub i32);
impl ENSMSubUVAnimation_Mode {
    pub const DIRECT_SET: ENSMSubUVAnimation_Mode = ENSMSubUVAnimation_Mode(0);
    pub const INFINITE_LOOP: ENSMSubUVAnimation_Mode = ENSMSubUVAnimation_Mode(1);
    pub const LINEAR: ENSMSubUVAnimation_Mode = ENSMSubUVAnimation_Mode(2);
    pub const RANDOM: ENSMSubUVAnimation_Mode = ENSMSubUVAnimation_Mode(3);
}
#[repr(transparent)]
pub struct EVolumeCacheType(pub u8);
impl EVolumeCacheType {
    pub const OPEN_VDB: EVolumeCacheType = EVolumeCacheType(0);
}
#[repr(transparent)]
pub struct ENiagaraFunctionDebugState(pub u8);
impl ENiagaraFunctionDebugState {
    pub const NO_DEBUG: ENiagaraFunctionDebugState = ENiagaraFunctionDebugState(0);
    pub const BASIC: ENiagaraFunctionDebugState = ENiagaraFunctionDebugState(1);
}
#[repr(transparent)]
pub struct ENiagaraInputNodeUsage(pub u8);
impl ENiagaraInputNodeUsage {
    pub const UNDEFINED: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(0);
    pub const PARAMETER: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(1);
    pub const ATTRIBUTE: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(2);
    pub const SYSTEM_CONSTANT: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(3);
    pub const TRANSLATOR_CONSTANT: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(4);
    pub const RAPID_ITERATION_PARAMETER: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(
        5,
    );
}
#[repr(transparent)]
pub struct ENiagaraValidationSeverity(pub i32);
impl ENiagaraValidationSeverity {
    pub const INFO: ENiagaraValidationSeverity = ENiagaraValidationSeverity(0);
    pub const WARNING: ENiagaraValidationSeverity = ENiagaraValidationSeverity(1);
    pub const ERROR: ENiagaraValidationSeverity = ENiagaraValidationSeverity(2);
}
