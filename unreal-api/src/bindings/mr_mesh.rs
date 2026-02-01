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
    pub u_mock_data_mesh_tracker_component_on_mock_data_mesh_tracker_updated_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_mock_data_mesh_tracker_component_disconnect_mr_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_mock_data_mesh_tracker_component_connect_mr_mesh: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_set_wireframe_material: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_set_wireframe_color: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_set_use_wireframe: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_set_enable_mesh_occlusion: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_request_nav_mesh_update: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_is_connected: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_get_wireframe_color: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_get_use_wireframe: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_get_enable_mesh_occlusion: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_force_nav_mesh_update: *mut crate::ffi::UFunctionOpague,
    pub umr_mesh_component_clear: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_reconstructor_base_stop_reconstruction: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_reconstructor_base_start_reconstruction: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_reconstructor_base_pause_reconstruction: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_reconstructor_base_is_reconstruction_started: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_reconstructor_base_is_reconstruction_paused: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_reconstructor_base_disconnect_mr_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_reconstructor_base_connect_mr_mesh: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_mock_data_mesh_tracker_component_on_mock_data_mesh_tracker_updated_delegate_signature: std::ptr::null_mut(),
            u_mock_data_mesh_tracker_component_disconnect_mr_mesh: std::ptr::null_mut(),
            u_mock_data_mesh_tracker_component_connect_mr_mesh: std::ptr::null_mut(),
            umr_mesh_component_set_wireframe_material: std::ptr::null_mut(),
            umr_mesh_component_set_wireframe_color: std::ptr::null_mut(),
            umr_mesh_component_set_use_wireframe: std::ptr::null_mut(),
            umr_mesh_component_set_enable_mesh_occlusion: std::ptr::null_mut(),
            umr_mesh_component_request_nav_mesh_update: std::ptr::null_mut(),
            umr_mesh_component_is_connected: std::ptr::null_mut(),
            umr_mesh_component_get_wireframe_color: std::ptr::null_mut(),
            umr_mesh_component_get_use_wireframe: std::ptr::null_mut(),
            umr_mesh_component_get_enable_mesh_occlusion: std::ptr::null_mut(),
            umr_mesh_component_force_nav_mesh_update: std::ptr::null_mut(),
            umr_mesh_component_clear: std::ptr::null_mut(),
            u_mesh_reconstructor_base_stop_reconstruction: std::ptr::null_mut(),
            u_mesh_reconstructor_base_start_reconstruction: std::ptr::null_mut(),
            u_mesh_reconstructor_base_pause_reconstruction: std::ptr::null_mut(),
            u_mesh_reconstructor_base_is_reconstruction_started: std::ptr::null_mut(),
            u_mesh_reconstructor_base_is_reconstruction_paused: std::ptr::null_mut(),
            u_mesh_reconstructor_base_disconnect_mr_mesh: std::ptr::null_mut(),
            u_mesh_reconstructor_base_connect_mr_mesh: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMockDataMeshTrackerComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "OnMockDataMeshTrackerUpdated__DelegateSignature",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_mock_data_mesh_tracker_component_on_mock_data_mesh_tracker_updated_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DisconnectMRMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_mock_data_mesh_tracker_component_disconnect_mr_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConnectMRMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_mock_data_mesh_tracker_component_connect_mr_mesh,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMRMeshComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWireframeMaterial"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_set_wireframe_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWireframeColor"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_set_wireframe_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUseWireframe"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_set_use_wireframe,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnableMeshOcclusion"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_set_enable_mesh_occlusion,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestNavMeshUpdate"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_request_nav_mesh_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsConnected"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_is_connected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWireframeColor"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_get_wireframe_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUseWireframe"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_get_use_wireframe,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnableMeshOcclusion"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_get_enable_mesh_occlusion,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForceNavMeshUpdate"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_force_nav_mesh_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Clear"),
                &raw mut __FUNCTION_PTRS.umr_mesh_component_clear,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshReconstructorBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopReconstruction"),
                &raw mut __FUNCTION_PTRS.u_mesh_reconstructor_base_stop_reconstruction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartReconstruction"),
                &raw mut __FUNCTION_PTRS.u_mesh_reconstructor_base_start_reconstruction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PauseReconstruction"),
                &raw mut __FUNCTION_PTRS.u_mesh_reconstructor_base_pause_reconstruction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsReconstructionStarted"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_reconstructor_base_is_reconstruction_started,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsReconstructionPaused"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_reconstructor_base_is_reconstruction_paused,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DisconnectMRMesh"),
                &raw mut __FUNCTION_PTRS.u_mesh_reconstructor_base_disconnect_mr_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConnectMRMesh"),
                &raw mut __FUNCTION_PTRS.u_mesh_reconstructor_base_connect_mr_mesh,
            );
        }
    }
}
#[repr(C, align(1))]
pub struct FMRMeshConfiguration {
    pub(crate) __padding_end: [u8; 1],
}
impl FMRMeshConfiguration {}
#[repr(C, align(16))]
pub struct UMockDataMeshTrackerComponent {
    #[doc(hidden)]
    pub(crate) __padding_680: [u8; 680],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMockDataMeshTrackerComponent")
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
    pub fn disconnect_mr_mesh(&mut self, in_mr_mesh_ptr: UPtr<UMRMeshComponent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mock_data_mesh_tracker_component_disconnect_mr_mesh,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mock_data_mesh_tracker_component_disconnect_mr_mesh,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mock_data_mesh_tracker_component_connect_mr_mesh,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mock_data_mesh_tracker_component_connect_mr_mesh,
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMRMeshBodyHolder")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMRMeshComponent")
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_set_wireframe_material,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_set_wireframe_material,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_set_wireframe_color,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_set_wireframe_color,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_set_use_wireframe,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_set_use_wireframe,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_set_enable_mesh_occlusion,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_set_enable_mesh_occlusion,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_request_nav_mesh_update,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_request_nav_mesh_update,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_is_connected,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_is_connected,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_get_wireframe_color,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_get_wireframe_color,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_get_use_wireframe,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_get_use_wireframe,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_get_enable_mesh_occlusion,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_get_enable_mesh_occlusion,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_force_nav_mesh_update,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .umr_mesh_component_force_nav_mesh_update,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS.umr_mesh_component_clear,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS.umr_mesh_component_clear,
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshReconstructorBase")
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
    pub fn stop_reconstruction(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mesh_reconstructor_base_stop_reconstruction,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mesh_reconstructor_base_stop_reconstruction,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mesh_reconstructor_base_start_reconstruction,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mesh_reconstructor_base_start_reconstruction,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mesh_reconstructor_base_pause_reconstruction,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mesh_reconstructor_base_pause_reconstruction,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mesh_reconstructor_base_is_reconstruction_started,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mesh_reconstructor_base_is_reconstruction_started,
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
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mesh_reconstructor_base_is_reconstruction_paused,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mr_mesh::__FUNCTION_PTRS
                    .u_mesh_reconstructor_base_is_reconstruction_paused,
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
