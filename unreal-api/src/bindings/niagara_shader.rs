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
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UNiagaraScriptBase {
    __padding_end: [u8; 48],
}
impl UNiagaraScriptBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScriptBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScriptBase")
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
#[repr(transparent)]
pub struct ENiagaraSimStageExecuteBehavior(pub u8);
impl ENiagaraSimStageExecuteBehavior {
    pub const ALWAYS: ENiagaraSimStageExecuteBehavior = ENiagaraSimStageExecuteBehavior(
        0,
    );
    pub const ON_SIMULATION_RESET: ENiagaraSimStageExecuteBehavior = ENiagaraSimStageExecuteBehavior(
        1,
    );
    pub const NOT_ON_SIMULATION_RESET: ENiagaraSimStageExecuteBehavior = ENiagaraSimStageExecuteBehavior(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraGpuDispatchType(pub u8);
impl ENiagaraGpuDispatchType {
    pub const ONE_D: ENiagaraGpuDispatchType = ENiagaraGpuDispatchType(0);
    pub const TWO_D: ENiagaraGpuDispatchType = ENiagaraGpuDispatchType(1);
    pub const THREE_D: ENiagaraGpuDispatchType = ENiagaraGpuDispatchType(2);
    pub const CUSTOM: ENiagaraGpuDispatchType = ENiagaraGpuDispatchType(3);
}
#[repr(transparent)]
pub struct ENiagaraDirectDispatchElementType(pub u8);
impl ENiagaraDirectDispatchElementType {
    pub const NUM_THREADS: ENiagaraDirectDispatchElementType = ENiagaraDirectDispatchElementType(
        0,
    );
    pub const NUM_THREADS_NO_CLIPPING: ENiagaraDirectDispatchElementType = ENiagaraDirectDispatchElementType(
        1,
    );
    pub const NUM_GROUPS: ENiagaraDirectDispatchElementType = ENiagaraDirectDispatchElementType(
        2,
    );
}
#[repr(transparent)]
pub struct FNiagaraCompileEventSeverity(pub u8);
impl FNiagaraCompileEventSeverity {
    pub const LOG: FNiagaraCompileEventSeverity = FNiagaraCompileEventSeverity(0);
    pub const DISPLAY: FNiagaraCompileEventSeverity = FNiagaraCompileEventSeverity(1);
    pub const WARNING: FNiagaraCompileEventSeverity = FNiagaraCompileEventSeverity(2);
    pub const ERROR: FNiagaraCompileEventSeverity = FNiagaraCompileEventSeverity(3);
}
#[repr(transparent)]
pub struct FNiagaraCompileEventSource(pub u8);
impl FNiagaraCompileEventSource {
    pub const UNSET: FNiagaraCompileEventSource = FNiagaraCompileEventSource(0);
    pub const SCRIPT_DEPENDENCY: FNiagaraCompileEventSource = FNiagaraCompileEventSource(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraMipMapGenerationType(pub u8);
impl ENiagaraMipMapGenerationType {
    pub const UNFILTERED: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(0);
    pub const LINEAR: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(1);
    pub const BLUR1: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(2);
    pub const BLUR2: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(3);
    pub const BLUR3: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(4);
    pub const BLUR4: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(5);
}
