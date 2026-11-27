use super::*;

/// Chart title and optional subtitle header configuration.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct EuvChartTitle {
    /// Primary chart title text.
    pub(crate) text: String,
    /// Optional secondary subtitle text.
    pub(crate) subtext: String,
    /// Horizontal alignment of the title block.
    #[get(type(copy))]
    pub(crate) align: EuvChartAlign,
}

/// Interactive chart legend configuration controlling series visibility chips.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartLegend {
    /// Whether the legend strip is displayed.
    #[get(type(copy))]
    pub(crate) show: bool,
    /// Docked position around the plot region.
    #[get(type(copy))]
    pub(crate) position: EuvChartPosition,
    /// Optional explicit ordered legend item labels (defaults to series names when empty).
    pub(crate) items: Vec<String>,
}

/// Cartesian subplot grid margins and framing configuration.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartGrid {
    /// Left margin in logical pixels.
    #[get(type(copy))]
    pub(crate) left: f32,
    /// Right margin in logical pixels.
    #[get(type(copy))]
    pub(crate) right: f32,
    /// Top margin in logical pixels.
    #[get(type(copy))]
    pub(crate) top: f32,
    /// Bottom margin in logical pixels.
    #[get(type(copy))]
    pub(crate) bottom: f32,
    /// When `true`, expands margins automatically so axis labels remain inside bounds.
    #[get(type(copy))]
    pub(crate) contain_label: bool,
    /// When `true`, draws an outer border box around the plot grid.
    #[get(type(copy))]
    pub(crate) show_border: bool,
}

/// Axis scale, tick, split-line, and label configuration for Cartesian, Polar, and SingleAxis charts.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartAxis {
    /// Scale domain kind (`Category`, `Value`, `Time`, `Log`).
    #[get(type(copy))]
    pub(crate) axis_type: EuvChartAxisType,
    /// Optional axis caption displayed near the end of the axis.
    pub(crate) name: String,
    /// Optional unit suffix appended to numeric tick labels (e.g. `"%"` or `"ms"`).
    pub(crate) unit_suffix: String,
    /// Discrete category labels used when `axis_type` is `Category`.
    pub(crate) categories: Vec<String>,
    /// Target subplot grid index inside `EuvChartOption::grid`.
    #[get(type(copy))]
    pub(crate) grid_index: usize,
    /// Optional explicit lower domain bound.
    #[get(skip)]
    pub(crate) min: Option<f32>,
    /// Optional explicit upper domain bound.
    #[get(skip)]
    pub(crate) max: Option<f32>,
    /// When `true`, reverses axis direction.
    #[get(type(copy))]
    pub(crate) inverse: bool,
    /// Target interval split count for nice tick generation.
    #[get(type(copy))]
    pub(crate) split_number: usize,
    /// Whether the main axis baseline stroke is drawn.
    #[get(type(copy))]
    pub(crate) show_axis_line: bool,
    /// Whether short tick marks are drawn along the axis.
    #[get(type(copy))]
    pub(crate) show_axis_tick: bool,
    /// Whether formatted tick labels are drawn.
    #[get(type(copy))]
    pub(crate) show_axis_label: bool,
    /// Whether dashed grid split lines cross the plot area.
    #[get(type(copy))]
    pub(crate) show_split_line: bool,
    /// Whether alternating subtle split bands tint the plot area.
    #[get(type(copy))]
    pub(crate) show_split_area: bool,
    /// Optional explicit axis side override for dual-axis layouts.
    #[get(skip)]
    pub(crate) position: Option<EuvChartPosition>,
}

/// Polar coordinate system pairing an angular axis and radial axis.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartPolar {
    /// Angular axis (typically `Category` or `Value` around 360 degrees).
    pub(crate) angle_axis: EuvChartAxis,
    /// Radial distance axis from center outward.
    pub(crate) radius_axis: EuvChartAxis,
    /// Outer radius ratio relative to half plot minimum side.
    #[get(type(copy))]
    pub(crate) radius_ratio: f32,
    /// Horizontal center ratio in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) center_x_ratio: f32,
    /// Vertical center ratio in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) center_y_ratio: f32,
}

/// Single multivariate indicator spoke inside a radar coordinate system.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct EuvChartRadarIndicator {
    /// Human-readable indicator dimension name.
    pub(crate) name: String,
    /// Lower domain bound at the radar center.
    #[get(type(copy))]
    pub(crate) min: f32,
    /// Upper domain bound at the outer radar ring.
    #[get(type(copy))]
    pub(crate) max: f32,
}

/// Radar coordinate system defining multivariate indicator spokes and concentric rings.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartRadar {
    /// Ordered indicator axes arranged clockwise from 12 o'clock.
    pub(crate) indicators: Vec<EuvChartRadarIndicator>,
    /// Ring shape (`Polygon` or `Circle`).
    #[get(type(copy))]
    pub(crate) shape: EuvChartRadarShape,
    /// Number of concentric split rings.
    #[get(type(copy))]
    pub(crate) split_number: usize,
    /// Outer radius ratio relative to half plot minimum side.
    #[get(type(copy))]
    pub(crate) radius_ratio: f32,
}

/// Single 1D axis coordinate system for compact bubble timelines or linear tracks.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartSingleAxis {
    /// Underlying axis scale and tick configuration.
    pub(crate) axis: EuvChartAxis,
    /// Vertical center ratio inside the plot rect.
    #[get(type(copy))]
    pub(crate) center_y_ratio: f32,
}

/// Calendar matrix coordinate system for week-by-day contribution heatmaps.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartCalendar {
    /// Display year caption (e.g. `2026`).
    #[get(type(copy))]
    pub(crate) year: u32,
    /// Number of week columns rendered along the primary axis.
    #[get(type(copy))]
    pub(crate) weeks: usize,
    /// Month header labels spaced across the week columns.
    pub(crate) month_labels: Vec<String>,
    /// Gap in logical pixels between day cells.
    #[get(type(copy))]
    pub(crate) cell_gap: f32,
    /// Matrix orientation (`Horizontal` weeks left-to-right or `Vertical` top-to-bottom).
    #[get(type(copy))]
    pub(crate) orient: EuvChartOrient,
}

