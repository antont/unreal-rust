#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EInterchangeCurveInterpMode(pub u8);
impl EInterchangeCurveInterpMode {
    pub const LINEAR: EInterchangeCurveInterpMode = EInterchangeCurveInterpMode(0);
    pub const CONSTANT: EInterchangeCurveInterpMode = EInterchangeCurveInterpMode(1);
    pub const CUBIC: EInterchangeCurveInterpMode = EInterchangeCurveInterpMode(2);
    pub const NONE: EInterchangeCurveInterpMode = EInterchangeCurveInterpMode(3);
}
#[repr(transparent)]
pub struct EInterchangeCurveTangentMode(pub u8);
impl EInterchangeCurveTangentMode {
    pub const AUTO: EInterchangeCurveTangentMode = EInterchangeCurveTangentMode(0);
    pub const USER: EInterchangeCurveTangentMode = EInterchangeCurveTangentMode(1);
    pub const BREAK: EInterchangeCurveTangentMode = EInterchangeCurveTangentMode(2);
    pub const NONE: EInterchangeCurveTangentMode = EInterchangeCurveTangentMode(3);
}
#[repr(transparent)]
pub struct EInterchangeCurveTangentWeightMode(pub u8);
impl EInterchangeCurveTangentWeightMode {
    pub const WEIGHTED_NONE: EInterchangeCurveTangentWeightMode = EInterchangeCurveTangentWeightMode(
        0,
    );
    pub const WEIGHTED_ARRIVE: EInterchangeCurveTangentWeightMode = EInterchangeCurveTangentWeightMode(
        1,
    );
    pub const WEIGHTED_LEAVE: EInterchangeCurveTangentWeightMode = EInterchangeCurveTangentWeightMode(
        2,
    );
    pub const WEIGHTED_BOTH: EInterchangeCurveTangentWeightMode = EInterchangeCurveTangentWeightMode(
        3,
    );
}
