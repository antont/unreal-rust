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
pub static mut U_KISMET_PROCEDURAL_MESH_LIBRARY_SLICE_PROCEDURAL_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_PROCEDURAL_MESH_LIBRARY_GET_SECTION_FROM_STATIC_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_PROCEDURAL_MESH_LIBRARY_GET_SECTION_FROM_PROCEDURAL_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_PROCEDURAL_MESH_LIBRARY_GENERATE_BOX_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_WELDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_SPLIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_PROCEDURAL_MESH_LIBRARY_COPY_PROCEDURAL_MESH_FROM_STATIC_MESH_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_PROCEDURAL_MESH_LIBRARY_CONVERT_QUAD_TO_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_PROCEDURAL_MESH_LIBRARY_CALCULATE_TANGENTS_FOR_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_UPDATE_MESH_SECTION_LINEAR_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_UPDATE_MESH_SECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_SET_MESH_SECTION_VISIBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_IS_MESH_SECTION_VISIBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_GET_NUM_SECTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_CREATE_MESH_SECTION_LINEAR_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_CREATE_MESH_SECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_CLEAR_MESH_SECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_CLEAR_COLLISION_CONVEX_MESHES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_CLEAR_ALL_MESH_SECTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_MESH_COMPONENT_ADD_COLLISION_CONVEX_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UKismetProceduralMeshLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SliceProceduralMesh"),
            &raw mut U_KISMET_PROCEDURAL_MESH_LIBRARY_SLICE_PROCEDURAL_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionFromStaticMesh"),
            &raw mut U_KISMET_PROCEDURAL_MESH_LIBRARY_GET_SECTION_FROM_STATIC_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionFromProceduralMesh"),
            &raw mut U_KISMET_PROCEDURAL_MESH_LIBRARY_GET_SECTION_FROM_PROCEDURAL_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateBoxMesh"),
            &raw mut U_KISMET_PROCEDURAL_MESH_LIBRARY_GENERATE_BOX_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateGridMeshWelded"),
            &raw mut U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_WELDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateGridMeshTriangles"),
            &raw mut U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateGridMeshSplit"),
            &raw mut U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_SPLIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyProceduralMeshFromStaticMeshComponent"),
            &raw mut U_KISMET_PROCEDURAL_MESH_LIBRARY_COPY_PROCEDURAL_MESH_FROM_STATIC_MESH_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertQuadToTriangles"),
            &raw mut U_KISMET_PROCEDURAL_MESH_LIBRARY_CONVERT_QUAD_TO_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CalculateTangentsForMesh"),
            &raw mut U_KISMET_PROCEDURAL_MESH_LIBRARY_CALCULATE_TANGENTS_FOR_MESH,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UProceduralMeshComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateMeshSection_LinearColor"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_UPDATE_MESH_SECTION_LINEAR_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateMeshSection"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_UPDATE_MESH_SECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeshSectionVisible"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_SET_MESH_SECTION_VISIBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsMeshSectionVisible"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_IS_MESH_SECTION_VISIBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumSections"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_GET_NUM_SECTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMeshSection_LinearColor"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_CREATE_MESH_SECTION_LINEAR_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMeshSection"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_CREATE_MESH_SECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMeshSection"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_CLEAR_MESH_SECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearCollisionConvexMeshes"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_CLEAR_COLLISION_CONVEX_MESHES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearAllMeshSections"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_CLEAR_ALL_MESH_SECTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCollisionConvexMesh"),
            &raw mut U_PROCEDURAL_MESH_COMPONENT_ADD_COLLISION_CONVEX_MESH,
        );
    }
}
#[repr(C, align(8))]
pub struct FProcMeshTangent {
    pub tangent_x: crate::bindings::core_u_object::FVector,
    pub b_flip_tangent_y: bool,
}
impl FProcMeshTangent {}
#[repr(C, align(8))]
pub struct FProcMeshVertex {
    pub position: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub tangent: FProcMeshTangent,
    pub color: crate::bindings::core_u_object::FColor,
    pub uv0: crate::bindings::core_u_object::FVector2D,
    pub uv1: crate::bindings::core_u_object::FVector2D,
    pub uv2: crate::bindings::core_u_object::FVector2D,
    pub uv3: crate::bindings::core_u_object::FVector2D,
}
impl FProcMeshVertex {}
#[repr(C, align(8))]
pub struct UKismetProceduralMeshLibrary {
    __padding_end: [u8; 48],
}
impl UKismetProceduralMeshLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UKismetProceduralMeshLibrary")
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
    pub fn slice_procedural_mesh(
        in_proc_mesh: UPtr<UProceduralMeshComponent>,
        plane_position: crate::bindings::core_u_object::FVector,
        plane_normal: crate::bindings::core_u_object::FVector,
        b_create_other_half: bool,
        out_other_half_proc_mesh: &mut UPtr<UProceduralMeshComponent>,
        cap_option: EProcMeshSliceCapOption,
        cap_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_SLICE_PROCEDURAL_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_proc_mesh,
                __buffer.add(0).cast::<UPtr<UProceduralMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &plane_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &plane_normal,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_other_half,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_other_half_proc_mesh,
                __buffer.add(64).cast::<UPtr<UProceduralMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &cap_option,
                __buffer.add(72).cast::<EProcMeshSliceCapOption>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &cap_material,
                __buffer
                    .add(80)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::procedural_mesh_component::UKismetProceduralMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_SLICE_PROCEDURAL_MESH,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(64)
                .cast::<UPtr<UProceduralMeshComponent>>()
                .swap(out_other_half_proc_mesh);
        }
    }
    pub fn get_section_from_static_mesh(
        in_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        section_index: i32,
        vertices: &mut TArray<crate::bindings::core_u_object::FVector>,
        triangles: &mut TArray<i32>,
        normals: &mut TArray<crate::bindings::core_u_object::FVector>,
        u_vs: &mut TArray<crate::bindings::core_u_object::FVector2D>,
        tangents: &mut TArray<FProcMeshTangent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_GET_SECTION_FROM_STATIC_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(32).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normals,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                u_vs,
                __buffer
                    .add(64)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tangents,
                __buffer.add(80).cast::<TArray<FProcMeshTangent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::procedural_mesh_component::UKismetProceduralMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_GET_SECTION_FROM_STATIC_MESH,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(vertices);
        }
        unsafe {
            __buffer.add(32).cast::<TArray<i32>>().swap(triangles);
        }
        unsafe {
            __buffer
                .add(48)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(normals);
        }
        unsafe {
            __buffer
                .add(64)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .swap(u_vs);
        }
        unsafe {
            __buffer.add(80).cast::<TArray<FProcMeshTangent>>().swap(tangents);
        }
    }
    pub fn get_section_from_procedural_mesh(
        in_proc_mesh: UPtr<UProceduralMeshComponent>,
        section_index: i32,
        vertices: &mut TArray<crate::bindings::core_u_object::FVector>,
        triangles: &mut TArray<i32>,
        normals: &mut TArray<crate::bindings::core_u_object::FVector>,
        u_vs: &mut TArray<crate::bindings::core_u_object::FVector2D>,
        tangents: &mut TArray<FProcMeshTangent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_GET_SECTION_FROM_PROCEDURAL_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_proc_mesh,
                __buffer.add(0).cast::<UPtr<UProceduralMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(32).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normals,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                u_vs,
                __buffer
                    .add(64)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tangents,
                __buffer.add(80).cast::<TArray<FProcMeshTangent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::procedural_mesh_component::UKismetProceduralMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_GET_SECTION_FROM_PROCEDURAL_MESH,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(vertices);
        }
        unsafe {
            __buffer.add(32).cast::<TArray<i32>>().swap(triangles);
        }
        unsafe {
            __buffer
                .add(48)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(normals);
        }
        unsafe {
            __buffer
                .add(64)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .swap(u_vs);
        }
        unsafe {
            __buffer.add(80).cast::<TArray<FProcMeshTangent>>().swap(tangents);
        }
    }
    pub fn generate_box_mesh(
        box_radius: crate::bindings::core_u_object::FVector,
        vertices: &mut TArray<crate::bindings::core_u_object::FVector>,
        triangles: &mut TArray<i32>,
        normals: &mut TArray<crate::bindings::core_u_object::FVector>,
        u_vs: &mut TArray<crate::bindings::core_u_object::FVector2D>,
        tangents: &mut TArray<FProcMeshTangent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_GENERATE_BOX_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &box_radius,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(40).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normals,
                __buffer
                    .add(56)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                u_vs,
                __buffer
                    .add(72)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tangents,
                __buffer.add(88).cast::<TArray<FProcMeshTangent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::procedural_mesh_component::UKismetProceduralMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_GENERATE_BOX_MESH,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(vertices);
        }
        unsafe {
            __buffer.add(40).cast::<TArray<i32>>().swap(triangles);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(normals);
        }
        unsafe {
            __buffer
                .add(72)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .swap(u_vs);
        }
        unsafe {
            __buffer.add(88).cast::<TArray<FProcMeshTangent>>().swap(tangents);
        }
    }
    pub fn create_grid_mesh_welded(
        num_x: i32,
        num_y: i32,
        triangles: &mut TArray<i32>,
        vertices: &mut TArray<crate::bindings::core_u_object::FVector>,
        u_vs: &mut TArray<crate::bindings::core_u_object::FVector2D>,
        grid_spacing: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_WELDED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&num_x, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&num_y, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(8).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                u_vs,
                __buffer
                    .add(40)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &grid_spacing,
                __buffer.add(56).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::procedural_mesh_component::UKismetProceduralMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_WELDED,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<i32>>().swap(triangles);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(vertices);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .swap(u_vs);
        }
    }
    pub fn create_grid_mesh_triangles(
        num_x: i32,
        num_y: i32,
        b_winding: bool,
        triangles: &mut TArray<i32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&num_x, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&num_y, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_winding, __buffer.add(8).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(16).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::procedural_mesh_component::UKismetProceduralMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<i32>>().swap(triangles);
        }
    }
    pub fn create_grid_mesh_split(
        num_x: i32,
        num_y: i32,
        triangles: &mut TArray<i32>,
        vertices: &mut TArray<crate::bindings::core_u_object::FVector>,
        u_vs: &mut TArray<crate::bindings::core_u_object::FVector2D>,
        uv1s: &mut TArray<crate::bindings::core_u_object::FVector2D>,
        grid_spacing: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_SPLIT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&num_x, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&num_y, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(8).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                u_vs,
                __buffer
                    .add(40)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv1s,
                __buffer
                    .add(56)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &grid_spacing,
                __buffer.add(72).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::procedural_mesh_component::UKismetProceduralMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_CREATE_GRID_MESH_SPLIT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<i32>>().swap(triangles);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(vertices);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .swap(u_vs);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .swap(uv1s);
        }
    }
    pub fn copy_procedural_mesh_from_static_mesh_component(
        static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
        lod_index: i32,
        proc_mesh_component: UPtr<UProceduralMeshComponent>,
        b_create_collision: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_COPY_PROCEDURAL_MESH_FROM_STATIC_MESH_COMPONENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UStaticMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &proc_mesh_component,
                __buffer.add(16).cast::<UPtr<UProceduralMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_collision,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::procedural_mesh_component::UKismetProceduralMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_COPY_PROCEDURAL_MESH_FROM_STATIC_MESH_COMPONENT,
                __buffer,
            )
        };
    }
    pub fn convert_quad_to_triangles(
        triangles: &mut TArray<i32>,
        vert0: i32,
        vert1: i32,
        vert2: i32,
        vert3: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_CONVERT_QUAD_TO_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&vert0, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&vert1, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&vert2, __buffer.add(24).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&vert3, __buffer.add(28).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::procedural_mesh_component::UKismetProceduralMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_CONVERT_QUAD_TO_TRIANGLES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<i32>>().swap(triangles);
        }
    }
    pub fn calculate_tangents_for_mesh(
        vertices: &TArray<crate::bindings::core_u_object::FVector>,
        triangles: &TArray<i32>,
        u_vs: &TArray<crate::bindings::core_u_object::FVector2D>,
        normals: &mut TArray<crate::bindings::core_u_object::FVector>,
        tangents: &mut TArray<FProcMeshTangent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_CALCULATE_TANGENTS_FOR_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(16).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                u_vs,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normals,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tangents,
                __buffer.add(64).cast::<TArray<FProcMeshTangent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::procedural_mesh_component::UKismetProceduralMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_KISMET_PROCEDURAL_MESH_LIBRARY_CALCULATE_TANGENTS_FOR_MESH,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(normals);
        }
        unsafe {
            __buffer.add(64).cast::<TArray<FProcMeshTangent>>().swap(tangents);
        }
    }
}
#[repr(C, align(16))]
pub struct UProceduralMeshComponent {
    #[doc(hidden)]
    __padding_1584: [u8; 1584],
    pub b_use_complex_as_simple_collision: bool,
    pub b_use_async_cooking: bool,
    __padding_end: [u8; 126],
}
impl UProceduralMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralMeshComponent")
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
    pub fn update_mesh_section_linear_color(
        &mut self,
        section_index: i32,
        vertices: &TArray<crate::bindings::core_u_object::FVector>,
        normals: &TArray<crate::bindings::core_u_object::FVector>,
        uv0: &TArray<crate::bindings::core_u_object::FVector2D>,
        uv1: &TArray<crate::bindings::core_u_object::FVector2D>,
        uv2: &TArray<crate::bindings::core_u_object::FVector2D>,
        uv3: &TArray<crate::bindings::core_u_object::FVector2D>,
        vertex_colors: &TArray<crate::bindings::core_u_object::FLinearColor>,
        tangents: &TArray<FProcMeshTangent>,
        b_srgb_conversion: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<137>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_UPDATE_MESH_SECTION_LINEAR_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normals,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv0,
                __buffer
                    .add(40)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv1,
                __buffer
                    .add(56)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv2,
                __buffer
                    .add(72)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv3,
                __buffer
                    .add(88)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_colors,
                __buffer
                    .add(104)
                    .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tangents,
                __buffer.add(120).cast::<TArray<FProcMeshTangent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_srgb_conversion,
                __buffer.add(136).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_UPDATE_MESH_SECTION_LINEAR_COLOR,
                __buffer,
            )
        };
    }
    pub fn update_mesh_section(
        &mut self,
        section_index: i32,
        vertices: &TArray<crate::bindings::core_u_object::FVector>,
        normals: &TArray<crate::bindings::core_u_object::FVector>,
        uv0: &TArray<crate::bindings::core_u_object::FVector2D>,
        vertex_colors: &TArray<crate::bindings::core_u_object::FColor>,
        tangents: &TArray<FProcMeshTangent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_UPDATE_MESH_SECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normals,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv0,
                __buffer
                    .add(40)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_colors,
                __buffer
                    .add(56)
                    .cast::<TArray<crate::bindings::core_u_object::FColor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tangents,
                __buffer.add(72).cast::<TArray<FProcMeshTangent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_UPDATE_MESH_SECTION,
                __buffer,
            )
        };
    }
    pub fn set_mesh_section_visible(
        &mut self,
        section_index: i32,
        b_new_visibility: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_SET_MESH_SECTION_VISIBLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_visibility,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_SET_MESH_SECTION_VISIBLE,
                __buffer,
            )
        };
    }
    pub fn is_mesh_section_visible(&self, section_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_IS_MESH_SECTION_VISIBLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
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
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_IS_MESH_SECTION_VISIBLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_num_sections(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_GET_NUM_SECTIONS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_GET_NUM_SECTIONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn create_mesh_section_linear_color(
        &mut self,
        section_index: i32,
        vertices: &TArray<crate::bindings::core_u_object::FVector>,
        triangles: &TArray<i32>,
        normals: &TArray<crate::bindings::core_u_object::FVector>,
        uv0: &TArray<crate::bindings::core_u_object::FVector2D>,
        uv1: &TArray<crate::bindings::core_u_object::FVector2D>,
        uv2: &TArray<crate::bindings::core_u_object::FVector2D>,
        uv3: &TArray<crate::bindings::core_u_object::FVector2D>,
        vertex_colors: &TArray<crate::bindings::core_u_object::FLinearColor>,
        tangents: &TArray<FProcMeshTangent>,
        b_create_collision: bool,
        b_srgb_conversion: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<154>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_CREATE_MESH_SECTION_LINEAR_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(24).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normals,
                __buffer
                    .add(40)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv0,
                __buffer
                    .add(56)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv1,
                __buffer
                    .add(72)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv2,
                __buffer
                    .add(88)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv3,
                __buffer
                    .add(104)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_colors,
                __buffer
                    .add(120)
                    .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tangents,
                __buffer.add(136).cast::<TArray<FProcMeshTangent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_collision,
                __buffer.add(152).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_srgb_conversion,
                __buffer.add(153).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_CREATE_MESH_SECTION_LINEAR_COLOR,
                __buffer,
            )
        };
    }
    pub fn create_mesh_section(
        &mut self,
        section_index: i32,
        vertices: &TArray<crate::bindings::core_u_object::FVector>,
        triangles: &TArray<i32>,
        normals: &TArray<crate::bindings::core_u_object::FVector>,
        uv0: &TArray<crate::bindings::core_u_object::FVector2D>,
        vertex_colors: &TArray<crate::bindings::core_u_object::FColor>,
        tangents: &TArray<FProcMeshTangent>,
        b_create_collision: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_CREATE_MESH_SECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(24).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normals,
                __buffer
                    .add(40)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                uv0,
                __buffer
                    .add(56)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_colors,
                __buffer
                    .add(72)
                    .cast::<TArray<crate::bindings::core_u_object::FColor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tangents,
                __buffer.add(88).cast::<TArray<FProcMeshTangent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_collision,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_CREATE_MESH_SECTION,
                __buffer,
            )
        };
    }
    pub fn clear_mesh_section(&mut self, section_index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_CLEAR_MESH_SECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
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
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_CLEAR_MESH_SECTION,
                __buffer,
            )
        };
    }
    pub fn clear_collision_convex_meshes(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_CLEAR_COLLISION_CONVEX_MESHES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_CLEAR_COLLISION_CONVEX_MESHES,
                __buffer,
            )
        };
    }
    pub fn clear_all_mesh_sections(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_CLEAR_ALL_MESH_SECTIONS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_CLEAR_ALL_MESH_SECTIONS,
                __buffer,
            )
        };
    }
    pub fn add_collision_convex_mesh(
        &mut self,
        convex_verts: TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_ADD_COLLISION_CONVEX_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &convex_verts,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::procedural_mesh_component::U_PROCEDURAL_MESH_COMPONENT_ADD_COLLISION_CONVEX_MESH,
                __buffer,
            )
        };
    }
}
#[repr(transparent)]
pub struct EProcMeshSliceCapOption(pub u8);
impl EProcMeshSliceCapOption {
    pub const NO_CAP: EProcMeshSliceCapOption = EProcMeshSliceCapOption(0);
    pub const CREATE_NEW_SECTION_FOR_CAP: EProcMeshSliceCapOption = EProcMeshSliceCapOption(
        1,
    );
    pub const USE_LAST_SECTION_FOR_CAP: EProcMeshSliceCapOption = EProcMeshSliceCapOption(
        2,
    );
}
