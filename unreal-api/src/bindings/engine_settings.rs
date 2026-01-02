#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UGameMapsSettings {
    __padding_end: [u8; 400],
}
impl UGameMapsSettings {}
#[repr(C, align(8))]
pub struct UGameNetworkManagerSettings {
    __padding_end: [u8; 96],
}
impl UGameNetworkManagerSettings {}
#[repr(C, align(8))]
pub struct UGameSessionSettings {
    __padding_end: [u8; 64],
}
impl UGameSessionSettings {}
#[repr(C, align(8))]
pub struct UGeneralEngineSettings {
    __padding_end: [u8; 48],
}
impl UGeneralEngineSettings {}
#[repr(C, align(8))]
pub struct UGeneralProjectSettings {
    __padding_end: [u8; 280],
}
impl UGeneralProjectSettings {}
#[repr(C, align(8))]
pub struct UHudSettings {
    __padding_end: [u8; 72],
}
impl UHudSettings {}
#[repr(C, align(8))]
pub struct UConsoleSettings {
    __padding_end: [u8; 120],
}
impl UConsoleSettings {}
#[repr(transparent)]
pub struct ETwoPlayerSplitScreenType(pub u8);
impl ETwoPlayerSplitScreenType {
    pub const HORIZONTAL: ETwoPlayerSplitScreenType = ETwoPlayerSplitScreenType(0);
    pub const VERTICAL: ETwoPlayerSplitScreenType = ETwoPlayerSplitScreenType(1);
}
#[repr(transparent)]
pub struct EThreePlayerSplitScreenType(pub u8);
impl EThreePlayerSplitScreenType {
    pub const FAVOR_TOP: EThreePlayerSplitScreenType = EThreePlayerSplitScreenType(0);
    pub const FAVOR_BOTTOM: EThreePlayerSplitScreenType = EThreePlayerSplitScreenType(1);
    pub const VERTICAL: EThreePlayerSplitScreenType = EThreePlayerSplitScreenType(2);
    pub const HORIZONTAL: EThreePlayerSplitScreenType = EThreePlayerSplitScreenType(3);
}
#[repr(transparent)]
pub struct EFourPlayerSplitScreenType(pub u8);
impl EFourPlayerSplitScreenType {
    pub const GRID: EFourPlayerSplitScreenType = EFourPlayerSplitScreenType(0);
    pub const VERTICAL: EFourPlayerSplitScreenType = EFourPlayerSplitScreenType(1);
    pub const HORIZONTAL: EFourPlayerSplitScreenType = EFourPlayerSplitScreenType(2);
}