/// Custom or built-in geographic polygon region rendered on `GeoMap`.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartGeoRegion {
    /// Display region name matching `EuvChartGeoDatum::region_name`.
    pub(crate) name: String,
    /// Short ISO or region code.
    pub(crate) code: String,
    /// Closed exterior polygon rings in `(longitude, latitude)` degrees (`lon in -180..=180`, `lat in -90..=90`).
    pub(crate) polygons: Vec<Vec<(f32, f32)>>,
    /// Optional explicit label anchor `(lon, lat)` overriding polygon centroid.
    #[get(skip)]
    pub(crate) label_center: Option<(f32, f32)>,
}

/// Geographic coordinate system backing `GeoMap`, `Scatter`, `EffectScatter`, and `Lines` overlays.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartGeo {
    /// Built-in polygon preset or `Custom`.
    #[get(type(copy))]
    pub(crate) preset: EuvChartGeoMapPreset,
    /// Additional or custom geographic regions.
    pub(crate) regions: Vec<EuvChartGeoRegion>,
    /// Whether region name labels are drawn atop polygons.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
    /// Whether pan/zoom roaming is enabled on the map canvas.
    #[get(type(copy))]
    pub(crate) roam: bool,
}

/// Interactive hover tooltip and axis-pointer configuration.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartTooltip {
    /// Whether hover tooltip cards are shown.
    #[get(type(copy))]
    pub(crate) show: bool,
    /// Snapping strategy (`Item`, `Axis`, or `None`).
    #[get(type(copy))]
    pub(crate) trigger: EuvChartTooltipTrigger,
    /// Crosshair pointer visual style (`Line`, `Shadow`, `Cross`, or `None`).
    #[get(type(copy))]
    pub(crate) axis_pointer: EuvChartAxisPointerType,
}

/// Interactive data-zoom range controller (`Inside` or `Slider`).
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartDataZoom {
    /// Controller mode (`Inside` wheel/drag or `Slider` bottom scrubber).
    #[get(type(copy))]
    pub(crate) kind: EuvChartDataZoomKind,
    /// Initial lower window percentage in `[0.0, 100.0]`.
    #[get(type(copy))]
    pub(crate) start_pct: f32,
    /// Initial upper window percentage in `[0.0, 100.0]`.
    #[get(type(copy))]
    pub(crate) end_pct: f32,
    /// Minimum allowed percentage span when zooming in.
    #[get(type(copy))]
    pub(crate) min_span_pct: f32,
    /// Whether the slider track draws a mini sparkline preview of the primary series.
    #[get(type(copy))]
    pub(crate) show_preview: bool,
}

/// Single discrete interval bucket inside a piecewise `EuvChartVisualMap`.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartVisualPiece {
    /// Optional inclusive lower threshold.
    #[get(skip)]
    pub(crate) min: Option<f32>,
    /// Optional inclusive upper threshold.
    #[get(skip)]
    pub(crate) max: Option<f32>,
    /// Display caption shown in the piecewise legend.
    pub(crate) label: String,
    /// Color assigned to values falling in this bucket.
    #[get(type(copy))]
    pub(crate) color: Color,
}

/// Continuous gradient or piecewise bucket color mapper for Heatmap, Scatter, GeoMap, Bar, and Parallel series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartVisualMap {
    /// Whether the visual-map bar/legend is drawn.
    #[get(type(copy))]
    pub(crate) show: bool,
    /// Mapping mode (`Continuous` or `Piecewise`).
    #[get(type(copy))]
    pub(crate) kind: EuvChartVisualMapKind,
    /// Lower numeric domain value.
    #[get(type(copy))]
    pub(crate) min: f32,
    /// Upper numeric domain value.
    #[get(type(copy))]
    pub(crate) max: f32,
    /// Ordered color stops used by `Continuous` interpolation.
    pub(crate) in_range_colors: Vec<Color>,
    /// Discrete threshold ranges used by `Piecewise` mapping.
    pub(crate) pieces: Vec<EuvChartVisualPiece>,
    /// Visual-map bar orientation.
    #[get(type(copy))]
    pub(crate) orient: EuvChartOrient,
    /// Visual-map dock edge.
    #[get(type(copy))]
    pub(crate) position: EuvChartPosition,
}

/// Compact icon-only chart toolbox toolbar configuration.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartToolbox {
    /// Whether the toolbox strip is shown.
    #[get(type(copy))]
    pub(crate) show: bool,
    /// Enables the Restore (reset zoom, legend, magic-type) action icon.
    #[get(type(copy))]
    pub(crate) restore: bool,
    /// Enables the MagicType (Line / Bar / Stack) mode toggle icon.
    #[get(type(copy))]
    pub(crate) magic_type: bool,
    /// Enables the DataView tabular inspection toggle icon.
    #[get(type(copy))]
    pub(crate) data_view: bool,
    /// Enables the Fullscreen modal expansion icon.
    #[get(type(copy))]
    pub(crate) fullscreen: bool,
}

/// Tabular dataset shared across multiple series via `EuvChartEncode`.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct EuvChartDataset {
    /// Ordered column dimension names.
    pub(crate) dimensions: Vec<String>,
    /// Ordered data rows aligned to `dimensions`.
    pub(crate) rows: Vec<Vec<EuvChartCell>>,
}

