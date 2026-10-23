use super::*;

/// Parses a percentage string such as `"50%"` into a fraction.
///
/// A bare number is returned as-is, so `"50"` means the same as `"50%"`.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<f64>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn parse_percent(input: &str) -> Option<f64> {
    let trimmed: &str = input.trim();
    if let Some(percent) = trimmed.strip_suffix('%') {
        return percent
            .trim()
            .parse::<f64>()
            .ok()
            .map(|value: f64| value / 100.0);
    }
    trimmed.parse::<f64>().ok()
}

/// Returns the number of decimals needed to show `value` without a
/// trailing `.0`.
///
/// Axis labels use this so that `1` prints as `1` and `1.5` prints as
/// `1.5`.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `String` - A `String` value.
pub(crate) fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return String::new();
    }
    let rounded: f64 = (value * 1_000_000.0).round() / 1_000_000.0;
    if (rounded - rounded.round()).abs() < f64::EPSILON {
        format!("{}", rounded.round() as i64)
    } else {
        let mut text = format!("{}", rounded);
        if text.contains('.') {
            while text.ends_with('0') {
                text.pop();
            }
            if text.ends_with('.') {
                text.pop();
            }
        }
        text
    }
}

/// Returns the name of the map a series draws.
///
/// A map series names its map itself, or it is bound to a geo
/// component, and the geo component names the map.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&ChartOption` - Shared reference to a `ChartOption`.
///
/// # Returns
///
/// - `Option<String>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn map_series_name(series: &SeriesOption, option: &ChartOption) -> Option<String> {
    if series.kind != SeriesType::Map {
        return None;
    }
    let map_options: MapOptions = series.map.clone().unwrap_or_default();
    if let Some(name) = &map_options.map_name {
        return Some(name.clone());
    }
    map_options
        .geo_index
        .and_then(|index: usize| option.geo.get(index))
        .and_then(|geo: &GeoOption| geo.map_name.clone())
}

/// Returns the factor that keeps the map of a series from looking
/// stretched.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&ChartOption` - Shared reference to a `ChartOption`.
///
/// # Returns
///
/// - `f64` - A 64-bit float.
pub(crate) fn map_series_aspect(series: &SeriesOption, option: &ChartOption) -> f64 {
    let map_options: MapOptions = series.map.clone().unwrap_or_default();
    if let Some(aspect) = map_options.aspect_scale {
        return aspect;
    }
    map_options
        .geo_index
        .and_then(|index: usize| option.geo.get(index))
        .and_then(|geo: &GeoOption| geo.aspect_scale)
        .unwrap_or(DEFAULT_ASPECT_SCALE)
}
