/// The size of a data point's symbol.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymbolSize {
    /// The same size on both axes.
    Uniform(f64),
    /// A different size on each axis.
    NonUniform(f64, f64),
}

/// ECharts data is loosely typed: a point can be a bare number, a
/// `[name, value]` pair, or an object with named fields. The chart crate
/// keeps the same flexibility, so an existing ECharts dataset can be
/// ported without reshaping it.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum DataValue {
    /// A numeric value.
    Number(f64),
    /// A text value.
    Text(String),
    /// A boolean value.
    Bool(bool),
    /// An explicit null.
    #[default]
    Null,
    /// A list of values.
    List(Vec<DataValue>),
    /// An object with named fields, in insertion order.
    Object(Vec<(String, DataValue)>),
}

/// The kind of an axis.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AxisType {
    /// A category axis: the data are discrete names.
    #[default]
    Category,
    /// A value axis: the data are continuous numbers.
    Value,
    /// A time axis: the data are timestamps.
    Time,
    /// A log axis: the data are continuous numbers on a logarithmic
    /// scale.
    Log,
}

/// Where a component sits relative to its container.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Position {
    /// The top edge.
    Top,
    /// The vertical centre.
    #[default]
    Middle,
    /// The bottom edge.
    Bottom,
    /// The left edge.
    Left,
    /// The right edge.
    Right,
    /// Inside the container.
    Inside,
    /// A distance from the edge.
    Offset(f64),
}

/// The direction a component lays its items out in.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    /// Horizontally, left to right.
    #[default]
    Horizontal,
    /// Vertically, top to bottom.
    Vertical,
}

/// How a component picks the item a tooltip describes.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TriggerType {
    /// The tooltip describes the item under the pointer.
    #[default]
    Item,
    /// The tooltip describes the axis under the pointer.
    Axis,
    /// Nothing is shown.
    None,
}

/// The shape drawn for a data point.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SymbolType {
    /// A circle.
    #[default]
    Circle,
    /// A rectangle.
    Rect,
    /// A rounded rectangle.
    RoundRect,
    /// A triangle.
    Triangle,
    /// A diamond.
    Diamond,
    /// A pin.
    Pin,
    /// An arrow.
    Arrow,
    /// No symbol is drawn.
    None,
}

/// The kind of a data zoom control.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum DataZoomType {
    /// A slider under the chart.
    #[default]
    Slider,
    /// A draggable region inside the chart.
    Inside,
}

/// The kind of a visual map.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum VisualMapType {
    /// A continuous colour ramp.
    #[default]
    Continuous,
    /// Discrete colour bands.
    Piecewise,
}

/// The kind of a line drawn by a series.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LineType {
    /// A straight line between the points.
    #[default]
    Solid,
    /// A dashed line.
    Dashed,
    /// A dotted line.
    Dotted,
}

/// How a line series connects its points.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LineSmooth {
    /// Straight segments.
    #[default]
    None,
    /// A smoothed curve.
    Smooth,
    /// A stepped curve.
    Step,
}

/// The kind of a chart series.
///
/// The variant names match the ECharts `series.type` strings.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SeriesType {
    /// A line chart.
    #[default]
    Line,
    /// A bar chart.
    Bar,
    /// A pie chart.
    Pie,
    /// A scatter chart.
    Scatter,
    /// A scatter chart with a ripple effect.
    EffectScatter,
    /// A radar chart.
    Radar,
    /// A gauge chart.
    Gauge,
    /// A funnel chart.
    Funnel,
    /// A heatmap chart.
    Heatmap,
    /// A graph chart.
    Graph,
    /// A treemap chart.
    Treemap,
    /// A sunburst chart.
    Sunburst,
    /// A boxplot chart.
    Boxplot,
    /// A candlestick chart.
    Candlestick,
    /// A theme-river chart.
    ThemeRiver,
    /// A sankey chart.
    Sankey,
    /// A parallel-coordinates chart.
    Parallel,
    /// A pictorial bar chart.
    PictorialBar,
    /// A lines chart.
    Lines,
    /// A map chart.
    Map,
    /// A chart driven by a custom renderer.
    Custom,
}

/// The kind of a dataset source.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum DatasetSourceType {
    /// A two-dimensional array of values.
    #[default]
    Array,
    /// An object keyed by dimension.
    Object,
    /// A list of rows, each an object.
    Rows,
    /// A list of rows, each a list of values.
    Columns,
}

/// How a mark line is drawn.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum MarkLineType {
    /// A horizontal line at a value.
    #[default]
    Average,
    /// A horizontal line at a value.
    Min,
    /// A horizontal line at a value.
    Max,
    /// A line between two data points.
    Start,
    /// A line between two data points.
    End,
    /// A difference between two values.
    Difference,
}

/// The layout of a treemap.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TreemapLayout {
    /// The classic slice-and-dice layout.
    #[default]
    SliceAndDice,
    /// A squarified layout.
    Squarify,
}

/// The kind of a graph layout.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GraphLayout {
    /// No layout is applied; the positions come from the data.
    #[default]
    None,
    /// A force-directed layout.
    Force,
    /// A circular layout.
    Circular,
}

/// The shape of a gauge's progress arc.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GaugeProgressType {
    /// A solid arc.
    #[default]
    Solid,
    /// A striped arc.
    Stripe,
}