/// Column mapping binding `EuvChartDataset` dimensions to series channels.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct EuvChartEncode {
    /// Dimension name mapped to the X channel.
    #[get(skip)]
    pub(crate) x: Option<String>,
    /// Dimension name mapped to the Y channel.
    #[get(skip)]
    pub(crate) y: Option<String>,
    /// Dimension name mapped to numeric value.
    #[get(skip)]
    pub(crate) value: Option<String>,
    /// Dimension name mapped to bubble or symbol size.
    #[get(skip)]
    pub(crate) size: Option<String>,
    /// Dimension name mapped to item label.
    #[get(skip)]
    pub(crate) item_name: Option<String>,
}

/// Point callout pin annotation attached to a Cartesian series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartMarkPoint {
    /// Target datum selection rule (`Min`, `Max`, `Average`, or explicit `Coord`).
    #[get(type(copy))]
    pub(crate) kind: EuvChartMarkPointKind,
    /// Label displayed inside or beside the callout badge.
    pub(crate) name: String,
    /// Optional explicit pin tint overriding series color.
    #[get(skip)]
    pub(crate) color: Option<Color>,
}

/// Reference line annotation attached to a Cartesian series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartMarkLine {
    /// Reference placement rule (`Average`, `Min`, `Max`, `YAxis`, `XAxis`).
    #[get(type(copy))]
    pub(crate) kind: EuvChartMarkLineKind,
    /// Pill label displayed along the reference line.
    pub(crate) name: String,
    /// Optional explicit stroke color overriding series color.
    #[get(skip)]
    pub(crate) color: Option<Color>,
    /// When `true`, renders as a dashed reference line.
    #[get(type(copy))]
    pub(crate) dashed: bool,
}

/// Shaded background highlight band attached to a Cartesian series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartMarkArea {
    /// Optional lower X coordinate or category index bound.
    #[get(skip)]
    pub(crate) start_x: Option<f32>,
    /// Optional upper X coordinate or category index bound.
    #[get(skip)]
    pub(crate) end_x: Option<f32>,
    /// Optional lower Y coordinate bound.
    #[get(skip)]
    pub(crate) start_y: Option<f32>,
    /// Optional upper Y coordinate bound.
    #[get(skip)]
    pub(crate) end_y: Option<f32>,
    /// Caption drawn at the top of the shaded highlight band.
    pub(crate) name: String,
    /// Optional explicit tint color (rendered translucent).
    #[get(skip)]
    pub(crate) color: Option<Color>,
}

/// Collection of `MarkPoint`, `MarkLine`, and `MarkArea` overlays on a Cartesian series.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct EuvChartAnnotations {
    /// Pin annotations on key extrema or coordinates.
    pub(crate) mark_points: Vec<EuvChartMarkPoint>,
    /// Horizontal or vertical reference threshold lines.
    pub(crate) mark_lines: Vec<EuvChartMarkLine>,
    /// Translucent shaded highlight bands.
    pub(crate) mark_areas: Vec<EuvChartMarkArea>,
}

/// Line and area series configuration supporting straight, smooth Catmull-Rom, stepped, stacked, and dashed strokes.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartLineSeries {
    /// Ordered category-aligned numeric values (`None` produces a data gap unless `connect_nulls`).
    pub(crate) values: Vec<Option<f32>>,
    /// Explicit `(x, y)` coordinate pairs used with `Value`, `Time`, `Log`, or `Polar` axes.
    pub(crate) xy_points: Vec<(f32, f32)>,
    /// When `true`, interpolates smooth Catmull-Rom cubic curves between adjacent points.
    #[get(type(copy))]
    pub(crate) smooth: bool,
    /// Step-line mode (`None`, `Start`, `Middle`, `End`).
    #[get(type(copy))]
    pub(crate) step: EuvChartStepMode,
    /// Area fill opacity under the line (`0.0` disables area fill).
    #[get(type(copy))]
    pub(crate) area_opacity: f32,
    /// When `true`, fades area fill smoothly toward transparent at the baseline.
    #[get(type(copy))]
    pub(crate) area_gradient: bool,
    /// Optional stack group identifier for stacked line/area charts.
    #[get(skip)]
    pub(crate) stack: Option<String>,
    /// When `true`, renders the polyline with crisp dashes.
    #[get(type(copy))]
    pub(crate) dashed: bool,
    /// Stroke width in logical pixels.
    #[get(type(copy))]
    pub(crate) line_width: f32,
    /// Data vertex marker shape.
    #[get(type(copy))]
    pub(crate) symbol: EuvChartSymbol,
    /// Data vertex marker radius in logical pixels.
    #[get(type(copy))]
    pub(crate) symbol_size: f32,
    /// When `true`, bridges across `None` gaps instead of breaking the stroke.
    #[get(type(copy))]
    pub(crate) connect_nulls: bool,
    /// When `true`, renders numeric value labels above plotted vertices.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
    /// Coordinate system hosting this series (`Cartesian2d`, `Polar`, or `SingleAxis`).
    #[get(type(copy))]
    pub(crate) coordinate_system: EuvChartCoordinateSystem,
    /// X axis index inside `EuvChartOption::x_axes`.
    #[get(type(copy))]
    pub(crate) x_axis_index: usize,
    /// Y axis index inside `EuvChartOption::y_axes`.
    #[get(type(copy))]
    pub(crate) y_axis_index: usize,
    /// Series mark points, mark lines, and mark areas.
    pub(crate) annotations: EuvChartAnnotations,
    /// Optional dataset dimension encoding.
    #[get(skip)]
    pub(crate) encode: Option<EuvChartEncode>,
}

