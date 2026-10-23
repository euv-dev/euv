use super::*;

/// The horizontal and vertical placement of a label relative to its
/// anchor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LabelOffset {
    /// The horizontal offset, in pixels.
    pub x: f64,
    /// The vertical offset, in pixels.
    pub y: f64,
}

/// The fill, stroke, and shadow of a shape, as configured on a series or
/// a data point.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemStyleOption {
    /// The fill colour, as a CSS colour string.
    pub color: Option<String>,
    /// The fill colour of the border.
    pub border_color: Option<String>,
    /// The width of the border.
    pub border_width: Option<f64>,
    /// The radius of the border's rounded corners.
    pub border_radius: Option<f64>,
    /// The shadow cast by the shape.
    pub shadow: Option<ShadowOption>,
    /// The opacity of the shape.
    pub opacity: Option<f64>,
    /// A gradient fill, as `[from, to]` in chart coordinates.
    pub gradient: Option<(String, String)>,
}

/// The shadow cast by a shape.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShadowOption {
    /// The horizontal offset of the shadow.
    pub offset_x: f64,
    /// The vertical offset of the shadow.
    pub offset_y: f64,
    /// The blur radius of the shadow.
    pub blur: f64,
    /// The colour of the shadow.
    pub color: Option<String>,
}

/// The stroke of a shape or a line.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LineStyleOption {
    /// The colour of the line.
    pub color: Option<String>,
    /// The width of the line.
    pub width: Option<f64>,
    /// The dash pattern type.
    pub line_type: Option<LineType>,
    /// The opacity of the line.
    pub opacity: Option<f64>,
    /// The cap drawn at the ends of the line.
    pub cap: Option<LineCap>,
    /// The join drawn between two segments.
    pub join: Option<LineJoin>,
    /// The shadow cast by the line.
    pub shadow: Option<ShadowOption>,
}

/// The text drawn next to a data point, an axis, or a legend entry.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct LabelOption {
    /// Whether the label is drawn.
    #[get(type(copy))]
    pub show: bool,
    /// The text of the label.
    ///
    /// `None` means "derive it from the data"; a template uses the
    /// ECharts syntax, for example `"{b}: {c}"`.
    #[get(type(clone))]
    pub text: Option<String>,
    /// The position of the label relative to its anchor.
    #[get(type(copy))]
    pub position: Option<Position>,
    /// The offset of the label from its anchor.
    #[get(type(copy))]
    pub offset: Option<LabelOffset>,
    /// The distance between the label and the anchor, in pixels.
    #[get(type(copy))]
    pub distance: Option<f64>,
    /// The rotation of the label, in degrees.
    #[get(type(copy))]
    pub rotate: Option<f64>,
    /// The alignment of the label.
    #[get(type(copy))]
    pub align: Option<TextAlign>,
    /// The vertical alignment of the label.
    #[get(type(copy))]
    pub vertical_align: Option<TextVerticalAlign>,
    /// The colour of the label.
    #[get(type(clone))]
    pub color: Option<String>,
    /// The font family of the label.
    #[get(type(clone))]
    pub font_family: Option<String>,
    /// The font size of the label, in pixels.
    #[get(type(copy))]
    pub font_size: Option<f64>,
    /// The font weight of the label.
    #[get(type(clone))]
    pub font_weight: Option<String>,
    /// The font style of the label.
    #[get(type(clone))]
    pub font_style: Option<String>,
    /// The line height of the label, as a multiple of the font size.
    #[get(type(copy))]
    pub line_height: Option<f64>,
    /// The background colour of the label.
    #[get(type(clone))]
    pub background_color: Option<String>,
    /// The padding inside the label's background.
    #[get(type(copy))]
    pub padding: Option<(f64, f64, f64, f64)>,
    /// The border colour of the label's background.
    #[get(type(clone))]
    pub border_color: Option<String>,
    /// The border width of the label's background.
    #[get(type(copy))]
    pub border_width: Option<f64>,
    /// The radius of the label's background corners.
    #[get(type(copy))]
    pub border_radius: Option<f64>,
    /// Whether the label is stroked with an outline.
    #[get(type(copy))]
    pub stroke: Option<bool>,
    /// The width of the label's outline.
    #[get(type(copy))]
    pub stroke_width: Option<f64>,
    /// The colour of the label's outline.
    #[get(type(clone))]
    pub stroke_color: Option<String>,
    /// Whether the label is hidden when it would overlap another one.
    #[get(type(copy))]
    pub hide_overlap: Option<bool>,
    /// The formatter used when the text is not a template.
    #[get(type(clone))]
    pub formatter: Option<String>,
}

/// The rules that move labels out of each other's way.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LabelLayoutOption {
    /// Whether the layout runs.
    pub hide_overlap: bool,
    /// The horizontal drift allowed when hiding overlaps, in pixels.
    pub move_tolerance: Option<f64>,
    /// The label fields that participate in the layout.
    pub label_line_lengths: Option<Vec<f64>>,
}

/// The style applied to a data point when it is highlighted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EmphasisOption {
    /// Whether the emphasis style is applied.
    pub enabled: bool,
    /// The fill of the highlighted shape.
    pub item_style: Option<ItemStyleOption>,
    /// The stroke of the highlighted shape.
    pub line_style: Option<LineStyleOption>,
    /// The label of the highlighted shape.
    pub label: Option<LabelOption>,
    /// The scale the highlighted shape is drawn at.
    pub scale: Option<bool>,
    /// The focus mode of the highlight.
    pub focus: Option<String>,
    /// The blur applied to the other data points.
    pub blur: Option<BlurOption>,
}

/// The blur applied to the data points that are not highlighted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BlurOption {
    /// The opacity of the blurred shapes.
    pub item_style: Option<ItemStyleOption>,
    /// The opacity of the blurred lines.
    pub line_style: Option<LineStyleOption>,
    /// The opacity of the blurred labels.
    pub label: Option<LabelOption>,
}

/// A mark drawn at a data point.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MarkPointOption {
    /// The symbol drawn at the point.
    pub symbol: Option<String>,
    /// The size of the symbol.
    pub symbol_size: Option<f64>,
    /// The fill of the symbol.
    pub item_style: Option<ItemStyleOption>,
    /// The label of the symbol.
    pub label: Option<LabelOption>,
    /// The points to mark.
    pub data: Vec<DataValue>,
}

