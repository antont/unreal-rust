#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UOptimusGroomCollisionReadDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomCollisionReadDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusGroomCollisionReadDataProvider {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub solver_component: UPtr<
        crate::bindings::hair_strands_solver::UGroomSolverComponent,
    >,
}
impl UOptimusGroomCollisionReadDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusGroomAttributeReadDataInterface {
    __padding_end: [u8; 88],
}
impl UOptimusGroomAttributeReadDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusGroomAttributeReadDataProvider {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
    __padding_end: [u8; 16],
}
impl UOptimusGroomAttributeReadDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusGroomExecDataInterface {
    __padding_end: [u8; 64],
}
impl UOptimusGroomExecDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusGroomExecDataProvider {
    __padding_end: [u8; 64],
}
impl UOptimusGroomExecDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusGroomAssetComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusGroomAssetComponentSource {}
#[repr(C, align(8))]
pub struct UOptimusGroomSolverComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusGroomSolverComponentSource {}
#[repr(C, align(8))]
pub struct UOptimusGroomCollisionComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusGroomCollisionComponentSource {}
#[repr(C, align(8))]
pub struct UOptimusGroomGuidesReadDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomGuidesReadDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusGroomGuidesReadDataProvider {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
impl UOptimusGroomGuidesReadDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusGroomGuidesWriteDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomGuidesWriteDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusGroomGuidesWriteDataProvider {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
    __padding_end: [u8; 8],
}
impl UOptimusGroomGuidesWriteDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusGroomMeshesReadDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomMeshesReadDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusGroomMeshesReadDataProvider {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
impl UOptimusGroomMeshesReadDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusGroomSolverReadDataInterface {
    __padding_end: [u8; 64],
}
impl UOptimusGroomSolverReadDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusGroomSolverReadDataProvider {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub solver_component: UPtr<
        crate::bindings::hair_strands_solver::UGroomSolverComponent,
    >,
    __padding_end: [u8; 24],
}
impl UOptimusGroomSolverReadDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusGroomStrandsReadDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomStrandsReadDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusGroomStrandsReadDataProvider {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
impl UOptimusGroomStrandsReadDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusGroomStrandsWriteDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomStrandsWriteDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusGroomStrandsWriteDataProvider {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
    __padding_end: [u8; 8],
}
impl UOptimusGroomStrandsWriteDataProvider {}
#[repr(transparent)]
pub struct EOptimusGroomExecDomain(pub u8);
impl EOptimusGroomExecDomain {
    pub const NONE: EOptimusGroomExecDomain = EOptimusGroomExecDomain(0);
    pub const CONTROL_POINT: EOptimusGroomExecDomain = EOptimusGroomExecDomain(1);
    pub const CURVE: EOptimusGroomExecDomain = EOptimusGroomExecDomain(2);
    pub const STRANDS_EDGES: EOptimusGroomExecDomain = EOptimusGroomExecDomain(3);
    pub const STRANDS_OBJECTS: EOptimusGroomExecDomain = EOptimusGroomExecDomain(4);
    pub const GUIDES_POINTS: EOptimusGroomExecDomain = EOptimusGroomExecDomain(5);
    pub const GUIDES_CURVES: EOptimusGroomExecDomain = EOptimusGroomExecDomain(6);
    pub const GUIDES_EDGES: EOptimusGroomExecDomain = EOptimusGroomExecDomain(7);
    pub const GUIDES_OBJECTS: EOptimusGroomExecDomain = EOptimusGroomExecDomain(8);
}
#[repr(transparent)]
pub struct EOptimusGroomAttributeTypes(pub u8);
impl EOptimusGroomAttributeTypes {
    pub const NONE: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(0);
    pub const BOOL: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(1);
    pub const INT: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(2);
    pub const INT_VECTOR2: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(3);
    pub const INT_VECTOR3: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(4);
    pub const INT_VECTOR4: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(5);
    pub const UINT: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(6);
    pub const FLOAT: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(7);
    pub const VECTOR2: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(8);
    pub const VECTOR3: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(9);
    pub const VECTOR4: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(10);
    pub const LINEAR_COLOR: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(
        11,
    );
    pub const QUAT: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(12);
    pub const ROTATOR: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(13);
    pub const TRANSFORM: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(14);
    pub const MATRIX3X4: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(15);
}