/// Vertical, horizontal, grouped, stacked, or polar bar series configuration.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartBarSeries {
    /// Ordered category-aligned bar values (`None` skips the slot).
    pub(crate) values: Vec<Option<f32>>,
    /// Optional stack group identifier for stacked bars.
    #[get(skip)]
    pub(crate) stack: Option<String>,
    /// Rounded cap corner radius in logical pixels.
    #[get(type(copy))]
    pub(crate) bar_radius: f32,
    /// Fraction of slot width occupied by bars in `(0.1, 0.95]`.
    #[get(type(copy))]
    pub(crate) bar_width_ratio: f32,
    /// When `true`, draws a subtle full-extent background track behind each bar.
    #[get(type(copy))]
    pub(crate) show_background: bool,
    /// When `true`, draws formatted value labels near bar caps.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
    /// Coordinate system hosting this series (`Cartesian2d` or `Polar`).
    #[get(type(copy))]
    pub(crate) coordinate_system: EuvChartCoordinateSystem,
    /// X axis index inside `EuvChartOption::x_axes`.
    #[get(type(copy))]
    pub(crate) x_axis_index: usize,
    /// Y axis index inside `EuvChartOption::y_axes`.
    #[get(type(copy))]
    pub(crate) y_axis_index: usize,
    /// Series mark points, mark lines, and mark areas.
    pub(crate) annotations: EuvChartAnnotations,
    /// Optional dataset dimension encoding.
    #[get(skip)]
    pub(crate) encode: Option<EuvChartEncode>,
}

/// Single labeled slice inside a pie, donut, or Nightingale Rose series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartPieSlice {
    /// Slice display name.
    pub(crate) name: String,
    /// Non-negative slice numeric weight.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Optional explicit fill color overriding the active palette.
    #[get(skip)]
    pub(crate) color: Option<Color>,
    /// When `true`, pops this slice radially outward.
    #[get(type(copy))]
    pub(crate) selected: bool,
}

/// Pie, donut, and Nightingale Rose series configuration.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartPieSeries {
    /// Ordered proportional slices.
    pub(crate) slices: Vec<EuvChartPieSlice>,
    /// Inner cutout ratio in `[0.0, 0.9]` (`0.0` = full pie, `> 0.0` = donut).
    #[get(type(copy))]
    pub(crate) inner_radius_ratio: f32,
    /// Outer radius ratio relative to half plot minimum side.
    #[get(type(copy))]
    pub(crate) outer_radius_ratio: f32,
    /// Optional Nightingale Rose variation (`None`, `Radius`, `Area`).
    #[get(type(copy))]
    pub(crate) rose_type: EuvChartRoseType,
    /// Angular gap between adjacent wedges in degrees.
    #[get(type(copy))]
    pub(crate) pad_angle_deg: f32,
    /// Starting sweep angle in degrees (`90.0` = 12 o'clock).
    #[get(type(copy))]
    pub(crate) start_angle_deg: f32,
    /// Whether slice name and percentage labels are rendered.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
    /// Whether external bent leader lines connect slices to outer labels.
    #[get(type(copy))]
    pub(crate) show_leader_lines: bool,
    /// Radial pop-out distance in logical pixels for selected slices.
    #[get(type(copy))]
    pub(crate) selected_offset: f32,
    /// Horizontal center ratio in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) center_x_ratio: f32,
    /// Vertical center ratio in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) center_y_ratio: f32,
}

/// Single 2D point or variable bubble plotted by `Scatter` or `EffectScatter`.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartScatterPoint {
    /// X coordinate (numeric domain value, category index, or longitude on `Geo`).
    #[get(type(copy))]
    pub(crate) x: f32,
    /// Y coordinate (numeric domain value or latitude on `Geo`).
    #[get(type(copy))]
    pub(crate) y: f32,
    /// Marker radius in logical pixels (`<= 0.0` falls back to series `default_size`).
    #[get(type(copy))]
    pub(crate) size: f32,
    /// Optional scalar value mapped through `VisualMap`.
    #[get(skip)]
    pub(crate) value: Option<f32>,
    /// Optional point caption shown on label or tooltip.
    pub(crate) label: String,
}

/// Scatter, bubble, and ripple `EffectScatter` series configuration.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartScatterSeries {
    /// Plotted `(x, y, size)` points.
    pub(crate) points: Vec<EuvChartScatterPoint>,
    /// Marker glyph shape.
    #[get(type(copy))]
    pub(crate) symbol: EuvChartSymbol,
    /// Default marker radius in logical pixels.
    #[get(type(copy))]
    pub(crate) default_size: f32,
    /// Concentric ripple ring count for `EffectScatter`.
    #[get(type(copy))]
    pub(crate) ripple_rings: usize,
    /// Outer ripple expansion multiplier for `EffectScatter`.
    #[get(type(copy))]
    pub(crate) ripple_scale: f32,
    /// Whether point captions are drawn beside markers.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
    /// Hosting coordinate system (`Cartesian2d`, `Polar`, `SingleAxis`, or `Geo`).
    #[get(type(copy))]
    pub(crate) coordinate_system: EuvChartCoordinateSystem,
    /// X axis index inside `EuvChartOption::x_axes`.
    #[get(type(copy))]
    pub(crate) x_axis_index: usize,
    /// Y axis index inside `EuvChartOption::y_axes`.
    #[get(type(copy))]
    pub(crate) y_axis_index: usize,
    /// Series mark points, mark lines, and mark areas.
    pub(crate) annotations: EuvChartAnnotations,
    /// Optional dataset dimension encoding.
    #[get(skip)]
    pub(crate) encode: Option<EuvChartEncode>,
}

/// Single multivariate polygon dataset inside a radar series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartRadarItem {
    /// Item legend/tooltip name.
    pub(crate) name: String,
    /// Numeric values aligned to `EuvChartRadar::indicators`.
    pub(crate) values: Vec<f32>,
    /// Optional explicit color overriding the active palette.
    #[get(skip)]
    pub(crate) color: Option<Color>,
}

/// Multivariate radar web series configuration.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartRadarSeries {
    /// Ordered radar polygon records.
    pub(crate) items: Vec<EuvChartRadarItem>,
    /// Semi-transparent interior fill opacity in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) area_opacity: f32,
    /// Polygon perimeter stroke width in logical pixels.
    #[get(type(copy))]
    pub(crate) line_width: f32,
    /// Whether crisp circular dots are drawn at each indicator vertex.
    #[get(type(copy))]
    pub(crate) show_vertices: bool,
}

