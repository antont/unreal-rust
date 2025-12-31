#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FLocationServicesData {
    pub timestamp: f32,
    pub longitude: f32,
    pub latitude: f32,
    pub horizontal_accuracy: f32,
    pub vertical_accuracy: f32,
    pub altitude: f32,
}
pub struct ULocationServicesImpl {
    pub on_location_changed: FLocationServicesImpl_OnLocationChanged,
}
pub struct ULocationServices {}
pub struct FLocationServicesImpl_OnLocationChanged;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocationAccuracy(pub u8);
impl ELocationAccuracy {
    pub const LA_THREE_KILOMETERS: ELocationAccuracy = ELocationAccuracy(0);
    pub const LA_ONE_KILOMETER: ELocationAccuracy = ELocationAccuracy(1);
    pub const LA_HUNDRED_METERS: ELocationAccuracy = ELocationAccuracy(2);
    pub const LA_TEN_METERS: ELocationAccuracy = ELocationAccuracy(3);
    pub const LA_BEST: ELocationAccuracy = ELocationAccuracy(4);
    pub const LA_NAVIGATION: ELocationAccuracy = ELocationAccuracy(5);
}
