#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FThreadingOptions {
    pub b_use_global_thread_pool: bool,
    pub intra_op_num_threads: i32,
    pub inter_op_num_threads: i32,
    pub execution_mode: EExecutionMode,
}
pub struct UNNERuntimeORTCpu {}
pub struct UNNERuntimeORTDmlProxy {}
pub struct UNNERuntimeORTDml_GPU_RDG_NPU {}
pub struct UNNERuntimeORTDml_GPU_RDG {}
pub struct UNNERuntimeORTDml_GPU_NPU {}
pub struct UNNERuntimeORTDml_RDG_NPU {}
pub struct UNNERuntimeORTDml_GPU {}
pub struct UNNERuntimeORTDml_RDG {}
pub struct UNNERuntimeORTDml_NPU {}
pub struct UNNERuntimeORTSettings {
    pub editor_threading_options: FThreadingOptions,
    pub game_threading_options: FThreadingOptions,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EExecutionMode(pub u8);
impl EExecutionMode {
    pub const SEQUENTIAL: EExecutionMode = EExecutionMode(0);
    pub const PARALLEL: EExecutionMode = EExecutionMode(1);
}
