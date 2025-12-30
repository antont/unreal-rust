#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UOptimusGroomCollisionReadDataInterface {}
pub struct UOptimusGroomCollisionReadDataProvider {
    pub solver_component: UPtr<UGroomSolverComponent>,
}
pub struct UOptimusGroomAttributeReadDataInterface {
    pub groom_attribute_name: FName,
    pub groom_attribute_group: EOptimusGroomExecDomain,
    pub groom_attribute_type: EOptimusGroomAttributeTypes,
}
pub struct UOptimusGroomAttributeReadDataProvider {
    pub mesh_component: UPtr<UMeshComponent>,
    pub groom_attribute_name: FName,
    pub groom_attribute_group: EOptimusGroomExecDomain,
    pub groom_attribute_type: EOptimusGroomAttributeTypes,
}
pub struct UOptimusGroomExecDataInterface {
    pub domain: EOptimusGroomExecDomain,
}
pub struct UOptimusGroomExecDataProvider {
    pub mesh_component: UPtr<UMeshComponent>,
    pub domain: EOptimusGroomExecDomain,
}
pub struct UOptimusGroomAssetComponentSource {}
pub struct UOptimusGroomSolverComponentSource {}
pub struct UOptimusGroomCollisionComponentSource {}
pub struct UOptimusGroomGuidesReadDataInterface {}
pub struct UOptimusGroomGuidesReadDataProvider {
    pub mesh_component: UPtr<UMeshComponent>,
}
pub struct UOptimusGroomGuidesWriteDataInterface {}
pub struct UOptimusGroomGuidesWriteDataProvider {
    pub mesh_component: UPtr<UMeshComponent>,
}
pub struct UOptimusGroomMeshesReadDataInterface {}
pub struct UOptimusGroomMeshesReadDataProvider {
    pub mesh_component: UPtr<UMeshComponent>,
}
pub struct UOptimusGroomSolverReadDataInterface {
    pub reset_simulation_trigger: FName,
}
pub struct UOptimusGroomSolverReadDataProvider {
    pub solver_component: UPtr<UGroomSolverComponent>,
    pub deformer_instance: UPtr<UOptimusDeformerInstance>,
}
pub struct UOptimusGroomStrandsReadDataInterface {}
pub struct UOptimusGroomStrandsReadDataProvider {
    pub mesh_component: UPtr<UMeshComponent>,
}
pub struct UOptimusGroomStrandsWriteDataInterface {}
pub struct UOptimusGroomStrandsWriteDataProvider {
    pub mesh_component: UPtr<UMeshComponent>,
}
