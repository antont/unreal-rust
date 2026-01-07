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
pub static mut U_STATIC_MESH_DESCRIPTION_SET_VERTEX_INSTANCE_UV: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATIC_MESH_DESCRIPTION_SET_POLYGON_GROUP_MATERIAL_SLOT_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATIC_MESH_DESCRIPTION_GET_VERTEX_INSTANCE_UV: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATIC_MESH_DESCRIPTION_CREATE_CUBE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStaticMeshDescription::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVertexInstanceUV"),
            &raw mut U_STATIC_MESH_DESCRIPTION_SET_VERTEX_INSTANCE_UV,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPolygonGroupMaterialSlotName"),
            &raw mut U_STATIC_MESH_DESCRIPTION_SET_POLYGON_GROUP_MATERIAL_SLOT_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexInstanceUV"),
            &raw mut U_STATIC_MESH_DESCRIPTION_GET_VERTEX_INSTANCE_UV,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateCube"),
            &raw mut U_STATIC_MESH_DESCRIPTION_CREATE_CUBE,
        );
    }
}
#[repr(C, align(8))]
pub struct FUVMapSettings {
    pub size: crate::bindings::core_u_object::FVector,
    pub uv_tile: crate::bindings::core_u_object::FVector2D,
    pub position: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
}
impl FUVMapSettings {}
#[repr(C, align(8))]
pub struct UStaticMeshDescription {
    __padding_end: [u8; 760],
}
impl UStaticMeshDescription {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshDescription")
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
    pub fn set_vertex_instance_uv(
        &mut self,
        vertex_instance_id: crate::bindings::mesh_description::FVertexInstanceID,
        uv: crate::bindings::core_u_object::FVector2D,
        uv_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_description::U_STATIC_MESH_DESCRIPTION_SET_VERTEX_INSTANCE_UV,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::mesh_description::FVertexInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&uv_index, __buffer.add(24).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_description::U_STATIC_MESH_DESCRIPTION_SET_VERTEX_INSTANCE_UV,
                __buffer,
            )
        };
    }
    pub fn set_polygon_group_material_slot_name(
        &mut self,
        polygon_group_id: crate::bindings::mesh_description::FPolygonGroupID,
        slot_name: &FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_description::U_STATIC_MESH_DESCRIPTION_SET_POLYGON_GROUP_MATERIAL_SLOT_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group_id,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::mesh_description::FPolygonGroupID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(slot_name, __buffer.add(4).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_description::U_STATIC_MESH_DESCRIPTION_SET_POLYGON_GROUP_MATERIAL_SLOT_NAME,
                __buffer,
            )
        };
    }
    pub fn get_vertex_instance_uv(
        &self,
        vertex_instance_id: crate::bindings::mesh_description::FVertexInstanceID,
        uv_index: i32,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_description::U_STATIC_MESH_DESCRIPTION_GET_VERTEX_INSTANCE_UV,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vertex_instance_id,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::mesh_description::FVertexInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&uv_index, __buffer.add(4).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_description::U_STATIC_MESH_DESCRIPTION_GET_VERTEX_INSTANCE_UV,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn create_cube(
        &mut self,
        center: crate::bindings::core_u_object::FVector,
        half_extents: crate::bindings::core_u_object::FVector,
        polygon_group: crate::bindings::mesh_description::FPolygonGroupID,
        polygon_id_plus_x: &mut crate::bindings::mesh_description::FPolygonID,
        polygon_id_minus_x: &mut crate::bindings::mesh_description::FPolygonID,
        polygon_id_plus_y: &mut crate::bindings::mesh_description::FPolygonID,
        polygon_id_minus_y: &mut crate::bindings::mesh_description::FPolygonID,
        polygon_id_plus_z: &mut crate::bindings::mesh_description::FPolygonID,
        polygon_id_minus_z: &mut crate::bindings::mesh_description::FPolygonID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_description::U_STATIC_MESH_DESCRIPTION_CREATE_CUBE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &center,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &half_extents,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &polygon_group,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::mesh_description::FPolygonGroupID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                polygon_id_plus_x,
                __buffer.add(52).cast::<crate::bindings::mesh_description::FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                polygon_id_minus_x,
                __buffer.add(56).cast::<crate::bindings::mesh_description::FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                polygon_id_plus_y,
                __buffer.add(60).cast::<crate::bindings::mesh_description::FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                polygon_id_minus_y,
                __buffer.add(64).cast::<crate::bindings::mesh_description::FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                polygon_id_plus_z,
                __buffer.add(68).cast::<crate::bindings::mesh_description::FPolygonID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                polygon_id_minus_z,
                __buffer.add(72).cast::<crate::bindings::mesh_description::FPolygonID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_description::U_STATIC_MESH_DESCRIPTION_CREATE_CUBE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(52)
                .cast::<crate::bindings::mesh_description::FPolygonID>()
                .swap(polygon_id_plus_x);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::mesh_description::FPolygonID>()
                .swap(polygon_id_minus_x);
        }
        unsafe {
            __buffer
                .add(60)
                .cast::<crate::bindings::mesh_description::FPolygonID>()
                .swap(polygon_id_plus_y);
        }
        unsafe {
            __buffer
                .add(64)
                .cast::<crate::bindings::mesh_description::FPolygonID>()
                .swap(polygon_id_minus_y);
        }
        unsafe {
            __buffer
                .add(68)
                .cast::<crate::bindings::mesh_description::FPolygonID>()
                .swap(polygon_id_plus_z);
        }
        unsafe {
            __buffer
                .add(72)
                .cast::<crate::bindings::mesh_description::FPolygonID>()
                .swap(polygon_id_minus_z);
        }
    }
}
