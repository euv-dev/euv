use super::*;

/// Full-featured native vector Apache ECharts component supporting all 20 series types, multi-axis grids, interactive legends, tooltips, DataZoom, VisualMap, and compact icon-only Toolbox.
#[derive(Clone, CustomDebug, Data, Props)]
pub struct EuvEChart {
    /// Static declarative chart option document.
    #[prop(custom)]
    pub(crate) option: EuvChartOption,
    /// Optional reactive chart option signal overriding `option` when provided.
    #[debug(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) option_signal: Option<Signal<EuvChartOption>>,
    /// Active UI theme tokens.
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) tokens: ThemeTokens,
    /// Component outer width (defaults to `Length::Percent(100.0)`).
    #[get(type(copy))]
    #[prop(custom, default = Length::Percent(100.0))]
    pub(crate) width: Length,
    /// Component canvas height (defaults to `Length::Px(340.0)`).
    #[get(type(copy))]
    #[prop(custom, default = Length::Px(ECHART_DEFAULT_HEIGHT))]
    pub(crate) height: Length,
    /// Whether hover tooltip, legend toggle, data-zoom, and click interaction are active (defaults to `true`).
    #[get(type(copy))]
    #[prop(custom, default = true)]
    pub(crate) interactive: bool,
    /// Optional callback invoked when a plotted datum or slice is clicked.
    #[debug(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) on_point_click: Option<EuvChartHitCallback>,
    /// Optional callback invoked when a legend item toggles `(series_name, now_visible)`.
    #[debug(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) on_legend_change: Option<EuvChartLegendCallback>,
    /// Optional callback invoked when DataZoom adjusts `(start_pct, end_pct)`.
    #[debug(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) on_zoom_change: Option<EuvChartZoomCallback>,
}

/// Bundled shared handles used to attach pointer, click, drag, and wheel listeners to `EuvEChart`.
pub(crate) struct ChartPointerWiring {
    /// Mounted chart canvas node handle.
    pub(crate) canvas_node: NodeId,
    /// Reactive interactive chart state signal.
    pub(crate) state_sig: Signal<EuvChartState>,
    /// Latest painted chart hit-testing snapshot.
    pub(crate) last_render: Rc<RefCell<EuvChartRenderResult>>,
    /// Latest painted logical canvas rect.
    pub(crate) last_bounds: Rc<RefCell<Rect>>,
    /// Active drag gesture cell `(target, start_x, orig_start_pct, orig_end_pct)`.
    pub(crate) drag_state: Rc<Cell<(ChartDragTarget, f32, f32, f32)>>,
    /// Static declarative option fallback.
    pub(crate) static_opt: Rc<EuvChartOption>,
    /// Optional reactive option signal.
    pub(crate) opt_sig: Option<Signal<EuvChartOption>>,
    /// Optional datum click callback.
    pub(crate) on_point_click: Option<EuvChartHitCallback>,
    /// Optional legend change callback.
    pub(crate) on_legend_change: Option<EuvChartLegendCallback>,
    /// Optional data-zoom change callback.
    pub(crate) on_zoom_change: Option<EuvChartZoomCallback>,
}

/// Cached vector `DrawCommand` snapshot reused across identical chart frames.
#[derive(Clone, Debug)]
pub(crate) struct ChartPaintCache {
    /// Logical bounds associated with this cached display list.
    pub(crate) bounds: Rect,
    /// Theme tokens active when cached.
    pub(crate) tokens: ThemeTokens,
    /// Chart interaction state active when cached.
    pub(crate) state: EuvChartState,
    /// Monotonic reactive option revision when cached.
    pub(crate) rev: u64,
    /// Cached vector drawing commands.
    pub(crate) commands: Vec<DrawCommand>,
}
