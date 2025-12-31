#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FKey {
    pub key_name: FName,
}
pub struct UInputCoreTypes {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EControllerHand(pub u8);
impl EControllerHand {
    pub const LEFT: EControllerHand = EControllerHand(0);
    pub const RIGHT: EControllerHand = EControllerHand(1);
    pub const ANY_HAND: EControllerHand = EControllerHand(2);
    pub const PAD: EControllerHand = EControllerHand(3);
    pub const EXTERNAL_CAMERA: EControllerHand = EControllerHand(4);
    pub const GUN: EControllerHand = EControllerHand(5);
    pub const HMD: EControllerHand = EControllerHand(6);
    pub const CHEST: EControllerHand = EControllerHand(7);
    pub const LEFT_SHOULDER: EControllerHand = EControllerHand(8);
    pub const RIGHT_SHOULDER: EControllerHand = EControllerHand(9);
    pub const LEFT_ELBOW: EControllerHand = EControllerHand(10);
    pub const RIGHT_ELBOW: EControllerHand = EControllerHand(11);
    pub const WAIST: EControllerHand = EControllerHand(12);
    pub const LEFT_KNEE: EControllerHand = EControllerHand(13);
    pub const RIGHT_KNEE: EControllerHand = EControllerHand(14);
    pub const LEFT_FOOT: EControllerHand = EControllerHand(15);
    pub const RIGHT_FOOT: EControllerHand = EControllerHand(16);
    pub const SPECIAL: EControllerHand = EControllerHand(17);
    pub const CONTROLLER_HAND_COUNT: EControllerHand = EControllerHand(18);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETouchIndex(pub u8);
impl ETouchIndex {
    pub const TOUCH1: ETouchIndex = ETouchIndex(0);
    pub const TOUCH2: ETouchIndex = ETouchIndex(1);
    pub const TOUCH3: ETouchIndex = ETouchIndex(2);
    pub const TOUCH4: ETouchIndex = ETouchIndex(3);
    pub const TOUCH5: ETouchIndex = ETouchIndex(4);
    pub const TOUCH6: ETouchIndex = ETouchIndex(5);
    pub const TOUCH7: ETouchIndex = ETouchIndex(6);
    pub const TOUCH8: ETouchIndex = ETouchIndex(7);
    pub const TOUCH9: ETouchIndex = ETouchIndex(8);
    pub const TOUCH10: ETouchIndex = ETouchIndex(9);
    pub const CURSOR_POINTER_INDEX: ETouchIndex = ETouchIndex(10);
    pub const MAX_TOUCHES: ETouchIndex = ETouchIndex(11);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConsoleForGamepadLabels(pub u8);
impl EConsoleForGamepadLabels {
    pub const NONE: EConsoleForGamepadLabels = EConsoleForGamepadLabels(0);
    pub const X_BOX_ONE: EConsoleForGamepadLabels = EConsoleForGamepadLabels(1);
    pub const PS4: EConsoleForGamepadLabels = EConsoleForGamepadLabels(2);
}
