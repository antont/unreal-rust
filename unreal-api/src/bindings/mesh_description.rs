#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_SET_VERTEX_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_SET_POLYGON_VERTEX_INSTANCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_SET_POLYGON_POLYGON_GROUP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_REVERSE_POLYGON_FACING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_VERTICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_VERTEX_INSTANCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_POLYGONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_POLYGON_GROUPS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_EDGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_VERTEX_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_VERTEX_ORPHANED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_VERTEX_INSTANCE_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_TRIANGLE_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_TRIANGLE_PART_OF_NGON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_POLYGON_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_POLYGON_GROUP_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_EMPTY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_EDGE_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_EDGE_INTERNAL_TO_POLYGON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_IS_EDGE_INTERNAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_VERTEX_INSTANCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_PAIR_EDGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_VERTEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_PAIR_EDGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_FOR_TRIANGLE_VERTEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_FOR_POLYGON_VERTEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_CONNECTED_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_CONNECTED_POLYGONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_POLYGONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_EDGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_ADJACENT_VERTICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTEX_INSTANCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTEX_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_POLYGON_GROUP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_POLYGON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_EDGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_ADJACENT_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_VERTICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_VERTEX_INSTANCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_POLYGON_GROUP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_PERIMETER_EDGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_INTERNAL_EDGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_GROUP_POLYGONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_GROUP_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_ADJACENT_POLYGONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_VERTEX_INSTANCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_INSTANCE_CONNECTED_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_INSTANCE_CONNECTED_POLYGONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_POLYGONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_EDGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_VERTICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_INTERNAL_EDGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_GROUP_POLYGONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_EDGE_CONNECTED_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_NUM_EDGE_CONNECTED_POLYGONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_EDGE_VERTICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_EDGE_VERTEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_EDGE_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_EDGE_CONNECTED_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_GET_EDGE_CONNECTED_POLYGONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_EMPTY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_DELETE_VERTEX_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_DELETE_VERTEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_DELETE_TRIANGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_DELETE_POLYGON_GROUP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_DELETE_POLYGON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_DELETE_EDGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_WITH_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_INSTANCE_WITH_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_VERTEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_TRIANGLE_WITH_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_TRIANGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_WITH_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_GROUP_WITH_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_GROUP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_POLYGON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_EDGE_WITH_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_CREATE_EDGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_DESCRIPTION_BASE_COMPUTE_POLYGON_TRIANGULATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMeshDescriptionBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVertexPosition"),
            &raw mut U_MESH_DESCRIPTION_BASE_SET_VERTEX_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPolygonVertexInstances"),
            &raw mut U_MESH_DESCRIPTION_BASE_SET_POLYGON_VERTEX_INSTANCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPolygonPolygonGroup"),
            &raw mut U_MESH_DESCRIPTION_BASE_SET_POLYGON_POLYGON_GROUP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReversePolygonFacing"),
            &raw mut U_MESH_DESCRIPTION_BASE_REVERSE_POLYGON_FACING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReserveNewVertices"),
            &raw mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_VERTICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReserveNewVertexInstances"),
            &raw mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_VERTEX_INSTANCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReserveNewTriangles"),
            &raw mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReserveNewPolygons"),
            &raw mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_POLYGONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReserveNewPolygonGroups"),
            &raw mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_POLYGON_GROUPS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReserveNewEdges"),
            &raw mut U_MESH_DESCRIPTION_BASE_RESERVE_NEW_EDGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsVertexValid"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_VERTEX_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsVertexOrphaned"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_VERTEX_ORPHANED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsVertexInstanceValid"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_VERTEX_INSTANCE_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTriangleValid"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_TRIANGLE_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTrianglePartOfNgon"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_TRIANGLE_PART_OF_NGON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPolygonValid"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_POLYGON_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPolygonGroupValid"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_POLYGON_GROUP_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEmpty"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_EMPTY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEdgeValid"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_EDGE_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEdgeInternalToPolygon"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_EDGE_INTERNAL_TO_POLYGON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEdgeInternal"),
            &raw mut U_MESH_DESCRIPTION_BASE_IS_EDGE_INTERNAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexVertexInstances"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_VERTEX_INSTANCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexPosition"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexPairEdge"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_PAIR_EDGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexInstanceVertex"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_VERTEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexInstancePairEdge"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_PAIR_EDGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexInstanceForTriangleVertex"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_FOR_TRIANGLE_VERTEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexInstanceForPolygonVertex"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_FOR_POLYGON_VERTEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexInstanceCount"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexInstanceConnectedTriangles"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_CONNECTED_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexInstanceConnectedPolygons"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_CONNECTED_POLYGONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexCount"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexConnectedTriangles"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexConnectedPolygons"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_POLYGONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexConnectedEdges"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_EDGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexAdjacentVertices"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_VERTEX_ADJACENT_VERTICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTriangleVertices"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTriangleVertexInstances"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTEX_INSTANCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTriangleVertexInstance"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTEX_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTrianglePolygonGroup"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_POLYGON_GROUP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTrianglePolygon"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_POLYGON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTriangleEdges"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_EDGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTriangleCount"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTriangleAdjacentTriangles"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_ADJACENT_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPolygonVertices"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_VERTICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPolygonVertexInstances"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_VERTEX_INSTANCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPolygonTriangles"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPolygonPolygonGroup"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_POLYGON_GROUP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPolygonPerimeterEdges"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_PERIMETER_EDGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPolygonInternalEdges"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_INTERNAL_EDGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPolygonGroupPolygons"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_GROUP_POLYGONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPolygonGroupCount"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_GROUP_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPolygonCount"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPolygonAdjacentPolygons"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_POLYGON_ADJACENT_POLYGONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumVertexVertexInstances"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_VERTEX_INSTANCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumVertexInstanceConnectedTriangles"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_INSTANCE_CONNECTED_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumVertexInstanceConnectedPolygons"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_INSTANCE_CONNECTED_POLYGONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumVertexConnectedTriangles"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumVertexConnectedPolygons"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_POLYGONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumVertexConnectedEdges"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_EDGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumPolygonVertices"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_VERTICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumPolygonTriangles"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumPolygonInternalEdges"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_INTERNAL_EDGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumPolygonGroupPolygons"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_GROUP_POLYGONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumEdgeConnectedTriangles"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_EDGE_CONNECTED_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumEdgeConnectedPolygons"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_NUM_EDGE_CONNECTED_POLYGONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEdgeVertices"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_EDGE_VERTICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEdgeVertex"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_EDGE_VERTEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEdgeCount"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_EDGE_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEdgeConnectedTriangles"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_EDGE_CONNECTED_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEdgeConnectedPolygons"),
            &raw mut U_MESH_DESCRIPTION_BASE_GET_EDGE_CONNECTED_POLYGONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Empty"),
            &raw mut U_MESH_DESCRIPTION_BASE_EMPTY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteVertexInstance"),
            &raw mut U_MESH_DESCRIPTION_BASE_DELETE_VERTEX_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteVertex"),
            &raw mut U_MESH_DESCRIPTION_BASE_DELETE_VERTEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteTriangle"),
            &raw mut U_MESH_DESCRIPTION_BASE_DELETE_TRIANGLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeletePolygonGroup"),
            &raw mut U_MESH_DESCRIPTION_BASE_DELETE_POLYGON_GROUP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeletePolygon"),
            &raw mut U_MESH_DESCRIPTION_BASE_DELETE_POLYGON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteEdge"),
            &raw mut U_MESH_DESCRIPTION_BASE_DELETE_EDGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateVertexWithID"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_WITH_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateVertexInstanceWithID"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_INSTANCE_WITH_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateVertexInstance"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateVertex"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_VERTEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateTriangleWithID"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_TRIANGLE_WITH_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateTriangle"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_TRIANGLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePolygonWithID"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_WITH_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePolygonGroupWithID"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_GROUP_WITH_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePolygonGroup"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_GROUP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePolygon"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_POLYGON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateEdgeWithID"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_EDGE_WITH_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateEdge"),
            &raw mut U_MESH_DESCRIPTION_BASE_CREATE_EDGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ComputePolygonTriangulation"),
            &raw mut U_MESH_DESCRIPTION_BASE_COMPUTE_POLYGON_TRIANGULATION,
        );
    }
}
#[repr(C, align(4))]
pub struct FElementID {
    pub id_value: i32,
}
impl FElementID {}
#[repr(C, align(4))]
pub struct FVertexID {
    __padding_end: [u8; 4],
}
impl FVertexID {}
#[repr(C, align(4))]
pub struct FVertexInstanceID {
    __padding_end: [u8; 4],
}
impl FVertexInstanceID {}
#[repr(C, align(4))]
pub struct FEdgeID {
    __padding_end: [u8; 4],
}
impl FEdgeID {}
#[repr(C, align(4))]
pub struct FUVID {
    __padding_end: [u8; 4],
}
impl FUVID {}
#[repr(C, align(4))]
pub struct FTriangleID {
    __padding_end: [u8; 4],
}
impl FTriangleID {}
#[repr(C, align(4))]
pub struct FPolygonGroupID {
    __padding_end: [u8; 4],
}
impl FPolygonGroupID {}
#[repr(C, align(4))]
pub struct FPolygonID {
    __padding_end: [u8; 4],
}
impl FPolygonID {}
#[repr(C, align(8))]
pub struct UMeshDescriptionBase {
    __padding_end: [u8; 760],
}
impl UMeshDescriptionBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshDescriptionBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_vertex_position(
        &mut self,
        vertex_id: FVertexID,
        position: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_SET_VERTEX_POSITION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_SET_VERTEX_POSITION,
                __buffer,
            )
        };
    }
    pub fn set_polygon_vertex_instances(
        &mut self,
        polygon_id: FPolygonID,
        vertex_instance_i_ds: &TArray<FVertexInstanceID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_SET_POLYGON_VERTEX_INSTANCES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_instance_i_ds,
                __buffer.add(8).cast::<TArray<FVertexInstanceID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_SET_POLYGON_VERTEX_INSTANCES,
                __buffer,
            )
        };
    }
    pub fn set_polygon_polygon_group(
        &mut self,
        polygon_id: FPolygonID,
        polygon_group_id: FPolygonGroupID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_SET_POLYGON_POLYGON_GROUP,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer.add(4).cast::<FPolygonGroupID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_SET_POLYGON_POLYGON_GROUP,
                __buffer,
            )
        };
    }
    pub fn reverse_polygon_facing(&mut self, polygon_id: FPolygonID) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_REVERSE_POLYGON_FACING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_REVERSE_POLYGON_FACING,
                __buffer,
            )
        };
    }
    pub fn reserve_new_vertices(&mut self, number_of_new_vertices: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_VERTICES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &number_of_new_vertices,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_VERTICES,
                __buffer,
            )
        };
    }
    pub fn reserve_new_vertex_instances(&mut self, number_of_new_vertex_instances: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_VERTEX_INSTANCES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &number_of_new_vertex_instances,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_VERTEX_INSTANCES,
                __buffer,
            )
        };
    }
    pub fn reserve_new_triangles(&mut self, number_of_new_triangles: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &number_of_new_triangles,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_TRIANGLES,
                __buffer,
            )
        };
    }
    pub fn reserve_new_polygons(&mut self, number_of_new_polygons: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &number_of_new_polygons,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_POLYGONS,
                __buffer,
            )
        };
    }
    pub fn reserve_new_polygon_groups(&mut self, number_of_new_polygon_groups: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_POLYGON_GROUPS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &number_of_new_polygon_groups,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_POLYGON_GROUPS,
                __buffer,
            )
        };
    }
    pub fn reserve_new_edges(&mut self, number_of_new_edges: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_EDGES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &number_of_new_edges,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_RESERVE_NEW_EDGES,
                __buffer,
            )
        };
    }
    pub fn is_vertex_valid(&self, vertex_id: FVertexID) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_VERTEX_VALID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_VERTEX_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_vertex_orphaned(&self, vertex_id: FVertexID) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_VERTEX_ORPHANED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_VERTEX_ORPHANED,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_vertex_instance_valid(
        &self,
        vertex_instance_id: FVertexInstanceID,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_VERTEX_INSTANCE_VALID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id,
                __buffer.add(0).cast::<FVertexInstanceID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_VERTEX_INSTANCE_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_triangle_valid(&self, triangle_id: FTriangleID) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_TRIANGLE_VALID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_TRIANGLE_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_triangle_part_of_ngon(&self, triangle_id: FTriangleID) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_TRIANGLE_PART_OF_NGON,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_TRIANGLE_PART_OF_NGON,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_polygon_valid(&self, polygon_id: FPolygonID) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_POLYGON_VALID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_POLYGON_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_polygon_group_valid(&self, polygon_group_id: FPolygonGroupID) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_POLYGON_GROUP_VALID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer.add(0).cast::<FPolygonGroupID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_POLYGON_GROUP_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_empty(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_EMPTY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_EMPTY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_edge_valid(&self, edge_id: FEdgeID) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_EDGE_VALID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_EDGE_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_edge_internal_to_polygon(
        &self,
        edge_id: FEdgeID,
        polygon_id: FPolygonID,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_EDGE_INTERNAL_TO_POLYGON,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(4).cast::<FPolygonID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_EDGE_INTERNAL_TO_POLYGON,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_edge_internal(&self, edge_id: FEdgeID) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_EDGE_INTERNAL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_IS_EDGE_INTERNAL,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_vertex_vertex_instances(
        &self,
        vertex_id: FVertexID,
        out_vertex_instance_i_ds: &mut TArray<FVertexInstanceID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_VERTEX_INSTANCES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_vertex_instance_i_ds,
                __buffer.add(8).cast::<TArray<FVertexInstanceID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_VERTEX_INSTANCES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FVertexInstanceID>>()
                .swap(out_vertex_instance_i_ds);
        }
    }
    pub fn get_vertex_position(
        &self,
        vertex_id: FVertexID,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_POSITION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_POSITION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_vertex_pair_edge(
        &self,
        vertex_id0: FVertexID,
        vertex_id1: FVertexID,
    ) -> FEdgeID {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_PAIR_EDGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id0,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id1,
                __buffer.add(4).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_PAIR_EDGE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FEdgeID>().read() }
    }
    pub fn get_vertex_instance_vertex(
        &self,
        vertex_instance_id: FVertexInstanceID,
    ) -> FVertexID {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_VERTEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id,
                __buffer.add(0).cast::<FVertexInstanceID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_VERTEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FVertexID>().read() }
    }
    pub fn get_vertex_instance_pair_edge(
        &self,
        vertex_instance_id0: FVertexInstanceID,
        vertex_instance_id1: FVertexInstanceID,
    ) -> FEdgeID {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_PAIR_EDGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id0,
                __buffer.add(0).cast::<FVertexInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id1,
                __buffer.add(4).cast::<FVertexInstanceID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_PAIR_EDGE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FEdgeID>().read() }
    }
    pub fn get_vertex_instance_for_triangle_vertex(
        &self,
        triangle_id: FTriangleID,
        vertex_id: FVertexID,
    ) -> FVertexInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_FOR_TRIANGLE_VERTEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(4).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_FOR_TRIANGLE_VERTEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FVertexInstanceID>().read() }
    }
    pub fn get_vertex_instance_for_polygon_vertex(
        &self,
        polygon_id: FPolygonID,
        vertex_id: FVertexID,
    ) -> FVertexInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_FOR_POLYGON_VERTEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(4).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_FOR_POLYGON_VERTEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FVertexInstanceID>().read() }
    }
    pub fn get_vertex_instance_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_COUNT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_vertex_instance_connected_triangles(
        &self,
        vertex_instance_id: FVertexInstanceID,
        out_connected_triangle_i_ds: &mut TArray<FTriangleID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id,
                __buffer.add(0).cast::<FVertexInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_connected_triangle_i_ds,
                __buffer.add(8).cast::<TArray<FTriangleID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FTriangleID>>()
                .swap(out_connected_triangle_i_ds);
        }
    }
    pub fn get_vertex_instance_connected_polygons(
        &self,
        vertex_instance_id: FVertexInstanceID,
        out_connected_polygon_i_ds: &mut TArray<FPolygonID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id,
                __buffer.add(0).cast::<FVertexInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_connected_polygon_i_ds,
                __buffer.add(8).cast::<TArray<FPolygonID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_INSTANCE_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FPolygonID>>()
                .swap(out_connected_polygon_i_ds);
        }
    }
    pub fn get_vertex_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_COUNT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_vertex_connected_triangles(
        &self,
        vertex_id: FVertexID,
        out_connected_triangle_i_ds: &mut TArray<FTriangleID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_connected_triangle_i_ds,
                __buffer.add(8).cast::<TArray<FTriangleID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FTriangleID>>()
                .swap(out_connected_triangle_i_ds);
        }
    }
    pub fn get_vertex_connected_polygons(
        &self,
        vertex_id: FVertexID,
        out_connected_polygon_i_ds: &mut TArray<FPolygonID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_connected_polygon_i_ds,
                __buffer.add(8).cast::<TArray<FPolygonID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FPolygonID>>()
                .swap(out_connected_polygon_i_ds);
        }
    }
    pub fn get_vertex_connected_edges(
        &self,
        vertex_id: FVertexID,
        out_edge_i_ds: &mut TArray<FEdgeID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_EDGES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_edge_i_ds,
                __buffer.add(8).cast::<TArray<FEdgeID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_CONNECTED_EDGES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FEdgeID>>().swap(out_edge_i_ds);
        }
    }
    pub fn get_vertex_adjacent_vertices(
        &self,
        vertex_id: FVertexID,
        out_adjacent_vertex_i_ds: &mut TArray<FVertexID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_ADJACENT_VERTICES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_adjacent_vertex_i_ds,
                __buffer.add(8).cast::<TArray<FVertexID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_VERTEX_ADJACENT_VERTICES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FVertexID>>().swap(out_adjacent_vertex_i_ds);
        }
    }
    pub fn get_triangle_vertices(
        &self,
        triangle_id: FTriangleID,
        out_vertex_i_ds: &mut TArray<FVertexID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTICES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_vertex_i_ds,
                __buffer.add(8).cast::<TArray<FVertexID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTICES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FVertexID>>().swap(out_vertex_i_ds);
        }
    }
    pub fn get_triangle_vertex_instances(
        &self,
        triangle_id: FTriangleID,
        out_vertex_instance_i_ds: &mut TArray<FVertexInstanceID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTEX_INSTANCES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_vertex_instance_i_ds,
                __buffer.add(8).cast::<TArray<FVertexInstanceID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTEX_INSTANCES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FVertexInstanceID>>()
                .swap(out_vertex_instance_i_ds);
        }
    }
    pub fn get_triangle_vertex_instance(
        &self,
        triangle_id: FTriangleID,
        index: i32,
    ) -> FVertexInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTEX_INSTANCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(4).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_VERTEX_INSTANCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FVertexInstanceID>().read() }
    }
    pub fn get_triangle_polygon_group(
        &self,
        triangle_id: FTriangleID,
    ) -> FPolygonGroupID {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_POLYGON_GROUP,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_POLYGON_GROUP,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FPolygonGroupID>().read() }
    }
    pub fn get_triangle_polygon(&self, triangle_id: FTriangleID) -> FPolygonID {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_POLYGON,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_POLYGON,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FPolygonID>().read() }
    }
    pub fn get_triangle_edges(
        &self,
        triangle_id: FTriangleID,
        out_edge_i_ds: &mut TArray<FEdgeID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_EDGES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_edge_i_ds,
                __buffer.add(8).cast::<TArray<FEdgeID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_EDGES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FEdgeID>>().swap(out_edge_i_ds);
        }
    }
    pub fn get_triangle_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_COUNT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_triangle_adjacent_triangles(
        &self,
        triangle_id: FTriangleID,
        out_triangle_i_ds: &mut TArray<FTriangleID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_ADJACENT_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_triangle_i_ds,
                __buffer.add(8).cast::<TArray<FTriangleID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_TRIANGLE_ADJACENT_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FTriangleID>>().swap(out_triangle_i_ds);
        }
    }
    pub fn get_polygon_vertices(
        &self,
        polygon_id: FPolygonID,
        out_vertex_i_ds: &mut TArray<FVertexID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_VERTICES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_vertex_i_ds,
                __buffer.add(8).cast::<TArray<FVertexID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_VERTICES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FVertexID>>().swap(out_vertex_i_ds);
        }
    }
    pub fn get_polygon_vertex_instances(
        &self,
        polygon_id: FPolygonID,
        out_vertex_instance_i_ds: &mut TArray<FVertexInstanceID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_VERTEX_INSTANCES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_vertex_instance_i_ds,
                __buffer.add(8).cast::<TArray<FVertexInstanceID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_VERTEX_INSTANCES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FVertexInstanceID>>()
                .swap(out_vertex_instance_i_ds);
        }
    }
    pub fn get_polygon_triangles(
        &self,
        polygon_id: FPolygonID,
        out_triangle_i_ds: &mut TArray<FTriangleID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_triangle_i_ds,
                __buffer.add(8).cast::<TArray<FTriangleID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FTriangleID>>().swap(out_triangle_i_ds);
        }
    }
    pub fn get_polygon_polygon_group(&self, polygon_id: FPolygonID) -> FPolygonGroupID {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_POLYGON_GROUP,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_POLYGON_GROUP,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FPolygonGroupID>().read() }
    }
    pub fn get_polygon_perimeter_edges(
        &self,
        polygon_id: FPolygonID,
        out_edge_i_ds: &mut TArray<FEdgeID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_PERIMETER_EDGES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_edge_i_ds,
                __buffer.add(8).cast::<TArray<FEdgeID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_PERIMETER_EDGES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FEdgeID>>().swap(out_edge_i_ds);
        }
    }
    pub fn get_polygon_internal_edges(
        &self,
        polygon_id: FPolygonID,
        out_edge_i_ds: &mut TArray<FEdgeID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_INTERNAL_EDGES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_edge_i_ds,
                __buffer.add(8).cast::<TArray<FEdgeID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_INTERNAL_EDGES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FEdgeID>>().swap(out_edge_i_ds);
        }
    }
    pub fn get_polygon_group_polygons(
        &self,
        polygon_group_id: FPolygonGroupID,
        out_polygon_i_ds: &mut TArray<FPolygonID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_GROUP_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer.add(0).cast::<FPolygonGroupID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_polygon_i_ds,
                __buffer.add(8).cast::<TArray<FPolygonID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_GROUP_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FPolygonID>>().swap(out_polygon_i_ds);
        }
    }
    pub fn get_polygon_group_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_GROUP_COUNT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_GROUP_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_polygon_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_COUNT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_polygon_adjacent_polygons(
        &self,
        polygon_id: FPolygonID,
        out_polygon_i_ds: &mut TArray<FPolygonID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_ADJACENT_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_polygon_i_ds,
                __buffer.add(8).cast::<TArray<FPolygonID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_POLYGON_ADJACENT_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FPolygonID>>().swap(out_polygon_i_ds);
        }
    }
    pub fn get_num_vertex_vertex_instances(&self, vertex_id: FVertexID) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_VERTEX_INSTANCES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_VERTEX_INSTANCES,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_vertex_instance_connected_triangles(
        &self,
        vertex_instance_id: FVertexInstanceID,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_INSTANCE_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id,
                __buffer.add(0).cast::<FVertexInstanceID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_INSTANCE_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_vertex_instance_connected_polygons(
        &self,
        vertex_instance_id: FVertexInstanceID,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_INSTANCE_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id,
                __buffer.add(0).cast::<FVertexInstanceID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_INSTANCE_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_vertex_connected_triangles(&self, vertex_id: FVertexID) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_vertex_connected_polygons(&self, vertex_id: FVertexID) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_vertex_connected_edges(&self, vertex_id: FVertexID) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_EDGES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_VERTEX_CONNECTED_EDGES,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_polygon_vertices(&self, polygon_id: FPolygonID) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_VERTICES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_VERTICES,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_polygon_triangles(&self, polygon_id: FPolygonID) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_TRIANGLES,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_polygon_internal_edges(&self, polygon_id: FPolygonID) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_INTERNAL_EDGES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_INTERNAL_EDGES,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_polygon_group_polygons(
        &self,
        polygon_group_id: FPolygonGroupID,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_GROUP_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer.add(0).cast::<FPolygonGroupID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_POLYGON_GROUP_POLYGONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_edge_connected_triangles(&self, edge_id: FEdgeID) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_EDGE_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_EDGE_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_edge_connected_polygons(&self, edge_id: FEdgeID) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_EDGE_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_NUM_EDGE_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_edge_vertices(
        &self,
        edge_id: FEdgeID,
        out_vertex_i_ds: &mut TArray<FVertexID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_EDGE_VERTICES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_vertex_i_ds,
                __buffer.add(8).cast::<TArray<FVertexID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_EDGE_VERTICES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FVertexID>>().swap(out_vertex_i_ds);
        }
    }
    pub fn get_edge_vertex(&self, edge_id: FEdgeID, vertex_number: i32) -> FVertexID {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_EDGE_VERTEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_number,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_EDGE_VERTEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FVertexID>().read() }
    }
    pub fn get_edge_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_EDGE_COUNT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_EDGE_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_edge_connected_triangles(
        &self,
        edge_id: FEdgeID,
        out_connected_triangle_i_ds: &mut TArray<FTriangleID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_EDGE_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_connected_triangle_i_ds,
                __buffer.add(8).cast::<TArray<FTriangleID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_EDGE_CONNECTED_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FTriangleID>>()
                .swap(out_connected_triangle_i_ds);
        }
    }
    pub fn get_edge_connected_polygons(
        &self,
        edge_id: FEdgeID,
        out_connected_polygon_i_ds: &mut TArray<FPolygonID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_EDGE_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_connected_polygon_i_ds,
                __buffer.add(8).cast::<TArray<FPolygonID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_GET_EDGE_CONNECTED_POLYGONS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FPolygonID>>()
                .swap(out_connected_polygon_i_ds);
        }
    }
    pub fn empty(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_EMPTY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_EMPTY,
                __buffer,
            )
        };
    }
    pub fn delete_vertex_instance(
        &mut self,
        vertex_instance_id: FVertexInstanceID,
        orphaned_vertices: &mut TArray<FVertexID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_VERTEX_INSTANCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id,
                __buffer.add(0).cast::<FVertexInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orphaned_vertices,
                __buffer.add(8).cast::<TArray<FVertexID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_VERTEX_INSTANCE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FVertexID>>().swap(orphaned_vertices);
        }
    }
    pub fn delete_vertex(&mut self, vertex_id: FVertexID) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_VERTEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_VERTEX,
                __buffer,
            )
        };
    }
    pub fn delete_triangle(
        &mut self,
        triangle_id: FTriangleID,
        orphaned_edges: &mut TArray<FEdgeID>,
        orphaned_vertex_instances: &mut TArray<FVertexInstanceID>,
        orphaned_polygon_groups_ptr: &mut TArray<FPolygonGroupID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_TRIANGLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orphaned_edges,
                __buffer.add(8).cast::<TArray<FEdgeID>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orphaned_vertex_instances,
                __buffer.add(24).cast::<TArray<FVertexInstanceID>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orphaned_polygon_groups_ptr,
                __buffer.add(40).cast::<TArray<FPolygonGroupID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_TRIANGLE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FEdgeID>>().swap(orphaned_edges);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<FVertexInstanceID>>()
                .swap(orphaned_vertex_instances);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<FPolygonGroupID>>()
                .swap(orphaned_polygon_groups_ptr);
        }
    }
    pub fn delete_polygon_group(&mut self, polygon_group_id: FPolygonGroupID) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_POLYGON_GROUP,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer.add(0).cast::<FPolygonGroupID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_POLYGON_GROUP,
                __buffer,
            )
        };
    }
    pub fn delete_polygon(
        &mut self,
        polygon_id: FPolygonID,
        orphaned_edges: &mut TArray<FEdgeID>,
        orphaned_vertex_instances: &mut TArray<FVertexInstanceID>,
        orphaned_polygon_groups: &mut TArray<FPolygonGroupID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_POLYGON,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orphaned_edges,
                __buffer.add(8).cast::<TArray<FEdgeID>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orphaned_vertex_instances,
                __buffer.add(24).cast::<TArray<FVertexInstanceID>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orphaned_polygon_groups,
                __buffer.add(40).cast::<TArray<FPolygonGroupID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_POLYGON,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FEdgeID>>().swap(orphaned_edges);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<FVertexInstanceID>>()
                .swap(orphaned_vertex_instances);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<FPolygonGroupID>>()
                .swap(orphaned_polygon_groups);
        }
    }
    pub fn delete_edge(
        &mut self,
        edge_id: FEdgeID,
        orphaned_vertices: &mut TArray<FVertexID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_EDGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orphaned_vertices,
                __buffer.add(8).cast::<TArray<FVertexID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_DELETE_EDGE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FVertexID>>().swap(orphaned_vertices);
        }
    }
    pub fn create_vertex_with_id(&mut self, vertex_id: FVertexID) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_WITH_ID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_WITH_ID,
                __buffer,
            )
        };
    }
    pub fn create_vertex_instance_with_id(
        &mut self,
        vertex_instance_id: FVertexInstanceID,
        vertex_id: FVertexID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_INSTANCE_WITH_ID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id,
                __buffer.add(0).cast::<FVertexInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(4).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_INSTANCE_WITH_ID,
                __buffer,
            )
        };
    }
    pub fn create_vertex_instance(&mut self, vertex_id: FVertexID) -> FVertexInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_INSTANCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_VERTEX_INSTANCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FVertexInstanceID>().read() }
    }
    pub fn create_vertex(&mut self) -> FVertexID {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_VERTEX,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_VERTEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FVertexID>().read() }
    }
    pub fn create_triangle_with_id(
        &mut self,
        triangle_id: FTriangleID,
        polygon_group_id: FPolygonGroupID,
        vertex_instance_i_ds: &TArray<FVertexInstanceID>,
        new_edge_i_ds: &mut TArray<FEdgeID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_TRIANGLE_WITH_ID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &triangle_id,
                __buffer.add(0).cast::<FTriangleID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer.add(4).cast::<FPolygonGroupID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_instance_i_ds,
                __buffer.add(8).cast::<TArray<FVertexInstanceID>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_edge_i_ds,
                __buffer.add(24).cast::<TArray<FEdgeID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_TRIANGLE_WITH_ID,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<FEdgeID>>().swap(new_edge_i_ds);
        }
    }
    pub fn create_triangle(
        &mut self,
        polygon_group_id: FPolygonGroupID,
        vertex_instance_i_ds: &TArray<FVertexInstanceID>,
        new_edge_i_ds: &mut TArray<FEdgeID>,
    ) -> FTriangleID {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_TRIANGLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer.add(0).cast::<FPolygonGroupID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_instance_i_ds,
                __buffer.add(8).cast::<TArray<FVertexInstanceID>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_edge_i_ds,
                __buffer.add(24).cast::<TArray<FEdgeID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_TRIANGLE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<FEdgeID>>().swap(new_edge_i_ds);
        }
        unsafe { __buffer.add(40).cast::<FTriangleID>().read() }
    }
    pub fn create_polygon_with_id(
        &mut self,
        polygon_id: FPolygonID,
        polygon_group_id: FPolygonGroupID,
        vertex_instance_i_ds: &mut TArray<FVertexInstanceID>,
        new_edge_i_ds: &mut TArray<FEdgeID>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_WITH_ID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer.add(4).cast::<FPolygonGroupID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_instance_i_ds,
                __buffer.add(8).cast::<TArray<FVertexInstanceID>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_edge_i_ds,
                __buffer.add(24).cast::<TArray<FEdgeID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_WITH_ID,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FVertexInstanceID>>()
                .swap(vertex_instance_i_ds);
        }
        unsafe {
            __buffer.add(24).cast::<TArray<FEdgeID>>().swap(new_edge_i_ds);
        }
    }
    pub fn create_polygon_group_with_id(&mut self, polygon_group_id: FPolygonGroupID) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_GROUP_WITH_ID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer.add(0).cast::<FPolygonGroupID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_GROUP_WITH_ID,
                __buffer,
            )
        };
    }
    pub fn create_polygon_group(&mut self) -> FPolygonGroupID {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_GROUP,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_POLYGON_GROUP,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FPolygonGroupID>().read() }
    }
    pub fn create_polygon(
        &mut self,
        polygon_group_id: FPolygonGroupID,
        vertex_instance_i_ds: &mut TArray<FVertexInstanceID>,
        new_edge_i_ds: &mut TArray<FEdgeID>,
    ) -> FPolygonID {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_POLYGON,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer.add(0).cast::<FPolygonGroupID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_instance_i_ds,
                __buffer.add(8).cast::<TArray<FVertexInstanceID>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_edge_i_ds,
                __buffer.add(24).cast::<TArray<FEdgeID>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_POLYGON,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FVertexInstanceID>>()
                .swap(vertex_instance_i_ds);
        }
        unsafe {
            __buffer.add(24).cast::<TArray<FEdgeID>>().swap(new_edge_i_ds);
        }
        unsafe { __buffer.add(40).cast::<FPolygonID>().read() }
    }
    pub fn create_edge_with_id(
        &mut self,
        edge_id: FEdgeID,
        vertex_id0: FVertexID,
        vertex_id1: FVertexID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_EDGE_WITH_ID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_id,
                __buffer.add(0).cast::<FEdgeID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id0,
                __buffer.add(4).cast::<FVertexID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id1,
                __buffer.add(8).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_EDGE_WITH_ID,
                __buffer,
            )
        };
    }
    pub fn create_edge(
        &mut self,
        vertex_id0: FVertexID,
        vertex_id1: FVertexID,
    ) -> FEdgeID {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_EDGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id0,
                __buffer.add(0).cast::<FVertexID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_id1,
                __buffer.add(4).cast::<FVertexID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_CREATE_EDGE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FEdgeID>().read() }
    }
    pub fn compute_polygon_triangulation(&mut self, polygon_id: FPolygonID) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_COMPUTE_POLYGON_TRIANGULATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_id,
                __buffer.add(0).cast::<FPolygonID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::U_MESH_DESCRIPTION_BASE_COMPUTE_POLYGON_TRIANGULATION,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMeshDescriptionBaseBulkData {
    __padding_end: [u8; 72],
}
impl UMeshDescriptionBaseBulkData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshDescriptionBaseBulkData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
