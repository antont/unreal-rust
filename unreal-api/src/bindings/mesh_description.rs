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
