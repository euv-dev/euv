use super::*;

/// Resolved numeric, category, time, or logarithmic axis scale ready for pixel projection.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartAxisScale {
    /// Active axis scale kind (`Category`, `Value`, `Time`, `Log`).
    #[get(type(copy))]
    pub(crate) axis_type: EuvChartAxisType,
    /// Resolved lower domain bound (numeric or category start index).
    #[get(type(copy))]
    pub(crate) min: f32,
    /// Resolved upper domain bound (numeric or category end index).
    #[get(type(copy))]
    pub(crate) max: f32,
    /// Whether projection direction is inverted.
    #[get(type(copy))]
    pub(crate) inverse: bool,
    /// Ordered tick domain values (or category indices).
    pub(crate) ticks: Vec<f32>,
    /// Pre-formatted tick labels aligned 1:1 with `ticks`.
    pub(crate) labels: Vec<String>,
    /// Visible category labels (sliced by active `DataZoom` window when applicable).
    pub(crate) categories: Vec<String>,
}

/// Active index/percentage slice window computed from `DataZoom` state.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct EuvChartZoomSlice {
    /// Lower window percentage in `[0.0, 100.0]`.
    #[get(type(copy))]
    pub(crate) start_pct: f32,
    /// Upper window percentage in `[0.0, 100.0]`.
    #[get(type(copy))]
    pub(crate) end_pct: f32,
    /// Inclusive first category/sample index inside the zoomed slice.
    #[get(type(copy))]
    pub(crate) start_index: usize,
    /// Exclusive upper category/sample index inside the zoomed slice.
    #[get(type(copy))]
    pub(crate) end_index: usize,
}

/// Resolved visual-map color evaluator supporting `Continuous` and `Piecewise` modes.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartVisualEvaluator {
    /// Active mapping mode (`Continuous` or `Piecewise`).
    #[get(type(copy))]
    pub(crate) kind: EuvChartVisualMapKind,
    /// Lower numeric domain value.
    #[get(type(copy))]
    pub(crate) min: f32,
    /// Upper numeric domain value.
    #[get(type(copy))]
    pub(crate) max: f32,
    /// Ordered continuous gradient stops (guaranteed non-empty).
    pub(crate) stops: Vec<Color>,
    /// Discrete piecewise range buckets.
    pub(crate) pieces: Vec<EuvChartVisualPiece>,
}
