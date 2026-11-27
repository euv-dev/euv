use super::*;

/// Horizontal text or title alignment inside a chart region.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartAlign {
    /// Left-aligned placement.
    #[default]
    Left,
    /// Horizontally centered placement.
    Center,
    /// Right-aligned placement.
    Right,
}

/// Docked placement edge for legends, axes, and visual-map bars.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartPosition {
    /// Docked along the top edge.
    #[default]
    Top,
    /// Docked along the bottom edge.
    Bottom,
    /// Docked along the left edge.
    Left,
    /// Docked along the right edge.
    Right,
}

/// Primary layout orientation for legends, calendars, and visual maps.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartOrient {
    /// Horizontal left-to-right layout.
    #[default]
    Horizontal,
    /// Vertical top-to-bottom layout.
    Vertical,
}

/// Apache ECharts axis scale kind.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartAxisType {
    /// Discrete labeled category bins.
    Category,
    /// Continuous linear numeric domain.
    #[default]
    Value,
    /// Continuous timestamp domain formatted as dates or clock times.
    Time,
    /// Base-10 logarithmic numeric domain for positive values.
    Log,
}

/// Vector marker glyph shape used by line, scatter, effect-scatter, and pictorial bar series.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartSymbol {
    /// Suppresses marker rendering.
    None,
    /// Solid circle marker.
    #[default]
    Circle,
    /// Sharp square marker.
    Rect,
    /// Rounded square or capsule marker.
    RoundRect,
    /// Upward-pointing triangle marker.
    Triangle,
    /// 45-degree rotated diamond marker.
    Diamond,
    /// Teardrop map pin marker.
    Pin,
}

/// Step-line interpolation mode matching Apache ECharts `series-line.step`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartStepMode {
    /// Normal direct or smooth point-to-point interpolation.
    #[default]
    None,
    /// Step vertically at the start of each segment.
    Start,
    /// Step vertically at the horizontal midpoint of each segment.
    Middle,
    /// Step vertically at the end of each segment.
    End,
}

/// Nightingale Rose pie variation mode.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartRoseType {
    /// Standard uniform-radius pie or donut chart.
    #[default]
    None,
    /// Both arc angle and radius scale with slice value.
    Radius,
    /// Equal arc angles with radius proportional to slice value.
    Area,
}

/// Radar coordinate background web geometry.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartRadarShape {
    /// Straight-edged regular polygon rings.
    #[default]
    Polygon,
    /// Smooth concentric circular rings.
    Circle,
}

/// Stage ordering mode for funnel conversion charts.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartFunnelSort {
    /// Widest stage at the top narrowing downward.
    #[default]
    Descending,
    /// Narrowest stage at the top widening downward.
    Ascending,
    /// Preserves the caller-provided stage order.
    None,
}

/// Node positioning layout strategy for relational graph charts.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartGraphLayout {
    /// Deterministic Fruchterman-Reingold force-directed layout.
    #[default]
    Force,
    /// Evenly spaced circular ring layout.
    Circular,
    /// Uses caller-provided normalized `(x, y)` coordinates directly.
    None,
}

/// Hierarchical node-link layout direction for tree charts.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartTreeLayout {
    /// Left-to-right orthogonal tree.
    #[default]
    OrthogonalLeftRight,
    /// Top-to-bottom orthogonal tree.
    OrthogonalTopBottom,
    /// Concentric radial polar tree.
    Radial,
}

/// Target coordinate system binding for a series.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartCoordinateSystem {
    /// Standard 2D Cartesian `(x, y)` grid.
    #[default]
    Cartesian2d,
    /// Polar `(angle, radius)` coordinate system.
    Polar,
    /// Single 1D axis coordinate system.
    SingleAxis,
    /// Calendar week-by-day matrix coordinate system.
    Calendar,
    /// Geographic longitude/latitude map projection.
    Geo,
}

/// Tooltip activation scope matching Apache ECharts `tooltip.trigger`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartTooltipTrigger {
    /// Disables interactive tooltip cards.
    None,
    /// Snaps to the single nearest plotted datum under the pointer.
    Item,
    /// Snaps to the active category column or axis coordinate across all visible series.
    #[default]
    Axis,
}

/// Crosshair guide rendered while hovering a Cartesian chart.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartAxisPointerType {
    /// No crosshair guide.
    None,
    /// Vertical hairline guide at the hovered X coordinate.
    #[default]
    Line,
    /// Translucent column highlight band over the hovered category slot.
    Shadow,
    /// Full horizontal and vertical crosshair lines intersecting at the pointer.
    Cross,
}

/// Interactive data-zoom controller variant.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartDataZoomKind {
    /// Wheel zoom and horizontal pointer-drag pan directly on the plot area.
    Inside,
    /// Interactive bottom range scrubber with draggable handles and mini preview.
    #[default]
    Slider,
}

