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
pub static mut U_CONTROL_RIG_BLUEPRINT_UPDATE_EXPOSED_MODULE_CONNECTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_TURN_INTO_STANDALONE_RIG_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_TURN_INTO_CONTROL_RIG_MODULE_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_SET_PREVIEW_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_RECOMPILE_MODULAR_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_IS_CONTROL_RIG_MODULE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_RIG_MODULE_ICON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_PREVIEW_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_MODULAR_RIG_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_HIERARCHY_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_DEBUGGED_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_CURRENTLY_OPEN_RIG_BLUEPRINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_CONTROL_RIG_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_FIND_REFERENCES_TO_MODULE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_CREATE_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_CONVERT_HIERARCHY_ELEMENTS_TO_SPAWNER_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_CAN_TURN_INTO_STANDALONE_RIG_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UControlRigBlueprint::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateExposedModuleConnectors"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_UPDATE_EXPOSED_MODULE_CONNECTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TurnIntoStandaloneRig_Blueprint"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_TURN_INTO_STANDALONE_RIG_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TurnIntoControlRigModule_Blueprint"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_TURN_INTO_CONTROL_RIG_MODULE_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPreviewMesh"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_SET_PREVIEW_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecompileModularRig"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_RECOMPILE_MODULAR_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsControlRigModule"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_IS_CONTROL_RIG_MODULE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigModuleIcon"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_RIG_MODULE_ICON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviewMesh"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_PREVIEW_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetModularRigController"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_MODULAR_RIG_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHierarchyController"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_HIERARCHY_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDebuggedControlRig"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_DEBUGGED_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentlyOpenRigBlueprints"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_CURRENTLY_OPEN_RIG_BLUEPRINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControlRigClass"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_CONTROL_RIG_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindReferencesToModule"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_FIND_REFERENCES_TO_MODULE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateControlRig"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_CREATE_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertHierarchyElementsToSpawnerNodes"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_CONVERT_HIERARCHY_ELEMENTS_TO_SPAWNER_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanTurnIntoStandaloneRig_Blueprint"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_CAN_TURN_INTO_STANDALONE_RIG_BLUEPRINT,
        );
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
    __padding_4648: [u8; 4648],
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_UPDATE_EXPOSED_MODULE_CONNECTORS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_UPDATE_EXPOSED_MODULE_CONNECTORS,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_TURN_INTO_STANDALONE_RIG_BLUEPRINT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_TURN_INTO_STANDALONE_RIG_BLUEPRINT,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_TURN_INTO_CONTROL_RIG_MODULE_BLUEPRINT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_TURN_INTO_CONTROL_RIG_MODULE_BLUEPRINT,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_SET_PREVIEW_MESH,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_SET_PREVIEW_MESH,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_RECOMPILE_MODULAR_RIG,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_RECOMPILE_MODULAR_RIG,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_IS_CONTROL_RIG_MODULE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_IS_CONTROL_RIG_MODULE,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_RIG_MODULE_ICON,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_RIG_MODULE_ICON,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_PREVIEW_MESH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_PREVIEW_MESH,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_MODULAR_RIG_CONTROLLER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_MODULAR_RIG_CONTROLLER,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_HIERARCHY_CONTROLLER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_HIERARCHY_CONTROLLER,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_DEBUGGED_CONTROL_RIG,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_DEBUGGED_CONTROL_RIG,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_CURRENTLY_OPEN_RIG_BLUEPRINTS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_developer::UControlRigBlueprint::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_CURRENTLY_OPEN_RIG_BLUEPRINTS,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_CONTROL_RIG_CLASS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_GET_CONTROL_RIG_CLASS,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_FIND_REFERENCES_TO_MODULE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_FIND_REFERENCES_TO_MODULE,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_CREATE_CONTROL_RIG,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_CREATE_CONTROL_RIG,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_CONVERT_HIERARCHY_ELEMENTS_TO_SPAWNER_NODES,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_CONVERT_HIERARCHY_ELEMENTS_TO_SPAWNER_NODES,
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
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_CAN_TURN_INTO_STANDALONE_RIG_BLUEPRINT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_developer::U_CONTROL_RIG_BLUEPRINT_CAN_TURN_INTO_STANDALONE_RIG_BLUEPRINT,
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
