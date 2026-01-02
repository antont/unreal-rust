#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UGeometryCache {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub material_slot_names: TArray<FName>,
    #[doc(hidden)]
    __padding_168: [u8; 64],
    pub start_frame: i32,
    pub end_frame: i32,
    __padding_end: [u8; 8],
}
impl UGeometryCache {}
#[repr(C, align(8))]
pub struct AGeometryCacheActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub geometry_cache_component: UPtr<UGeometryCacheComponent>,
}
impl AGeometryCacheActor {}
#[repr(C, align(8))]
pub struct UGeometryCacheCodecBase {
    __padding_end: [u8; 72],
}
impl UGeometryCacheCodecBase {}
#[repr(C, align(8))]
pub struct UGeometryCacheCodecRaw {
    __padding_end: [u8; 96],
}
impl UGeometryCacheCodecRaw {}
#[repr(C, align(8))]
pub struct UGeometryCacheCodecV1 {
    __padding_end: [u8; 112],
}
impl UGeometryCacheCodecV1 {}
#[repr(C, align(16))]
pub struct UGeometryCacheComponent {
    #[doc(hidden)]
    __padding_1576: [u8; 1576],
    pub geometry_cache: UPtr<UGeometryCache>,
    pub b_running: bool,
    pub b_looping: bool,
    pub start_time_offset: f32,
    pub playback_speed: f32,
    pub motion_vector_scale: f32,
    #[doc(hidden)]
    __padding_1684: [u8; 84],
    pub duration: f32,
    __padding_end: [u8; 24],
}
impl UGeometryCacheComponent {}
#[repr(C, align(8))]
pub struct UGeometryCacheTrack {
    __padding_end: [u8; 96],
}
impl UGeometryCacheTrack {}
#[repr(C, align(8))]
pub struct UDEPRECATED_GeometryCacheTrack_FlipbookAnimation {
    __padding_end: [u8; 136],
}
impl UDEPRECATED_GeometryCacheTrack_FlipbookAnimation {}
#[repr(C, align(8))]
pub struct UGeometryCacheTrackStreamable {
    __padding_end: [u8; 256],
}
impl UGeometryCacheTrackStreamable {}
#[repr(C, align(8))]
pub struct UDEPRECATED_GeometryCacheTrack_TransformAnimation {
    __padding_end: [u8; 312],
}
impl UDEPRECATED_GeometryCacheTrack_TransformAnimation {}
#[repr(C, align(8))]
pub struct UDEPRECATED_GeometryCacheTrack_TransformGroupAnimation {
    __padding_end: [u8; 312],
}
impl UDEPRECATED_GeometryCacheTrack_TransformGroupAnimation {}
#[repr(C, align(8))]
pub struct UNiagaraGeometryCacheRendererProperties {
    __padding_end: [u8; 3576],
}
impl UNiagaraGeometryCacheRendererProperties {}