/// A line drawn across the series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MarkLineOption {
    /// Whether the mark line is drawn.
    pub silent: bool,
    /// The stroke of the mark line.
    pub line_style: Option<LineStyleOption>,
    /// The label of the mark line.
    pub label: Option<LabelOption>,
    /// The kind of the mark line.
    pub mark_type: Option<MarkLineType>,
    /// The data the mark line is computed from.
    pub data: Vec<DataValue>,
}

/// An area highlighted across the series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MarkAreaOption {
    /// Whether the marked area is silent to pointer events.
    pub silent: bool,
    /// The fill of the marked area.
    pub item_style: Option<ItemStyleOption>,
    /// The label of the marked area.
    pub label: Option<LabelOption>,
    /// The data the marked area is computed from.
    pub data: Vec<DataValue>,
}

/// The options specific to a bar series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BarOptions {
    /// The width of the bars, in pixels.
    pub bar_width: Option<f64>,
    /// The maximum width of the bars, in pixels.
    pub bar_max_width: Option<f64>,
    /// The minimum width of the bars, in pixels.
    pub bar_min_width: Option<f64>,
    /// The width of the gap between bars, as a fraction of the bar width.
    pub bar_gap: Option<String>,
    /// The width of the gap between bar groups, as a fraction of the bar
    /// width.
    pub bar_category_gap: Option<String>,
    /// Whether the bars are drawn horizontally.
    pub horizontal: bool,
    /// The colour of the background behind the bars.
    pub show_background: bool,
    /// The fill of the background behind the bars.
    pub background_style: Option<ItemStyleOption>,
    /// The border radius of the bars.
    pub bar_min_radius: Option<f64>,
    /// Whether the bars are stacked.
    pub stack: Option<String>,
}

/// The options specific to a line series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LineOptions {
    /// Whether the area under the line is filled.
    pub area_style: Option<ItemStyleOption>,
    /// How the points are connected.
    pub smooth: Option<LineSmooth>,
    /// How steep a smoothed curve may get, from `0.0` to `1.0`.
    pub smooth_monotone: Option<String>,
    /// Whether the line is drawn.
    pub show_symbol: Option<bool>,
    /// The shape of the data point symbols.
    pub symbol: Option<SymbolType>,
    /// The size of the data point symbols.
    pub symbol_size: Option<SymbolSize>,
    /// The rotation of the data point symbols, in degrees.
    pub symbol_rotate: Option<f64>,
    /// The number of pixels between two symbols; symbols are hidden when
    /// the points are closer than this.
    pub show_all_symbol: Option<bool>,
    /// Whether the data points are clipped by the grid.
    pub clip: Option<bool>,
    /// The stack the line belongs to.
    pub stack: Option<String>,
}

/// The options specific to a pie series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PieOptions {
    /// The radius of the pie, as a percentage of the smaller of the
    /// container width and height.
    pub radius: Option<String>,
    /// The inner radius of the doughnut, as a percentage of the outer
    /// radius.
    pub inner_radius: Option<String>,
    /// The centre of the pie, as a percentage of the container.
    pub center: Option<Vec<String>>,
    /// The angle the first slice starts at, in degrees.
    pub start_angle: Option<f64>,
    /// The angle between two slices, in degrees.
    pub min_angle: Option<f64>,
    /// The angle the slices are drawn through, in degrees.
    pub max_angle: Option<f64>,
    /// The angle of the selected slice, in degrees.
    pub selected_offset: Option<f64>,
    /// Whether the slices are sorted by value.
    pub sort: Option<bool>,
    /// Whether the pie is drawn as a rose chart.
    pub rose_type: Option<String>,
    /// The label of a slice.
    pub label: Option<LabelOption>,
    /// The line between a slice and its label.
    pub label_line: Option<LineStyleOption>,
    /// Whether the edges between slices are rounded.
    pub round_cap: Option<bool>,
    /// The number of pixels the pie is pulled in by, to avoid overlapping
    /// the edge of the container.
    pub avoid_label_overlap: Option<bool>,
}

/// The options specific to a scatter series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScatterOptions {
    /// The shape of the symbols.
    pub symbol: Option<SymbolType>,
    /// The size of the symbols.
    pub symbol_size: Option<SymbolSize>,
    /// The rotation of the symbols, in degrees.
    pub symbol_rotate: Option<f64>,
    /// The largest size a symbol may take, when the size is computed from
    /// the data.
    pub symbol_max_size: Option<f64>,
    /// Whether the symbols are drawn in a separate pass, which keeps them
    /// readable when they overlap.
    pub large: bool,
    /// The threshold, in pixels, above which the large mode is used.
    pub large_threshold: Option<f64>,
    /// Whether the series is drawn in the large mode.
    pub progressive: Option<usize>,
}

/// The options specific to a radar series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RadarOptions {
    /// The centre of the radar, as a percentage of the container.
    pub center: Option<Vec<String>>,
    /// The radius of the radar, as a percentage of the container.
    pub radius: Option<String>,
    /// The number of sides of the radar.
    pub split_number: Option<usize>,
    /// The shape of the radar.
    pub shape: Option<String>,
    /// The starting angle of the radar, in degrees.
    pub start_angle: Option<f64>,
    /// The area under the radar is filled.
    pub area_style: Option<ItemStyleOption>,
}

/// The options specific to a gauge series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GaugeOptions {
    /// The centre of the gauge, as a percentage of the container.
    pub center: Option<Vec<String>>,
    /// The radius of the gauge, as a percentage of the container.
    pub radius: Option<String>,
    /// The start angle of the gauge, in degrees.
    pub start_angle: Option<f64>,
    /// The end angle of the gauge, in degrees.
    pub end_angle: Option<f64>,
    /// The minimum value of the gauge.
    pub min: Option<f64>,
    /// The maximum value of the gauge.
    pub max: Option<f64>,
    /// The number of segments the gauge is split into.
    pub split_number: Option<usize>,
    /// The progress arc of the gauge.
    pub progress: Option<GaugeProgressOption>,
    /// The pointer of the gauge.
    pub axis_line: Option<AxisLineOption>,
    /// The axis line of the gauge.
    pub axis_tick: Option<AxisTickOption>,
    /// The split line of the gauge.
    pub split_line: Option<SplitLineOption>,
    /// The pointer of the gauge.
    pub pointer: Option<PointerOption>,
    /// The anchor of the gauge's pointer.
    pub anchor: Option<PointerOption>,
    /// The title of the gauge.
    pub title: Option<LabelOption>,
    /// The detail of the gauge.
    pub detail: Option<LabelOption>,
}

