#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UTedsAlertsFactory {
    __padding_end: [u8; 104],
}
impl UTedsAlertsFactory {}
#[repr(C, align(8))]
pub struct UAlertWidgetFactory {
    __padding_end: [u8; 48],
}
impl UAlertWidgetFactory {}
#[repr(transparent)]
pub struct FTedsAlertColumnType(pub u8);
impl FTedsAlertColumnType {
    pub const WARNING: FTedsAlertColumnType = FTedsAlertColumnType(0);
    pub const ERROR: FTedsAlertColumnType = FTedsAlertColumnType(1);
    pub const MAX: FTedsAlertColumnType = FTedsAlertColumnType(2);
}
