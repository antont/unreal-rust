#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UCsvActorCountMetric {}
pub struct UCsvMetricsSubsystem {
    pub metric_classes: TArray<TSubclassOf<UWorldMetricInterface>>,
}
