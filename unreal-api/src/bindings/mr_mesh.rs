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
pub static mut U_MOCK_DATA_MESH_TRACKER_COMPONENT_ON_MOCK_DATA_MESH_TRACKER_UPDATED_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOCK_DATA_MESH_TRACKER_COMPONENT_DISCONNECT_MR_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOCK_DATA_MESH_TRACKER_COMPONENT_CONNECT_MR_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_SET_WIREFRAME_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_SET_WIREFRAME_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_SET_USE_WIREFRAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_SET_ENABLE_MESH_OCCLUSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_REQUEST_NAV_MESH_UPDATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_IS_CONNECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_GET_WIREFRAME_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_GET_USE_WIREFRAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_GET_ENABLE_MESH_OCCLUSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_FORCE_NAV_MESH_UPDATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UMR_MESH_COMPONENT_CLEAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_RECONSTRUCTOR_BASE_STOP_RECONSTRUCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_RECONSTRUCTOR_BASE_START_RECONSTRUCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_RECONSTRUCTOR_BASE_PAUSE_RECONSTRUCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_RECONSTRUCTOR_BASE_IS_RECONSTRUCTION_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_RECONSTRUCTOR_BASE_IS_RECONSTRUCTION_PAUSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_RECONSTRUCTOR_BASE_DISCONNECT_MR_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_RECONSTRUCTOR_BASE_CONNECT_MR_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMockDataMeshTrackerComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMockDataMeshTrackerUpdated__DelegateSignature"),
            &raw mut U_MOCK_DATA_MESH_TRACKER_COMPONENT_ON_MOCK_DATA_MESH_TRACKER_UPDATED_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisconnectMRMesh"),
            &raw mut U_MOCK_DATA_MESH_TRACKER_COMPONENT_DISCONNECT_MR_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectMRMesh"),
            &raw mut U_MOCK_DATA_MESH_TRACKER_COMPONENT_CONNECT_MR_MESH,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMRMeshComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWireframeMaterial"),
            &raw mut UMR_MESH_COMPONENT_SET_WIREFRAME_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWireframeColor"),
            &raw mut UMR_MESH_COMPONENT_SET_WIREFRAME_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUseWireframe"),
            &raw mut UMR_MESH_COMPONENT_SET_USE_WIREFRAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableMeshOcclusion"),
            &raw mut UMR_MESH_COMPONENT_SET_ENABLE_MESH_OCCLUSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestNavMeshUpdate"),
            &raw mut UMR_MESH_COMPONENT_REQUEST_NAV_MESH_UPDATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsConnected"),
            &raw mut UMR_MESH_COMPONENT_IS_CONNECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWireframeColor"),
            &raw mut UMR_MESH_COMPONENT_GET_WIREFRAME_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUseWireframe"),
            &raw mut UMR_MESH_COMPONENT_GET_USE_WIREFRAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnableMeshOcclusion"),
            &raw mut UMR_MESH_COMPONENT_GET_ENABLE_MESH_OCCLUSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForceNavMeshUpdate"),
            &raw mut UMR_MESH_COMPONENT_FORCE_NAV_MESH_UPDATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Clear"),
            &raw mut UMR_MESH_COMPONENT_CLEAR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMeshReconstructorBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopReconstruction"),
            &raw mut U_MESH_RECONSTRUCTOR_BASE_STOP_RECONSTRUCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartReconstruction"),
            &raw mut U_MESH_RECONSTRUCTOR_BASE_START_RECONSTRUCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PauseReconstruction"),
            &raw mut U_MESH_RECONSTRUCTOR_BASE_PAUSE_RECONSTRUCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReconstructionStarted"),
            &raw mut U_MESH_RECONSTRUCTOR_BASE_IS_RECONSTRUCTION_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReconstructionPaused"),
            &raw mut U_MESH_RECONSTRUCTOR_BASE_IS_RECONSTRUCTION_PAUSED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisconnectMRMesh"),
            &raw mut U_MESH_RECONSTRUCTOR_BASE_DISCONNECT_MR_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectMRMesh"),
            &raw mut U_MESH_RECONSTRUCTOR_BASE_CONNECT_MR_MESH,
        );
    }
}
#[repr(C, align(1))]
pub struct FMRMeshConfiguration {
    __padding_end: [u8; 1],
}
impl FMRMeshConfiguration {}
#[repr(C, align(16))]
pub struct UMockDataMeshTrackerComponent {
    #[doc(hidden)]
    __padding_680: [u8; 680],
    pub scan_world: bool,
    pub request_normals: bool,
    pub request_vertex_confidence: bool,
    pub vertex_color_mode: EMeshTrackerVertexColorMode,
    pub block_vertex_colors: TArray<crate::bindings::core_u_object::FColor>,
    pub vertex_color_from_confidence_zero: crate::bindings::core_u_object::FLinearColor,
    pub vertex_color_from_confidence_one: crate::bindings::core_u_object::FLinearColor,
    pub update_interval: f32,
    __padding_end: [u8; 44],
}
impl UMockDataMeshTrackerComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMockDataMeshTrackerComponent")
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
    pub fn disconnect_mr_mesh(&mut self, in_mr_mesh_ptr: UPtr<UMRMeshComponent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::U_MOCK_DATA_MESH_TRACKER_COMPONENT_DISCONNECT_MR_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mr_mesh_ptr,
                __buffer.add(0).cast::<UPtr<UMRMeshComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::U_MOCK_DATA_MESH_TRACKER_COMPONENT_DISCONNECT_MR_MESH,
                __buffer,
            )
        };
    }
    pub fn connect_mr_mesh(&mut self, in_mr_mesh_ptr: UPtr<UMRMeshComponent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::U_MOCK_DATA_MESH_TRACKER_COMPONENT_CONNECT_MR_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mr_mesh_ptr,
                __buffer.add(0).cast::<UPtr<UMRMeshComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::U_MOCK_DATA_MESH_TRACKER_COMPONENT_CONNECT_MR_MESH,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMRMeshBodyHolder {
    __padding_end: [u8; 600],
}
impl UMRMeshBodyHolder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMRMeshBodyHolder")
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
pub struct UMRMeshComponent {
    __padding_end: [u8; 1648],
}
impl UMRMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMRMeshComponent")
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
    pub fn set_wireframe_material(
        &mut self,
        in_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_SET_WIREFRAME_MATERIAL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_SET_WIREFRAME_MATERIAL,
                __buffer,
            )
        };
    }
    pub fn set_wireframe_color(
        &mut self,
        in_color: &crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_SET_WIREFRAME_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_SET_WIREFRAME_COLOR,
                __buffer,
            )
        };
    }
    pub fn set_use_wireframe(&mut self, b_use_wireframe: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_SET_USE_WIREFRAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_wireframe,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_SET_USE_WIREFRAME,
                __buffer,
            )
        };
    }
    pub fn set_enable_mesh_occlusion(&mut self, b_enable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_SET_ENABLE_MESH_OCCLUSION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enable, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_SET_ENABLE_MESH_OCCLUSION,
                __buffer,
            )
        };
    }
    pub fn request_nav_mesh_update(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_REQUEST_NAV_MESH_UPDATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_REQUEST_NAV_MESH_UPDATE,
                __buffer,
            )
        };
    }
    pub fn is_connected(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_IS_CONNECTED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_IS_CONNECTED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_wireframe_color(&self) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_GET_WIREFRAME_COLOR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_GET_WIREFRAME_COLOR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>().read()
        }
    }
    pub fn get_use_wireframe(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_GET_USE_WIREFRAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_GET_USE_WIREFRAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_enable_mesh_occlusion(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_GET_ENABLE_MESH_OCCLUSION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_GET_ENABLE_MESH_OCCLUSION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn force_nav_mesh_update(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_FORCE_NAV_MESH_UPDATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_FORCE_NAV_MESH_UPDATE,
                __buffer,
            )
        };
    }
    pub fn clear(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_CLEAR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::UMR_MESH_COMPONENT_CLEAR,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMeshReconstructorBase {
    __padding_end: [u8; 48],
}
impl UMeshReconstructorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshReconstructorBase")
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
    pub fn stop_reconstruction(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::U_MESH_RECONSTRUCTOR_BASE_STOP_RECONSTRUCTION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::U_MESH_RECONSTRUCTOR_BASE_STOP_RECONSTRUCTION,
                __buffer,
            )
        };
    }
    pub fn start_reconstruction(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::U_MESH_RECONSTRUCTOR_BASE_START_RECONSTRUCTION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::U_MESH_RECONSTRUCTOR_BASE_START_RECONSTRUCTION,
                __buffer,
            )
        };
    }
    pub fn pause_reconstruction(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::U_MESH_RECONSTRUCTOR_BASE_PAUSE_RECONSTRUCTION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::U_MESH_RECONSTRUCTOR_BASE_PAUSE_RECONSTRUCTION,
                __buffer,
            )
        };
    }
    pub fn is_reconstruction_started(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::U_MESH_RECONSTRUCTOR_BASE_IS_RECONSTRUCTION_STARTED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::U_MESH_RECONSTRUCTOR_BASE_IS_RECONSTRUCTION_STARTED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_reconstruction_paused(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::U_MESH_RECONSTRUCTOR_BASE_IS_RECONSTRUCTION_PAUSED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::U_MESH_RECONSTRUCTOR_BASE_IS_RECONSTRUCTION_PAUSED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct FMockDataMeshTrackerComponent_OnMeshTrackerUpdated {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EMeshTrackerVertexColorMode(pub u8);
impl EMeshTrackerVertexColorMode {
    pub const NONE: EMeshTrackerVertexColorMode = EMeshTrackerVertexColorMode(0);
    pub const CONFIDENCE: EMeshTrackerVertexColorMode = EMeshTrackerVertexColorMode(1);
    pub const BLOCK: EMeshTrackerVertexColorMode = EMeshTrackerVertexColorMode(2);
}