/// Single OHLC candle bar `(open, close, low, high)` for financial K-Line charts.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct EuvChartOhlcPoint {
    /// Period opening price.
    #[get(type(copy))]
    pub(crate) open: f32,
    /// Period closing price.
    #[get(type(copy))]
    pub(crate) close: f32,
    /// Period lowest price.
    #[get(type(copy))]
    pub(crate) low: f32,
    /// Period highest price.
    #[get(type(copy))]
    pub(crate) high: f32,
}

/// Financial candlestick (K-Line) series composable with overlaid moving-average line series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartCandlestickSeries {
    /// Ordered OHLC candles aligned with the X category axis.
    pub(crate) candles: Vec<EuvChartOhlcPoint>,
    /// Optional bullish candle color (`close >= open`).
    #[get(skip)]
    pub(crate) bull_color: Option<Color>,
    /// Optional bearish candle color (`close < open`).
    #[get(skip)]
    pub(crate) bear_color: Option<Color>,
    /// X axis index inside `EuvChartOption::x_axes`.
    #[get(type(copy))]
    pub(crate) x_axis_index: usize,
    /// Y axis index inside `EuvChartOption::y_axes`.
    #[get(type(copy))]
    pub(crate) y_axis_index: usize,
    /// Series mark points, mark lines, and mark areas.
    pub(crate) annotations: EuvChartAnnotations,
}

/// Five-number statistical summary `[min, q1, median, q3, max]` with optional outliers.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartBoxplotItem {
    /// Lower whisker minimum.
    #[get(type(copy))]
    pub(crate) min: f32,
    /// First quartile (25th percentile).
    #[get(type(copy))]
    pub(crate) q1: f32,
    /// Median (50th percentile).
    #[get(type(copy))]
    pub(crate) median: f32,
    /// Third quartile (75th percentile).
    #[get(type(copy))]
    pub(crate) q3: f32,
    /// Upper whisker maximum.
    #[get(type(copy))]
    pub(crate) max: f32,
    /// Optional outlier scalar observations rendered as scatter dots.
    pub(crate) outliers: Vec<f32>,
}

/// Statistical box-and-whisker distribution series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartBoxplotSeries {
    /// Ordered box summary records aligned with categories.
    pub(crate) items: Vec<EuvChartBoxplotItem>,
    /// X axis index inside `EuvChartOption::x_axes`.
    #[get(type(copy))]
    pub(crate) x_axis_index: usize,
    /// Y axis index inside `EuvChartOption::y_axes`.
    #[get(type(copy))]
    pub(crate) y_axis_index: usize,
    /// Series mark points, mark lines, and mark areas.
    pub(crate) annotations: EuvChartAnnotations,
}

/// Single matrix cell plotted inside a Cartesian or Calendar heatmap.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartHeatmapCell {
    /// Column index (X category index or calendar week index).
    #[get(type(copy))]
    pub(crate) x_index: usize,
    /// Row index (Y category index or calendar day-of-week `0..7`).
    #[get(type(copy))]
    pub(crate) y_index: usize,
    /// Numeric intensity value mapped through `VisualMap`.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Optional custom label overriding formatted numeric cell value.
    pub(crate) label: String,
}

/// 2D Cartesian matrix or Calendar heatmap series colored via `VisualMap`.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartHeatmapSeries {
    /// Ordered heatmap cells.
    pub(crate) cells: Vec<EuvChartHeatmapCell>,
    /// When `true`, displays contrast-safe values inside cells.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
    /// Inset gap between adjacent cells in logical pixels.
    #[get(type(copy))]
    pub(crate) cell_gap: f32,
    /// Coordinate system (`Cartesian2d` or `Calendar`).
    #[get(type(copy))]
    pub(crate) coordinate_system: EuvChartCoordinateSystem,
    /// X axis index inside `EuvChartOption::x_axes`.
    #[get(type(copy))]
    pub(crate) x_axis_index: usize,
    /// Y axis index inside `EuvChartOption::y_axes`.
    #[get(type(copy))]
    pub(crate) y_axis_index: usize,
}

/// Recursive hierarchical datum shared by `Treemap`, `Sunburst`, and `Tree` series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartHierarchyNode {
    /// Node display label.
    pub(crate) name: String,
    /// Leaf or aggregated numeric weight.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Optional explicit color tint inherited by descendants.
    #[get(skip)]
    pub(crate) color: Option<Color>,
    /// Nested child nodes.
    pub(crate) children: Vec<EuvChartHierarchyNode>,
}

/// Squarified area-proportional hierarchical treemap series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartTreemapSeries {
    /// Top-level hierarchical groups or leaves.
    pub(crate) nodes: Vec<EuvChartHierarchyNode>,
    /// Whether parent group header bars are drawn above nested children.
    #[get(type(copy))]
    pub(crate) show_headers: bool,
    /// Gap in logical pixels between sibling rectangles.
    #[get(type(copy))]
    pub(crate) cell_gap: f32,
    /// Corner radius in logical pixels for treemap tiles.
    #[get(type(copy))]
    pub(crate) corner_radius: f32,
}

/// Concentric multi-ring hierarchical sunburst chart series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartSunburstSeries {
    /// Top-level hierarchical rings.
    pub(crate) nodes: Vec<EuvChartHierarchyNode>,
    /// Inner hollow radius ratio in `[0.0, 0.5]`.
    #[get(type(copy))]
    pub(crate) inner_radius_ratio: f32,
    /// Outer radius ratio relative to half plot minimum side.
    #[get(type(copy))]
    pub(crate) outer_radius_ratio: f32,
    /// Angular gap between sibling wedges in degrees.
    #[get(type(copy))]
    pub(crate) pad_angle_deg: f32,
    /// Whether wedge labels are drawn inside sufficiently large sectors.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
}

