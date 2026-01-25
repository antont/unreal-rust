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
    pub u_clothing_simulation_interactor_set_num_substeps: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_set_num_iterations: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_set_max_num_iterations: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_set_anim_drive_spring_stiffness: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_physics_asset_updated: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_get_simulation_time: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_get_num_substeps: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_get_num_kinematic_particles: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_get_num_iterations: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_get_num_dynamic_particles: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_get_num_cloths: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_get_clothing_interactor: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_enable_gravity_override: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_disable_gravity_override: *mut crate::ffi::UFunctionOpague,
    pub u_clothing_simulation_interactor_cloth_config_updated: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_clothing_simulation_interactor_set_num_substeps: std::ptr::null_mut(),
            u_clothing_simulation_interactor_set_num_iterations: std::ptr::null_mut(),
            u_clothing_simulation_interactor_set_max_num_iterations: std::ptr::null_mut(),
            u_clothing_simulation_interactor_set_anim_drive_spring_stiffness: std::ptr::null_mut(),
            u_clothing_simulation_interactor_physics_asset_updated: std::ptr::null_mut(),
            u_clothing_simulation_interactor_get_simulation_time: std::ptr::null_mut(),
            u_clothing_simulation_interactor_get_num_substeps: std::ptr::null_mut(),
            u_clothing_simulation_interactor_get_num_kinematic_particles: std::ptr::null_mut(),
            u_clothing_simulation_interactor_get_num_iterations: std::ptr::null_mut(),
            u_clothing_simulation_interactor_get_num_dynamic_particles: std::ptr::null_mut(),
            u_clothing_simulation_interactor_get_num_cloths: std::ptr::null_mut(),
            u_clothing_simulation_interactor_get_clothing_interactor: std::ptr::null_mut(),
            u_clothing_simulation_interactor_enable_gravity_override: std::ptr::null_mut(),
            u_clothing_simulation_interactor_disable_gravity_override: std::ptr::null_mut(),
            u_clothing_simulation_interactor_cloth_config_updated: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UClothingSimulationInteractor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNumSubsteps"),
            &raw mut __FUNCTION_PTRS.u_clothing_simulation_interactor_set_num_substeps,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNumIterations"),
            &raw mut __FUNCTION_PTRS.u_clothing_simulation_interactor_set_num_iterations,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxNumIterations"),
            &raw mut __FUNCTION_PTRS
                .u_clothing_simulation_interactor_set_max_num_iterations,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimDriveSpringStiffness"),
            &raw mut __FUNCTION_PTRS
                .u_clothing_simulation_interactor_set_anim_drive_spring_stiffness,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PhysicsAssetUpdated"),
            &raw mut __FUNCTION_PTRS
                .u_clothing_simulation_interactor_physics_asset_updated,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSimulationTime"),
            &raw mut __FUNCTION_PTRS.u_clothing_simulation_interactor_get_simulation_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumSubsteps"),
            &raw mut __FUNCTION_PTRS.u_clothing_simulation_interactor_get_num_substeps,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumKinematicParticles"),
            &raw mut __FUNCTION_PTRS
                .u_clothing_simulation_interactor_get_num_kinematic_particles,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumIterations"),
            &raw mut __FUNCTION_PTRS.u_clothing_simulation_interactor_get_num_iterations,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumDynamicParticles"),
            &raw mut __FUNCTION_PTRS
                .u_clothing_simulation_interactor_get_num_dynamic_particles,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumCloths"),
            &raw mut __FUNCTION_PTRS.u_clothing_simulation_interactor_get_num_cloths,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetClothingInteractor"),
            &raw mut __FUNCTION_PTRS
                .u_clothing_simulation_interactor_get_clothing_interactor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableGravityOverride"),
            &raw mut __FUNCTION_PTRS
                .u_clothing_simulation_interactor_enable_gravity_override,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisableGravityOverride"),
            &raw mut __FUNCTION_PTRS
                .u_clothing_simulation_interactor_disable_gravity_override,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClothConfigUpdated"),
            &raw mut __FUNCTION_PTRS
                .u_clothing_simulation_interactor_cloth_config_updated,
        );
    }
}
#[repr(C, align(8))]
pub struct UClothConfigBase {
    __padding_end: [u8; 48],
}
impl UClothConfigBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothConfigBase")
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
pub struct UDEPRECATED_ClothSharedSimConfigBase {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_ClothSharedSimConfigBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ClothSharedSimConfigBase")
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
pub struct UClothingAssetBase {
    __padding_end: [u8; 80],
}
impl UClothingAssetBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingAssetBase")
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
pub struct UClothingSimulationFactory {
    __padding_end: [u8; 48],
}
impl UClothingSimulationFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingSimulationFactory")
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
pub struct UClothingInteractor {
    __padding_end: [u8; 56],
}
impl UClothingInteractor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingInteractor")
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
pub struct UClothingSimulationInteractor {
    __padding_end: [u8; 152],
}
impl UClothingSimulationInteractor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingSimulationInteractor")
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
    pub fn set_num_substeps(&mut self, num_substeps: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_set_num_substeps,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &num_substeps,
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
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_set_num_substeps,
                __buffer,
            )
        };
    }
    pub fn set_num_iterations(&mut self, num_iterations: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_set_num_iterations,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &num_iterations,
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
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_set_num_iterations,
                __buffer,
            )
        };
    }
    pub fn set_max_num_iterations(&mut self, max_num_iterations: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_set_max_num_iterations,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_num_iterations,
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
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_set_max_num_iterations,
                __buffer,
            )
        };
    }
    pub fn set_anim_drive_spring_stiffness(&mut self, in_stiffness: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_set_anim_drive_spring_stiffness,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_stiffness,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_set_anim_drive_spring_stiffness,
                __buffer,
            )
        };
    }
    pub fn physics_asset_updated(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_physics_asset_updated,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_physics_asset_updated,
                __buffer,
            )
        };
    }
    pub fn get_simulation_time(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_simulation_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_simulation_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_num_substeps(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_num_substeps,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_num_substeps,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_num_kinematic_particles(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_num_kinematic_particles,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_num_kinematic_particles,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_num_iterations(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_num_iterations,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_num_iterations,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_num_dynamic_particles(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_num_dynamic_particles,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_num_dynamic_particles,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_num_cloths(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_num_cloths,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_num_cloths,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_clothing_interactor(
        &self,
        clothing_asset_name: FName,
    ) -> UPtr<UClothingInteractor> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_clothing_interactor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clothing_asset_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_get_clothing_interactor,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UClothingInteractor>>().read() }
    }
    pub fn enable_gravity_override(
        &mut self,
        in_vector: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_enable_gravity_override,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_vector,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_enable_gravity_override,
                __buffer,
            )
        };
    }
    pub fn disable_gravity_override(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_disable_gravity_override,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_disable_gravity_override,
                __buffer,
            )
        };
    }
    pub fn cloth_config_updated(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_cloth_config_updated,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::clothing_system_runtime_interface::__FUNCTION_PTRS
                    .u_clothing_simulation_interactor_cloth_config_updated,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UClothPhysicalMeshDataBase_Legacy {
    __padding_end: [u8; 248],
}
impl UClothPhysicalMeshDataBase_Legacy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPhysicalMeshDataBase_Legacy")
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
pub struct EClothingTeleportMode(pub u8);
impl EClothingTeleportMode {
    pub const NONE: EClothingTeleportMode = EClothingTeleportMode(0);
    pub const TELEPORT: EClothingTeleportMode = EClothingTeleportMode(1);
    pub const TELEPORT_AND_RESET: EClothingTeleportMode = EClothingTeleportMode(2);
    pub const HARD_RESET: EClothingTeleportMode = EClothingTeleportMode(3);
}
