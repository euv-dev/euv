use super::*;

/// Per-iteration latency percentile distribution, frame-budget misses, and damage telemetry.
#[derive(Clone, Copy, Debug, Default, Deserialize, Getter, PartialEq, Serialize)]
pub struct LatencyDistribution {
    /// Number of measured iterations included in this distribution.
    #[get(type(copy))]
    pub(crate) iterations: usize,
    /// Median (50th percentile) per-iteration latency in microseconds.
    #[get(type(copy))]
    pub(crate) p50_us: f64,
    /// 95th percentile per-iteration latency in microseconds.
    #[get(type(copy))]
    pub(crate) p95_us: f64,
    /// 99th percentile per-iteration latency in microseconds.
    #[get(type(copy))]
    pub(crate) p99_us: f64,
    /// Maximum single-iteration latency in microseconds.
    #[get(type(copy))]
    pub(crate) max_us: f64,
    /// Arithmetic mean per-iteration latency in microseconds.
    #[get(type(copy))]
    pub(crate) mean_us: f64,
    /// Number of iterations exceeding the 144Hz frame budget (`6.94ms`).
    #[get(type(copy))]
    pub(crate) over_6_94ms_count: usize,
    /// Number of iterations exceeding the 60Hz frame budget (`16.67ms`).
    #[get(type(copy))]
    pub(crate) over_16_67ms_count: usize,
    /// Arithmetic mean `FrameTelemetry::dirty_area_ratio` across measured frames (`0.0` when not applicable).
    #[get(type(copy))]
    pub(crate) mean_dirty_area_ratio: f64,
    /// Count of measured frames that promoted to full-viewport redraw (`FullRedrawReason != None`).
    #[get(type(copy))]
    pub(crate) full_redraw_count: usize,
}

/// Single scenario entry recorded inside a `StressMatrixReport`.
#[derive(Clone, Debug, Deserialize, Getter, New, PartialEq, Serialize)]
pub struct StressMatrixEntry {
    /// Canonical stress scenario identifier.
    pub(crate) name: String,
    /// Recorded per-iteration latency distribution and frame-budget telemetry.
    #[get(type(copy))]
    pub(crate) latency: LatencyDistribution,
    /// Human-readable verification metadata and scenario invariants.
    pub(crate) detail: String,
}

/// Interleaved A/B comparison validating P1 surgical partial-layout damage against full redraw.
#[derive(Clone, Debug, Deserialize, Getter, PartialEq, Serialize)]
pub struct PartialDamageAbSummary {
    /// Canonical A/B scenario identifier.
    pub(crate) name: String,
    /// Distribution captured with `app.set_partial_layout_damage(false)`.
    #[get(type(copy))]
    pub(crate) baseline_full_redraw: LatencyDistribution,
    /// Distribution captured with `app.set_partial_layout_damage(true)`.
    #[get(type(copy))]
    pub(crate) candidate_partial_damage: LatencyDistribution,
    /// Percentage latency reduction at p50 (`> 0.0` means partial damage is faster).
    #[get(type(copy))]
    pub(crate) p50_speedup_pct: f64,
    /// Percentage latency reduction at p95 (`> 0.0` means partial damage is faster).
    #[get(type(copy))]
    pub(crate) p95_speedup_pct: f64,
    /// Percentage latency reduction at p99 (`> 0.0` means partial damage is faster).
    #[get(type(copy))]
    pub(crate) p99_speedup_pct: f64,
    /// Percentage reduction in mean `dirty_area_ratio` from full redraw to surgical partial damage.
    #[get(type(copy))]
    pub(crate) dirty_area_ratio_drop_pct: f64,
    /// `true` when every single measured frame produced bit-exact RGBA framebuffer parity.
    #[get(type(copy))]
    pub(crate) bit_exact_parity: bool,
    /// Number of interleaved A/B frames verified for hash parity.
    #[get(type(copy))]
    pub(crate) verified_steps: usize,
    /// Concise summary of resolution, parity, and dirty-area telemetry.
    pub(crate) detail: String,
}

/// Complete serialized report produced by `run_stress_matrix` / `run_filtered_stress_matrix`.
#[derive(Clone, Debug, Deserialize, Getter, New, PartialEq, Serialize)]
pub struct StressMatrixReport {
    /// Ordered stress scenario results.
    pub(crate) entries: Vec<StressMatrixEntry>,
    /// Optional structured interleaved A/B partial-layout damage comparison when that scenario ran.
    #[get(skip)]
    pub(crate) partial_damage_ab: Option<PartialDamageAbSummary>,
}
