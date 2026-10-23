/// The z-order of the chart's background.
pub(crate) const Z_BACKGROUND: i32 = 0;

/// The z-order of the chart's grid.
pub(crate) const Z_GRID: i32 = 10;

/// The z-order of the chart's axes.
pub(crate) const Z_AXIS: i32 = 20;

/// The z-order the first series is drawn at.
pub(crate) const Z_SERIES: i32 = 100;

/// The z-order of the chart's title.
pub(crate) const Z_TITLE: i32 = 1000;

/// The z-order of the chart's legend.
pub(crate) const Z_LEGEND: i32 = 1010;

/// The z-order of the chart's tooltip.
pub(crate) const Z_TOOLTIP: i32 = 2000;

/// The radius of the circle a custom series draws, in pixels.
pub(crate) const DEFAULT_CUSTOM_RADIUS: f64 = 10.0;

/// The key of a data item's name.
pub(crate) const SERIES_DATA_NAME_KEY: &str = "name";

/// The key of a data item's value.
pub(crate) const SERIES_DATA_VALUE_KEY: &str = "value";

/// The key of a graph link's source node.
pub(crate) const SERIES_DATA_SOURCE_KEY: &str = "source";

/// The key of a graph link's target node.
pub(crate) const SERIES_DATA_TARGET_KEY: &str = "target";

/// The key of a hierarchy node's children.
pub(crate) const SERIES_DATA_CHILDREN_KEY: &str = "children";

/// The name of the rectangular shape of a custom series.
pub(crate) const CUSTOM_SHAPE_RECT: &str = "rect";

/// The name of the rectangular symbol.
pub(crate) const SYMBOL_RECT: &str = "rect";
