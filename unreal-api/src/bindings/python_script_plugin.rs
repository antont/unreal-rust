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
pub static mut U_PY_TEST_INTERFACE_FUNC_INTERFACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_CHILD_INTERFACE_FUNC_INTERFACE_CHILD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OTHER_INTERFACE_FUNC_INTERFACE_OTHER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_PYTHON_SCRIPTING_LIBRARY_SET_KEEP_PYTHON_SCRIPT_ALIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_PYTHON_SCRIPTING_LIBRARY_GET_KEEP_PYTHON_SCRIPT_ALIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_LIBRARY_SET_BOOL_MUTABLE_VIA_REF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_LIBRARY_SET_BOOL_MUTABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_LIBRARY_LEGACY_IS_BOOL_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_LIBRARY_IS_BOOL_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_LIBRARY_GET_CONSTANT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_LIBRARY_CLEAR_BOOL_MUTABLE_VIA_REF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_LIBRARY_CLEAR_BOOL_MUTABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_LIBRARY_ADD_STR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_LIBRARY_ADD_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_LIBRARY_ADD_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_RETURN_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_RETURN_MAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_RETURN_FIELD_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_RETURN_ARRAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_MULTICAST_DELEGATE_PROPERTY_CALLBACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_LEGACY_FUNC_TAKING_PY_TEST_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_GET_CONSTANT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_FUNC_TAKING_PY_TEST_STRUCT_DEFAULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_FUNC_TAKING_PY_TEST_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_FUNC_TAKING_PY_TEST_DELEGATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_FUNC_TAKING_PY_TEST_CHILD_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_FUNC_TAKING_FIELD_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_FUNC_BLUEPRINT_NATIVE_REF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_FUNC_BLUEPRINT_NATIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_FUNC_BLUEPRINT_IMPLEMENTABLE_PACKED_GETTER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_FUNC_BLUEPRINT_IMPLEMENTABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_EMIT_SCRIPT_WARNING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_EMIT_SCRIPT_ERROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_DELEGATE_PROPERTY_CALLBACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_CALL_FUNC_BLUEPRINT_NATIVE_REF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_CALL_FUNC_BLUEPRINT_NATIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_CALL_FUNC_BLUEPRINT_IMPLEMENTABLE_PACKED_GETTER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_CALL_FUNC_BLUEPRINT_IMPLEMENTABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_LIBRARY_IS_BOOL_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_OBJECT_LIBRARY_GET_OTHER_CONSTANT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_STRUCT_DELEGATE_ON_NAME_COLLISION_DELEGATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_VECTOR_DELEGATE_ON_NAME_COLLISION_DELEGATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_GET_STRING_CONST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_GET_INT_CONST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_TUPLE_RETURN_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_TEXT_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_STRUCT_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_STRING_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_STATIC_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_SET_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_OBJECT_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_NAME_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_MAP_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_INTEGER_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_FLOAT_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_FIELD_PATH_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_ENUM_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_DELEGATE_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_BOOL_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PY_TEST_TYPE_HINT_CHECK_ARRAY_TYPE_HINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PYTHON_SCRIPT_LIBRARY_IS_PYTHON_INITIALIZED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PYTHON_SCRIPT_LIBRARY_IS_PYTHON_CONFIGURED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PYTHON_SCRIPT_LIBRARY_IS_PYTHON_AVAILABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PYTHON_SCRIPT_LIBRARY_FORCE_ENABLE_PYTHON_AT_RUNTIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PYTHON_SCRIPT_LIBRARY_EXECUTE_PYTHON_SCRIPT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PYTHON_SCRIPT_LIBRARY_EXECUTE_PYTHON_COMMAND_EX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PYTHON_SCRIPT_LIBRARY_EXECUTE_PYTHON_COMMAND: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncInterface"),
            &raw mut U_PY_TEST_INTERFACE_FUNC_INTERFACE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestChildInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncInterfaceChild"),
            &raw mut U_PY_TEST_CHILD_INTERFACE_FUNC_INTERFACE_CHILD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestOtherInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncInterfaceOther"),
            &raw mut U_PY_TEST_OTHER_INTERFACE_FUNC_INTERFACE_OTHER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorPythonScriptingLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetKeepPythonScriptAlive"),
            &raw mut U_EDITOR_PYTHON_SCRIPTING_LIBRARY_SET_KEEP_PYTHON_SCRIPT_ALIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeepPythonScriptAlive"),
            &raw mut U_EDITOR_PYTHON_SCRIPTING_LIBRARY_GET_KEEP_PYTHON_SCRIPT_ALIVE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestStructLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolMutableViaRef"),
            &raw mut U_PY_TEST_STRUCT_LIBRARY_SET_BOOL_MUTABLE_VIA_REF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolMutable"),
            &raw mut U_PY_TEST_STRUCT_LIBRARY_SET_BOOL_MUTABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LegacyIsBoolSet"),
            &raw mut U_PY_TEST_STRUCT_LIBRARY_LEGACY_IS_BOOL_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsBoolSet"),
            &raw mut U_PY_TEST_STRUCT_LIBRARY_IS_BOOL_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConstantValue"),
            &raw mut U_PY_TEST_STRUCT_LIBRARY_GET_CONSTANT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearBoolMutableViaRef"),
            &raw mut U_PY_TEST_STRUCT_LIBRARY_CLEAR_BOOL_MUTABLE_VIA_REF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearBoolMutable"),
            &raw mut U_PY_TEST_STRUCT_LIBRARY_CLEAR_BOOL_MUTABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddStr"),
            &raw mut U_PY_TEST_STRUCT_LIBRARY_ADD_STR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInt"),
            &raw mut U_PY_TEST_STRUCT_LIBRARY_ADD_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFloat"),
            &raw mut U_PY_TEST_STRUCT_LIBRARY_ADD_FLOAT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestObject::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReturnSet"),
            &raw mut U_PY_TEST_OBJECT_RETURN_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReturnMap"),
            &raw mut U_PY_TEST_OBJECT_RETURN_MAP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReturnFieldPath"),
            &raw mut U_PY_TEST_OBJECT_RETURN_FIELD_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReturnArray"),
            &raw mut U_PY_TEST_OBJECT_RETURN_ARRAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MulticastDelegatePropertyCallback"),
            &raw mut U_PY_TEST_OBJECT_MULTICAST_DELEGATE_PROPERTY_CALLBACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LegacyFuncTakingPyTestStruct"),
            &raw mut U_PY_TEST_OBJECT_LEGACY_FUNC_TAKING_PY_TEST_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConstantValue"),
            &raw mut U_PY_TEST_OBJECT_GET_CONSTANT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncTakingPyTestStructDefault"),
            &raw mut U_PY_TEST_OBJECT_FUNC_TAKING_PY_TEST_STRUCT_DEFAULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncTakingPyTestStruct"),
            &raw mut U_PY_TEST_OBJECT_FUNC_TAKING_PY_TEST_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncTakingPyTestDelegate"),
            &raw mut U_PY_TEST_OBJECT_FUNC_TAKING_PY_TEST_DELEGATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncTakingPyTestChildStruct"),
            &raw mut U_PY_TEST_OBJECT_FUNC_TAKING_PY_TEST_CHILD_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncTakingFieldPath"),
            &raw mut U_PY_TEST_OBJECT_FUNC_TAKING_FIELD_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncBlueprintNativeRef"),
            &raw mut U_PY_TEST_OBJECT_FUNC_BLUEPRINT_NATIVE_REF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncBlueprintNative"),
            &raw mut U_PY_TEST_OBJECT_FUNC_BLUEPRINT_NATIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncBlueprintImplementablePackedGetter"),
            &raw mut U_PY_TEST_OBJECT_FUNC_BLUEPRINT_IMPLEMENTABLE_PACKED_GETTER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncBlueprintImplementable"),
            &raw mut U_PY_TEST_OBJECT_FUNC_BLUEPRINT_IMPLEMENTABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EmitScriptWarning"),
            &raw mut U_PY_TEST_OBJECT_EMIT_SCRIPT_WARNING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EmitScriptError"),
            &raw mut U_PY_TEST_OBJECT_EMIT_SCRIPT_ERROR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DelegatePropertyCallback"),
            &raw mut U_PY_TEST_OBJECT_DELEGATE_PROPERTY_CALLBACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CallFuncBlueprintNativeRef"),
            &raw mut U_PY_TEST_OBJECT_CALL_FUNC_BLUEPRINT_NATIVE_REF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CallFuncBlueprintNative"),
            &raw mut U_PY_TEST_OBJECT_CALL_FUNC_BLUEPRINT_NATIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CallFuncBlueprintImplementablePackedGetter"),
            &raw mut U_PY_TEST_OBJECT_CALL_FUNC_BLUEPRINT_IMPLEMENTABLE_PACKED_GETTER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CallFuncBlueprintImplementable"),
            &raw mut U_PY_TEST_OBJECT_CALL_FUNC_BLUEPRINT_IMPLEMENTABLE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestObjectLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsBoolSet"),
            &raw mut U_PY_TEST_OBJECT_LIBRARY_IS_BOOL_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOtherConstantValue"),
            &raw mut U_PY_TEST_OBJECT_LIBRARY_GET_OTHER_CONSTANT_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestStructDelegate::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnNameCollisionDelegate__DelegateSignature"),
            &raw mut U_PY_TEST_STRUCT_DELEGATE_ON_NAME_COLLISION_DELEGATE_DELEGATE_SIGNATURE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestVectorDelegate::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnNameCollisionDelegate__DelegateSignature"),
            &raw mut U_PY_TEST_VECTOR_DELEGATE_ON_NAME_COLLISION_DELEGATE_DELEGATE_SIGNATURE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestTypeHint::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStringConst"),
            &raw mut U_PY_TEST_TYPE_HINT_GET_STRING_CONST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIntConst"),
            &raw mut U_PY_TEST_TYPE_HINT_GET_INT_CONST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTupleReturnType"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_TUPLE_RETURN_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTextTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_TEXT_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckStructTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_STRUCT_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckStringTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_STRING_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckStaticFunction"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_STATIC_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSetTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_SET_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckObjectTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_OBJECT_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckNameTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_NAME_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckMapTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_MAP_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckIntegerTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_INTEGER_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckFloatTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_FLOAT_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckFieldPathTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_FIELD_PATH_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckEnumTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_ENUM_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckDelegateTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_DELEGATE_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckBoolTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_BOOL_TYPE_HINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckArrayTypeHints"),
            &raw mut U_PY_TEST_TYPE_HINT_CHECK_ARRAY_TYPE_HINTS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPythonScriptLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPythonInitialized"),
            &raw mut U_PYTHON_SCRIPT_LIBRARY_IS_PYTHON_INITIALIZED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPythonConfigured"),
            &raw mut U_PYTHON_SCRIPT_LIBRARY_IS_PYTHON_CONFIGURED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPythonAvailable"),
            &raw mut U_PYTHON_SCRIPT_LIBRARY_IS_PYTHON_AVAILABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForceEnablePythonAtRuntime"),
            &raw mut U_PYTHON_SCRIPT_LIBRARY_FORCE_ENABLE_PYTHON_AT_RUNTIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExecutePythonScript"),
            &raw mut U_PYTHON_SCRIPT_LIBRARY_EXECUTE_PYTHON_SCRIPT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExecutePythonCommandEx"),
            &raw mut U_PYTHON_SCRIPT_LIBRARY_EXECUTE_PYTHON_COMMAND_EX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExecutePythonCommand"),
            &raw mut U_PYTHON_SCRIPT_LIBRARY_EXECUTE_PYTHON_COMMAND,
        );
    }
}
#[repr(C, align(8))]
pub struct FPyTestStruct {
    pub bool: bool,
    pub int: i32,
    pub float: f32,
    pub enum_: EPyTestEnum,
    pub string: FString,
    pub name: FName,
    pub text: FText,
    pub field_path: TFieldPath<FProperty>,
    pub struct_field_path: TFieldPath<FStructProperty>,
    pub string_array: TArray<FString>,
    pub string_set: TSet<FString>,
    pub string_int_map: TMap<FString, i32>,
    #[doc(hidden)]
    __padding_342: [u8; 6],
    pub bool_mutable: bool,
    __padding_end: [u8; 1],
}
impl FPyTestStruct {}
#[repr(C, align(8))]
pub struct FPythonLogOutputEntry {
    pub ty: EPythonLogOutputType,
    pub output: FString,
}
impl FPythonLogOutputEntry {}
#[repr(C, align(8))]
pub struct FPyTestChildStruct {
    __padding_end: [u8; 344],
}
impl FPyTestChildStruct {}
#[repr(C, align(4))]
pub struct FPyTestClassSparseData {
    pub int_from_sparse_data: i32,
}
impl FPyTestClassSparseData {}
pub struct IPyTestInterface {}
#[repr(C, align(8))]
pub struct UPyTestInterface {
    __padding_end: [u8; 48],
}
impl UPyTestInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestInterface")
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
pub struct IPyTestChildInterface {}
#[repr(C, align(8))]
pub struct UPyTestChildInterface {
    __padding_end: [u8; 48],
}
impl UPyTestChildInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestChildInterface")
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
pub struct IPyTestOtherInterface {}
#[repr(C, align(8))]
pub struct UPyTestOtherInterface {
    __padding_end: [u8; 48],
}
impl UPyTestOtherInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestOtherInterface")
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
pub struct UEditorPythonScriptingLibrary {
    __padding_end: [u8; 48],
}
impl UEditorPythonScriptingLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorPythonScriptingLibrary")
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
pub struct UK2Node_ExecutePythonScript {
    __padding_end: [u8; 368],
}
impl UK2Node_ExecutePythonScript {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_ExecutePythonScript")
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
pub struct UPyTestStructLibrary {
    __padding_end: [u8; 48],
}
impl UPyTestStructLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestStructLibrary")
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
pub struct UPyTestObject {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub bool: bool,
    pub int: i32,
    pub float: f32,
    pub enum_: EPyTestEnum,
    pub string: FString,
    pub name: FName,
    pub text: FText,
    pub field_path: TFieldPath<FProperty>,
    pub struct_field_path: TFieldPath<FStructProperty>,
    pub string_array: TArray<FString>,
    pub string_set: TSet<FString>,
    pub string_int_map: TMap<FString, i32>,
    pub delegate: FPyTestObject_Delegate,
    #[doc(hidden)]
    __padding_456: [u8; 24],
    pub _struct: FPyTestStruct,
    pub struct_array: TArray<FPyTestStruct>,
    pub child_struct: FPyTestChildStruct,
    __padding_end: [u8; 8],
}
impl UPyTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestObject")
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
pub struct UPyTestChildObject {
    __padding_end: [u8; 1168],
}
impl UPyTestChildObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestChildObject")
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
pub struct UDEPRECATED_LegacyPyTestObject {
    __padding_end: [u8; 1168],
}
impl UDEPRECATED_LegacyPyTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_LegacyPyTestObject")
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
pub struct UPyTestObjectLibrary {
    __padding_end: [u8; 48],
}
impl UPyTestObjectLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestObjectLibrary")
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
pub struct UPyTestStructDelegate {
    __padding_end: [u8; 72],
}
impl UPyTestStructDelegate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestStructDelegate")
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
pub struct UPyTestVectorDelegate {
    __padding_end: [u8; 72],
}
impl UPyTestVectorDelegate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestVectorDelegate")
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
pub struct UPyTestTypeHint {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub bool_prop: bool,
    pub int_prop: i32,
    pub float_prop: f32,
    pub enum_prop: EPyTestEnum,
    pub string_prop: FString,
    pub name_prop: FName,
    pub text_prop: FText,
    pub field_path_prop: TFieldPath<FProperty>,
    pub struct_prop: FPyTestStruct,
    pub object_prop: UPtr<UPyTestObject>,
    pub str_array_prop: TArray<FString>,
    pub name_array_prop: TArray<FName>,
    pub text_array_prop: TArray<FText>,
    pub object_array_prop: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub set_prop: TSet<FString>,
    pub map_prop: TMap<i32, FString>,
    pub delegate_prop: FPyTestTypeHint_DelegateProp,
    #[doc(hidden)]
    __padding_792: [u8; 24],
    pub slate_tick_delegate: FPyTestTypeHint_SlateTickDelegate,
}
impl UPyTestTypeHint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestTypeHint")
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
pub struct UPythonOnlineDocsCommandlet {
    __padding_end: [u8; 136],
}
impl UPythonOnlineDocsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonOnlineDocsCommandlet")
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
pub struct UPythonScriptCommandlet {
    __padding_end: [u8; 136],
}
impl UPythonScriptCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonScriptCommandlet")
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
pub struct UPythonScriptLibrary {
    __padding_end: [u8; 48],
}
impl UPythonScriptLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonScriptLibrary")
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
pub struct UPythonScriptPluginSettings {
    __padding_end: [u8; 232],
}
impl UPythonScriptPluginSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonScriptPluginSettings")
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
pub struct UPythonScriptPluginUserSettings {
    __padding_end: [u8; 112],
}
impl UPythonScriptPluginUserSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonScriptPluginUserSettings")
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
pub struct IPythonResourceOwner {}
#[repr(C, align(8))]
pub struct UPythonResourceOwner {
    __padding_end: [u8; 48],
}
impl UPythonResourceOwner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonResourceOwner")
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
pub struct UPythonObjectHandle {
    __padding_end: [u8; 64],
}
impl UPythonObjectHandle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonObjectHandle")
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
pub struct UPythonCallableForDelegate {
    __padding_end: [u8; 64],
}
impl UPythonCallableForDelegate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonCallableForDelegate")
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
pub struct UPythonGeneratedEnum {
    __padding_end: [u8; 216],
}
impl UPythonGeneratedEnum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonGeneratedEnum")
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
pub struct UPythonGeneratedClass {
    __padding_end: [u8; 1048],
}
impl UPythonGeneratedClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonGeneratedClass")
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
pub struct UPythonGeneratedStruct {
    __padding_end: [u8; 1040],
}
impl UPythonGeneratedStruct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonGeneratedStruct")
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
pub struct FFuncTakingPyTestDelegate_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCheckDelegateTypeHints_Param1 {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCheckDelegateTypeHints_ReturnValue {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPyTestObject_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPyTestObject_MulticastDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestStructDelegate_OnNameCollisionTestDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestVectorDelegate_OnNameCollisionTestDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestTypeHint_DelegateProp {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPyTestTypeHint_MulticastDelegateProp {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestTypeHint_SlateTickDelegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EPyTestEnum(pub u8);
impl EPyTestEnum {
    pub const ONE: EPyTestEnum = EPyTestEnum(0);
    pub const TWO: EPyTestEnum = EPyTestEnum(1);
}
#[repr(transparent)]
pub struct EPythonLogOutputType(pub u8);
impl EPythonLogOutputType {
    pub const INFO: EPythonLogOutputType = EPythonLogOutputType(0);
    pub const WARNING: EPythonLogOutputType = EPythonLogOutputType(1);
    pub const ERROR: EPythonLogOutputType = EPythonLogOutputType(2);
}
#[repr(transparent)]
pub struct EPythonCommandExecutionMode(pub u8);
impl EPythonCommandExecutionMode {
    pub const EXECUTE_FILE: EPythonCommandExecutionMode = EPythonCommandExecutionMode(0);
    pub const EXECUTE_STATEMENT: EPythonCommandExecutionMode = EPythonCommandExecutionMode(
        1,
    );
    pub const EVALUATE_STATEMENT: EPythonCommandExecutionMode = EPythonCommandExecutionMode(
        2,
    );
}
#[repr(transparent)]
pub struct EPythonFileExecutionScope(pub u8);
impl EPythonFileExecutionScope {
    pub const PRIVATE: EPythonFileExecutionScope = EPythonFileExecutionScope(0);
    pub const PUBLIC: EPythonFileExecutionScope = EPythonFileExecutionScope(1);
}
#[repr(transparent)]
pub struct EPythonEnabledOverrideState(pub u8);
impl EPythonEnabledOverrideState {
    pub const NONE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(0);
    pub const ENABLE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(1);
    pub const DISABLE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(2);
}
#[repr(transparent)]
pub struct ETypeHintingMode(pub u8);
impl ETypeHintingMode {
    pub const OFF: ETypeHintingMode = ETypeHintingMode(0);
    pub const AUTO_COMPLETION: ETypeHintingMode = ETypeHintingMode(1);
    pub const TYPE_CHECKER: ETypeHintingMode = ETypeHintingMode(2);
}