/// Named node column block inside a Sankey flow diagram.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartSankeyNode {
    /// Unique node name referenced by `EuvChartSankeyLink`.
    pub(crate) name: String,
    /// Optional explicit fill color.
    #[get(skip)]
    pub(crate) color: Option<Color>,
    /// Optional explicit topological column depth override.
    #[get(skip)]
    pub(crate) depth: Option<usize>,
}

/// Weighted directed flow ribbon between two Sankey nodes.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct EuvChartSankeyLink {
    /// Source node name.
    pub(crate) source: String,
    /// Destination node name.
    pub(crate) target: String,
    /// Positive flow magnitude controlling ribbon thickness.
    #[get(type(copy))]
    pub(crate) value: f32,
}

/// Layered topological Sankey flow diagram series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartSankeySeries {
    /// Ordered Sankey nodes.
    pub(crate) nodes: Vec<EuvChartSankeyNode>,
    /// Directed weighted flow links.
    pub(crate) links: Vec<EuvChartSankeyLink>,
    /// Vertical node rectangle width in logical pixels.
    #[get(type(copy))]
    pub(crate) node_width: f32,
    /// Vertical spacing gap between nodes in the same column.
    #[get(type(copy))]
    pub(crate) node_gap: f32,
    /// Translucent opacity applied to connecting cubic Bézier ribbons.
    #[get(type(copy))]
    pub(crate) ribbon_opacity: f32,
}

/// Single conversion stage trapezoid inside a funnel series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartFunnelStage {
    /// Stage display name.
    pub(crate) name: String,
    /// Stage numeric count or percentage weight.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Optional explicit fill color.
    #[get(skip)]
    pub(crate) color: Option<Color>,
}

/// Ascending or descending conversion funnel trapezoid series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartFunnelSeries {
    /// Ordered conversion funnel stages.
    pub(crate) stages: Vec<EuvChartFunnelStage>,
    /// Vertical sorting order (`Descending`, `Ascending`, `None`).
    #[get(type(copy))]
    pub(crate) sort: EuvChartFunnelSort,
    /// Vertical gap in logical pixels between consecutive trapezoids.
    #[get(type(copy))]
    pub(crate) gap: f32,
    /// Whether stage labels are rendered.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
    /// Whether conversion percentage badges are included in labels.
    #[get(type(copy))]
    pub(crate) show_percent: bool,
}

/// Single color zone along a gauge dial arc ending at cumulative ratio `ratio` in `(0.0, 1.0]`.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct EuvChartGaugeSegment {
    /// Cumulative upper ratio of this color band in `(0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) ratio: f32,
    /// Band stroke color.
    #[get(type(copy))]
    pub(crate) color: Color,
}

/// Circular instrument gauge series with multi-segment ring, ticks, progress arc, needle, and readout.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartGaugeSeries {
    /// Current numeric reading displayed by needle and progress arc.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Minimum scale value.
    #[get(type(copy))]
    pub(crate) min: f32,
    /// Maximum scale value.
    #[get(type(copy))]
    pub(crate) max: f32,
    /// Unit suffix rendered next to the center numeric readout (e.g. `"%"` or `"km/h"`).
    pub(crate) unit: String,
    /// Caption label displayed below the dial center.
    pub(crate) title: String,
    /// Sweep start angle in degrees (`225.0` = bottom-left).
    #[get(type(copy))]
    pub(crate) start_angle_deg: f32,
    /// Sweep end angle in degrees (`-45.0` = bottom-right).
    #[get(type(copy))]
    pub(crate) end_angle_deg: f32,
    /// Number of major tick intervals around the dial.
    #[get(type(copy))]
    pub(crate) split_number: usize,
    /// When `true`, highlights the swept progress arc from `min` to `value`.
    #[get(type(copy))]
    pub(crate) show_progress: bool,
    /// When `true`, draws the tapered center pointer needle.
    #[get(type(copy))]
    pub(crate) show_pointer: bool,
    /// Optional color zone segments along the outer dial ring.
    pub(crate) segments: Vec<EuvChartGaugeSegment>,
}

/// Single vertex inside a relational `Graph` series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartGraphNode {
    /// Unique vertex identifier referenced by edges.
    pub(crate) id: String,
    /// Human-readable display label.
    pub(crate) name: String,
    /// Associated numeric weight.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Visual circle radius in logical pixels.
    #[get(type(copy))]
    pub(crate) size: f32,
    /// Category index controlling node tint color.
    #[get(type(copy))]
    pub(crate) category: usize,
    /// Optional pre-assigned normalized X position in `[0.0, 1.0]`.
    #[get(skip)]
    pub(crate) x: Option<f32>,
    /// Optional pre-assigned normalized Y position in `[0.0, 1.0]`.
    #[get(skip)]
    pub(crate) y: Option<f32>,
    /// Optional explicit color overriding category palette.
    #[get(skip)]
    pub(crate) color: Option<Color>,
}

/// Directed or undirected connector edge inside a relational `Graph` series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartGraphEdge {
    /// Source node `id`.
    pub(crate) source: String,
    /// Target node `id`.
    pub(crate) target: String,
    /// Relative stroke weight.
    #[get(type(copy))]
    pub(crate) weight: f32,
    /// Optional edge label.
    pub(crate) label: String,
    /// Quadratic curve bow ratio (`0.0` = straight segment).
    #[get(type(copy))]
    pub(crate) curveness: f32,
}

/// Relational network graph series supporting deterministic force-directed and circular layouts.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartGraphSeries {
    /// Graph vertices.
    pub(crate) nodes: Vec<EuvChartGraphNode>,
    /// Graph edges.
    pub(crate) edges: Vec<EuvChartGraphEdge>,
    /// Category legend labels indexed by `EuvChartGraphNode::category`.
    pub(crate) categories: Vec<String>,
    /// Layout algorithm (`Force`, `Circular`, or `None`).
    #[get(type(copy))]
    pub(crate) layout: EuvChartGraphLayout,
    /// When `true`, draws directional arrowheads at edge destinations.
    #[get(type(copy))]
    pub(crate) directed: bool,
    /// When `true`, draws node captions next to vertices.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
    /// Force-directed relaxation iteration count.
    #[get(type(copy))]
    pub(crate) iterations: usize,
}

