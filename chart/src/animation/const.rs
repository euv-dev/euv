/// The number of milliseconds in a second.
///
/// The engine's tween counts in seconds and the chart's API counts in
/// milliseconds, and this is where the two meet.
pub(crate) const MILLISECONDS_PER_SECOND: f64 = 1000.0;

/// The key of the animated value inside a data point.
///
/// A data point that is an object carries its value under this key, the
/// way ECharts writes it.
pub(crate) const DATA_VALUE_KEY: &str = "value";
