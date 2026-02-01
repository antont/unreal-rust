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
    pub u_control_rig_blueprint_update_exposed_module_connectors: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_turn_into_standalone_rig_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_turn_into_control_rig_module_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_set_preview_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_recompile_modular_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_is_control_rig_module: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_get_rig_module_icon: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_get_preview_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_get_modular_rig_controller: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_get_hierarchy_controller: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_get_debugged_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_get_currently_open_rig_blueprints: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_get_control_rig_class: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_find_references_to_module: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_create_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_convert_hierarchy_elements_to_spawner_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_can_turn_into_standalone_rig_blueprint: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_control_rig_blueprint_update_exposed_module_connectors: std::ptr::null_mut(),
            u_control_rig_blueprint_turn_into_standalone_rig_blueprint: std::ptr::null_mut(),
            u_control_rig_blueprint_turn_into_control_rig_module_blueprint: std::ptr::null_mut(),
            u_control_rig_blueprint_set_preview_mesh: std::ptr::null_mut(),
            u_control_rig_blueprint_recompile_modular_rig: std::ptr::null_mut(),
            u_control_rig_blueprint_is_control_rig_module: std::ptr::null_mut(),
            u_control_rig_blueprint_get_rig_module_icon: std::ptr::null_mut(),
            u_control_rig_blueprint_get_preview_mesh: std::ptr::null_mut(),
            u_control_rig_blueprint_get_modular_rig_controller: std::ptr::null_mut(),
            u_control_rig_blueprint_get_hierarchy_controller: std::ptr::null_mut(),
            u_control_rig_blueprint_get_debugged_control_rig: std::ptr::null_mut(),
            u_control_rig_blueprint_get_currently_open_rig_blueprints: std::ptr::null_mut(),
            u_control_rig_blueprint_get_control_rig_class: std::ptr::null_mut(),
            u_control_rig_blueprint_find_references_to_module: std::ptr::null_mut(),
            u_control_rig_blueprint_create_control_rig: std::ptr::null_mut(),
            u_control_rig_blueprint_convert_hierarchy_elements_to_spawner_nodes: std::ptr::null_mut(),
            u_control_rig_blueprint_can_turn_into_standalone_rig_blueprint: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigBlueprint::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateExposedModuleConnectors"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_update_exposed_module_connectors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TurnIntoStandaloneRig_Blueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_turn_into_standalone_rig_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TurnIntoControlRigModule_Blueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_turn_into_control_rig_module_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPreviewMesh"),
                &raw mut __FUNCTION_PTRS.u_control_rig_blueprint_set_preview_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RecompileModularRig"),
                &raw mut __FUNCTION_PTRS.u_control_rig_blueprint_recompile_modular_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsControlRigModule"),
                &raw mut __FUNCTION_PTRS.u_control_rig_blueprint_is_control_rig_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRigModuleIcon"),
                &raw mut __FUNCTION_PTRS.u_control_rig_blueprint_get_rig_module_icon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPreviewMesh"),
                &raw mut __FUNCTION_PTRS.u_control_rig_blueprint_get_preview_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModularRigController"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_get_modular_rig_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHierarchyController"),
                &raw mut __FUNCTION_PTRS.u_control_rig_blueprint_get_hierarchy_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDebuggedControlRig"),
                &raw mut __FUNCTION_PTRS.u_control_rig_blueprint_get_debugged_control_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentlyOpenRigBlueprints"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_get_currently_open_rig_blueprints,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlRigClass"),
                &raw mut __FUNCTION_PTRS.u_control_rig_blueprint_get_control_rig_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindReferencesToModule"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_find_references_to_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateControlRig"),
                &raw mut __FUNCTION_PTRS.u_control_rig_blueprint_create_control_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertHierarchyElementsToSpawnerNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_convert_hierarchy_elements_to_spawner_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanTurnIntoStandaloneRig_Blueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_can_turn_into_standalone_rig_blueprint,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ControlRig {
    __padding_end: [u8; 2104],
}
impl UAnimGraphNode_ControlRig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimGraphNode_ControlRig")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimGraphNode_ControlRig")
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
pub struct IControlRigAssetInterface {}
#[repr(C, align(8))]
pub struct UControlRigAssetInterface {
    __padding_end: [u8; 48],
}
impl UControlRigAssetInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigAssetInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigAssetInterface")
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
#[repr(C, align(8))]
pub struct UControlRigBlueprint {
    #[doc(hidden)]
    pub(crate) __padding_4648: [u8; 4648],
    pub hierarchy: UPtr<crate::bindings::control_rig::URigHierarchy>,
    pub modular_rig_model: crate::bindings::control_rig::FModularRigModel,
    __padding_end: [u8; 288],
}
impl UControlRigBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprint")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprint")
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
    pub fn update_exposed_module_connectors(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_update_exposed_module_connectors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_update_exposed_module_connectors,
                __buffer,
            )
        };
    }
    pub fn turn_into_standalone_rig(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_turn_into_standalone_rig_blueprint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_turn_into_standalone_rig_blueprint,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn turn_into_control_rig_module(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_turn_into_control_rig_module_blueprint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_turn_into_control_rig_module_blueprint,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_preview_mesh(
        &mut self,
        preview_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        b_mark_as_dirty: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_set_preview_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mark_as_dirty,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_set_preview_mesh,
                __buffer,
            )
        };
    }
    pub fn recompile_modular_rig(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_recompile_modular_rig,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_recompile_modular_rig,
                __buffer,
            )
        };
    }
    pub fn is_control_rig_module(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_is_control_rig_module,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_is_control_rig_module,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_rig_module_icon(&self) -> UPtr<crate::bindings::engine::UTexture2D> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_rig_module_icon,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_rig_module_icon,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture2D>>().read()
        }
    }
    pub fn get_preview_mesh(&self) -> UPtr<crate::bindings::engine::USkeletalMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_preview_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_preview_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>().read()
        }
    }
    pub fn get_modular_rig_controller(
        &mut self,
    ) -> UPtr<crate::bindings::control_rig::UModularRigController> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_modular_rig_controller,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_modular_rig_controller,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::control_rig::UModularRigController>>()
                .read()
        }
    }
    pub fn get_hierarchy_controller(
        &mut self,
    ) -> UPtr<crate::bindings::control_rig::URigHierarchyController> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_hierarchy_controller,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_hierarchy_controller,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::control_rig::URigHierarchyController>>()
                .read()
        }
    }
    pub fn get_debugged_control_rig(
        &mut self,
    ) -> UPtr<crate::bindings::control_rig::UControlRig> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_debugged_control_rig,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_debugged_control_rig,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::control_rig::UControlRig>>()
                .read()
        }
    }
    pub fn get_currently_open_rig_blueprints() -> TArray<UPtr<UControlRigBlueprint>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_currently_open_rig_blueprints,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_developer::UControlRigBlueprint::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_currently_open_rig_blueprints,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UControlRigBlueprint>>>().read() }
    }
    pub fn get_control_rig_class(
        &self,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_control_rig_class,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_get_control_rig_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn find_references_to_module(
        &self,
    ) -> TArray<crate::bindings::control_rig::FModuleReferenceData> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_find_references_to_module,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_find_references_to_module,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::control_rig::FModuleReferenceData>>()
                .read()
        }
    }
    pub fn create_control_rig(
        &mut self,
    ) -> UPtr<crate::bindings::control_rig::UControlRig> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_create_control_rig,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_create_control_rig,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::control_rig::UControlRig>>()
                .read()
        }
    }
    pub fn convert_hierarchy_elements_to_spawner_nodes(
        &mut self,
        in_hierarchy: UPtr<crate::bindings::control_rig::URigHierarchy>,
        in_keys: TArray<crate::bindings::control_rig::FRigElementKey>,
        b_remove_elements: bool,
    ) -> TArray<UPtr<crate::bindings::rig_vm_developer::URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_convert_hierarchy_elements_to_spawner_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_hierarchy,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::URigHierarchy>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_keys,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::control_rig::FRigElementKey>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_elements,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_convert_hierarchy_elements_to_spawner_nodes,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::rig_vm_developer::URigVMNode>>>()
                .read()
        }
    }
    pub fn can_turn_into_standalone_rig(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_can_turn_into_standalone_rig_blueprint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::__FUNCTION_PTRS
                    .u_control_rig_blueprint_can_turn_into_standalone_rig_blueprint,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UControlRigSchema {
    __padding_end: [u8; 80],
}
impl UControlRigSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSchema")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSchema")
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
#[repr(C, align(8))]
pub struct UControlRigGraph {
    __padding_end: [u8; 592],
}
impl UControlRigGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigGraph")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigGraph")
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
#[repr(C, align(8))]
pub struct UControlRigGraphNode {
    __padding_end: [u8; 1152],
}
impl UControlRigGraphNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigGraphNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigGraphNode")
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
#[repr(C, align(8))]
pub struct UControlRigGraphSchema {
    __padding_end: [u8; 168],
}
impl UControlRigGraphSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigGraphSchema")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigGraphSchema")
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
