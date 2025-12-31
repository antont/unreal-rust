#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UActorFactoryGeometryCollection {}
pub struct UAssetDefinition_GeometryCollection {}
pub struct UGeometryCollectionCacheFactory {
    pub target_collection: UPtr<
        crate::bindings::geometry_collection_engine::UGeometryCollection,
    >,
}
pub struct UGeometryCollectionFactory {}
pub struct UGeometryCollectionThumbnailRenderer {}
