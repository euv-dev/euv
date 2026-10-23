use super::*;

/// A mapping from data values to pixel positions.
///
/// One scale serves one axis. The scale knows the data domain (the range
/// of values the data cover) and the pixel range (the extent of the grid
/// along the axis), and maps between them.
#[derive(Clone, Debug, lombok_macros::Data, PartialEq)]
pub struct Scale {
    /// The kind of the scale.
    #[get(type(copy))]
    pub kind: ScaleKind,
    /// The lower end of the data domain.
    #[get(type(copy))]
    pub domain_min: f64,
    /// The upper end of the data domain.
    #[get(type(copy))]
    pub domain_max: f64,
    /// The lower end of the pixel range.
    #[get(type(copy))]
    pub range_min: f64,
    /// The upper end of the pixel range.
    #[get(type(copy))]
    pub range_max: f64,
    /// The categories of an ordinal scale.
    pub categories: Vec<String>,
    /// The base of a logarithmic scale.
    #[get(type(copy))]
    pub log_base: f64,
    /// The width of one band of an ordinal scale, in pixels.
    ///
    /// A band is the space one category occupies. It is larger than the
    /// distance between two category centres by exactly one band, which
    /// is what lets a bar chart draw a bar that fills its band without
    /// spilling into the neighbouring one.
    #[get(type(copy))]
    pub band: f64,
    /// The number of categories of an ordinal scale.
    ///
    /// The count is kept apart from the names because the two are not
    /// the same: a chart over a million unnamed points has a million
    /// categories and not one name.
    #[get(type(copy))]
    pub count: usize,
}

/// One tick of an axis.
#[derive(Clone, Debug, PartialEq)]
pub struct AxisTick {
    /// The data value of the tick.
    pub value: f64,
    /// The text of the tick's label.
    pub label: String,
    /// The pixel position of the tick along the axis.
    pub position: f64,
}

/// The layout of one axis.
#[derive(Clone, Debug, lombok_macros::Data, PartialEq)]
pub struct AxisLayout {
    /// The index of the axis in the option.
    #[get(type(copy))]
    pub index: usize,
    /// The kind of the axis.
    #[get(type(copy))]
    pub kind: AxisType,
    /// The scale the axis maps with.
    pub scale: Scale,
    /// The ticks of the axis.
    pub ticks: Vec<AxisTick>,
    /// The side of the grid the axis sits on.
    #[get(type(copy))]
    pub side: AxisSide,
    /// The pixel position of the axis' line, perpendicular to the axis.
    #[get(type(copy))]
    pub offset: f64,
    /// The space the axis' labels need, perpendicular to the axis.
    #[get(type(copy))]
    pub label_space: f64,
    /// The name of the axis.
    #[get(type(clone))]
    pub name: Option<String>,
    /// The space the axis' name needs, perpendicular to the axis.
    #[get(type(copy))]
    pub name_space: f64,
    /// Whether the axis is drawn.
    #[get(type(copy))]
    pub show: bool,
    /// The labels of the axis.
    pub labels: Vec<AxisTick>,
    /// How many ticks pass between two labelled ones.
    ///
    /// A category axis with a thousand categories cannot show a
    /// thousand labels: they would overlap into a smear. The interval
    /// is the number of ticks between two labels, so that the labels
    /// are far enough apart to be read.
    #[get(type(copy))]
    pub label_interval: usize,
}

/// The layout of one cartesian grid.
#[derive(Clone, Debug, lombok_macros::Data, PartialEq)]
pub struct GridLayout {
    /// The index of the grid in the option.
    #[get(type(copy))]
    pub index: usize,
    /// The rectangle the grid occupies, in canvas pixels.
    #[get(type(copy))]
    pub rect: BBox,
    /// The x-axes of the grid.
    pub x_axes: Vec<AxisLayout>,
    /// The y-axes of the grid.
    pub y_axes: Vec<AxisLayout>,
}

/// One data point of a series, in pixel space.
#[derive(Clone, Debug, PartialEq)]
pub struct SeriesPoint {
    /// The horizontal position, in canvas pixels.
    pub x: f64,
    /// The vertical position, in canvas pixels.
    pub y: f64,
    /// The value of the point.
    pub value: f64,
    /// The name of the point.
    pub name: String,
    /// The index of the point in the series' data.
    pub data_index: usize,
}

/// The layout of one series.
#[derive(Clone, Debug, PartialEq)]
pub struct SeriesLayout {
    /// The index of the series in the option.
    pub index: usize,
    /// The kind of the series.
    pub kind: SeriesType,
    /// The name of the series.
    pub name: String,
    /// The index of the grid the series is drawn in.
    pub grid_index: usize,
    /// The points of the series, in pixel space.
    pub points: Vec<SeriesPoint>,
    /// The stack the series is stacked in, if any.
    pub stack: Option<String>,
    /// The index of the stack the series belongs to, within its group.
    pub stack_index: usize,
    /// The number of series stacked together with this one.
    pub stack_size: usize,
    /// The plot area the chart's non-cartesian series are drawn in.
    pub plot_rect: BBox,
    /// The width of one category band, in pixels.
    ///
    /// Zero for a series whose x-axis is not ordinal.
    pub band_width: f64,
}

/// The whole layout of a chart.
#[derive(Clone, Debug, Default, lombok_macros::Data, PartialEq)]
pub struct ChartLayout {
    /// The width of the chart, in canvas pixels.
    #[get(type(copy))]
    pub width: f64,
    /// The height of the chart, in canvas pixels.
    #[get(type(copy))]
    pub height: f64,
    /// The grids of the chart.
    pub grids: Vec<GridLayout>,
    /// The series of the chart.
    pub series: Vec<SeriesLayout>,
    /// The maps of the chart, projected into the plot area.
    pub geo: Vec<GeoLayout>,
    /// The rectangle the title occupies.
    #[get(type(copy))]
    pub title_rect: Option<BBox>,
    /// The rectangle the legend occupies.
    #[get(type(copy))]
    pub legend_rect: Option<BBox>,
    /// The rectangle the chart's plot area occupies.
    #[get(type(copy))]
    pub plot_rect: BBox,
}

/// One band of a piecewise visual map.
#[derive(Clone, Debug, PartialEq)]
pub struct VisualPiece {
    /// The lowest value of the band.
    pub min: f64,
    /// The highest value of the band.
    pub max: f64,
    /// The colour of the band.
    pub color: Color,
}

/// A visual map, resolved for drawing.
///
/// A visual map turns a value into a colour. The chart resolves the
/// option once, and then asks the map for the colour of every value it
/// draws — which is what keeps a map of a country from walking the
/// palette for every region.
#[derive(Clone, Debug, lombok_macros::Data, PartialEq)]
pub struct VisualMap {
    /// The kind of the visual map.
    #[get(type(copy))]
    pub visual_type: VisualMapType,
    /// The lowest value the map covers.
    #[get(type(copy))]
    pub min: f64,
    /// The highest value the map covers.
    #[get(type(copy))]
    pub max: f64,
    /// The colours of the map, from the lowest value to the highest.
    pub colors: Vec<Color>,
    /// The number of bands a piecewise map is cut into.
    #[get(type(copy))]
    pub split_number: usize,
    /// The bands of a piecewise map, when the option names them.
    pub pieces: Vec<VisualPiece>,
    /// The colour of the values outside the range.
    #[get(type(copy))]
    pub out_of_range: Option<Color>,
}