/// Hierarchical node-link tree series supporting orthogonal (`LeftRight` / `TopBottom`) and `Radial` layouts.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartTreeSeries {
    /// Root hierarchical tree node.
    pub(crate) root: EuvChartHierarchyNode,
    /// Tree orientation (`OrthogonalLeftRight`, `OrthogonalTopBottom`, `Radial`).
    #[get(type(copy))]
    pub(crate) layout: EuvChartTreeLayout,
    /// Node marker circle radius in logical pixels.
    #[get(type(copy))]
    pub(crate) symbol_size: f32,
    /// Whether node labels are displayed.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
}

/// Single vertical dimension axis inside a parallel coordinates series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartParallelAxis {
    /// Dimension axis heading.
    pub(crate) name: String,
    /// Optional explicit lower bound.
    #[get(skip)]
    pub(crate) min: Option<f32>,
    /// Optional explicit upper bound.
    #[get(skip)]
    pub(crate) max: Option<f32>,
    /// Optional categorical tick labels for discrete dimensions.
    pub(crate) categories: Vec<String>,
}

/// Single multivariate polyline observation inside a parallel coordinates series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartParallelRow {
    /// Row identifier or label.
    pub(crate) name: String,
    /// Dimension values aligned to `EuvChartParallelSeries::axes`.
    pub(crate) values: Vec<f32>,
    /// Category index used for categorical coloring when `VisualMap` is absent.
    #[get(type(copy))]
    pub(crate) category: usize,
    /// Optional explicit stroke color.
    #[get(skip)]
    pub(crate) color: Option<Color>,
}

/// Multidimensional parallel coordinates polyline series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartParallelSeries {
    /// Ordered parallel dimension axes.
    pub(crate) axes: Vec<EuvChartParallelAxis>,
    /// Multivariate observation polylines.
    pub(crate) rows: Vec<EuvChartParallelRow>,
    /// Polyline stroke width in logical pixels.
    #[get(type(copy))]
    pub(crate) line_width: f32,
    /// Polyline opacity in `(0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) line_opacity: f32,
}

/// Single directed curved flow arc between two Cartesian or Geo coordinates.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartFlowArc {
    /// Origin location or label.
    pub(crate) from_name: String,
    /// Destination location or label.
    pub(crate) to_name: String,
    /// Origin X coordinate (or longitude in degrees on `Geo`).
    #[get(type(copy))]
    pub(crate) from_x: f32,
    /// Origin Y coordinate (or latitude in degrees on `Geo`).
    #[get(type(copy))]
    pub(crate) from_y: f32,
    /// Destination X coordinate (or longitude in degrees on `Geo`).
    #[get(type(copy))]
    pub(crate) to_x: f32,
    /// Destination Y coordinate (or latitude in degrees on `Geo`).
    #[get(type(copy))]
    pub(crate) to_y: f32,
    /// Flow magnitude value.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Optional explicit arc color.
    #[get(skip)]
    pub(crate) color: Option<Color>,
}

/// Quadratic curved directional flow arcs series on Cartesian or Geo coordinates.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartLinesSeries {
    /// Ordered directed flow arcs.
    pub(crate) arcs: Vec<EuvChartFlowArc>,
    /// Quadratic perpendicular curvature factor (`0.2` produces a gentle flight arc).
    #[get(type(copy))]
    pub(crate) curveness: f32,
    /// Arc stroke width in logical pixels.
    #[get(type(copy))]
    pub(crate) line_width: f32,
    /// When `true`, draws directional arrowheads at each destination point.
    #[get(type(copy))]
    pub(crate) show_arrows: bool,
    /// Hosting coordinate system (`Cartesian2d` or `Geo`).
    #[get(type(copy))]
    pub(crate) coordinate_system: EuvChartCoordinateSystem,
    /// X axis index inside `EuvChartOption::x_axes`.
    #[get(type(copy))]
    pub(crate) x_axis_index: usize,
    /// Y axis index inside `EuvChartOption::y_axes`.
    #[get(type(copy))]
    pub(crate) y_axis_index: usize,
}

/// Single `(time, value, theme)` entry inside a centered ThemeRiver streamgraph.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct EuvChartThemeRiverPoint {
    /// Numeric timestamp or time step.
    #[get(type(copy))]
    pub(crate) time: f32,
    /// Non-negative stream thickness at `time`.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Stream theme category name.
    pub(crate) theme: String,
}

/// Centered stacked streamgraph ribbon series over time.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartThemeRiverSeries {
    /// Stream observations across time steps and themes.
    pub(crate) points: Vec<EuvChartThemeRiverPoint>,
    /// Whether stream theme labels are drawn on the leading edge.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
    /// Whether stream boundaries use smooth curve interpolation.
    #[get(type(copy))]
    pub(crate) smooth: bool,
}

/// Stacked glyph-symbol pictorial bar series.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartPictorialBarSeries {
    /// Ordered category-aligned numeric values.
    pub(crate) values: Vec<f32>,
    /// Repeated symbol glyph (`RoundRect`, `Diamond`, `Circle`, `Rect`, `Triangle`).
    #[get(type(copy))]
    pub(crate) symbol: EuvChartSymbol,
    /// When `true`, stacks discrete symbols up to the value height; when `false`, stretches one glyph.
    #[get(type(copy))]
    pub(crate) symbol_repeat: bool,
    /// Individual glyph size in logical pixels.
    #[get(type(copy))]
    pub(crate) symbol_size: f32,
    /// X axis index inside `EuvChartOption::x_axes`.
    #[get(type(copy))]
    pub(crate) x_axis_index: usize,
    /// Y axis index inside `EuvChartOption::y_axes`.
    #[get(type(copy))]
    pub(crate) y_axis_index: usize,
}

