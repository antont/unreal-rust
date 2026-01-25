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
    pub u_rig_vm_editor_menu_context_is_alt_down: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_menu_context_get_rig_vm_host: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_menu_context_get_rig_vm_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_menu_context_get_rig_vm_asset_interface: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_menu_context_get_graph_menu_context: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_request_auto_vm_recompilation: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_recompile_vm_if_required: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_recompile_vm: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_load_assets_with_node_filter_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_load_assets_with_blueprint_filter_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_filter_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_and_node_filters_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_and_blueprint_filters_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_load_assets_by_class: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_load_assets: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_get_model: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_get_controller: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_editor_blueprint_library_get_assets_with_filter_for_blueprint: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_rig_vm_editor_menu_context_is_alt_down: std::ptr::null_mut(),
            u_rig_vm_editor_menu_context_get_rig_vm_host: std::ptr::null_mut(),
            u_rig_vm_editor_menu_context_get_rig_vm_blueprint: std::ptr::null_mut(),
            u_rig_vm_editor_menu_context_get_rig_vm_asset_interface: std::ptr::null_mut(),
            u_rig_vm_editor_menu_context_get_graph_menu_context: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_request_auto_vm_recompilation: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_recompile_vm_if_required: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_recompile_vm: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_load_assets_with_node_filter_for_blueprint: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_load_assets_with_blueprint_filter_for_blueprint: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_filter_for_blueprint: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_and_node_filters_for_blueprint: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_and_blueprint_filters_for_blueprint: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_load_assets_by_class: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_load_assets: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_get_model: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_get_controller: std::ptr::null_mut(),
            u_rig_vm_editor_blueprint_library_get_assets_with_filter_for_blueprint: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMEditorMenuContext::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAltDown"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_editor_menu_context_is_alt_down,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigVMHost"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_editor_menu_context_get_rig_vm_host,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigVMBlueprint"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_editor_menu_context_get_rig_vm_blueprint,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigVMAssetInterface"),
            &raw mut __FUNCTION_PTRS
                .u_rig_vm_editor_menu_context_get_rig_vm_asset_interface,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphMenuContext"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_editor_menu_context_get_graph_menu_context,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMEditorBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestAutoVMRecompilation"),
            &raw mut __FUNCTION_PTRS
                .u_rig_vm_editor_blueprint_library_request_auto_vm_recompilation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecompileVMIfRequired"),
            &raw mut __FUNCTION_PTRS
                .u_rig_vm_editor_blueprint_library_recompile_vm_if_required,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecompileVM"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_editor_blueprint_library_recompile_vm,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAssetsWithNodeFilter_ForBlueprint"),
            &raw mut __FUNCTION_PTRS
                .u_rig_vm_editor_blueprint_library_load_assets_with_node_filter_for_blueprint,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAssetsWithBlueprintFilter_ForBlueprint"),
            &raw mut __FUNCTION_PTRS
                .u_rig_vm_editor_blueprint_library_load_assets_with_blueprint_filter_for_blueprint,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAssetsWithAssetDataFilter_ForBlueprint"),
            &raw mut __FUNCTION_PTRS
                .u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_filter_for_blueprint,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "LoadAssetsWithAssetDataAndNodeFilters_ForBlueprint",
            ),
            &raw mut __FUNCTION_PTRS
                .u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_and_node_filters_for_blueprint,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "LoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint",
            ),
            &raw mut __FUNCTION_PTRS
                .u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_and_blueprint_filters_for_blueprint,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAssetsByClass"),
            &raw mut __FUNCTION_PTRS
                .u_rig_vm_editor_blueprint_library_load_assets_by_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAssets"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_editor_blueprint_library_load_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetModel"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_editor_blueprint_library_get_model,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetController"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_editor_blueprint_library_get_controller,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsWithFilter_ForBlueprint"),
            &raw mut __FUNCTION_PTRS
                .u_rig_vm_editor_blueprint_library_get_assets_with_filter_for_blueprint,
        );
    }
}
#[repr(C, align(8))]
pub struct FRigVMBlueprintLoadLogEntry {
    pub severity: ERigVMBlueprintLoadLogSeverity,
    pub subject: UPtr<crate::bindings::core_u_object::UObject>,
    pub message: FString,
}
impl FRigVMBlueprintLoadLogEntry {}
#[repr(C, align(8))]
pub struct FRigVMEditorGraphMenuContext {
    pub graph: UPtr<crate::bindings::rig_vm_developer::URigVMGraph>,
    pub node: UPtr<crate::bindings::rig_vm_developer::URigVMNode>,
    pub pin: UPtr<crate::bindings::rig_vm_developer::URigVMPin>,
}
impl FRigVMEditorGraphMenuContext {}
#[repr(C, align(8))]
pub struct URigVMEdGraphNodeBlueprintSpawner {
    __padding_end: [u8; 288],
}
impl URigVMEdGraphNodeBlueprintSpawner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraphNodeBlueprintSpawner")
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
pub struct URigVMBlueprintCompilerExtension {
    __padding_end: [u8; 48],
}
impl URigVMBlueprintCompilerExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMBlueprintCompilerExtension")
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
pub struct URigVMDetailsViewWrapperObject {
    __padding_end: [u8; 96],
}
impl URigVMDetailsViewWrapperObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMDetailsViewWrapperObject")
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
pub struct URigVMEditorMenuContext {
    __padding_end: [u8; 88],
}
impl URigVMEditorMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEditorMenuContext")
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
    pub fn is_alt_down(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_menu_context_is_alt_down,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_menu_context_is_alt_down,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_rig_vm_host(&self) -> UPtr<crate::bindings::rig_vm::URigVMHost> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_menu_context_get_rig_vm_host,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_menu_context_get_rig_vm_host,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::rig_vm::URigVMHost>>().read()
        }
    }
    pub fn get_rig_vm_blueprint(
        &self,
    ) -> UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_menu_context_get_rig_vm_blueprint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_menu_context_get_rig_vm_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>()
                .read()
        }
    }
    pub fn get_rig_vm_asset_interface(
        &self,
    ) -> TScriptInterface<crate::bindings::rig_vm_developer::URigVMAssetInterface> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_menu_context_get_rig_vm_asset_interface,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_menu_context_get_rig_vm_asset_interface,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TScriptInterface<
                        crate::bindings::rig_vm_developer::URigVMAssetInterface,
                    >,
                >()
                .read()
        }
    }
    pub fn get_graph_menu_context(&mut self) -> FRigVMEditorGraphMenuContext {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_menu_context_get_graph_menu_context,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_menu_context_get_graph_menu_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FRigVMEditorGraphMenuContext>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMEditorBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl URigVMEditorBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEditorBlueprintLibrary")
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
    pub fn request_auto_vm_recompilation(
        in_blueprint: UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_request_auto_vm_recompilation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_request_auto_vm_recompilation,
                __buffer,
            )
        };
    }
    pub fn recompile_vm_if_required(
        in_blueprint: UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_recompile_vm_if_required,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_recompile_vm_if_required,
                __buffer,
            )
        };
    }
    pub fn recompile_vm(
        in_blueprint: UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_recompile_vm,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_recompile_vm,
                __buffer,
            )
        };
    }
    pub fn load_assets_with_node_filter(
        in_class: TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
        in_node_filter: FLoadAssetsWithNodeFilter_ForBlueprint_InNodeFilter,
    ) -> TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_with_node_filter_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_filter,
                __buffer
                    .add(8)
                    .cast::<FLoadAssetsWithNodeFilter_ForBlueprint_InNodeFilter>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_with_node_filter_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<
                    TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>,
                >()
                .read()
        }
    }
    pub fn load_assets_with_blueprint_filter(
        in_class: TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
        in_blueprint_filter: FLoadAssetsWithBlueprintFilter_ForBlueprint_InBlueprintFilter,
    ) -> TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_with_blueprint_filter_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint_filter,
                __buffer
                    .add(8)
                    .cast::<
                        FLoadAssetsWithBlueprintFilter_ForBlueprint_InBlueprintFilter,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_with_blueprint_filter_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<
                    TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>,
                >()
                .read()
        }
    }
    pub fn load_assets_with_asset_data_filter(
        in_class: TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
        in_asset_data_filter: FLoadAssetsWithAssetDataFilter_ForBlueprint_InAssetDataFilter,
    ) -> TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_filter_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset_data_filter,
                __buffer
                    .add(8)
                    .cast::<
                        FLoadAssetsWithAssetDataFilter_ForBlueprint_InAssetDataFilter,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_filter_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<
                    TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>,
                >()
                .read()
        }
    }
    pub fn load_assets_with_asset_data_and_node_filters(
        in_class: TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
        in_asset_data_filter: FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InAssetDataFilter,
        in_node_filter: FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InNodeFilter,
    ) -> TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_and_node_filters_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset_data_filter,
                __buffer
                    .add(8)
                    .cast::<
                        FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InAssetDataFilter,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_filter,
                __buffer
                    .add(40)
                    .cast::<
                        FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InNodeFilter,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_and_node_filters_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(72)
                .cast::<
                    TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>,
                >()
                .read()
        }
    }
    pub fn load_assets_with_asset_data_and_blueprint_filters(
        in_class: TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
        in_asset_data_filter: FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InAssetDataFilter,
        in_blueprint_filter: FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InBlueprintFilter,
    ) -> TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_and_blueprint_filters_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset_data_filter,
                __buffer
                    .add(8)
                    .cast::<
                        FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InAssetDataFilter,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint_filter,
                __buffer
                    .add(40)
                    .cast::<
                        FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InBlueprintFilter,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_with_asset_data_and_blueprint_filters_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(72)
                .cast::<
                    TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>,
                >()
                .read()
        }
    }
    pub fn load_assets_by_class(
        in_class: TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
    ) -> TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_by_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets_by_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>,
                >()
                .read()
        }
    }
    pub fn load_assets() -> TArray<
        UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_load_assets,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TArray<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>,
                >()
                .read()
        }
    }
    pub fn get_model(
        in_blueprint: UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>,
    ) -> UPtr<crate::bindings::rig_vm_developer::URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_get_model,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_get_model,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::rig_vm_developer::URigVMGraph>>()
                .read()
        }
    }
    pub fn get_controller(
        in_blueprint: UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>,
    ) -> UPtr<crate::bindings::rig_vm_developer::URigVMController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_get_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::rig_vm_developer::URigVMBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_get_controller,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::rig_vm_developer::URigVMController>>()
                .read()
        }
    }
    pub fn get_assets_with_filter(
        in_class: TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
        in_asset_data_filter: FGetAssetsWithFilter_ForBlueprint_InAssetDataFilter,
    ) -> TArray<crate::bindings::core_u_object::FAssetData> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_get_assets_with_filter_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<crate::bindings::rig_vm_developer::URigVMBlueprint>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset_data_filter,
                __buffer
                    .add(8)
                    .cast::<FGetAssetsWithFilter_ForBlueprint_InAssetDataFilter>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_editor::URigVMEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_editor::__FUNCTION_PTRS
                    .u_rig_vm_editor_blueprint_library_get_assets_with_filter_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct FGetAssetsWithFilter_ForBlueprint_InAssetDataFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InAssetDataFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InBlueprintFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InAssetDataFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InNodeFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithAssetDataFilter_ForBlueprint_InAssetDataFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithBlueprintFilter_ForBlueprint_InBlueprintFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithNodeFilter_ForBlueprint_InNodeFilter {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct ERigVMBlueprintLoadLogSeverity(pub u8);
impl ERigVMBlueprintLoadLogSeverity {
    pub const DISPLAY: ERigVMBlueprintLoadLogSeverity = ERigVMBlueprintLoadLogSeverity(
        0,
    );
    pub const WARNING: ERigVMBlueprintLoadLogSeverity = ERigVMBlueprintLoadLogSeverity(
        1,
    );
    pub const ERROR: ERigVMBlueprintLoadLogSeverity = ERigVMBlueprintLoadLogSeverity(2);
}