/// The progress arc of a gauge.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GaugeProgressOption {
    /// Whether the arc is drawn.
    pub show: bool,
    /// The width of the arc.
    pub width: Option<f64>,
    /// The colour of the arc.
    pub item_style: Option<ItemStyleOption>,
    /// The kind of the arc.
    pub round_cap: Option<bool>,
}

/// The axis line of a gauge.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AxisLineOption {
    /// Whether the axis line is drawn.
    pub show: bool,
    /// The line style of the axis line.
    pub line_style: Option<LineStyleOption>,
}

/// The tick marks of a gauge.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AxisTickOption {
    /// Whether the ticks are drawn.
    pub show: bool,
    /// The length of the ticks.
    pub length: Option<f64>,
    /// The line style of the ticks.
    pub line_style: Option<LineStyleOption>,
}

/// The split lines of a gauge.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SplitLineOption {
    /// Whether the split lines are drawn.
    pub show: bool,
    /// The length of the split lines.
    pub length: Option<f64>,
    /// The line style of the split lines.
    pub line_style: Option<LineStyleOption>,
}

/// The pointer of a gauge.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PointerOption {
    /// Whether the pointer is drawn.
    pub show: bool,
    /// The width of the pointer.
    pub width: Option<f64>,
    /// The length of the pointer.
    pub length: Option<String>,
    /// The line style of the pointer.
    pub item_style: Option<ItemStyleOption>,
}

/// The options specific to a funnel series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FunnelOptions {
    /// The centre of the funnel.
    pub center: Option<Vec<String>>,
    /// The width of the funnel.
    pub width: Option<String>,
    /// The height of the funnel.
    pub height: Option<String>,
    /// The gap between two stages, as a fraction of the stage height.
    pub gap: Option<f64>,
    /// The label of a stage.
    pub label: Option<LabelOption>,
    /// The line between a stage and its label.
    pub label_line: Option<LineStyleOption>,
    /// The minimum size of a stage, as a percentage of the widest one.
    pub min_size: Option<String>,
    /// The maximum size of a stage, as a percentage of the widest one.
    pub max_size: Option<String>,
    /// Whether the stages are sorted by value.
    pub sort: Option<bool>,
}

/// The options specific to a heatmap series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeatmapOptions {
    /// The border width between cells.
    pub border_width: Option<f64>,
    /// The border colour between cells.
    pub border_color: Option<String>,
    /// The radius of the cells' corners.
    pub item_style: Option<ItemStyleOption>,
    /// The minimum size of a cell, in pixels.
    pub min_size: Option<f64>,
    /// The label of a cell.
    pub label: Option<LabelOption>,
}

/// The options specific to a graph series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GraphOptions {
    /// The layout used to place the nodes.
    pub layout: Option<GraphLayout>,
    /// Whether the graph can be panned and zoomed.
    pub roam: Option<bool>,
    /// Whether the nodes are dragged.
    pub draggable: Option<bool>,
    /// The size of the node symbols.
    pub symbol_size: Option<SymbolSize>,
    /// The shape of the node symbols.
    pub symbol: Option<SymbolType>,
    /// The label of a node.
    pub label: Option<LabelOption>,
    /// The stroke of the links.
    pub line_style: Option<LineStyleOption>,
    /// The curvature of the links.
    pub edge_curveness: Option<f64>,
    /// The orientation of the links.
    pub edge_symbol: Option<Vec<String>>,
    /// Whether the links are drawn.
    pub links: Option<Vec<DataValue>>,
}

/// The options specific to a treemap series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TreemapOptions {
    /// The layout of the treemap.
    pub layout: Option<TreemapLayout>,
    /// The width of the gap between the tiles.
    pub left: Option<f64>,
    /// The right gap between the tiles.
    pub right: Option<f64>,
    /// The top gap between the tiles.
    pub top: Option<f64>,
    /// The bottom gap between the tiles.
    pub bottom: Option<f64>,
    /// Whether the treemap is navigable.
    pub roam: Option<bool>,
    /// The depth of the treemap.
    pub node_click: Option<String>,
    /// The label of a tile.
    pub label: Option<LabelOption>,
    /// The upper level of the label.
    pub upper_label: Option<LabelOption>,
}

/// The options specific to a sunburst series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SunburstOptions {
    /// The centre of the sunburst.
    pub center: Option<Vec<String>>,
    /// The radius of the sunburst.
    pub radius: Option<Vec<String>>,
    /// The starting angle of the sunburst, in degrees.
    pub start_angle: Option<f64>,
    /// The label of a sector.
    pub label: Option<LabelOption>,
    /// Whether the sectors are clickable.
    pub node_click: Option<String>,
    /// The level of the highlighted sector.
    pub highlight: Option<String>,
}

/// The options specific to a boxplot series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoxplotOptions {
    /// The width of the boxes.
    pub box_width: Option<f64>,
    /// The label of a box.
    pub label: Option<LabelOption>,
}

/// The options specific to a candlestick series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CandlestickOptions {
    /// The width of the candles.
    pub bar_width: Option<f64>,
    /// The width of the candle's body.
    pub bar_max_width: Option<f64>,
    /// The minimum width of the candle's body.
    pub bar_min_width: Option<f64>,
    /// The colour of a rising candle.
    pub color: Option<String>,
    /// The colour of a falling candle.
    pub color0: Option<String>,
    /// The border colour of a rising candle.
    pub border_color: Option<String>,
    /// The border colour of a falling candle.
    pub border_color0: Option<String>,
    /// The label of a candle.
    pub label: Option<LabelOption>,
}

/// The options specific to a theme-river series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThemeRiverOptions {
    /// The thickness of the river.
    pub single_width: Option<f64>,
    /// The gap between the rivers.
    pub item_style: Option<ItemStyleOption>,
    /// The label of a river.
    pub label: Option<LabelOption>,
}

