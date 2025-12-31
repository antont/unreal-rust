#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UOptimusGroomCollisionReadDataInterface {}
pub struct UOptimusGroomCollisionReadDataProvider {
    pub solver_component: UPtr<
        crate::bindings::hair_strands_solver::UGroomSolverComponent,
    >,
}
pub struct UOptimusGroomAttributeReadDataInterface {
    pub groom_attribute_name: FName,
    pub groom_attribute_group: EOptimusGroomExecDomain,
    pub groom_attribute_type: EOptimusGroomAttributeTypes,
}
pub struct UOptimusGroomAttributeReadDataProvider {
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
    pub groom_attribute_name: FName,
    pub groom_attribute_group: EOptimusGroomExecDomain,
    pub groom_attribute_type: EOptimusGroomAttributeTypes,
}
pub struct UOptimusGroomExecDataInterface {
    pub domain: EOptimusGroomExecDomain,
}
pub struct UOptimusGroomExecDataProvider {
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
    pub domain: EOptimusGroomExecDomain,
}
pub struct UOptimusGroomAssetComponentSource {}
pub struct UOptimusGroomSolverComponentSource {}
pub struct UOptimusGroomCollisionComponentSource {}
pub struct UOptimusGroomGuidesReadDataInterface {}
pub struct UOptimusGroomGuidesReadDataProvider {
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
pub struct UOptimusGroomGuidesWriteDataInterface {}
pub struct UOptimusGroomGuidesWriteDataProvider {
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
pub struct UOptimusGroomMeshesReadDataInterface {}
pub struct UOptimusGroomMeshesReadDataProvider {
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
pub struct UOptimusGroomSolverReadDataInterface {
    pub reset_simulation_trigger: FName,
}
pub struct UOptimusGroomSolverReadDataProvider {
    pub solver_component: UPtr<
        crate::bindings::hair_strands_solver::UGroomSolverComponent,
    >,
    pub deformer_instance: UPtr<crate::bindings::optimus_core::UOptimusDeformerInstance>,
}
pub struct UOptimusGroomStrandsReadDataInterface {}
pub struct UOptimusGroomStrandsReadDataProvider {
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
pub struct UOptimusGroomStrandsWriteDataInterface {}
pub struct UOptimusGroomStrandsWriteDataProvider {
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
