#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UCsvActorCountMetric {}
pub struct UCsvMetricsSubsystem {
    pub metric_classes: TArray<
        TSubclassOf<crate::bindings::world_metrics_core::UWorldMetricInterface>,
    >,
}
