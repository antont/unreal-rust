#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct AISMPoolActor {
    pub ism_pool_comp: UPtr<UISMPoolComponent>,
    pub ism_pool_debug_draw_comp: UPtr<UISMPoolDebugDrawComponent>,
}
pub struct UISMPoolComponent {}
pub struct UISMPoolSubSystem {}
pub struct UISMPoolDebugDrawComponent {
    pub b_show_global_stats: bool,
    pub b_show_stats: bool,
    pub b_show_bounds: bool,
    pub selected_component: UPtr<UInstancedStaticMeshComponent>,
}
