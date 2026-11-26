use super::*;

/// Measured performance telemetry for a single interactive UI stress benchmark scenario.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct StressCaseResult {
    /// Stable scenario identifier matching one of `STRESS_ID_*`.
    pub(crate) id: String,
    /// Human-readable benchmark title displayed on the scenario card.
    pub(crate) title: String,
    /// Short category tag (`LIST`, `LAYOUT`, `REACTIVE`, `WASM OTA`, etc.).
    pub(crate) category: String,
    /// Short scenario workload description.
    pub(crate) description: String,
    /// Action button label triggering this scenario.
    pub(crate) button_label: String,
    /// Compliance status (`"IDLE"`, `"PASS"`, or `"OVER"`).
    pub(crate) status: String,
    /// Target per-iteration frame/operation latency budget in milliseconds.
    #[get(type(copy))]
    pub(crate) budget_ms: f64,
    /// Number of measured iterations executed in the most recent run.
    #[get(type(copy))]
    pub(crate) iterations: usize,
    /// Mean latency per iteration in milliseconds.
    #[get(type(copy))]
    pub(crate) avg_ms: f64,
    /// 95th-percentile latency per iteration in milliseconds.
    #[get(type(copy))]
    pub(crate) p95_ms: f64,
    /// Worst-case iteration latency in milliseconds.
    #[get(type(copy))]
    pub(crate) max_ms: f64,
    /// Formatted throughput readout (e.g. `"1,420 FPS"` or `"9,800 ops/s"`).
    pub(crate) throughput: String,
    /// Scenario-specific diagnostic details (node count, cache hits, shaded pixels, drift, etc.).
    pub(crate) details: String,
    /// Cumulative number of times the user has run this scenario in the current session.
    #[get(type(copy))]
    pub(crate) runs: u32,
}
