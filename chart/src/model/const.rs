/// The default left inset of a grid, as a fraction of the chart's width.
pub(crate) const DEFAULT_GRID_LEFT: f64 = 0.1;

/// The default right inset of a grid, as a fraction of the chart's width.
pub(crate) const DEFAULT_GRID_RIGHT: f64 = 0.1;

/// The default top inset of a grid, in pixels.
pub(crate) const DEFAULT_GRID_TOP: f64 = 60.0;

/// The default bottom inset of a grid, in pixels.
pub(crate) const DEFAULT_GRID_BOTTOM: f64 = 60.0;

/// The number of split lines a value axis aims for.
///
/// Five is the sweet spot: enough ticks to read a value off the axis,
/// few enough that the labels do not collide.
pub(crate) const DEFAULT_SPLIT_NUMBER: usize = 5;

/// The largest number of ticks an unnamed category axis produces.
///
/// A chart over a million unnamed points cannot show a million ticks;
/// the axis shows the first ones and leaves the rest to the series.
pub(crate) const MAX_CATEGORY_TICKS: usize = 200;

/// The colours a visual map uses when the option names none.
///
/// These are the colours ECharts falls back to, and they are the ones
/// a chart of a country looks right in.
pub(crate) const DEFAULT_VISUAL_MAP_COLORS: &[&str] = &["#bf444c", "#d88273", "#f6efa6"];

/// The number of bands a piecewise visual map is cut into.
pub(crate) const DEFAULT_VISUAL_MAP_SPLIT: usize = 5;

/// The field of a data object that carries the name of the point.
pub(crate) const DATA_FIELD_NAME: &str = "name";

/// The field of a data object that carries the value of the point.
pub(crate) const DATA_FIELD_VALUE: &str = "value";

/// The field of a visual-map piece that carries its lowest value.
pub(crate) const PIECE_FIELD_MIN: &str = "min";

/// The field of a visual-map piece that carries its highest value.
pub(crate) const PIECE_FIELD_MAX: &str = "max";

/// The field of a visual-map piece that carries its colour.
pub(crate) const PIECE_FIELD_COLOR: &str = "color";
