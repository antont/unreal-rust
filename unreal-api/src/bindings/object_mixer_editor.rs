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
pub static mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_SHOULD_INCLUDE_UNSUPPORTED_PROPERTIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_SHOW_TRANSIENT_OBJECTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_PROPERTIES_THAT_REQUIRE_LIST_REFRESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_MIXER_PROPERTY_INHERITANCE_INCLUSION_OPTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_MIXER_PLACEMENT_CLASS_INCLUSION_OPTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_CLASSES_TO_PLACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_CLASSES_TO_FILTER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_FORCE_ADDED_COLUMNS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_COLUMNS_TO_SHOW_BY_DEFAULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_COLUMNS_TO_EXCLUDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UObjectMixerBlueprintObjectFilter::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldIncludeUnsupportedProperties"),
            &raw mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_SHOULD_INCLUDE_UNSUPPORTED_PROPERTIES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetShowTransientObjects"),
            &raw mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_SHOW_TRANSIENT_OBJECTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPropertiesThatRequireListRefresh"),
            &raw mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_PROPERTIES_THAT_REQUIRE_LIST_REFRESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "GetObjectMixerPropertyInheritanceInclusionOptions",
            ),
            &raw mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_MIXER_PROPERTY_INHERITANCE_INCLUSION_OPTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectMixerPlacementClassInclusionOptions"),
            &raw mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_MIXER_PLACEMENT_CLASS_INCLUSION_OPTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectClassesToPlace"),
            &raw mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_CLASSES_TO_PLACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectClassesToFilter"),
            &raw mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_CLASSES_TO_FILTER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetForceAddedColumns"),
            &raw mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_FORCE_ADDED_COLUMNS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetColumnsToShowByDefault"),
            &raw mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_COLUMNS_TO_SHOW_BY_DEFAULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetColumnsToExclude"),
            &raw mut U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_COLUMNS_TO_EXCLUDE,
        );
    }
}
#[repr(C, align(8))]
pub struct FObjectMixerWidgetUserConfig {
    pub default_filter_class: TSubclassOf<UObjectMixerObjectFilter>,
}
impl FObjectMixerWidgetUserConfig {}
#[repr(C, align(8))]
pub struct UObjectMixerEditorSettings {
    __padding_end: [u8; 112],
}
impl UObjectMixerEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectMixerEditorSettings")
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
pub struct UObjectMixerObjectFilter {
    __padding_end: [u8; 48],
}
impl UObjectMixerObjectFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectMixerObjectFilter")
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
pub struct UObjectMixerBlueprintObjectFilter {
    __padding_end: [u8; 48],
}
impl UObjectMixerBlueprintObjectFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectMixerBlueprintObjectFilter")
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
    pub fn should_include_unsupported_properties(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_SHOULD_INCLUDE_UNSUPPORTED_PROPERTIES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_SHOULD_INCLUDE_UNSUPPORTED_PROPERTIES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_show_transient_objects(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_SHOW_TRANSIENT_OBJECTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_SHOW_TRANSIENT_OBJECTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_properties_that_require_list_refresh(&self) -> TSet<FName> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_PROPERTIES_THAT_REQUIRE_LIST_REFRESH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_PROPERTIES_THAT_REQUIRE_LIST_REFRESH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TSet<FName>>().read() }
    }
    pub fn get_object_mixer_property_inheritance_inclusion_options(
        &self,
    ) -> EObjectMixerInheritanceInclusionOptions {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_MIXER_PROPERTY_INHERITANCE_INCLUSION_OPTIONS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_MIXER_PROPERTY_INHERITANCE_INCLUSION_OPTIONS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EObjectMixerInheritanceInclusionOptions>().read()
        }
    }
    pub fn get_object_mixer_placement_class_inclusion_options(
        &self,
    ) -> EObjectMixerInheritanceInclusionOptions {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_MIXER_PLACEMENT_CLASS_INCLUSION_OPTIONS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_MIXER_PLACEMENT_CLASS_INCLUSION_OPTIONS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EObjectMixerInheritanceInclusionOptions>().read()
        }
    }
    pub fn get_object_classes_to_place(
        &self,
    ) -> TSet<TSubclassOf<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_CLASSES_TO_PLACE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_CLASSES_TO_PLACE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSet<TSubclassOf<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_object_classes_to_filter(
        &self,
    ) -> TSet<TSubclassOf<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_CLASSES_TO_FILTER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_OBJECT_CLASSES_TO_FILTER,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSet<TSubclassOf<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn get_force_added_columns(&self) -> TSet<FName> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_FORCE_ADDED_COLUMNS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_FORCE_ADDED_COLUMNS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TSet<FName>>().read() }
    }
    pub fn get_columns_to_show_by_default(&self) -> TSet<FName> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_COLUMNS_TO_SHOW_BY_DEFAULT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_COLUMNS_TO_SHOW_BY_DEFAULT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TSet<FName>>().read() }
    }
    pub fn get_columns_to_exclude(&self) -> TSet<FName> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_COLUMNS_TO_EXCLUDE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::object_mixer_editor::U_OBJECT_MIXER_BLUEPRINT_OBJECT_FILTER_GET_COLUMNS_TO_EXCLUDE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TSet<FName>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UObjectMixerEditorSerializedData {
    __padding_end: [u8; 192],
}
impl UObjectMixerEditorSerializedData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectMixerEditorSerializedData")
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
pub struct UObjectMixerBlueprintFilterFactory {
    __padding_end: [u8; 144],
}
impl UObjectMixerBlueprintFilterFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectMixerBlueprintFilterFactory")
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
pub struct UObjectMixerOutlinerModeEditorConfig {
    __padding_end: [u8; 128],
}
impl UObjectMixerOutlinerModeEditorConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectMixerOutlinerModeEditorConfig")
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
pub struct UObjectMixerEditorListMenuContext {
    __padding_end: [u8; 80],
}
impl UObjectMixerEditorListMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectMixerEditorListMenuContext")
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
pub struct UObjectMixerEditorUWidget {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub object_mixer_widget_user_config: FObjectMixerWidgetUserConfig,
}
impl UObjectMixerEditorUWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectMixerEditorUWidget")
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
pub struct EListViewColumnType(pub i32);
impl EListViewColumnType {
    pub const BUILT_IN: EListViewColumnType = EListViewColumnType(0);
    pub const PROPERTY_GENERATED: EListViewColumnType = EListViewColumnType(1);
}
#[repr(transparent)]
pub struct EObjectMixerInheritanceInclusionOptions(pub u8);
impl EObjectMixerInheritanceInclusionOptions {
    pub const NONE: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        0,
    );
    pub const INCLUDE_ONLY_IMMEDIATE_PARENT: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        1,
    );
    pub const INCLUDE_ONLY_IMMEDIATE_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        2,
    );
    pub const INCLUDE_ONLY_IMMEDIATE_PARENT_AND_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        3,
    );
    pub const INCLUDE_ALL_PARENTS: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        4,
    );
    pub const INCLUDE_ALL_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        5,
    );
    pub const INCLUDE_ALL_PARENTS_AND_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        6,
    );
    pub const INCLUDE_ALL_PARENTS_AND_ONLY_IMMEDIATE_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        7,
    );
    pub const INCLUDE_ONLY_IMMEDIATE_PARENT_AND_ALL_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        8,
    );
}
#[repr(transparent)]
pub struct EObjectMixerHybridMode(pub u8);
impl EObjectMixerHybridMode {
    pub const HYBRID_ACTOR: EObjectMixerHybridMode = EObjectMixerHybridMode(0);
    pub const HYBRID_COMPONENT: EObjectMixerHybridMode = EObjectMixerHybridMode(1);
    pub const HYBRID_NONE: EObjectMixerHybridMode = EObjectMixerHybridMode(2);
}
