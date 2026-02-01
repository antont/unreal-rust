#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_mesh_description_base_set_vertex_position: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_set_polygon_vertex_instances: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_set_polygon_polygon_group: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_reverse_polygon_facing: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_reserve_new_vertices: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_reserve_new_vertex_instances: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_reserve_new_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_reserve_new_polygons: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_reserve_new_polygon_groups: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_reserve_new_edges: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_vertex_valid: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_vertex_orphaned: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_vertex_instance_valid: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_triangle_valid: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_triangle_part_of_ngon: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_polygon_valid: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_polygon_group_valid: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_empty: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_edge_valid: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_edge_internal_to_polygon: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_is_edge_internal: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_vertex_instances: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_position: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_pair_edge: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_instance_vertex: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_instance_pair_edge: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_instance_for_triangle_vertex: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_instance_for_polygon_vertex: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_instance_count: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_instance_connected_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_instance_connected_polygons: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_count: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_connected_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_connected_polygons: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_connected_edges: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_vertex_adjacent_vertices: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_triangle_vertices: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_triangle_vertex_instances: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_triangle_vertex_instance: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_triangle_polygon_group: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_triangle_polygon: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_triangle_edges: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_triangle_count: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_triangle_adjacent_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_polygon_vertices: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_polygon_vertex_instances: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_polygon_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_polygon_polygon_group: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_polygon_perimeter_edges: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_polygon_internal_edges: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_polygon_group_polygons: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_polygon_group_count: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_polygon_count: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_polygon_adjacent_polygons: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_vertex_vertex_instances: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_vertex_instance_connected_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_vertex_instance_connected_polygons: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_vertex_connected_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_vertex_connected_polygons: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_vertex_connected_edges: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_polygon_vertices: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_polygon_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_polygon_internal_edges: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_polygon_group_polygons: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_edge_connected_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_num_edge_connected_polygons: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_edge_vertices: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_edge_vertex: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_edge_count: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_edge_connected_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_get_edge_connected_polygons: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_empty: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_delete_vertex_instance: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_delete_vertex: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_delete_triangle: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_delete_polygon_group: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_delete_polygon: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_delete_edge: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_vertex_with_id: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_vertex_instance_with_id: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_vertex_instance: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_vertex: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_triangle_with_id: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_triangle: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_polygon_with_id: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_polygon_group_with_id: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_polygon_group: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_polygon: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_edge_with_id: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_create_edge: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_description_base_compute_polygon_triangulation: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_mesh_description_base_set_vertex_position: std::ptr::null_mut(),
            u_mesh_description_base_set_polygon_vertex_instances: std::ptr::null_mut(),
            u_mesh_description_base_set_polygon_polygon_group: std::ptr::null_mut(),
            u_mesh_description_base_reverse_polygon_facing: std::ptr::null_mut(),
            u_mesh_description_base_reserve_new_vertices: std::ptr::null_mut(),
            u_mesh_description_base_reserve_new_vertex_instances: std::ptr::null_mut(),
            u_mesh_description_base_reserve_new_triangles: std::ptr::null_mut(),
            u_mesh_description_base_reserve_new_polygons: std::ptr::null_mut(),
            u_mesh_description_base_reserve_new_polygon_groups: std::ptr::null_mut(),
            u_mesh_description_base_reserve_new_edges: std::ptr::null_mut(),
            u_mesh_description_base_is_vertex_valid: std::ptr::null_mut(),
            u_mesh_description_base_is_vertex_orphaned: std::ptr::null_mut(),
            u_mesh_description_base_is_vertex_instance_valid: std::ptr::null_mut(),
            u_mesh_description_base_is_triangle_valid: std::ptr::null_mut(),
            u_mesh_description_base_is_triangle_part_of_ngon: std::ptr::null_mut(),
            u_mesh_description_base_is_polygon_valid: std::ptr::null_mut(),
            u_mesh_description_base_is_polygon_group_valid: std::ptr::null_mut(),
            u_mesh_description_base_is_empty: std::ptr::null_mut(),
            u_mesh_description_base_is_edge_valid: std::ptr::null_mut(),
            u_mesh_description_base_is_edge_internal_to_polygon: std::ptr::null_mut(),
            u_mesh_description_base_is_edge_internal: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_vertex_instances: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_position: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_pair_edge: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_instance_vertex: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_instance_pair_edge: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_instance_for_triangle_vertex: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_instance_for_polygon_vertex: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_instance_count: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_instance_connected_triangles: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_instance_connected_polygons: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_count: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_connected_triangles: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_connected_polygons: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_connected_edges: std::ptr::null_mut(),
            u_mesh_description_base_get_vertex_adjacent_vertices: std::ptr::null_mut(),
            u_mesh_description_base_get_triangle_vertices: std::ptr::null_mut(),
            u_mesh_description_base_get_triangle_vertex_instances: std::ptr::null_mut(),
            u_mesh_description_base_get_triangle_vertex_instance: std::ptr::null_mut(),
            u_mesh_description_base_get_triangle_polygon_group: std::ptr::null_mut(),
            u_mesh_description_base_get_triangle_polygon: std::ptr::null_mut(),
            u_mesh_description_base_get_triangle_edges: std::ptr::null_mut(),
            u_mesh_description_base_get_triangle_count: std::ptr::null_mut(),
            u_mesh_description_base_get_triangle_adjacent_triangles: std::ptr::null_mut(),
            u_mesh_description_base_get_polygon_vertices: std::ptr::null_mut(),
            u_mesh_description_base_get_polygon_vertex_instances: std::ptr::null_mut(),
            u_mesh_description_base_get_polygon_triangles: std::ptr::null_mut(),
            u_mesh_description_base_get_polygon_polygon_group: std::ptr::null_mut(),
            u_mesh_description_base_get_polygon_perimeter_edges: std::ptr::null_mut(),
            u_mesh_description_base_get_polygon_internal_edges: std::ptr::null_mut(),
            u_mesh_description_base_get_polygon_group_polygons: std::ptr::null_mut(),
            u_mesh_description_base_get_polygon_group_count: std::ptr::null_mut(),
            u_mesh_description_base_get_polygon_count: std::ptr::null_mut(),
            u_mesh_description_base_get_polygon_adjacent_polygons: std::ptr::null_mut(),
            u_mesh_description_base_get_num_vertex_vertex_instances: std::ptr::null_mut(),
            u_mesh_description_base_get_num_vertex_instance_connected_triangles: std::ptr::null_mut(),
            u_mesh_description_base_get_num_vertex_instance_connected_polygons: std::ptr::null_mut(),
            u_mesh_description_base_get_num_vertex_connected_triangles: std::ptr::null_mut(),
            u_mesh_description_base_get_num_vertex_connected_polygons: std::ptr::null_mut(),
            u_mesh_description_base_get_num_vertex_connected_edges: std::ptr::null_mut(),
            u_mesh_description_base_get_num_polygon_vertices: std::ptr::null_mut(),
            u_mesh_description_base_get_num_polygon_triangles: std::ptr::null_mut(),
            u_mesh_description_base_get_num_polygon_internal_edges: std::ptr::null_mut(),
            u_mesh_description_base_get_num_polygon_group_polygons: std::ptr::null_mut(),
            u_mesh_description_base_get_num_edge_connected_triangles: std::ptr::null_mut(),
            u_mesh_description_base_get_num_edge_connected_polygons: std::ptr::null_mut(),
            u_mesh_description_base_get_edge_vertices: std::ptr::null_mut(),
            u_mesh_description_base_get_edge_vertex: std::ptr::null_mut(),
            u_mesh_description_base_get_edge_count: std::ptr::null_mut(),
            u_mesh_description_base_get_edge_connected_triangles: std::ptr::null_mut(),
            u_mesh_description_base_get_edge_connected_polygons: std::ptr::null_mut(),
            u_mesh_description_base_empty: std::ptr::null_mut(),
            u_mesh_description_base_delete_vertex_instance: std::ptr::null_mut(),
            u_mesh_description_base_delete_vertex: std::ptr::null_mut(),
            u_mesh_description_base_delete_triangle: std::ptr::null_mut(),
            u_mesh_description_base_delete_polygon_group: std::ptr::null_mut(),
            u_mesh_description_base_delete_polygon: std::ptr::null_mut(),
            u_mesh_description_base_delete_edge: std::ptr::null_mut(),
            u_mesh_description_base_create_vertex_with_id: std::ptr::null_mut(),
            u_mesh_description_base_create_vertex_instance_with_id: std::ptr::null_mut(),
            u_mesh_description_base_create_vertex_instance: std::ptr::null_mut(),
            u_mesh_description_base_create_vertex: std::ptr::null_mut(),
            u_mesh_description_base_create_triangle_with_id: std::ptr::null_mut(),
            u_mesh_description_base_create_triangle: std::ptr::null_mut(),
            u_mesh_description_base_create_polygon_with_id: std::ptr::null_mut(),
            u_mesh_description_base_create_polygon_group_with_id: std::ptr::null_mut(),
            u_mesh_description_base_create_polygon_group: std::ptr::null_mut(),
            u_mesh_description_base_create_polygon: std::ptr::null_mut(),
            u_mesh_description_base_create_edge_with_id: std::ptr::null_mut(),
            u_mesh_description_base_create_edge: std::ptr::null_mut(),
            u_mesh_description_base_compute_polygon_triangulation: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshDescriptionBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVertexPosition"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_set_vertex_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPolygonVertexInstances"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_set_polygon_vertex_instances,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPolygonPolygonGroup"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_set_polygon_polygon_group,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReversePolygonFacing"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_reverse_polygon_facing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReserveNewVertices"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_reserve_new_vertices,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReserveNewVertexInstances"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_vertex_instances,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReserveNewTriangles"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_reserve_new_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReserveNewPolygons"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_reserve_new_polygons,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReserveNewPolygonGroups"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_polygon_groups,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReserveNewEdges"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_reserve_new_edges,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsVertexValid"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_is_vertex_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsVertexOrphaned"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_is_vertex_orphaned,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsVertexInstanceValid"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_is_vertex_instance_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTriangleValid"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_is_triangle_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTrianglePartOfNgon"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_is_triangle_part_of_ngon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPolygonValid"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_is_polygon_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPolygonGroupValid"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_is_polygon_group_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEmpty"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_is_empty,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEdgeValid"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_is_edge_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEdgeInternalToPolygon"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_is_edge_internal_to_polygon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEdgeInternal"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_is_edge_internal,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexVertexInstances"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_vertex_instances,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexPosition"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_vertex_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexPairEdge"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_vertex_pair_edge,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexInstanceVertex"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_vertex,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexInstancePairEdge"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_pair_edge,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexInstanceForTriangleVertex"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_for_triangle_vertex,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexInstanceForPolygonVertex"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_for_polygon_vertex,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexInstanceCount"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexInstanceConnectedTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_connected_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexInstanceConnectedPolygons"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_connected_polygons,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexCount"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_vertex_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexConnectedTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_connected_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexConnectedPolygons"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_connected_polygons,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexConnectedEdges"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_connected_edges,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexAdjacentVertices"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_adjacent_vertices,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTriangleVertices"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_triangle_vertices,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTriangleVertexInstances"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_vertex_instances,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTriangleVertexInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_vertex_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTrianglePolygonGroup"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_polygon_group,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTrianglePolygon"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_triangle_polygon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTriangleEdges"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_triangle_edges,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTriangleCount"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_triangle_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTriangleAdjacentTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_adjacent_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPolygonVertices"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_polygon_vertices,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPolygonVertexInstances"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_vertex_instances,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPolygonTriangles"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_polygon_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPolygonPolygonGroup"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_polygon_group,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPolygonPerimeterEdges"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_perimeter_edges,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPolygonInternalEdges"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_internal_edges,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPolygonGroupPolygons"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_group_polygons,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPolygonGroupCount"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_polygon_group_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPolygonCount"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_polygon_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPolygonAdjacentPolygons"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_adjacent_polygons,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumVertexVertexInstances"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_vertex_instances,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumVertexInstanceConnectedTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_instance_connected_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumVertexInstanceConnectedPolygons"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_instance_connected_polygons,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumVertexConnectedTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_connected_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumVertexConnectedPolygons"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_connected_polygons,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumVertexConnectedEdges"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_connected_edges,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumPolygonVertices"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_num_polygon_vertices,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumPolygonTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumPolygonInternalEdges"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_internal_edges,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumPolygonGroupPolygons"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_group_polygons,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumEdgeConnectedTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_edge_connected_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumEdgeConnectedPolygons"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_num_edge_connected_polygons,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEdgeVertices"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_edge_vertices,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEdgeVertex"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_edge_vertex,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEdgeCount"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_get_edge_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEdgeConnectedTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_connected_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEdgeConnectedPolygons"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_connected_polygons,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Empty"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_empty,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteVertexInstance"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_delete_vertex_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteVertex"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_delete_vertex,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteTriangle"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_delete_triangle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeletePolygonGroup"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_delete_polygon_group,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeletePolygon"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_delete_polygon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteEdge"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_delete_edge,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateVertexWithID"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_create_vertex_with_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateVertexInstanceWithID"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_create_vertex_instance_with_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateVertexInstance"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_create_vertex_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateVertex"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_create_vertex,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTriangleWithID"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_create_triangle_with_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTriangle"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_create_triangle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreatePolygonWithID"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_create_polygon_with_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreatePolygonGroupWithID"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_create_polygon_group_with_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreatePolygonGroup"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_create_polygon_group,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreatePolygon"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_create_polygon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateEdgeWithID"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_create_edge_with_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateEdge"),
                &raw mut __FUNCTION_PTRS.u_mesh_description_base_create_edge,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputePolygonTriangulation"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_description_base_compute_polygon_triangulation,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FElementID {
    pub id_value: i32,
}
impl FElementID {}
#[repr(C, align(4))]
pub struct FVertexID {
    pub(crate) __padding_end: [u8; 4],
}
impl FVertexID {}
#[repr(C, align(4))]
pub struct FVertexInstanceID {
    pub(crate) __padding_end: [u8; 4],
}
impl FVertexInstanceID {}
#[repr(C, align(4))]
pub struct FEdgeID {
    pub(crate) __padding_end: [u8; 4],
}
impl FEdgeID {}
#[repr(C, align(4))]
pub struct FUVID {
    pub(crate) __padding_end: [u8; 4],
}
impl FUVID {}
#[repr(C, align(4))]
pub struct FTriangleID {
    pub(crate) __padding_end: [u8; 4],
}
impl FTriangleID {}
#[repr(C, align(4))]
pub struct FPolygonGroupID {
    pub(crate) __padding_end: [u8; 4],
}
impl FPolygonGroupID {}
#[repr(C, align(4))]
pub struct FPolygonID {
    pub(crate) __padding_end: [u8; 4],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshDescriptionBase")
            .copied()
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_set_vertex_position,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_set_vertex_position,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_set_polygon_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_set_polygon_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_set_polygon_polygon_group,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_set_polygon_polygon_group,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reverse_polygon_facing,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reverse_polygon_facing,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_polygon_groups,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_polygon_groups,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_reserve_new_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_vertex_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_vertex_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_vertex_orphaned,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_vertex_orphaned,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_vertex_instance_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_vertex_instance_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_triangle_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_triangle_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_triangle_part_of_ngon,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_triangle_part_of_ngon,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_polygon_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_polygon_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_polygon_group_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_polygon_group_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_empty,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_empty,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_edge_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_edge_valid,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_edge_internal_to_polygon,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_edge_internal_to_polygon,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_edge_internal,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_is_edge_internal,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_position,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_position,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_pair_edge,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_pair_edge,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_pair_edge,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_pair_edge,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_for_triangle_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_for_triangle_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_for_polygon_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_for_polygon_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_count,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_instance_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_count,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_connected_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_connected_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_adjacent_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_vertex_adjacent_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_vertex_instance,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_vertex_instance,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_polygon_group,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_polygon_group,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_polygon,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_polygon,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_count,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_adjacent_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_triangle_adjacent_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_polygon_group,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_polygon_group,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_perimeter_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_perimeter_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_internal_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_internal_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_group_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_group_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_group_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_group_count,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_count,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_adjacent_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_polygon_adjacent_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_vertex_instances,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_instance_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_instance_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_instance_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_instance_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_connected_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_vertex_connected_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_internal_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_internal_edges,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_group_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_polygon_group_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_edge_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_edge_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_edge_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_num_edge_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_vertices,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_count,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_connected_triangles,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_get_edge_connected_polygons,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_empty,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_empty,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_vertex_instance,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_vertex_instance,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_triangle,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_triangle,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_polygon_group,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_polygon_group,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_polygon,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_polygon,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_edge,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_delete_edge,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_vertex_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_vertex_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_vertex_instance_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_vertex_instance_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_vertex_instance,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_vertex_instance,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_vertex,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_vertex,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_triangle_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_triangle_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_triangle,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_triangle,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_polygon_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_polygon_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_polygon_group_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_polygon_group_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_polygon_group,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_polygon_group,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_polygon,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_polygon,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_edge_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_edge_with_id,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_edge,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_create_edge,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_compute_polygon_triangulation,
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
                crate::bindings::mesh_description::__FUNCTION_PTRS
                    .u_mesh_description_base_compute_polygon_triangulation,
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshDescriptionBaseBulkData")
            .copied()
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
