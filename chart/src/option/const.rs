/// The default palette, matching ECharts' default colour list.
///
/// The list is ordered: a chart without an explicit palette takes the
/// first colour for the first series, the second for the second series,
/// and so on. The order matters — it is what makes two charts built from
/// the same options look the same.
pub(crate) const DEFAULT_PALETTE: &[&str] = &[
    "#5470c6", "#91cc75", "#fac858", "#ee6666", "#73c0de", "#3ba272", "#fc8452", "#9a60b4",
    "#ea7ccc",
];

/// The default background colour of a chart.
pub(crate) const DEFAULT_BACKGROUND: &str = "#ffffff";

/// The default colour of an axis' text.
pub(crate) const DEFAULT_TEXT_COLOR: &str = "#6e7079";

/// The default colour of an axis' line.
///
/// ECharts draws the line of an axis in the same grey as its text.
pub(crate) const DEFAULT_AXIS_LINE_COLOR: &str = "#6e7079";

/// The default colour of a split line.
///
/// The colour is the pale blue ECharts uses for the grid of a chart.
pub(crate) const DEFAULT_SPLIT_LINE_COLOR: &str = "#e0e6f1";

/// The default width of a split line, in pixels.
pub(crate) const DEFAULT_SPLIT_LINE_WIDTH: f64 = 1.0;

/// The default width of an axis' line, in pixels.
pub(crate) const DEFAULT_AXIS_LINE_WIDTH: f64 = 1.0;

/// The default length of an axis' tick marks, in pixels.
pub(crate) const DEFAULT_AXIS_TICK_LENGTH: f64 = 5.0;

/// The default gap between an axis and its labels, in pixels.
pub(crate) const DEFAULT_AXIS_LABEL_GAP: f64 = 8.0;

/// The default font size of an axis' labels, in pixels.
pub(crate) const DEFAULT_AXIS_LABEL_FONT_SIZE: f64 = 12.0;

/// The default font size of a series' labels, in pixels.
pub(crate) const DEFAULT_LABEL_FONT_SIZE: f64 = 12.0;

/// The default font size of the chart's title, in pixels.
pub(crate) const DEFAULT_TITLE_FONT_SIZE: f64 = 18.0;

/// The default font size of the chart's subtitle, in pixels.
pub(crate) const DEFAULT_SUBTITLE_FONT_SIZE: f64 = 12.0;

/// The default font size of the chart's legend, in pixels.
pub(crate) const DEFAULT_LEGEND_FONT_SIZE: f64 = 12.0;

/// The default font size of the chart's tooltip, in pixels.
pub(crate) const DEFAULT_TOOLTIP_FONT_SIZE: f64 = 14.0;

/// The default width of a series' line, in pixels.
pub(crate) const DEFAULT_SERIES_LINE_WIDTH: f64 = 2.0;

/// The default width of a series' bar, in pixels.
pub(crate) const DEFAULT_BAR_WIDTH: f64 = 20.0;

/// The default radius of a pie's outer ring, as a fraction of the
/// container's smaller side.
pub(crate) const DEFAULT_PIE_RADIUS: f64 = 0.75;

/// The default gap between two axis categories, as a fraction of the
/// category's width.
pub(crate) const DEFAULT_BAR_CATEGORY_GAP: f64 = 0.2;

/// The default gap between two bars of a group, as a fraction of the
/// bar's width.
pub(crate) const DEFAULT_BAR_GAP: f64 = 0.1;

/// The default radius of a data point's symbol, in pixels.
pub(crate) const DEFAULT_SYMBOL_SIZE: f64 = 4.0;

/// The default duration of a chart's animation, in milliseconds.
pub(crate) const DEFAULT_ANIMATION_DURATION: f64 = 1000.0;

/// The ECharts name of a line series.
pub(crate) const SERIES_TYPE_LINE: &str = "line";

/// The ECharts name of a bar series.
pub(crate) const SERIES_TYPE_BAR: &str = "bar";

/// The ECharts name of a pie series.
pub(crate) const SERIES_TYPE_PIE: &str = "pie";

/// The ECharts name of a scatter series.
pub(crate) const SERIES_TYPE_SCATTER: &str = "scatter";

/// The ECharts name of an effect-scatter series.
pub(crate) const SERIES_TYPE_EFFECT_SCATTER: &str = "effectScatter";

/// The ECharts name of a radar series.
pub(crate) const SERIES_TYPE_RADAR: &str = "radar";

/// The ECharts name of a gauge series.
pub(crate) const SERIES_TYPE_GAUGE: &str = "gauge";

/// The ECharts name of a funnel series.
pub(crate) const SERIES_TYPE_FUNNEL: &str = "funnel";

/// The ECharts name of a heatmap series.
pub(crate) const SERIES_TYPE_HEATMAP: &str = "heatmap";

/// The ECharts name of a graph series.
pub(crate) const SERIES_TYPE_GRAPH: &str = "graph";

/// The ECharts name of a treemap series.
pub(crate) const SERIES_TYPE_TREEMAP: &str = "treemap";

/// The ECharts name of a sunburst series.
pub(crate) const SERIES_TYPE_SUNBURST: &str = "sunburst";

/// The ECharts name of a boxplot series.
pub(crate) const SERIES_TYPE_BOXPLOT: &str = "boxplot";

/// The ECharts name of a candlestick series.
pub(crate) const SERIES_TYPE_CANDLESTICK: &str = "candlestick";

/// The ECharts name of a theme-river series.
pub(crate) const SERIES_TYPE_THEME_RIVER: &str = "themeRiver";

/// The ECharts name of a sankey series.
pub(crate) const SERIES_TYPE_SANKEY: &str = "sankey";

/// The ECharts name of a parallel-coordinates series.
pub(crate) const SERIES_TYPE_PARALLEL: &str = "parallel";

/// The ECharts name of a pictorial-bar series.
pub(crate) const SERIES_TYPE_PICTORIAL_BAR: &str = "pictorialBar";

/// The ECharts name of a lines series.
pub(crate) const SERIES_TYPE_LINES: &str = "lines";

/// The ECharts name of a map series.
pub(crate) const SERIES_TYPE_MAP: &str = "map";

/// The ECharts name of a custom series.
pub(crate) const SERIES_TYPE_CUSTOM: &str = "custom";