/// The options specific to a sankey series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SankeyOptions {
    /// The alignment of the sankey.
    pub align: Option<String>,
    /// The gap between the nodes, in pixels.
    pub node_gap: Option<f64>,
    /// The width of the links.
    pub node_width: Option<f64>,
    /// The curvature of the links.
    pub link_curveness: Option<f64>,
    /// Whether the sankey can be panned and zoomed.
    pub roam: Option<bool>,
    /// The label of a node.
    pub label: Option<LabelOption>,
    /// The line style of the links.
    pub line_style: Option<LineStyleOption>,
}

/// The options specific to a parallel-coordinates series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParallelOptions {
    /// The layout of the axes.
    pub layout: Option<String>,
    /// The gap between the axes.
    pub distance: Option<f64>,
    /// The width of the axis line.
    pub line_width: Option<f64>,
    /// The stroke of the links.
    pub line_style: Option<LineStyleOption>,
    /// The label of an axis.
    pub axis_label: Option<LabelOption>,
}

/// The options specific to a pictorial-bar series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PictorialBarOptions {
    /// The symbol used for the bars.
    pub symbol: Option<String>,
    /// The size of the symbol.
    pub symbol_size: Option<Vec<String>>,
    /// The position of the symbol relative to the bar.
    pub symbol_position: Option<String>,
    /// The offset of the symbol.
    pub symbol_offset: Option<Vec<f64>>,
    /// The rotation of the symbol.
    pub symbol_rotate: Option<f64>,
    /// Whether the symbol is repeated along the bar.
    pub symbol_repeat: Option<bool>,
    /// Whether the symbols are clipped by the bar.
    pub symbol_clip: Option<bool>,
}

/// The options specific to a lines series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LinesOptions {
    /// The curvature of the lines.
    pub polyline: bool,
    /// The stroke of the lines.
    pub line_style: Option<LineStyleOption>,
    /// The effect trail of the lines.
    pub effect: Option<LinesEffectOption>,
    /// The blend mode of the lines.
    pub blend_mode: Option<String>,
}

/// The trail drawn behind a moving line.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LinesEffectOption {
    /// Whether the trail is drawn.
    pub show: bool,
    /// The period of the trail, in milliseconds.
    pub period: Option<f64>,
    /// The scale of the trail's constant speed.
    pub constant_speed: Option<f64>,
    /// The colour of the trail.
    pub trail_width: Option<f64>,
    /// The colour of the trail.
    pub trail_color: Option<String>,
    /// The length of the trail.
    pub trail_length: Option<f64>,
}

/// The options specific to a map series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MapOptions {
    /// The name of the map region.
    pub map_name: Option<String>,
    /// The name of the geo component the map is bound to.
    pub geo_index: Option<usize>,
    /// Whether the map can be panned and zoomed.
    pub roam: Option<bool>,
    /// The label of a region.
    pub label: Option<LabelOption>,
    /// The factor that keeps the map from looking stretched.
    pub aspect_scale: Option<f64>,
}

/// The options specific to a custom series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomOptions {
    /// The name of the custom renderer.
    pub renderer: Option<String>,
    /// The items the renderer receives.
    pub data: Vec<DataValue>,
}

/// The options of one series.
///
/// One type covers every ECharts series kind: the fields that only make
/// sense for some kinds are grouped into the matching options struct, and
/// the renderer ignores the ones that do not apply to its kind.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct SeriesOption {
    /// The kind of the series.
    #[get(type(copy))]
    pub kind: SeriesType,
    /// The name of the series, used by the legend and the tooltip.
    pub name: String,
    /// The data of the series.
    pub data: Vec<DataValue>,
    /// The index of the x-axis the series is bound to.
    #[get(type(copy))]
    pub x_axis_index: Option<usize>,
    /// The index of the y-axis the series is bound to.
    #[get(type(copy))]
    pub y_axis_index: Option<usize>,
    /// The index of the polar coordinate system the series is bound to.
    #[get(type(copy))]
    pub polar_index: Option<usize>,
    /// The index of the radar coordinate system the series is bound to.
    #[get(type(copy))]
    pub radar_index: Option<usize>,
    /// The index of the geo component the series is bound to.
    #[get(type(copy))]
    pub geo_index: Option<usize>,
    /// The index of the calendar component the series is bound to.
    #[get(type(copy))]
    pub calendar_index: Option<usize>,
    /// The name of the dataset the series draws its data from.
    #[get(type(copy))]
    pub dataset_index: Option<usize>,
    /// The stack the series is stacked in.
    #[get(type(clone))]
    pub stack: Option<String>,
    /// The paint order of the series.
    #[get(type(copy))]
    pub z: Option<i32>,
    /// The canvas layer the series is drawn on.
    #[get(type(copy))]
    pub zlevel: Option<i32>,
    /// Whether the series ignores pointer events.
    #[get(type(copy))]
    pub silent: bool,
    /// Whether the series is drawn at all.
    #[get(type(copy))]
    pub show: bool,
    /// Whether the series is animated.
    #[get(type(copy))]
    pub animation: Option<bool>,
    /// The fill of the series' shapes.
    #[get(type(clone))]
    pub item_style: Option<ItemStyleOption>,
    /// The stroke of the series' shapes.
    #[get(type(clone))]
    pub line_style: Option<LineStyleOption>,
    /// The label of the series' data points.
    #[get(type(clone))]
    pub label: Option<LabelOption>,
    /// The rules that move the series' labels out of each other's way.
    #[get(type(clone))]
    pub label_layout: Option<LabelLayoutOption>,
    /// The style applied when a data point is highlighted.
    #[get(type(clone))]
    pub emphasis: Option<EmphasisOption>,
    /// The style applied when a data point is not highlighted.
    #[get(type(clone))]
    pub blur: Option<BlurOption>,
    /// The mark drawn at the series' data points.
    #[get(type(clone))]
    pub mark_point: Option<MarkPointOption>,
    /// The line drawn across the series.
    #[get(type(clone))]
    pub mark_line: Option<MarkLineOption>,
    /// The area highlighted across the series.
    #[get(type(clone))]
    pub mark_area: Option<MarkAreaOption>,
    /// The colour of the series, overriding the theme.
    #[get(type(clone))]
    pub color: Option<String>,
    /// The options specific to a bar series.
    #[get(type(clone))]
    pub bar: Option<BarOptions>,
    /// The options specific to a line series.
    #[get(type(clone))]
    pub line: Option<LineOptions>,
    /// The options specific to a pie series.
    #[get(type(clone))]
    pub pie: Option<PieOptions>,
    /// The options specific to a scatter series.
    #[get(type(clone))]
    pub scatter: Option<ScatterOptions>,
    /// The options specific to a radar series.
    #[get(type(clone))]
    pub radar: Option<RadarOptions>,
    /// The options specific to a gauge series.
    #[get(type(clone))]
    pub gauge: Option<GaugeOptions>,
    /// The options specific to a funnel series.
    #[get(type(clone))]
    pub funnel: Option<FunnelOptions>,
    /// The options specific to a heatmap series.
    #[get(type(clone))]
    pub heatmap: Option<HeatmapOptions>,
    /// The options specific to a graph series.
    #[get(type(clone))]
    pub graph: Option<GraphOptions>,
    /// The options specific to a treemap series.
    #[get(type(clone))]
    pub treemap: Option<TreemapOptions>,
    /// The options specific to a sunburst series.
    #[get(type(clone))]
    pub sunburst: Option<SunburstOptions>,
    /// The options specific to a boxplot series.
    #[get(type(clone))]
    pub boxplot: Option<BoxplotOptions>,
    /// The options specific to a candlestick series.
    #[get(type(clone))]
    pub candlestick: Option<CandlestickOptions>,
    /// The options specific to a theme-river series.
    #[get(type(clone))]
    pub theme_river: Option<ThemeRiverOptions>,
    /// The options specific to a sankey series.
    #[get(type(clone))]
    pub sankey: Option<SankeyOptions>,
    /// The options specific to a parallel-coordinates series.
    #[get(type(clone))]
    pub parallel: Option<ParallelOptions>,
    /// The options specific to a pictorial-bar series.
    #[get(type(clone))]
    pub pictorial_bar: Option<PictorialBarOptions>,
    /// The options specific to a lines series.
    #[get(type(clone))]
    pub lines: Option<LinesOptions>,
    /// The options specific to a map series.
    #[get(type(clone))]
    pub map: Option<MapOptions>,
    /// The options specific to a custom series.
    #[get(type(clone))]
    pub custom: Option<CustomOptions>,
}

