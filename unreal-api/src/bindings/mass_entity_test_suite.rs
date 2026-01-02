#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FFarmVisualDataRow {
    __padding_end: [u8; 24],
}
impl FFarmVisualDataRow {}
#[repr(C, align(16))]
pub struct UMassTestProcessorBase {
    __padding_end: [u8; 1216],
}
impl UMassTestProcessorBase {}
#[repr(C, align(16))]
pub struct UMassTestProcessor_A {
    __padding_end: [u8; 1216],
}
impl UMassTestProcessor_A {}
#[repr(C, align(16))]
pub struct UMassTestProcessor_B {
    __padding_end: [u8; 1216],
}
impl UMassTestProcessor_B {}
#[repr(C, align(16))]
pub struct UMassTestProcessor_C {
    __padding_end: [u8; 1216],
}
impl UMassTestProcessor_C {}
#[repr(C, align(16))]
pub struct UMassTestProcessor_D {
    __padding_end: [u8; 1216],
}
impl UMassTestProcessor_D {}
#[repr(C, align(16))]
pub struct UMassTestProcessor_E {
    __padding_end: [u8; 1216],
}
impl UMassTestProcessor_E {}
#[repr(C, align(16))]
pub struct UMassTestProcessor_F {
    __padding_end: [u8; 1216],
}
impl UMassTestProcessor_F {}
#[repr(C, align(16))]
pub struct UMassTestProcessor_Floats {
    __padding_end: [u8; 1232],
}
impl UMassTestProcessor_Floats {}
#[repr(C, align(16))]
pub struct UMassTestProcessor_Ints {
    __padding_end: [u8; 1232],
}
impl UMassTestProcessor_Ints {}
#[repr(C, align(16))]
pub struct UMassTestProcessor_FloatsInts {
    __padding_end: [u8; 1248],
}
impl UMassTestProcessor_FloatsInts {}
#[repr(C, align(8))]
pub struct UMassTestStaticCounterProcessor {
    __padding_end: [u8; 240],
}
impl UMassTestStaticCounterProcessor {}
#[repr(C, align(16))]
pub struct UMassTestProcessorAutoExecuteQuery {
    __padding_end: [u8; 1120],
}
impl UMassTestProcessorAutoExecuteQuery {}
#[repr(C, align(16))]
pub struct UMassTestProcessorAutoExecuteQueryComparison {
    __padding_end: [u8; 2096],
}
impl UMassTestProcessorAutoExecuteQueryComparison {}
#[repr(C, align(16))]
pub struct UMassTestProcessorAutoExecuteQueryComparison_Parallel {
    __padding_end: [u8; 2096],
}
impl UMassTestProcessorAutoExecuteQueryComparison_Parallel {}
#[repr(C, align(8))]
pub struct UMassTestWorldSubsystem {
    __padding_end: [u8; 72],
}
impl UMassTestWorldSubsystem {}
#[repr(C, align(8))]
pub struct UMassTestParallelSubsystem {
    __padding_end: [u8; 72],
}
impl UMassTestParallelSubsystem {}
#[repr(C, align(8))]
pub struct UMassTestEngineSubsystem {
    __padding_end: [u8; 56],
}
impl UMassTestEngineSubsystem {}
#[repr(C, align(8))]
pub struct UMassTestLocalPlayerSubsystem {
    __padding_end: [u8; 56],
}
impl UMassTestLocalPlayerSubsystem {}
#[repr(C, align(8))]
pub struct UMassTestGameInstanceSubsystem {
    __padding_end: [u8; 56],
}
impl UMassTestGameInstanceSubsystem {}
#[repr(C, align(16))]
pub struct UFarmProcessorBase {
    __padding_end: [u8; 1120],
}
impl UFarmProcessorBase {}
#[repr(C, align(16))]
pub struct UFarmWaterProcessor {
    __padding_end: [u8; 1120],
}
impl UFarmWaterProcessor {}
#[repr(C, align(16))]
pub struct UFarmHarvestTimerSystem_Flowers {
    __padding_end: [u8; 1120],
}
impl UFarmHarvestTimerSystem_Flowers {}
#[repr(C, align(16))]
pub struct UFarmHarvestTimerSystem_Crops {
    __padding_end: [u8; 1120],
}
impl UFarmHarvestTimerSystem_Crops {}
#[repr(C, align(16))]
pub struct UFarmHarvestTimerExpired {
    __padding_end: [u8; 1120],
}
impl UFarmHarvestTimerExpired {}
#[repr(C, align(16))]
pub struct UFarmHarvestTimerSetIcon {
    __padding_end: [u8; 1152],
}
impl UFarmHarvestTimerSetIcon {}
#[repr(C, align(8))]
pub struct AMassEntityTestFarmPlot {
    #[doc(hidden)]
    __padding_1288: [u8; 1288],
    pub harvest_icon_ismc: UPtr<
        crate::bindings::engine::UHierarchicalInstancedStaticMeshComponent,
    >,
    __padding_end: [u8; 32],
}
impl AMassEntityTestFarmPlot {}
