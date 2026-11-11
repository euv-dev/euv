/// The prefix of the custom property that carries the colour of a series.
///
/// The plot inside a chart reads the colour of a series from a custom
/// property named after the series, so the property is `--color-<series>`.
pub(crate) const CHART_COLOR_VAR_PREFIX: &str = "--color-";