/// The options of the chart's title.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct TitleOption {
    /// The main title.
    pub text: String,
    /// The subtitle.
    pub subtext: String,
    /// The position of the title.
    #[get(type(copy))]
    pub left: Option<Position>,
    /// The position of the title.
    #[get(type(copy))]
    pub top: Option<Position>,
    /// The position of the title.
    #[get(type(copy))]
    pub right: Option<Position>,
    /// The position of the title.
    #[get(type(copy))]
    pub bottom: Option<Position>,
    /// The style of the main title.
    #[get(type(clone))]
    pub text_style: Option<LabelOption>,
    /// The style of the subtitle.
    #[get(type(clone))]
    pub subtext_style: Option<LabelOption>,
    /// The gap between the title and the subtitle.
    #[get(type(copy))]
    pub item_gap: Option<f64>,
    /// Whether the title is drawn.
    #[get(type(copy))]
    pub show: bool,
    /// The link the title points at.
    #[get(type(clone))]
    pub link: Option<String>,
    /// The link the subtitle points at.
    #[get(type(clone))]
    pub sublink: Option<String>,
}

/// The options of the chart's legend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LegendOption {
    /// The names of the series the legend shows.
    pub data: Vec<String>,
    /// Whether the legend is drawn.
    pub show: bool,
    /// The direction the entries are laid out in.
    pub orient: Option<Orientation>,
    /// The position of the legend.
    pub left: Option<Position>,
    /// The position of the legend.
    pub top: Option<Position>,
    /// The position of the legend.
    pub right: Option<Position>,
    /// The position of the legend.
    pub bottom: Option<Position>,
    /// The width of the legend's symbol.
    pub item_width: Option<f64>,
    /// The height of the legend's symbol.
    pub item_height: Option<f64>,
    /// The gap between the legend's symbol and its label.
    pub item_gap: Option<f64>,
    /// The gap between the legend's entries.
    pub item_gap_horizontal: Option<f64>,
    /// The gap between the legend's entries.
    pub item_gap_vertical: Option<f64>,
    /// The style of the legend's labels.
    pub text_style: Option<LabelOption>,
    /// The style of the legend's symbols.
    pub item_style: Option<ItemStyleOption>,
    /// The names of the series that start out hidden.
    pub selected: Option<Vec<String>>,
    /// Whether the legend scrolls when it does not fit.
    pub type_scroll: Option<String>,
    /// The background colour of the legend's scrollbar.
    pub scroll: Option<String>,
}

/// The options of a cartesian grid.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridOption {
    /// The left edge of the grid.
    pub left: Option<Position>,
    /// The right edge of the grid.
    pub right: Option<Position>,
    /// The top edge of the grid.
    pub top: Option<Position>,
    /// The bottom edge of the grid.
    pub bottom: Option<Position>,
    /// Whether the grid's size accounts for the axis labels.
    pub contain_label: Option<bool>,
    /// The width of the grid.
    pub width: Option<f64>,
    /// The height of the grid.
    pub height: Option<f64>,
    /// The background colour of the grid.
    pub background_color: Option<String>,
    /// Whether the grid is drawn.
    pub show: bool,
    /// The border colour of the grid.
    pub border_color: Option<String>,
    /// The border width of the grid.
    pub border_width: Option<f64>,
}

