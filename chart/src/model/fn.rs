use super::*;

/// Returns the number of decimals a tick label should show.
///
/// Very large and very small numbers get fewer decimals, so that the
/// labels stay short enough not to collide.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `usize` - A non-negative integer.
pub(crate) fn tick_decimals(value: f64) -> usize {
    if value == 0.0 {
        return 0;
    }
    let magnitude: i32 = value.abs().log10().floor() as i32;
    match magnitude {
        m if m >= 6 => 0,
        m if m >= 3 => 1,
        m if m >= 0 => 2,
        _ => 4,
    }
}

/// Formats the label of a tick.
///
/// The number of decimals follows the size of the value, so that a tick
/// of `1 000` does not read `1000.00` and a tick of `0.05` does not read
/// `0.05` when the neighbouring tick is `0.10`.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `String` - A `String` value.
pub(crate) fn format_tick(value: f64, interval: f64) -> String {
    let decimals: usize = tick_decimals(value).min(tick_decimals(interval) + 2);
    let rounded: f64 = (value * 10f64.powi(decimals as i32)).round() / 10f64.powi(decimals as i32);
    let mut text = format!("{:.*}", decimals, rounded);
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

/// Parses the colours of a visual map.
///
/// The colours that cannot be read are left out: a palette with one
/// broken colour in it still works.
///
/// # Arguments
///
/// - `&[String]` - Shared reference to a `[String]`.
///
/// # Returns
///
/// - `Vec<Color>` - A `Vec<Color>` value.
pub(crate) fn parse_colors(colors: &[String]) -> Vec<Color> {
    if colors.is_empty() {
        return DEFAULT_VISUAL_MAP_COLORS
            .iter()
            .filter_map(|color: &&str| Color::parse(color))
            .collect();
    }
    colors
        .iter()
        .filter_map(|color: &String| Color::parse(color))
        .collect()
}

/// Parses the bands of a piecewise visual map.
///
/// A band names its range and its colour: `{ "min": 0, "max": 10,
/// "color": "#bf444c" }`.
///
/// # Arguments
///
/// - `&[DataValue]` - Shared reference to a `[DataValue]`.
///
/// # Returns
///
/// - `Vec<VisualPiece>` - A `Vec<VisualPiece>` value.
pub(crate) fn parse_pieces(pieces: &[DataValue]) -> Vec<VisualPiece> {
    let mut result: Vec<VisualPiece> = Vec::new();
    for piece in pieces {
        let min: Option<f64> = piece
            .get(PIECE_FIELD_MIN)
            .and_then(|value: &DataValue| value.to_number());
        let max: Option<f64> = piece
            .get(PIECE_FIELD_MAX)
            .and_then(|value: &DataValue| value.to_number());
        let color: Option<Color> = piece
            .get(PIECE_FIELD_COLOR)
            .and_then(|value: &DataValue| value.as_text())
            .and_then(Color::parse);
        let (Some(min), Some(max), Some(color)) = (min, max, color) else {
            continue;
        };
        result.push(VisualPiece { min, max, color });
    }
    result
}

/// Resolves one edge of a container against a position setting.
///
/// `total` is the size of the container along the edge's axis.
///
/// # Arguments
///
/// - `Option<&Position>` - A `Option<&Position>` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `f64` - A 64-bit float.
pub(crate) fn resolve_edge(position: Option<&Position>, default: f64, total: f64) -> f64 {
    match position {
        None => default,
        Some(Position::Offset(distance)) => *distance,
        Some(Position::Middle) | Some(Position::Inside) => total * 0.5,
        Some(Position::Left) | Some(Position::Top) => 0.0,
        Some(Position::Right) | Some(Position::Bottom) => total,
    }
}

/// Returns the CSS `font` shorthand for a font size.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `String` - A `String` value.
pub(crate) fn font_string(size: f64) -> String {
    format!("{}px sans-serif", size)
}

/// Collects the categories of the cartesian series of a chart.
///
/// A series whose points carry no name contributes no category names:
/// the axis falls back to the number of points, which saves a chart
/// over a million unnamed points from allocating a million empty
/// strings.
///
/// The names are looked for before they are collected. Building the
/// list first would allocate a vector with one entry per point — a
/// chart over a million unnamed points would pay for twenty-odd
/// megabytes of empty strings only to throw them away.
///
/// # Arguments
///
/// - `&[SeriesOption]` - Shared reference to a `[SeriesOption]`.
///
/// # Returns
///
/// - `Vec<String>` - A `Vec<String>` value.
pub(crate) fn collect_categories(series: &[SeriesOption]) -> Vec<String> {
    for item in series {
        if !is_cartesian(item) {
            continue;
        }
        if !item
            .data
            .iter()
            .any(|value: &DataValue| !point_label(value).is_empty())
        {
            continue;
        }
        return item.data.iter().map(point_label).collect();
    }
    Vec::new()
}

/// Returns the name a data point carries.
///
/// The name is the first element of a list, the `name` field of an
/// object, and the text of a plain value.
///
/// # Arguments
///
/// - `&DataValue` - Shared reference to a `DataValue`.
///
/// # Returns
///
/// - `String` - A `String` value.
fn point_label(value: &DataValue) -> String {
    match value {
        DataValue::List(items) => items
            .first()
            .and_then(|item: &DataValue| item.as_text())
            .map(|text: &str| text.to_string())
            .unwrap_or_default(),
        DataValue::Object(_) => value
            .get(DATA_FIELD_NAME)
            .and_then(|item: &DataValue| item.as_text())
            .unwrap_or_default()
            .to_string(),
        other => other.as_text().unwrap_or_default().to_string(),
    }
}

/// Returns `true` when a series is drawn on a cartesian grid.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
///
/// # Returns
///
/// - `bool` - A boolean.
fn is_cartesian(series: &SeriesOption) -> bool {
    matches!(
        series.kind,
        SeriesType::Line
            | SeriesType::Bar
            | SeriesType::Scatter
            | SeriesType::EffectScatter
            | SeriesType::Candlestick
            | SeriesType::Boxplot
            | SeriesType::Heatmap
            | SeriesType::PictorialBar
            | SeriesType::Custom
    )
}

/// Returns the number of points of the largest cartesian series.
///
/// # Arguments
///
/// - `&[SeriesOption]` - Shared reference to a `[SeriesOption]`.
///
/// # Returns
///
/// - `usize` - A non-negative integer.
pub(crate) fn category_count(series: &[SeriesOption]) -> usize {
    series
        .iter()
        .filter(|item: &&SeriesOption| is_cartesian(item))
        .map(|item: &SeriesOption| item.data.len())
        .max()
        .unwrap_or(0)
}

/// Returns the extent of the values bound to an axis.
///
/// `axis` selects which side of a cartesian series to read; `None` reads
/// the value side of every series.
///
/// # Arguments
///
/// - `&[SeriesOption]` - Shared reference to a `[SeriesOption]`.
/// - `Option<&AxisOption>` - A `Option<&AxisOption>` parameter.
///
/// # Returns
///
/// - `(f64, f64)` - A `(f64, f64)` value.
pub(crate) fn value_extent(series: &[SeriesOption], axis: Option<&AxisOption>) -> (f64, f64) {
    let mut min: f64 = f64::INFINITY;
    let mut max: f64 = f64::NEG_INFINITY;
    for item in series {
        if !matches!(
            item.kind,
            SeriesType::Line
                | SeriesType::Bar
                | SeriesType::Scatter
                | SeriesType::EffectScatter
                | SeriesType::Candlestick
                | SeriesType::Boxplot
                | SeriesType::Heatmap
        ) {
            continue;
        }
        for value in &item.data {
            let numbers: Vec<f64> = match value {
                DataValue::List(items) => items
                    .iter()
                    .filter_map(|item: &DataValue| item.to_number())
                    .collect(),
                DataValue::Object(_) => value
                    .get(DATA_FIELD_VALUE)
                    .and_then(|item: &DataValue| item.to_number())
                    .map(|number: f64| vec![number])
                    .unwrap_or_default(),
                other => other
                    .to_number()
                    .map(|number: f64| vec![number])
                    .unwrap_or_default(),
            };
            let selected: Option<f64> = match axis {
                Some(AxisOption {
                    axis_type: AxisType::Category,
                    ..
                }) => numbers.first().copied(),
                _ => match numbers.len() {
                    0 => None,
                    1 => numbers.first().copied(),
                    _ => numbers.last().copied(),
                },
            };
            if let Some(number) = selected {
                min = min.min(number);
                max = max.max(number);
            }
        }
    }
    if min.is_finite() && max.is_finite() {
        (min, max)
    } else {
        (0.0, 1.0)
    }
}

/// Rounds an extent to human-friendly bounds and returns the interval.
///
/// The algorithm is the classic "nice numbers" one: the range is rounded
/// to a power of ten, the interval is rounded to a 1/2/5 multiple of it,
/// and the bounds are snapped to multiples of that interval.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `(f64, f64)` - A `(f64, f64)` value.
pub(crate) fn nice_extent(min: f64, max: f64, split: usize) -> (f64, f64) {
    let (min, max, _) = nice_scale(min, max, split);
    (min, max)
}

/// Rounds an extent to human-friendly bounds and returns the interval
/// that produced them.
///
/// The interval is returned as well because the ticks have to be spaced
/// by the very interval the bounds were snapped to: recomputing it from
/// the snapped bounds would give a different number, and the last tick
/// would no longer land on the last bound.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `(f64, f64, f64)` - A `(f64, f64, f64)` value.
pub(crate) fn nice_scale(min: f64, max: f64, split: usize) -> (f64, f64, f64) {
    if !min.is_finite() || !max.is_finite() {
        return (0.0, 1.0, 1.0);
    }
    if min == max {
        let padding: f64 = if min.abs() < f64::EPSILON {
            1.0
        } else {
            min.abs() * 0.1
        };
        return (min - padding, max + padding, padding * 2.0);
    }
    let (min, max) = if min < max { (min, max) } else { (max, min) };
    let split: usize = split.max(2);
    let range: f64 = nice_number(max - min, false);
    let interval: f64 = nice_number(range / (split - 1) as f64, true);
    if interval <= 0.0 {
        return (min, max, 1.0);
    }
    let nice_min: f64 = (min / interval).floor() * interval;
    let nice_max: f64 = (max / interval).ceil() * interval;
    (
        round_to(nice_min, interval),
        round_to(nice_max, interval),
        interval,
    )
}

/// Rounds a value to a 1/2/5 multiple of a power of ten.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `f64` - A 64-bit float.
pub(crate) fn nice_number(value: f64, round: bool) -> f64 {
    if value <= 0.0 {
        return 0.0;
    }
    let exponent: f64 = value.log10().floor();
    let power: f64 = 10f64.powf(exponent);
    let fraction: f64 = value / power;
    let nice: f64 = if round {
        if fraction < 1.5 {
            1.0
        } else if fraction < 3.0 {
            2.0
        } else if fraction < 7.0 {
            5.0
        } else {
            10.0
        }
    } else if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice * power
}

/// Rounds a value to a multiple of the interval, killing the floating
/// point noise that `floor` and `ceil` leave behind.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `f64` - A 64-bit float.
fn round_to(value: f64, interval: f64) -> f64 {
    if interval <= 0.0 {
        return value;
    }
    let decimals: i32 = -(interval.log10().floor()).max(0.0) as i32 + 2;
    let factor: f64 = 10f64.powi(decimals);
    (value * factor).round() / factor
}

/// Builds the ticks of a value axis.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
/// - `f64` - A 64-bit float (`f64`).
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `Vec<AxisTick>` - A `Vec<AxisTick>` value.
pub(crate) fn value_ticks(domain_min: f64, domain_max: f64, split: usize) -> Vec<AxisTick> {
    let (min, max, interval) = nice_scale(domain_min, domain_max, split);
    ticks_at_interval(min, max, interval)
}

/// Puts the ticks of an axis at their place in the chart.
///
/// A tick knows its value; the scale knows where that value is on the
/// screen. Every axis has to ask the scale, and a tick that is not
/// placed would be drawn at the origin — which is how a value axis
/// ends up with all of its labels piled up on the edge of the chart.
///
/// # Arguments
///
/// - `&Scale` - Shared reference to a `Scale`.
/// - `Vec<AxisTick>` - A `Vec<AxisTick>` parameter.
///
/// # Returns
///
/// - `Vec<AxisTick>` - A `Vec<AxisTick>` value.
pub(crate) fn place_ticks(scale: &Scale, ticks: Vec<AxisTick>) -> Vec<AxisTick> {
    ticks
        .into_iter()
        .map(|mut tick: AxisTick| {
            tick.position = scale.map(tick.value);
            tick
        })
        .collect()
}

/// Builds the ticks of a value axis at a fixed interval.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
/// - `f64` - A 64-bit float (`f64`).
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `Vec<AxisTick>` - A `Vec<AxisTick>` value.
pub(crate) fn ticks_at_interval(min: f64, max: f64, interval: f64) -> Vec<AxisTick> {
    if interval <= 0.0 {
        return vec![AxisTick {
            value: min,
            label: format_tick(min, interval),
            position: 0.0,
        }];
    }
    let mut ticks: Vec<AxisTick> = Vec::new();
    let mut value: f64 = min;
    let mut guard: i32 = 0;
    while value <= max + interval * 0.5 && guard < 1000 {
        ticks.push(AxisTick {
            value,
            label: format_tick(value, interval),
            position: 0.0,
        });
        value += interval;
        guard += 1;
    }
    ticks
}

/// Builds the ticks of a logarithmic axis.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
/// - `f64` - A 64-bit float (`f64`).
/// - `f64` - A 64-bit float (`f64`).
/// - `f64` - A 64-bit float (`f64`).
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `Vec<AxisTick>` - A `Vec<AxisTick>` value.
pub(crate) fn log_ticks(
    domain_min: f64,
    domain_max: f64,
    base: f64,
    range_min: f64,
    range_max: f64,
) -> Vec<AxisTick> {
    let mut ticks: Vec<AxisTick> = Vec::new();
    let mut exponent: i32 = domain_min.max(f64::MIN_POSITIVE).log(base).floor() as i32;
    let last: i32 = domain_max.max(f64::MIN_POSITIVE).log(base).ceil() as i32;
    let scale: Scale = Scale::log(
        domain_min.max(f64::MIN_POSITIVE),
        domain_max.max(f64::MIN_POSITIVE),
        range_min,
        range_max,
        base,
    );
    while exponent <= last {
        let value: f64 = base.powi(exponent);
        ticks.push(AxisTick {
            value,
            label: format_number(value),
            position: scale.map(value),
        });
        exponent += 1;
    }
    ticks
}

/// Builds the ticks of a category axis.
///
/// An axis never builds a tick for every category: a million ticks
/// would cost a million items in the display list, and the labels of
/// all but a handful of them would be too close together to be read
/// anyway. The ticks are sampled, and the label interval tells the
/// renderer how many of them it may skip.
///
/// # Arguments
///
/// - `&Scale` - Shared reference to a `Scale`.
///
/// # Returns
///
/// - `Vec<AxisTick>` - A `Vec<AxisTick>` value.
pub(crate) fn category_ticks(scale: &Scale) -> Vec<AxisTick> {
    let count: usize = if scale.categories.is_empty() {
        scale.count
    } else {
        scale.categories.len()
    };
    if count == 0 {
        return Vec::new();
    }
    let sampled: usize = count.min(MAX_CATEGORY_TICKS);
    let step: usize = ((count as f64 / sampled as f64).ceil() as usize).max(1);
    let mut result: Vec<AxisTick> = Vec::with_capacity(sampled);
    let mut index: usize = 0usize;
    while index < count && result.len() < MAX_CATEGORY_TICKS {
        let label: String = match scale.categories.get(index) {
            Some(name) => name.clone(),
            None => index.to_string(),
        };
        result.push(AxisTick {
            value: index as f64,
            label,
            position: scale.map(index as f64),
        });
        index += step;
    }
    result
}

/// Returns the position of a series within its stack, and the number of
/// series in the stack.
///
/// # Arguments
///
/// - `&ChartOption` - Shared reference to a `ChartOption`.
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `(usize, usize)` - A `(usize, usize)` value.
pub(crate) fn stack_position(option: &ChartOption, index: usize) -> (usize, usize) {
    let Some(stack) = option.series[index].stack.clone() else {
        return (0, 1);
    };
    let mut stack_index: usize = 0;
    let mut stack_size: usize = 0;
    for (position, series) in option.series.iter().enumerate() {
        if series.stack.as_deref() == Some(stack.as_str()) {
            if position == index {
                stack_index = stack_size;
            }
            stack_size += 1;
        }
    }
    (stack_index, stack_size.max(1))
}
