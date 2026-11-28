use super::*;

/// Shared frame-level rendering context passed to coordinate and series renderers.
#[derive(Clone, Debug)]
pub(crate) struct ChartDrawFrame<'a> {
    /// Active UI theme tokens.
    pub(crate) tokens: ThemeTokens,
    /// Resolved categorical color palette.
    pub(crate) palette: &'a [Color],
    /// Optional visual-map color evaluator.
    pub(crate) visual_eval: Option<&'a EuvChartVisualEvaluator>,
    /// Optional shared tabular dataset.
    pub(crate) dataset: Option<&'a EuvChartDataset>,
    /// Optional active pointer position in logical canvas coordinates.
    pub(crate) hover_pos: Option<Point>,
    /// Optional interactively popped-out pie slice index.
    pub(crate) selected_pie_slice: Option<usize>,
}

/// Lightweight per-series metadata passed into individual series painters.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SeriesMeta<'a> {
    /// Zero-based index inside `EuvChartOption::series`.
    pub(crate) index: usize,
    /// Series legend and tooltip name.
    pub(crate) name: &'a str,
    /// Resolved base series color.
    pub(crate) color: Color,
}

/// Hit-testable plotted datum emitted by every chart series for hover tooltips and click callbacks.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartDatumHit {
    /// Zero-based index of the owning series.
    #[get(type(copy))]
    pub(crate) series_index: usize,
    /// Display name of the owning series.
    pub(crate) series_name: String,
    /// Zero-based index of the datum inside the series.
    #[get(type(copy))]
    pub(crate) data_index: usize,
    /// Category label or datum caption.
    pub(crate) category_label: String,
    /// Primary numeric scalar value.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Formatted detail string shown in tooltips.
    pub(crate) detail: String,
    /// Rendered series or datum swatch color.
    #[get(type(copy))]
    pub(crate) color: Color,
    /// Logical canvas anchor point for tooltip placement and distance testing.
    #[get(type(copy))]
    pub(crate) anchor: Point,
    /// Circular hit radius in logical pixels when `hit_rect` is absent.
    #[get(type(copy))]
    pub(crate) hit_radius: f32,
    /// Optional bounding box for rectangular datums (bars, cells, treemap tiles, candles).
    #[get(skip)]
    pub(crate) hit_rect: Option<Rect>,
}

/// Interactive hit box for a single rendered legend chip.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartLegendHit {
    /// Owning series index (`usize::MAX` for multi-item slices/categories).
    #[get(type(copy))]
    pub(crate) series_index: usize,
    /// Legend entry name matching `EuvChartSeries::name`.
    pub(crate) name: String,
    /// Swatch indicator color.
    #[get(type(copy))]
    pub(crate) color: Color,
    /// Current visibility state.
    #[get(type(copy))]
    pub(crate) visible: bool,
    /// Clickable chip bounds in logical canvas coordinates.
    #[get(type(copy))]
    pub(crate) rect: Rect,
}

/// Logical geometry of the bottom interactive `DataZoom::Slider` scrubber.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct EuvChartSliderRects {
    /// Full scrubber track rectangle.
    #[get(type(copy))]
    pub(crate) track_rect: Rect,
    /// Active selected percentage range rectangle.
    #[get(type(copy))]
    pub(crate) window_rect: Rect,
    /// Left draggable start handle rectangle.
    #[get(type(copy))]
    pub(crate) left_handle: Rect,
    /// Right draggable end handle rectangle.
    #[get(type(copy))]
    pub(crate) right_handle: Rect,
}

/// Tabular snapshot generated for the interactive `DataView` toolbox mode.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct EuvChartDataViewTable {
    /// Column header titles.
    pub(crate) headers: Vec<String>,
    /// Ordered tabular string rows.
    pub(crate) rows: Vec<Vec<String>>,
}

/// Mutable interactive runtime state governing legend toggles, zoom window, magic-type, data-view, pie pop-out, and hover pointer.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartState {
    /// Series names currently hidden via interactive legend toggles.
    pub(crate) hidden_series: Vec<String>,
    /// Active data-zoom lower window percentage in `[0.0, 100.0]`.
    #[get(type(copy))]
    pub(crate) zoom_start_pct: f32,
    /// Active data-zoom upper window percentage in `[0.0, 100.0]`.
    #[get(type(copy))]
    pub(crate) zoom_end_pct: f32,
    /// Active toolbox MagicType override (`Default`, `Line`, `Bar`, `Stack`).
    #[get(type(copy))]
    pub(crate) magic_type: EuvChartMagicType,
    /// Whether the tabular `DataView` overlay panel is currently open.
    #[get(type(copy))]
    pub(crate) data_view_open: bool,
    /// Optional interactively clicked pie slice index popped outward.
    #[get(skip)]
    pub(crate) selected_pie_slice: Option<usize>,
    /// Optional active pointer coordinate in logical canvas space.
    #[get(skip)]
    pub(crate) hover_pos: Option<Point>,
}

/// Complete deterministic layout and hit-testing output returned by `render_echart_to_display_list`.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct EuvChartRenderResult {
    /// Primary plot area rectangle inside canvas bounds.
    #[get(type(copy))]
    pub(crate) plot_rect: Rect,
    /// All hit-testable plotted datums emitted during this frame.
    pub(crate) hits: Vec<EuvChartDatumHit>,
    /// All clickable legend chip boxes emitted during this frame.
    pub(crate) legend_hits: Vec<EuvChartLegendHit>,
    /// Optional bottom `DataZoom::Slider` interactive geometry.
    #[get(skip)]
    pub(crate) slider_rects: Option<EuvChartSliderRects>,
    /// Active datums snapped by the current hover pointer for tooltip rendering.
    pub(crate) active_tooltip_hits: Vec<EuvChartDatumHit>,
    /// Computed X axis scales for Cartesian subplots.
    pub(crate) x_scales: Vec<EuvChartAxisScale>,
    /// Computed Y axis scales for Cartesian subplots.
    pub(crate) y_scales: Vec<EuvChartAxisScale>,
}

/// Bundled Cartesian subplot inputs passed into `render_cartesian_subplots`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SubplotRenderInput<'a> {
    /// Declarative chart option.
    pub(crate) option: &'a EuvChartOption,
    /// Active interactive chart state.
    pub(crate) state: &'a EuvChartState,
    /// Effective transformed chart series slice.
    pub(crate) series: &'a [EuvChartSeries],
    /// Series visibility flags aligned with `series`.
    pub(crate) visible: &'a [bool],
}