/// Read-only coordinate and theme context passed into an `EuvChartCustomPainter` callback.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartCustomDrawContext {
    /// Active plot bounds in logical pixels.
    #[get(type(copy))]
    pub(crate) plot_rect: Rect,
    /// Resolved X-axis minimum value.
    #[get(type(copy))]
    pub(crate) x_min: f32,
    /// Resolved X-axis maximum value.
    #[get(type(copy))]
    pub(crate) x_max: f32,
    /// Resolved Y-axis minimum value.
    #[get(type(copy))]
    pub(crate) y_min: f32,
    /// Resolved Y-axis maximum value.
    #[get(type(copy))]
    pub(crate) y_max: f32,
    /// Active categorical color palette.
    pub(crate) palette: Vec<Color>,
    /// Active UI theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Current pointer coordinates in logical pixels when hovering the plot.
    #[get(skip)]
    pub(crate) hover_pos: Option<Point>,
}

/// Custom user vector painter series invoking a caller-provided `DrawCommand` callback.
#[derive(Clone, CustomDebug, Data)]
pub struct EuvChartCustomSeries {
    /// Numeric sample values contributing to auto-axis domain sizing.
    pub(crate) values: Vec<f32>,
    /// Optional custom `DrawCommand` painter closure.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) painter: Option<EuvChartCustomPainter>,
    /// X axis index inside `EuvChartOption::x_axes`.
    #[get(type(copy))]
    pub(crate) x_axis_index: usize,
    /// Y axis index inside `EuvChartOption::y_axes`.
    #[get(type(copy))]
    pub(crate) y_axis_index: usize,
}

/// Single region choropleth value for `GeoMap`.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartGeoDatum {
    /// Region name matching a built-in or custom `EuvChartGeoRegion::name`.
    pub(crate) region_name: String,
    /// Numeric metric mapped through `VisualMap`.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Optional explicit fill color overriding `VisualMap`.
    #[get(skip)]
    pub(crate) color: Option<Color>,
}

/// Geographic choropleth map series over built-in world/continent or custom polygons.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EuvChartGeoMapSeries {
    /// Region metric records.
    pub(crate) data: Vec<EuvChartGeoDatum>,
    /// Whether region name captions are drawn atop filled polygons.
    #[get(type(copy))]
    pub(crate) show_labels: bool,
    /// Polygon border stroke width in logical pixels.
    #[get(type(copy))]
    pub(crate) border_width: f32,
}

/// Named series wrapper pairing a legend name, optional tint, visibility flag, and typed series payload.
#[derive(Clone, Data, Debug)]
pub struct EuvChartSeries {
    /// Legend and tooltip series name.
    pub(crate) name: String,
    /// Optional explicit series color overriding `color_palette`.
    #[get(skip)]
    pub(crate) color: Option<Color>,
    /// Initial visibility state (toggled interactively via the legend).
    #[get(type(copy))]
    pub(crate) visible: bool,
    /// Concrete series variant and dataset.
    pub(crate) data: EuvChartSeriesData,
}

/// Complete declarative Apache ECharts option document rendered natively by `EuvEChart`.
#[derive(Clone, Data, Debug, Default)]
pub struct EuvChartOption {
    /// Optional chart title and subtitle block.
    #[get(skip)]
    pub(crate) title: Option<EuvChartTitle>,
    /// Optional interactive legend strip.
    #[get(skip)]
    pub(crate) legend: Option<EuvChartLegend>,
    /// One or more Cartesian subplot grids.
    pub(crate) grid: Vec<EuvChartGrid>,
    /// Horizontal axes (supporting dual/multi-axis via `x_axis_index`).
    pub(crate) x_axes: Vec<EuvChartAxis>,
    /// Vertical axes (supporting dual/multi-axis via `y_axis_index`).
    pub(crate) y_axes: Vec<EuvChartAxis>,
    /// Optional polar coordinate system.
    #[get(skip)]
    pub(crate) polar: Option<EuvChartPolar>,
    /// Optional multivariate radar coordinate system.
    #[get(skip)]
    pub(crate) radar: Option<EuvChartRadar>,
    /// Optional single 1D axis coordinate system.
    #[get(skip)]
    pub(crate) single_axis: Option<EuvChartSingleAxis>,
    /// Optional calendar week-by-day matrix coordinate system.
    #[get(skip)]
    pub(crate) calendar: Option<EuvChartCalendar>,
    /// Optional geographic coordinate system.
    #[get(skip)]
    pub(crate) geo: Option<EuvChartGeo>,
    /// Optional interactive hover tooltip and crosshair axis-pointer configuration.
    #[get(skip)]
    pub(crate) tooltip: Option<EuvChartTooltip>,
    /// Interactive `Inside` and `Slider` data-zoom controllers.
    pub(crate) data_zoom: Vec<EuvChartDataZoom>,
    /// Optional continuous gradient or piecewise discrete color visual-map controller.
    #[get(skip)]
    pub(crate) visual_map: Option<EuvChartVisualMap>,
    /// Optional compact icon-only toolbox toolbar.
    #[get(skip)]
    pub(crate) toolbox: Option<EuvChartToolbox>,
    /// Optional shared tabular dataset referenced by series `encode` mappings.
    #[get(skip)]
    pub(crate) dataset: Option<EuvChartDataset>,
    /// Optional explicit color palette overriding the theme-derived ECharts palette.
    #[get(skip)]
    pub(crate) color_palette: Option<Vec<Color>>,
    /// Ordered series list rendered back-to-front.
    pub(crate) series: Vec<EuvChartSeries>,
    /// Optional explicit canvas background color overriding `ThemePalette::card`.
    #[get(skip)]
    pub(crate) background: Option<Color>,
}
