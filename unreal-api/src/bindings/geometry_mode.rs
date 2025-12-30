#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UGeomModifier {
    pub description: FText,
    pub tooltip: FText,
    pub toolbar_icon_name: FName,
    pub flags_92: u8,
    pub cached_polys: UPtr<UPolys>,
}
pub struct UGeomModifier_Edit {}
pub struct UGeomModifier_Clip {
    pub flags_104: u8,
    pub clip_markers: TArray<FVector>,
    pub snapped_mouse_world_space_pos: FVector,
}
pub struct UGeomModifier_Create {}
pub struct UGeomModifier_Delete {}
pub struct UGeomModifier_Extrude {
    pub length: i32,
    pub segments: i32,
    pub save_coord_system: i32,
}
pub struct UGeomModifier_Flip {}
pub struct UGeomModifier_Lathe {
    pub total_segments: i32,
    pub segments: i32,
    pub flags_112: u8,
    pub axis: EAxis,
}
pub struct UGeomModifier_Triangulate {}
pub struct UGeomModifier_Optimize {}
pub struct UGeomModifier_Pen {
    pub flags_104: u8,
    pub extrude_depth: i32,
    pub shape_vertices: TArray<FVector>,
    pub mouse_world_space_pos: FVector,
}
pub struct UGeomModifier_Split {}
pub struct UGeomModifier_Turn {}
pub struct UGeomModifier_Weld {}
pub struct UBrushEditingSubsystemImpl {}
