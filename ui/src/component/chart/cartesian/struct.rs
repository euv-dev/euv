use super::*;

/// Precomputed group slot layout for grouped and stacked bar series.
#[derive(Clone, Debug, Default)]
pub(crate) struct BarSlotLayout {
    /// Ordered unique bar slot keys (stack key or individual series index key).
    pub(crate) slot_keys: Vec<String>,
    /// Positive stacked baseline accumulators keyed by `(slot_key, category_idx)`.
    pub(crate) pos_acc: HashMap<(String, usize), f32>,
    /// Negative stacked baseline accumulators keyed by `(slot_key, category_idx)`.
    pub(crate) neg_acc: HashMap<(String, usize), f32>,
}

/// Shared rendering context passed into Cartesian series painters.
#[derive(Clone, Debug)]
pub(crate) struct CartesianRenderCtx<'a> {
    /// Active plot rectangle in logical pixels.
    pub(crate) plot_rect: Rect,
    /// Active X axis scale.
    pub(crate) x_scale: &'a EuvChartAxisScale,
    /// Active Y axis scale.
    pub(crate) y_scale: &'a EuvChartAxisScale,
    /// Active data-zoom index slice window.
    pub(crate) zoom: EuvChartZoomSlice,
    /// Shared chart frame context.
    pub(crate) frame: &'a ChartDrawFrame<'a>,
}

/// Bundled Cartesian axes configuration and computed scales for grid rendering.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CartesianAxesBundle<'a> {
    /// Configured X axes.
    pub(crate) x_axes: &'a [EuvChartAxis],
    /// Computed X axis scales.
    pub(crate) x_scales: &'a [EuvChartAxisScale],
    /// Configured Y axes.
    pub(crate) y_axes: &'a [EuvChartAxis],
    /// Computed Y axis scales.
    pub(crate) y_scales: &'a [EuvChartAxisScale],
    /// Whether to draw an outer border box around the plot rect.
    pub(crate) show_border: bool,
}

/// Projected vertex entry inside a contiguous line series segment run.
#[derive(Clone, Debug)]
pub(crate) struct LineRunPoint {
    /// Upper plotted point in logical pixels.
    pub(crate) top: Point,
    /// Lower stacked or zero-baseline point in logical pixels.
    pub(crate) bot: Point,
    /// Zero-based visible datum index.
    pub(crate) idx: usize,
    /// Formatted category or X caption.
    pub(crate) cat: String,
    /// Original unstacked numeric datum value.
    pub(crate) val: f32,
}

/// Computed extrema and mean summary for Cartesian mark annotations.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SampleExtrema {
    /// Coordinates `(x, y)` of the minimum Y sample.
    pub(crate) min_pt: (f32, f32),
    /// Coordinates `(x, y)` of the maximum Y sample.
    pub(crate) max_pt: (f32, f32),
    /// Arithmetic mean across all Y samples.
    pub(crate) mean_y: f32,
}
