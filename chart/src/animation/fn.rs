use super::*;

/// Collects the animated values of every series of an option.
///
/// Only the value of a data point animates; the name and the other
/// fields of a data point are structural and stay as they are.
///
/// # Arguments
///
/// - `&ChartOption` - Shared reference to a `ChartOption`.
///
/// # Returns
///
/// - `Vec<Vec<f64>>` - A `Vec<Vec<f64>>` value.
pub(crate) fn collect_values(option: &ChartOption) -> Vec<Vec<f64>> {
    option
        .series
        .iter()
        .map(|series: &SeriesOption| {
            series
                .data
                .iter()
                .map(|value: &DataValue| animated_value(value).unwrap_or(0.0))
                .collect()
        })
        .collect()
}

/// Returns the value of a data point that takes part in the animation.
///
/// # Arguments
///
/// - `&DataValue` - Shared reference to a `DataValue`.
///
/// # Returns
///
/// - `Option<f64>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn animated_value(value: &DataValue) -> Option<f64> {
    match value {
        DataValue::Number(number) => Some(*number),
        DataValue::List(items) => items.get(1).and_then(|item: &DataValue| item.to_number()),
        DataValue::Object(_) => value
            .get(DATA_VALUE_KEY)
            .and_then(|item: &DataValue| item.to_number()),
        _ => None,
    }
}

/// Writes an animated value back into a data point.
///
/// The shape of the data point is preserved: a bare number stays a bare
/// number, and a `[name, value]` pair keeps its name.
///
/// # Arguments
///
/// - `&mut DataValue` - Mutable reference to a `DataValue` (mutated in place).
/// - `f64` - A 64-bit float (`f64`).
pub(crate) fn write_value(data: &mut DataValue, value: f64) {
    match data {
        DataValue::Number(number) => *number = value,
        DataValue::List(items) => {
            if let Some(slot) = items.get_mut(1) {
                *slot = DataValue::Number(value);
            }
        }
        DataValue::Object(_) => {
            if let Some(slot) = data.get_mut(DATA_VALUE_KEY) {
                *slot = DataValue::Number(value);
            }
        }
        _ => {}
    }
}

/// Makes the starting values line up with the target values.
///
/// A series that is new has no starting values; it starts at zero, which
/// is what makes it grow out of the axis. A series that lost data keeps
/// the values it had, so that the points that remain do not jump.
///
/// # Arguments
///
/// - `&[Vec<f64>]` - Shared reference to a `[Vec<f64>]`.
///
/// # Returns
///
/// - `Vec<Vec<f64>>` - A `Vec<Vec<f64>>` value.
pub(crate) fn align_values(current: &[Vec<f64>], target: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut aligned: Vec<Vec<f64>> = Vec::with_capacity(target.len());
    for (index, values) in target.iter().enumerate() {
        let previous: Option<&Vec<f64>> = current.get(index);
        let mut series: Vec<f64> = Vec::with_capacity(values.len());
        for (position, _value) in values.iter().enumerate() {
            match previous.and_then(|previous: &Vec<f64>| previous.get(position)) {
                Some(previous) => series.push(*previous),
                None => series.push(0.0),
            }
        }
        aligned.push(series);
    }
    aligned
}

/// Interpolates between the starting and the target values.
///
/// # Arguments
///
/// - `&[Vec<f64>]` - Shared reference to a `[Vec<f64>]`.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `Vec<Vec<f64>>` - A `Vec<Vec<f64>>` value.
pub(crate) fn interpolate(from: &[Vec<f64>], to: &[Vec<f64>], progress: f64) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::with_capacity(to.len());
    for (index, values) in to.iter().enumerate() {
        let previous: Option<&Vec<f64>> = from.get(index);
        let mut series: Vec<f64> = Vec::with_capacity(values.len());
        for (position, value) in values.iter().enumerate() {
            let start: f64 = previous
                .and_then(|previous: &Vec<f64>| previous.get(position))
                .copied()
                .unwrap_or(0.0);
            series.push(start + (value - start) * progress);
        }
        result.push(series);
    }
    result
}
