#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UBaseSequencerAnimTool {}
pub struct IBaseSequencerAnimTool {}
pub struct UMotionTrailToolBuilder {}
pub struct UMotionTrailTool {
    pub transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub trs_gizmo: UPtr<
        crate::bindings::editor_interactive_tools_framework::UTransformGizmo,
    >,
    pub left_click_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickInputBehavior,
    >,
    pub right_click_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickInputBehavior,
    >,
}
pub struct USequencerPivotToolBuilder {}
pub struct USequencerPivotTool {
    pub click_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickInputBehavior,
    >,
    pub transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub trs_gizmo: UPtr<
        crate::bindings::editor_interactive_tools_framework::UTransformGizmo,
    >,
}
pub struct USequencerToolsEditMode {}
