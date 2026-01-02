#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMediaPlateResource {
    __padding_end: [u8; 128],
}
impl FMediaPlateResource {}
#[repr(C, align(8))]
pub struct AMediaPlate {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub media_plate_component: UPtr<UMediaPlateComponent>,
    pub static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    pub b_enable_holdout_composite: bool,
    __padding_end: [u8; 23],
}
impl AMediaPlate {}
#[repr(C, align(8))]
pub struct UMediaPlateAssetUserData {
    __padding_end: [u8; 72],
}
impl UMediaPlateAssetUserData {}
#[repr(C, align(8))]
pub struct UMediaPlateComponent {
    #[doc(hidden)]
    __padding_248: [u8; 248],
    pub b_play_on_open: bool,
    pub start_time: f32,
    #[doc(hidden)]
    __padding_296: [u8; 40],
    pub media_plate_resource: FMediaPlateResource,
    pub playlist_index: i32,
    #[doc(hidden)]
    __padding_436: [u8; 8],
    pub b_is_media_plate_playing: bool,
    #[doc(hidden)]
    __padding_456: [u8; 19],
    pub b_play_only_when_visible: bool,
    pub b_loop: bool,
    #[doc(hidden)]
    __padding_464: [u8; 6],
    pub b_is_aspect_ratio_auto: bool,
    __padding_end: [u8; 239],
}
impl UMediaPlateComponent {}
#[repr(transparent)]
pub struct EMediaPlateResourceType(pub u8);
impl EMediaPlateResourceType {
    pub const PLAYLIST: EMediaPlateResourceType = EMediaPlateResourceType(0);
    pub const EXTERNAL: EMediaPlateResourceType = EMediaPlateResourceType(1);
    pub const ASSET: EMediaPlateResourceType = EMediaPlateResourceType(2);
}
#[repr(transparent)]
pub struct EMediaPlateEventState(pub u8);
impl EMediaPlateEventState {
    pub const PLAY: EMediaPlateEventState = EMediaPlateEventState(0);
    pub const OPEN: EMediaPlateEventState = EMediaPlateEventState(1);
    pub const CLOSE: EMediaPlateEventState = EMediaPlateEventState(2);
    pub const PAUSE: EMediaPlateEventState = EMediaPlateEventState(3);
    pub const REVERSE: EMediaPlateEventState = EMediaPlateEventState(4);
    pub const FORWARD: EMediaPlateEventState = EMediaPlateEventState(5);
    pub const REWIND: EMediaPlateEventState = EMediaPlateEventState(6);
    pub const NEXT: EMediaPlateEventState = EMediaPlateEventState(7);
    pub const PREVIOUS: EMediaPlateEventState = EMediaPlateEventState(8);
    pub const MAX: EMediaPlateEventState = EMediaPlateEventState(9);
}