/// The options of one axis.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct AxisOption {
    /// The kind of the axis.
    #[get(type(copy))]
    pub axis_type: AxisType,
    /// The categories or the tick values of the axis.
    pub data: Vec<DataValue>,
    /// The name of the axis.
    #[get(type(clone))]
    pub name: Option<String>,
    /// The position of the axis' name.
    #[get(type(clone))]
    pub name_location: Option<String>,
    /// The gap between the axis' name and the axis.
    #[get(type(copy))]
    pub name_gap: Option<f64>,
    /// The style of the axis' name.
    #[get(type(clone))]
    pub name_text_style: Option<LabelOption>,
    /// The position of the axis.
    #[get(type(copy))]
    pub position: Option<Position>,
    /// Whether the axis is drawn.
    #[get(type(copy))]
    pub show: bool,
    /// The labels of the axis.
    #[get(type(clone))]
    pub axis_label: Option<LabelOption>,
    /// The line of the axis.
    #[get(type(clone))]
    pub axis_line: Option<AxisLineOption>,
    /// The tick marks of the axis.
    #[get(type(clone))]
    pub axis_tick: Option<AxisTickOption>,
    /// The split lines of the axis.
    #[get(type(clone))]
    pub split_line: Option<SplitLineOption>,
    /// The minimum value of the axis.
    #[get(type(copy))]
    pub min: Option<f64>,
    /// The maximum value of the axis.
    #[get(type(copy))]
    pub max: Option<f64>,
    /// The gap between the axis and its split lines, as a fraction of the
    /// axis' length.
    #[get(type(copy))]
    pub split_gap: Option<f64>,
    /// The number of split lines.
    #[get(type(copy))]
    pub split_number: Option<usize>,
    /// The interval between two split lines.
    #[get(type(copy))]
    pub interval: Option<f64>,
    /// The minimum interval between two split lines.
    #[get(type(copy))]
    pub min_interval: Option<f64>,
    /// The maximum interval between two split lines.
    #[get(type(copy))]
    pub max_interval: Option<f64>,
    /// Whether the axis uses a logarithmic scale.
    #[get(type(copy))]
    pub log_base: Option<f64>,
    /// The boundary gap of a category axis.
    #[get(type(copy))]
    pub boundary_gap: Option<f64>,
    /// Whether the axis is aligned with the other axis of the same grid.
    #[get(type(copy))]
    pub align_ticks: Option<bool>,
    /// The index of the grid the axis belongs to.
    #[get(type(copy))]
    pub grid_index: Option<usize>,
    /// Whether the axis is drawn on the other side of the grid.
    #[get(type(copy))]
    pub offset: Option<f64>,
    /// The colour of the axis' area.
    #[get(type(clone))]
    pub axis_pointer: Option<AxisPointerOption>,
    /// The formatter used for the axis' labels.
    #[get(type(clone))]
    pub formatter: Option<String>,
    /// The colour of the axis.
    #[get(type(clone))]
    pub color: Option<String>,
}

/// The pointer that follows the mouse along an axis.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AxisPointerOption {
    /// Whether the pointer is drawn.
    pub show: bool,
    /// The type of the pointer.
    pub pointer_type: Option<String>,
    /// The style of the pointer.
    pub label: Option<LabelOption>,
    /// The line style of the pointer.
    pub line_style: Option<LineStyleOption>,
    /// The shadow of the pointer.
    pub shadow: Option<ShadowOption>,
    /// Whether the pointer snaps to the nearest tick.
    pub snap: Option<bool>,
}

/// The options of the chart's tooltip.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct TooltipOption {
    /// Whether the tooltip is drawn.
    #[get(type(copy))]
    pub show: bool,
    /// What the tooltip describes.
    #[get(type(copy))]
    pub trigger: TriggerType,
    /// The formatter of the tooltip's content.
    #[get(type(clone))]
    pub formatter: Option<String>,
    /// The background colour of the tooltip.
    #[get(type(clone))]
    pub background_color: Option<String>,
    /// The border colour of the tooltip.
    #[get(type(clone))]
    pub border_color: Option<String>,
    /// The border width of the tooltip.
    #[get(type(copy))]
    pub border_width: Option<f64>,
    /// The padding inside the tooltip.
    #[get(type(copy))]
    pub padding: Option<(f64, f64, f64, f64)>,
    /// The style of the tooltip's text.
    #[get(type(clone))]
    pub text_style: Option<LabelOption>,
    /// The position of the tooltip, as `[x, y]` percentages.
    #[get(type(clone))]
    pub position: Option<Vec<f64>>,
    /// Whether the tooltip follows the pointer.
    #[get(type(copy))]
    pub confine: Option<bool>,
    /// Whether the tooltip is shown when the pointer is over the axis.
    #[get(type(clone))]
    pub axis_pointer: Option<AxisPointerOption>,
    /// The extra CSS applied to the tooltip's element.
    #[get(type(clone))]
    pub extra_css_text: Option<String>,
    /// The transition of the tooltip.
    #[get(type(copy))]
    pub transition_duration: Option<f64>,
    /// The order the tooltip's entries are sorted in.
    #[get(type(clone))]
    pub order: Option<String>,
}

/// The options of the chart's toolbox.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToolboxOption {
    /// Whether the toolbox is drawn.
    pub show: bool,
    /// The position of the toolbox.
    pub left: Option<Position>,
    /// The position of the toolbox.
    pub top: Option<Position>,
    /// The position of the toolbox.
    pub right: Option<Position>,
    /// The position of the toolbox.
    pub bottom: Option<Position>,
    /// The features the toolbox offers.
    pub feature: Option<ToolboxFeatureOption>,
    /// The orientation of the toolbox's icons.
    pub orient: Option<Orientation>,
    /// The item gap of the toolbox.
    pub item_gap: Option<f64>,
    /// The item size of the toolbox.
    pub item_size: Option<f64>,
    /// The style of the toolbox's icons.
    pub icon_style: Option<ItemStyleOption>,
    /// The emphasis style of the toolbox's icons.
    pub emphasis: Option<EmphasisOption>,
}

/// The features the toolbox offers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToolboxFeatureOption {
    /// The data-zoom feature.
    pub data_zoom: Option<ToolboxFeatureSettings>,
    /// The restore feature.
    pub restore: Option<ToolboxFeatureSettings>,
    /// The save-as-image feature.
    pub save_as_image: Option<ToolboxFeatureSettings>,
    /// The data-view feature.
    pub data_view: Option<ToolboxFeatureSettings>,
    /// The magic-type feature.
    pub magic_type: Option<ToolboxFeatureSettings>,
    /// The brush feature.
    pub brush: Option<ToolboxFeatureSettings>,
}

/// A toolbox feature's settings.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToolboxFeatureSettings {
    /// Whether the feature is drawn.
    pub show: bool,
    /// The title of the feature's button.
    pub title: Option<String>,
}

