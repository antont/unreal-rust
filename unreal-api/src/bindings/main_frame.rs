#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UHomeScreenContext {
    __padding_end: [u8; 64],
}
impl UHomeScreenContext {}
#[repr(C, align(8))]
pub struct UHomeScreenSettings {
    __padding_end: [u8; 88],
}
impl UHomeScreenSettings {}
#[repr(C, align(8))]
pub struct UHomeScreenWeb {
    __padding_end: [u8; 120],
}
impl UHomeScreenWeb {}
#[repr(transparent)]
pub struct EMainSectionMenu(pub u8);
impl EMainSectionMenu {
    pub const NONE: EMainSectionMenu = EMainSectionMenu(0);
    pub const HOME: EMainSectionMenu = EMainSectionMenu(1);
    pub const NEWS: EMainSectionMenu = EMainSectionMenu(2);
    pub const GETTING_STARTED: EMainSectionMenu = EMainSectionMenu(3);
    pub const SAMPLE_PROJECTS: EMainSectionMenu = EMainSectionMenu(4);
}
#[repr(transparent)]
pub struct EAutoLoadProject(pub i32);
impl EAutoLoadProject {
    pub const HOME_SCREEN: EAutoLoadProject = EAutoLoadProject(0);
    pub const LAST_PROJECT: EAutoLoadProject = EAutoLoadProject(1);
    pub const MAX: EAutoLoadProject = EAutoLoadProject(2);
}
