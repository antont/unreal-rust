#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTedsAlertColumn {
    pub message: FText,
    pub name: FName,
    pub alert_type: FTedsAlertColumnType,
    pub priority: u8,
}
#[repr(C, align(8))]
pub struct FTedsChildAlertColumn {}
#[repr(C, align(16))]
pub struct FTedsAlertActionColumn {}
#[repr(C, align(1))]
pub struct FTedsAlertChainTag {}
#[repr(C, align(1))]
pub struct FTedsUnsortedAlertChainTag {}
#[repr(C, align(8))]
pub struct FAlertWidgetConstructor {}
#[repr(C, align(1))]
pub struct FAlertWidgetTag {}
#[repr(C, align(8))]
pub struct FAlertHeaderWidgetConstructor {}
#[repr(C, align(1))]
pub struct FAlertHeaderWidgetTag {}
#[repr(C, align(1))]
pub struct FAlertHeaderActiveWidgetTag {}
pub struct UTedsAlertsFactory {}
pub struct UAlertWidgetFactory {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct FTedsAlertColumnType(pub u8);
impl FTedsAlertColumnType {
    pub const WARNING: FTedsAlertColumnType = FTedsAlertColumnType(0);
    pub const ERROR: FTedsAlertColumnType = FTedsAlertColumnType(1);
    pub const MAX: FTedsAlertColumnType = FTedsAlertColumnType(2);
}
