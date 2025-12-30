#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FTestFragment_Float {}
#[repr(C, align(4))]
pub struct FTestFragment_Int {}
#[repr(C, align(1))]
pub struct FTestFragment_Bool {}
#[repr(C, align(1))]
pub struct FTestFragment_Large {}
#[repr(C, align(8))]
pub struct FTestFragment_Array {}
#[repr(C, align(8))]
pub struct FFragmentWithSharedPtr {}
#[repr(C, align(4))]
pub struct FTestChunkFragment_Int {}
#[repr(C, align(4))]
pub struct FTestChunkFragment_Float {}
#[repr(C, align(4))]
pub struct FTestSharedFragment_Int {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FTestSharedFragment_Array {
    pub value: TArray<i32>,
}
#[repr(C, align(4))]
pub struct FTestConstSharedFragment_Int {
    pub value: i32,
}
#[repr(C, align(4))]
pub struct FTestSharedFragment_Float {
    pub value: f32,
}
#[repr(C, align(4))]
pub struct FTestConstSharedFragment_Float {
    pub value: f32,
}
#[repr(C, align(1))]
pub struct FTestFragment_Tag {}
#[repr(C, align(1))]
pub struct FTestTag_A {}
#[repr(C, align(1))]
pub struct FTestTag_B {}
#[repr(C, align(1))]
pub struct FTestTag_C {}
#[repr(C, align(1))]
pub struct FTestTag_D {}
#[repr(C, align(8))]
pub struct FFarmVisualDataRow {
    pub mesh: UPtr<UStaticMesh>,
    pub material_override: UPtr<UMaterialInterface>,
}
#[repr(C, align(1))]
pub struct FFarmJustBecameReadyToHarvestTag {}
#[repr(C, align(1))]
pub struct FFarmReadyToHarvestTag {}
#[repr(C, align(2))]
pub struct FFarmGridCellData {}
#[repr(C, align(4))]
pub struct FFarmWaterFragment {}
#[repr(C, align(4))]
pub struct FFarmFlowerFragment {}
#[repr(C, align(2))]
pub struct FFarmCropFragment {}
#[repr(C, align(4))]
pub struct FFarmVisualFragment {}
#[repr(C, align(4))]
pub struct FHarvestTimerFragment {}
pub struct UMassTestProcessorBase {}
pub struct UMassTestProcessor_A {}
pub struct UMassTestProcessor_B {}
pub struct UMassTestProcessor_C {}
pub struct UMassTestProcessor_D {}
pub struct UMassTestProcessor_E {}
pub struct UMassTestProcessor_F {}
pub struct UMassTestProcessor_Floats {}
pub struct UMassTestProcessor_Ints {}
pub struct UMassTestProcessor_FloatsInts {}
pub struct UMassTestStaticCounterProcessor {}
pub struct UMassTestProcessorAutoExecuteQuery {}
pub struct UMassTestProcessorAutoExecuteQueryComparison {}
pub struct UMassTestProcessorAutoExecuteQueryComparison_Parallel {}
pub struct UMassTestWorldSubsystem {}
pub struct UMassTestParallelSubsystem {}
pub struct UMassTestEngineSubsystem {}
pub struct UMassTestLocalPlayerSubsystem {}
pub struct UMassTestGameInstanceSubsystem {}
pub struct UFarmProcessorBase {}
pub struct UFarmWaterProcessor {}
pub struct UFarmHarvestTimerSystem_Flowers {}
pub struct UFarmHarvestTimerSystem_Crops {}
pub struct UFarmHarvestTimerExpired {}
pub struct UFarmHarvestTimerSetIcon {
    pub harvest_icon_ismc: UPtr<UHierarchicalInstancedStaticMeshComponent>,
}
pub struct AMassEntityTestFarmPlot {
    pub grid_cell_width: f32,
    pub grid_cell_height: f32,
    pub harvest_icon_scale: f32,
    pub visual_data_table: TArray<FFarmVisualDataRow>,
    pub visual_data_ism_cs: TArray<UPtr<UHierarchicalInstancedStaticMeshComponent>>,
    pub per_frame_systems: TArray<UPtr<UMassProcessor>>,
    pub per_second_systems: TArray<UPtr<UMassProcessor>>,
    pub test_data_flower_indicies: TArray<u16>,
    pub test_data_crop_indicies: TArray<u16>,
    pub visual_near_cull_distance: u32,
    pub visual_far_cull_distance: u32,
    pub icon_near_cull_distance: u32,
    pub icon_far_cull_distance: u32,
    pub harvest_icon_ismc: UPtr<UHierarchicalInstancedStaticMeshComponent>,
}
