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
    __padding_end: [u8; 7],
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