/// The options of one data-zoom control.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DataZoomOption {
    /// The kind of the control.
    pub zoom_type: DataZoomType,
    /// The index of the x-axis the control zooms.
    pub x_axis_index: Option<usize>,
    /// The index of the y-axis the control zooms.
    pub y_axis_index: Option<usize>,
    /// The index of the grid the control zooms.
    pub grid_index: Option<usize>,
    /// The start of the zoom window, as a percentage.
    pub start: Option<f64>,
    /// The end of the zoom window, as a percentage.
    pub end: Option<f64>,
    /// The start of the zoom window, as a data index.
    pub start_value: Option<f64>,
    /// The end of the zoom window, as a data index.
    pub end_value: Option<f64>,
    /// Whether the control is drawn.
    pub show: bool,
    /// The colour of the selected region.
    pub filler_color: Option<String>,
    /// The border colour of the selected region.
    pub border_color: Option<String>,
    /// The width of the selected region's border.
    pub border_width: Option<f64>,
    /// The handle size of the slider.
    pub handle_size: Option<String>,
    /// The label of the slider.
    pub label: Option<LabelOption>,
    /// The background colour of the slider.
    pub background_color: Option<String>,
    /// The data the control zooms over.
    pub data: Vec<DataValue>,
    /// The colour of the moved region.
    pub move_handle_size: Option<String>,
    /// The brush style of the inside control.
    pub zoom_lock: Option<bool>,
    /// Whether the inside control is disabled.
    pub disabled: Option<bool>,
    /// The throttle of the control's response, in milliseconds.
    pub throttle: Option<f64>,
}

/// The options of one visual map.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VisualMapOption {
    /// The kind of the visual map.
    pub visual_type: VisualMapType,
    /// Whether the visual map is drawn.
    pub show: bool,
    /// The minimum value of the visual map.
    pub min: Option<f64>,
    /// The maximum value of the visual map.
    pub max: Option<f64>,
    /// The dimension the visual map applies to.
    pub dimension: Option<usize>,
    /// The index of the series the visual map applies to.
    pub series_index: Option<usize>,
    /// The number of bands of a piecewise visual map.
    pub split_number: Option<usize>,
    /// The colours of the visual map.
    pub colors: Vec<String>,
    /// The index of the piece the value falls into.
    pub pieces: Vec<DataValue>,
    /// The index of the band the value falls into.
    pub out_of_range: Option<String>,
    /// Whether the visual map is horizontal.
    pub orient: Option<Orientation>,
    /// The position of the visual map.
    pub left: Option<Position>,
    /// The position of the visual map.
    pub top: Option<Position>,
    /// The position of the visual map.
    pub right: Option<Position>,
    /// The position of the visual map.
    pub bottom: Option<Position>,
    /// The width of the visual map.
    pub item_width: Option<f64>,
    /// The height of the visual map.
    pub item_height: Option<f64>,
    /// The index of the continuous band the value falls into.
    pub calculable: Option<bool>,
    /// The label of the visual map.
    pub text: Option<Vec<String>>,
    /// The style of the visual map's labels.
    pub text_style: Option<LabelOption>,
    /// The range of the visual map's handles.
    pub range: Option<Vec<f64>>,
    /// The precision of the visual map's labels.
    pub precision: Option<usize>,
}

/// The options of a polar coordinate system.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PolarOption {
    /// The centre of the polar system.
    pub center: Option<Vec<String>>,
    /// The radius of the polar system.
    pub radius: Option<String>,
    /// The starting angle of the polar system, in degrees.
    pub start_angle: Option<f64>,
    /// Whether the polar system is drawn.
    pub show: bool,
    /// The axis of the polar system.
    pub axis_line: Option<AxisLineOption>,
    /// The split lines of the polar system.
    pub split_line: Option<SplitLineOption>,
    /// The split area of the polar system.
    pub split_area: Option<SplitAreaOption>,
    /// The angle axis of the polar system.
    pub angle_axis: Option<AxisOption>,
    /// The radius axis of the polar system.
    pub radius_axis: Option<AxisOption>,
}

/// The alternating background of a polar system's sectors.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SplitAreaOption {
    /// Whether the split areas are drawn.
    pub show: bool,
    /// The fill of the split areas.
    pub area_style: Option<ItemStyleOption>,
}

/// The options of a radar coordinate system.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RadarOption {
    /// The indicators of the radar.
    pub indicator: Vec<DataValue>,
    /// The centre of the radar.
    pub center: Option<Vec<String>>,
    /// The radius of the radar.
    pub radius: Option<String>,
    /// The starting angle of the radar, in degrees.
    pub start_angle: Option<f64>,
    /// The number of sides of the radar.
    pub split_number: Option<usize>,
    /// The shape of the radar.
    pub shape: Option<String>,
    /// Whether the radar is drawn.
    pub show: bool,
    /// The name of the radar.
    pub name: Option<String>,
    /// The line of the radar's axes.
    pub axis_line: Option<AxisLineOption>,
    /// The split lines of the radar.
    pub split_line: Option<SplitLineOption>,
    /// The split area of the radar.
    pub split_area: Option<SplitAreaOption>,
}

/// The options of a geo component.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeoOption {
    /// The name of the map region.
    pub map_name: Option<String>,
    /// Whether the map can be panned and zoomed.
    pub roam: Option<bool>,
    /// The centre of the map.
    pub center: Option<Vec<f64>>,
    /// The zoom level of the map.
    pub zoom: Option<f64>,
    /// The scale of the map.
    pub scale: Option<f64>,
    /// The regions of the map.
    pub regions: Vec<DataValue>,
    /// The label of a region.
    pub label: Option<LabelOption>,
    /// The fill of a region.
    pub item_style: Option<ItemStyleOption>,
    /// The stroke of a region.
    pub line_style: Option<LineStyleOption>,
    /// The emphasis style of a region.
    pub emphasis: Option<EmphasisOption>,
    /// Whether the map is drawn.
    pub show: bool,
    /// The factor that keeps the map from looking stretched.
    pub aspect_scale: Option<f64>,
}

