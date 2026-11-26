use super::*;

/// Timing and throughput telemetry for a single micro- or macro-benchmark.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Getter, New)]
pub struct BenchMetric {
    /// Canonical benchmark name.
    pub(crate) name: String,
    /// Number of measured iterations.
    #[get(type(copy))]
    pub(crate) iterations: usize,
    /// Total elapsed wall time in microseconds.
    #[get(type(copy))]
    pub(crate) total_micros: u64,
    /// Average nanoseconds per iteration.
    #[get(type(copy))]
    pub(crate) ns_per_op: f64,
    /// Throughput in operations per second.
    #[get(type(copy))]
    pub(crate) ops_per_sec: f64,
    /// Additional benchmark-specific verification metadata.
    pub(crate) detail: String,
}

/// Complete serialized report produced by `run_full_benchmark_suite`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Getter, New)]
pub struct BenchReport {
    /// Ordered benchmark results.
    pub(crate) metrics: Vec<BenchMetric>,
}
