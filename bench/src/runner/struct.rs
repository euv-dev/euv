use super::*;

/// A raw, reproducible benchmark report; all suites share one configuration.
#[derive(Clone, Debug, Getter, New, Serialize)]
#[new(pub(crate))]
pub(crate) struct Report {
    /// Flattened schema and build-mode provenance.
    #[serde(flatten)]
    #[get(pub(crate))]
    pub(crate) header: Header,
    /// Exact options shared by every workload in this report.
    #[get(pub(crate))]
    pub(crate) config: Config,
    /// Complete initialization and raw-round records for each workload.
    #[get(pub(crate))]
    pub(crate) measurements: Vec<Measurement>,
}

/// Identifies the report schema, measurement mode and build environment.
#[derive(Clone, Debug, Getter, New, Serialize)]
#[new(pub(crate))]
pub(crate) struct Header {
    /// Serialized report format revision.
    #[get(pub(crate), type(copy))]
    pub(crate) schema_version: u32,
    /// Separates uninstrumented timing from allocation counting.
    #[get(pub(crate), type(copy))]
    pub(crate) mode: &'static str,
    /// Flattened host and compiler information captured outside measured ticks.
    #[serde(flatten)]
    #[get(pub(crate))]
    pub(crate) environment: Environment,
}

/// Build and host metadata recorded outside the measured frame loop.
#[derive(Clone, Debug, Getter, New, Serialize)]
#[new(pub(crate))]
pub(crate) struct Environment {
    /// Build-time source directory identifying the measured checkout.
    #[get(pub(crate))]
    pub(crate) source_root: String,
    /// Operating-system identifier supplied by the standard library.
    #[get(pub(crate))]
    pub(crate) os: String,
    /// Architecture identifier supplied by the standard library.
    #[get(pub(crate))]
    pub(crate) architecture: String,
    /// Available logical CPUs, absent when the host cannot report them.
    #[get(pub(crate))]
    pub(crate) logical_cpus: Option<usize>,
    /// Verbose compiler provenance command output.
    #[get(pub(crate))]
    pub(crate) compiler: String,
    /// Wall-clock report creation time in Unix seconds.
    #[get(pub(crate), type(copy))]
    pub(crate) unix_seconds: u64,
    /// Explicit scope of metrics and intentionally unmeasured resources.
    #[get(pub(crate))]
    pub(crate) limitations: String,
}

/// Initialization, warmup, steady state and teardown belong to separate phases.
#[derive(Clone, Debug, Getter, New, Serialize)]
#[new(pub(crate))]
pub(crate) struct Measurement {
    /// Real subsystem combination executed by this fixture.
    #[get(pub(crate), type(copy))]
    pub(crate) suite: Suite,
    /// Initialization duration, absent in allocation mode.
    #[get(pub(crate))]
    pub(crate) initialization_ns: Option<u64>,
    /// Requested-heap accounting for fixture construction.
    #[get(pub(crate))]
    pub(crate) initialization_allocations: Option<AllocationDelta>,
    /// Warmup accounting separated from steady-state samples.
    #[get(pub(crate))]
    pub(crate) warmup_allocations: Option<AllocationDelta>,
    /// Signed retained requested bytes after fixture destruction.
    #[get(pub(crate))]
    pub(crate) retained_after_drop_bytes: Option<i64>,
    /// Every measured round, without best-round filtering.
    #[get(pub(crate))]
    pub(crate) samples: Vec<Sample>,
}

/// One measured round, including actual per-frame latency and subsystem work.
#[derive(Clone, Copy, Debug, Getter, New, Serialize)]
#[new(pub(crate))]
pub(crate) struct Sample {
    /// Zero-based round number within the workload.
    #[get(pub(crate), type(copy))]
    pub(crate) round: usize,
    /// Number of real frames executed during this round.
    #[get(pub(crate), type(copy))]
    pub(crate) ticks: usize,
    /// Flattened optional timing values from an uninstrumented build.
    #[serde(flatten)]
    #[get(pub(crate))]
    pub(crate) timings: Timings,
    /// Allocation counters, absent in timing mode.
    #[get(pub(crate))]
    pub(crate) allocations: Option<AllocationDelta>,
    /// Actual subsystem counts and observable output checksum.
    #[get(pub(crate), type(copy))]
    pub(crate) work: Work,
}

/// Optional CPU measurements, absent in the allocation-counting build.
#[derive(Clone, Copy, Debug, Default, Getter, New, Serialize)]
#[new(pub(crate))]
pub(crate) struct Timings {
    /// Complete measured-round duration in nanoseconds.
    #[get(pub(crate))]
    pub(crate) elapsed_ns: Option<u64>,
    /// Nearest-rank median frame duration in nanoseconds.
    #[get(pub(crate))]
    pub(crate) p50_ns: Option<u64>,
    /// Nearest-rank ninety-fifth percentile frame duration.
    #[get(pub(crate))]
    pub(crate) p95_ns: Option<u64>,
    /// Nearest-rank ninety-ninth percentile frame duration.
    #[get(pub(crate))]
    pub(crate) p99_ns: Option<u64>,
    /// Throughput derived from observed frames and elapsed duration.
    #[get(pub(crate))]
    pub(crate) ticks_per_second: Option<f64>,
}