/// The options of a calendar component.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CalendarOption {
    /// The range of dates the calendar covers.
    pub range: Option<Vec<String>>,
    /// The size of one cell, in pixels.
    pub cell_size: Option<f64>,
    /// The orientation of the calendar.
    pub orient: Option<Orientation>,
    /// The top-left corner of the calendar.
    pub top: Option<f64>,
    /// The left edge of the calendar.
    pub left: Option<f64>,
    /// The item style of a cell.
    pub item_style: Option<ItemStyleOption>,
    /// The label of a cell.
    pub label: Option<LabelOption>,
    /// The line style of the cell borders.
    pub line_style: Option<LineStyleOption>,
    /// The split lines of the calendar.
    pub split_line: Option<SplitLineOption>,
    /// The year range the calendar shows.
    pub year_label: Option<LabelOption>,
    /// The month range the calendar shows.
    pub month_label: Option<LabelOption>,
    /// The day range the calendar shows.
    pub day_label: Option<LabelOption>,
    /// The ranges of the calendar.
    pub ranges: Vec<DataValue>,
}

/// The options of a dataset.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DatasetOption {
    /// The source of the data.
    pub source: Vec<DataValue>,
    /// The kind of the source.
    pub source_type: Option<DatasetSourceType>,
    /// The names of the dimensions.
    pub dimensions: Vec<String>,
    /// The dimensions the source is encoded by.
    pub dimensions_defined: Option<String>,
    /// The indexes of the dimensions that are not numeric.
    pub encode: Option<DataValue>,
    /// The mapping of the dimensions to their display names.
    pub source_header: Option<bool>,
}

/// The options of a graphic element.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GraphicOption {
    /// The kind of the element.
    pub element_type: Option<String>,
    /// The left position of the element.
    pub left: Option<Position>,
    /// The top position of the element.
    pub top: Option<Position>,
    /// The right position of the element.
    pub right: Option<Position>,
    /// The bottom position of the element.
    pub bottom: Option<Position>,
    /// The bounding rectangle of the element.
    pub bounding: Option<String>,
    /// The rotation of the element, in degrees.
    pub rotation: Option<f64>,
    /// The width of the element.
    pub width: Option<f64>,
    /// The height of the element.
    pub height: Option<f64>,
    /// The children of the element.
    pub children: Vec<GraphicOption>,
    /// The style of the element.
    pub style: Option<ItemStyleOption>,
    /// The text of the element.
    pub text: Option<LabelOption>,
    /// The text content of the element.
    pub text_content: Option<String>,
}

/// The options of the brush component.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BrushOption {
    /// Whether the brush is drawn.
    pub show: bool,
    /// The areas the brush covers.
    pub toolbox: Option<Vec<String>>,
    /// The rectangle of the brush.
    pub x_axis_index: Option<usize>,
    /// The rectangle of the brush.
    pub y_axis_index: Option<usize>,
    /// The areas the brush covers.
    pub brush_link: Option<Vec<String>>,
    /// The areas the brush covers.
    pub areas: Vec<DataValue>,
    /// The style of the brush.
    pub brush_style: Option<ItemStyleOption>,
    /// The throttle of the brush's response.
    pub throttle_type: Option<String>,
    /// The throttle of the brush's response.
    pub throttle_delay: Option<f64>,
}

/// The whole configuration of a chart.
///
/// The field names mirror ECharts' option object, so an ECharts
/// configuration can be translated field by field.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct ChartOption {
    /// The title of the chart.
    #[get(type(clone))]
    pub title: Option<TitleOption>,
    /// The legend of the chart.
    #[get(type(clone))]
    pub legend: Option<LegendOption>,
    /// The cartesian grid of the chart.
    #[get(type(clone))]
    pub grid: Option<GridOption>,
    /// The x-axes of the chart.
    pub x_axis: Vec<AxisOption>,
    /// The y-axes of the chart.
    pub y_axis: Vec<AxisOption>,
    /// The polar coordinate systems of the chart.
    pub polar: Vec<PolarOption>,
    /// The radar coordinate systems of the chart.
    pub radar: Vec<RadarOption>,
    /// The geo components of the chart.
    pub geo: Vec<GeoOption>,
    /// The calendar components of the chart.
    pub calendar: Vec<CalendarOption>,
    /// The dataset the chart draws its data from.
    #[get(type(clone))]
    pub dataset: Option<DatasetOption>,
    /// The series of the chart.
    pub series: Vec<SeriesOption>,
    /// The tooltip of the chart.
    #[get(type(clone))]
    pub tooltip: Option<TooltipOption>,
    /// The toolbox of the chart.
    #[get(type(clone))]
    pub toolbox: Option<ToolboxOption>,
    /// The data-zoom controls of the chart.
    pub data_zoom: Vec<DataZoomOption>,
    /// The visual maps of the chart.
    pub visual_map: Vec<VisualMapOption>,
    /// The graphic elements of the chart.
    pub graphic: Vec<GraphicOption>,
    /// The brush of the chart.
    #[get(type(clone))]
    pub brush: Option<BrushOption>,
    /// The axis pointer of the chart.
    #[get(type(clone))]
    pub axis_pointer: Option<AxisPointerOption>,
    /// The palette the series take their colours from.
    pub color: Vec<String>,
    /// The background colour of the chart.
    #[get(type(clone))]
    pub background_color: Option<String>,
    /// The default text style of the chart.
    #[get(type(clone))]
    pub text_style: Option<LabelOption>,
    /// Whether the chart animates.
    #[get(type(copy))]
    pub animation: bool,
    /// The duration of the chart's animation, in milliseconds.
    #[get(type(copy))]
    pub animation_duration: Option<f64>,
    /// The easing of the chart's animation.
    #[get(type(clone))]
    pub animation_easing: Option<String>,
    /// The delay of the chart's animation, in milliseconds.
    #[get(type(copy))]
    pub animation_delay: Option<f64>,
    /// The duration of the update animation, in milliseconds.
    #[get(type(copy))]
    pub animation_duration_update: Option<f64>,
    /// The easing of the update animation.
    #[get(type(clone))]
    pub animation_easing_update: Option<String>,
    /// Whether the chart is rendered on a separate canvas layer.
    #[get(type(copy))]
    pub use_utc: Option<bool>,
    /// The accessible description of the chart.
    #[get(type(clone))]
    pub aria: Option<AriaOption>,
}

/// The accessible description of a chart.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AriaOption {
    /// Whether the description is generated.
    pub enabled: bool,
    /// The description of the chart.
    pub description: Option<String>,
    /// The description of the chart's data.
    pub decal: Option<DataValue>,
}