/// Visual-map color mapping behavior.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartVisualMapKind {
    /// Smooth multi-stop color gradient bar.
    #[default]
    Continuous,
    /// Discrete threshold-bucketed color legend.
    Piecewise,
}

/// Built-in stylized geographic vector polygon preset.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartGeoMapPreset {
    /// Built-in world continents and major economic regions.
    #[default]
    World,
    /// Simplified six-continent macro map.
    Continents,
    /// Uses exclusively caller-supplied `EuvChartGeoRegion` polygons.
    Custom,
}

/// Active toolbox magic-type transformation overriding Cartesian line/bar display.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartMagicType {
    /// Renders series in their authored representation.
    #[default]
    Default,
    /// Converts bar/line series into crisp line series.
    Line,
    /// Converts line/bar series into grouped bar series.
    Bar,
    /// Converts line/bar series into stacked area or stacked bar series.
    Stack,
}

/// Single tabular cell value stored inside an `EuvChartDataset`.
#[derive(Clone, Debug, PartialEq)]
pub enum EuvChartCell {
    /// Text category or label cell.
    Text(String),
    /// Numeric scalar cell.
    Number(f32),
    /// Missing or null data gap cell.
    Null,
}

/// Target point specification for a Cartesian `MarkPoint` annotation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum EuvChartMarkPointKind {
    /// Annotates the minimum value in the series.
    Min,
    /// Annotates the maximum value in the series.
    #[default]
    Max,
    /// Annotates the arithmetic mean point of the series.
    Average,
    /// Annotates an explicit `(x, y)` data coordinate.
    Coord(f32, f32),
}

/// Target reference line specification for a Cartesian `MarkLine` annotation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum EuvChartMarkLineKind {
    /// Horizontal dashed reference line at the series mean.
    #[default]
    Average,
    /// Horizontal dashed reference line at the series minimum.
    Min,
    /// Horizontal dashed reference line at the series maximum.
    Max,
    /// Horizontal reference line at a fixed Y value.
    YAxis(f32),
    /// Vertical reference line at a fixed X value or category index.
    XAxis(f32),
}

/// Complete union of all 20 Apache ECharts series visualizations supported natively in `euv-ui`.
#[derive(Clone, Debug)]
pub enum EuvChartSeriesData {
    /// Straight, smooth Catmull-Rom, stepped, stacked, or area line series.
    Line(EuvChartLineSeries),
    /// Vertical, horizontal, grouped, stacked, or rounded bar series.
    Bar(EuvChartBarSeries),
    /// Standard pie, donut, or Nightingale Rose circular chart.
    Pie(EuvChartPieSeries),
    /// 2D scatter or variable-size bubble series.
    Scatter(EuvChartScatterSeries),
    /// Animated-style concentric ripple highlight scatter series.
    EffectScatter(EuvChartScatterSeries),
    /// Multivariate radar web polygon series.
    Radar(EuvChartRadarSeries),
    /// Financial OHLC candlestick (K-Line) series composable with moving-average lines.
    Candlestick(EuvChartCandlestickSeries),
    /// Statistical five-number summary box-and-whisker series with outliers.
    Boxplot(EuvChartBoxplotSeries),
    /// Cartesian 2D or calendar matrix heatmap colored through `VisualMap`.
    Heatmap(EuvChartHeatmapSeries),
    /// Area-proportional squarified hierarchical treemap series.
    Treemap(EuvChartTreemapSeries),
    /// Multi-ring concentric hierarchical sunburst wedge series.
    Sunburst(EuvChartSunburstSeries),
    /// Layered topological Sankey flow diagram with cubic Bézier ribbons.
    Sankey(EuvChartSankeySeries),
    /// Ascending or descending conversion funnel trapezoid series.
    Funnel(EuvChartFunnelSeries),
    /// Circular dial gauge with segmented ring, ticks, progress arc, and needle.
    Gauge(EuvChartGaugeSeries),
    /// Force-directed or circular node-link network graph series.
    Graph(EuvChartGraphSeries),
    /// Orthogonal or radial hierarchical node-link tree series.
    Tree(EuvChartTreeSeries),
    /// Multi-axis parallel coordinates polyline series.
    Parallel(EuvChartParallelSeries),
    /// Directional quadratic curved flow arcs on Cartesian or Geo coordinates.
    Lines(EuvChartLinesSeries),
    /// Centered stacked streamgraph ribbon series over time.
    ThemeRiver(EuvChartThemeRiverSeries),
    /// Stacked glyph-symbol pictorial bar series.
    PictorialBar(EuvChartPictorialBarSeries),
    /// Custom user vector painter hook emitting arbitrary `DrawCommand` primitives.
    Custom(EuvChartCustomSeries),
    /// Vector choropleth map over built-in world/continent or custom Geo regions.
    GeoMap(EuvChartGeoMapSeries),
}
