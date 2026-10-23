use super::*;

/// Returns the baseline the bars of a series grow from.
///
/// The baseline is the position of a zero value on the value axis, which
/// the caller has already folded into the points' pixel positions; the
/// baseline is therefore the extreme of the points themselves.
///
/// # Arguments
///
/// - `&[SeriesPoint]` - Shared reference to a `[SeriesPoint]`.
///
/// # Returns
///
/// - `f64` - A 64-bit float.
pub(crate) fn baseline_of(points: &[SeriesPoint]) -> f64 {
    if points.is_empty() {
        return 0.0;
    }
    let mut baseline: f64 = points[0].y;
    for point in points {
        if point.value >= 0.0 {
            baseline = baseline.max(point.y);
        }
    }
    // A series whose values are all negative grows downwards, so its
    // baseline is the lowest point rather than the highest.
    if points.iter().all(|point: &SeriesPoint| point.value < 0.0) {
        return points
            .iter()
            .map(|point: &SeriesPoint| point.y)
            .fold(f64::INFINITY, f64::min);
    }
    baseline
}

/// Returns the centre and the radius of a pie.
///
/// The centre defaults to the middle of the plot area and the radius to
/// three quarters of the smaller side, which is what ECharts uses.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
///
/// # Returns
///
/// - `(f64, f64, f64)` - A `(f64, f64, f64)` value.
pub(crate) fn pie_geometry(series: &SeriesOption, layout: &SeriesLayout) -> (f64, f64, f64) {
    let plot: &BBox = &layout.plot_rect;
    let pie: Option<&PieOptions> = series.pie.as_ref();
    let center_x: f64 = pie
        .and_then(|pie: &PieOptions| pie.center.as_ref())
        .and_then(|center: &Vec<String>| center.first())
        .and_then(|value: &String| parse_percent(value))
        .map(|value: f64| plot.min_x + value * plot.width())
        .unwrap_or(plot.center_x());
    let center_y: f64 = pie
        .and_then(|pie: &PieOptions| pie.center.as_ref())
        .and_then(|center: &Vec<String>| center.get(1))
        .and_then(|value: &String| parse_percent(value))
        .map(|value: f64| plot.min_y + value * plot.height())
        .unwrap_or(plot.center_y());
    let maximum: f64 = plot.width().min(plot.height()) * 0.5;
    let radius: f64 = pie
        .and_then(|pie: &PieOptions| pie.radius.as_deref())
        .and_then(parse_percent)
        .map(|value: f64| value * maximum)
        .unwrap_or(DEFAULT_PIE_RADIUS * maximum);
    (center_x, center_y, radius)
}

/// Builds the path of an annular sector.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `Path` - A `Path` value.
pub(crate) fn sector_path(
    center_x: f64,
    center_y: f64,
    radius: f64,
    inner_radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> Path {
    let mut builder: PathBuilder = Path::builder();
    if inner_radius <= 0.0 {
        builder = builder.move_to(center_x as f32, center_y as f32);
        builder = builder.arc(
            center_x as f32,
            center_y as f32,
            radius as f32,
            radius as f32,
            start_angle,
            end_angle,
        );
        return builder.close().build();
    }
    builder = builder.arc(
        center_x as f32,
        center_y as f32,
        radius as f32,
        radius as f32,
        start_angle,
        end_angle,
    );
    let (sin, cos) = end_angle.sin_cos();
    builder = builder.line_to(
        (center_x + inner_radius * cos) as f32,
        (center_y + inner_radius * sin) as f32,
    );
    builder = builder.arc(
        center_x as f32,
        center_y as f32,
        inner_radius as f32,
        inner_radius as f32,
        end_angle,
        start_angle,
    );
    builder.close().build()
}

/// Returns the average distance between two neighbouring points.
///
/// The distance decides whether a symbol can be drawn at every point:
/// when the points are closer together than the symbol is wide, the
/// symbols overlap into a smear, so the chart drops them. This is the
/// same rule ECharts applies, and it is what keeps a chart with a
/// million points from turning into a million draw commands.
///
/// # Arguments
///
/// - `&[SeriesPoint]` - Shared reference to a `[SeriesPoint]`.
///
/// # Returns
///
/// - `f64` - A 64-bit float.
pub(crate) fn average_spacing(points: &[SeriesPoint]) -> f64 {
    if points.len() < 2 {
        return f64::INFINITY;
    }
    let mut total: f64 = 0.0;
    for pair in points.windows(2) {
        let dx: f64 = pair[1].x - pair[0].x;
        let dy: f64 = pair[1].y - pair[0].y;
        total += (dx * dx + dy * dy).sqrt();
    }
    total / (points.len() - 1) as f64
}

/// The colour a series takes when the option does not name one.
///
/// The palette is shared by every series of the chart, so the first
/// series of one chart and the first series of another chart get the same
/// colour — which is what makes two charts look like they belong
/// together.
///
/// # Arguments
///
/// - `&ChartOption` - Shared reference to a `ChartOption`.
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `Color` - A `Color` value.
pub(crate) fn palette_color(option: &ChartOption, index: usize) -> Color {
    if option.color.is_empty() {
        return Color::parse(DEFAULT_PALETTE[index % DEFAULT_PALETTE.len()])
            .unwrap_or(Color::rgb(84, 112, 198));
    }
    let index: usize = index % option.color.len();
    Color::parse(&option.color[index]).unwrap_or(Color::rgb(84, 112, 198))
}

/// Returns the fill of a series, honouring the option's `itemStyle`.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `Color` - A `Color` parameter.
///
/// # Returns
///
/// - `Fill` - A `Fill` value.
pub(crate) fn series_fill(option: &SeriesOption, color: Color) -> Fill {
    if let Some(item_style) = &option.item_style {
        if let Some(value) = &item_style.color
            && let Some(parsed) = Color::parse(value)
        {
            return Fill::Solid(parsed);
        }
        if let Some((from, to)) = &item_style.gradient {
            let start: Color = Color::parse(from).unwrap_or(color);
            let end: Color = Color::parse(to).unwrap_or(color);
            return Fill::Linear(
                LinearGradient::new((0.0, 0.0), (0.0, 1.0))
                    .with_stop(0.0, start)
                    .with_stop(1.0, end),
            );
        }
    }
    Fill::Solid(color)
}

/// Returns the stroke of a series, honouring the option's `lineStyle`.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `Color` - A `Color` parameter.
///
/// # Returns
///
/// - `Stroke` - A `Stroke` value.
pub(crate) fn series_stroke(option: &SeriesOption, color: Color) -> Stroke {
    if let Some(line_style) = &option.line_style
        && let Some(value) = &line_style.color
        && let Some(parsed) = Color::parse(value)
    {
        return Stroke::Solid(parsed);
    }
    Stroke::Solid(color)
}

/// Returns the line style of a series, with the option's overrides
/// applied.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `Color` - A `Color` parameter.
///
/// # Returns
///
/// - `LineStyle` - A `LineStyle` value.
pub(crate) fn series_line_style(option: &SeriesOption, color: Color) -> LineStyle {
    let mut style: LineStyle = LineStyle::new();
    style.width = DEFAULT_SERIES_LINE_WIDTH;
    style.color = color;
    if let Some(line_style) = &option.line_style {
        if let Some(value) = line_style.width {
            style.width = value;
        }
        if let Some(value) = line_style.opacity {
            style.opacity = value;
        }
        if let Some(value) = line_style.line_type {
            style.dash = LineStyle::dash_for(
                match value {
                    LineType::Solid => LineDashType::Solid,
                    LineType::Dashed => LineDashType::Dashed,
                    LineType::Dotted => LineDashType::Dotted,
                },
                style.width,
            );
        }
        if let Some(value) = line_style.cap {
            style.cap = value;
        }
        if let Some(value) = line_style.join {
            style.join = value;
        }
    }
    style
}

/// Returns the text style of a series' labels.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `Color` - A `Color` parameter.
///
/// # Returns
///
/// - `TextStyle` - A `TextStyle` value.
pub(crate) fn series_label_style(option: &SeriesOption, color: Color) -> TextStyle {
    let mut style: TextStyle = TextStyle::new();
    style.size = DEFAULT_LABEL_FONT_SIZE;
    style.color = color;
    if let Some(label) = &option.label {
        if let Some(value) = label.font_size {
            style.size = value;
        }
        if let Some(value) = &label.font_family {
            style.family = value.clone();
        }
        if let Some(value) = &label.font_weight {
            style.weight = value.clone();
        }
        if let Some(value) = &label.font_style {
            style.style = value.clone();
        }
        if let Some(value) = label
            .color
            .as_ref()
            .and_then(|color: &String| Color::parse(color))
        {
            style.color = value;
        }
        if let Some(value) = label.align {
            style.align = value;
        }
        if let Some(value) = label.vertical_align {
            style.vertical_align = value;
        }
        if let Some(value) = label
            .background_color
            .as_ref()
            .and_then(|color: &String| Color::parse(color))
        {
            style.background = Some(value);
        }
    }
    style
}

/// Builds the path of a data point's symbol.
///
/// The symbol is centred on the point, which is what the axis labels and
/// the tooltips assume.
///
/// # Arguments
///
/// - `SymbolType` - A `SymbolType` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `Path` - A `Path` value.
pub(crate) fn symbol_path(kind: SymbolType, x: f64, y: f64, size: f64) -> Path {
    let radius: f64 = size * 0.5;
    match kind {
        SymbolType::None => Path::new(),
        SymbolType::Circle => Path::builder()
            .move_to(x as f32, y as f32)
            .ellipse(x as f32, y as f32, radius as f32, radius as f32)
            .build(),
        SymbolType::Rect => Path::builder()
            .rect(
                (x - radius) as f32,
                (y - radius) as f32,
                radius as f32 * 2.0,
                radius as f32 * 2.0,
            )
            .build(),
        SymbolType::RoundRect => Path::builder()
            .round_rect(
                (x - radius) as f32,
                (y - radius) as f32,
                radius as f32 * 2.0,
                radius as f32 * 2.0,
                radius as f32 * 0.5,
            )
            .build(),
        SymbolType::Diamond => Path::builder()
            .move_to(x as f32, (y - radius) as f32)
            .line_to((x + radius) as f32, y as f32)
            .line_to(x as f32, (y + radius) as f32)
            .line_to((x - radius) as f32, y as f32)
            .close()
            .build(),
        SymbolType::Triangle => Path::builder()
            .move_to(x as f32, (y - radius) as f32)
            .line_to((x + radius) as f32, (y + radius) as f32)
            .line_to((x - radius) as f32, (y + radius) as f32)
            .close()
            .build(),
        SymbolType::Pin => Path::builder()
            .move_to(x as f32, (y + radius) as f32)
            .line_to((x - radius * 0.7) as f32, y as f32)
            .arc(
                x as f32,
                y as f32,
                radius as f32 * 0.7,
                radius as f32 * 0.7,
                core::f64::consts::PI,
                core::f64::consts::TAU,
            )
            .close()
            .build(),
        SymbolType::Arrow => Path::builder()
            .move_to((x - radius) as f32, (y - radius) as f32)
            .line_to((x + radius) as f32, y as f32)
            .line_to((x - radius) as f32, (y + radius) as f32)
            .line_to((x - radius * 0.4) as f32, y as f32)
            .close()
            .build(),
    }
}

/// Builds the path of a polyline through the given points.
///
/// # Arguments
///
/// - `&[Point]` - Shared reference to a `[Point]`.
///
/// # Returns
///
/// - `Path` - A `Path` value.
pub(crate) fn polyline_path(points: &[Point]) -> Path {
    let mut builder: PathBuilder = Path::builder();
    if let Some(first) = points.first() {
        builder = builder.move_to(first.x, first.y);
        for point in &points[1..] {
            builder = builder.line_to(point.x, point.y);
        }
    }
    builder.build()
}

/// Builds the path of a smooth polyline through the given points.
///
/// The curve is a Catmull-Rom spline converted to cubic Béziers, which
/// is the same shape ECharts' `smooth: true` produces.
///
/// # Arguments
///
/// - `&[Point]` - Shared reference to a `[Point]`.
///
/// # Returns
///
/// - `Path` - A `Path` value.
pub(crate) fn smooth_path(points: &[Point]) -> Path {
    let mut builder: PathBuilder = Path::builder();
    let count: usize = points.len();
    if count < 3 {
        return polyline_path(points);
    }
    builder = builder.move_to(points[0].x, points[0].y);
    for index in 0..count - 1 {
        let previous: Point = points[index];
        let current: Point = points[index + 1];
        let next: Point = if index + 2 < count {
            points[index + 2]
        } else {
            current
        };
        let control1_x: f32 = previous.x + (current.x - previous.x) / 6.0;
        let control1_y: f32 = previous.y + (current.y - previous.y) / 6.0;
        let control2_x: f32 = current.x - (next.x - previous.x) / 6.0;
        let control2_y: f32 = current.y - (next.y - previous.y) / 6.0;
        builder = builder.cubic_to(
            control1_x, control1_y, control2_x, control2_y, current.x, current.y,
        );
    }
    builder.build()
}

/// Builds the path of a stepped polyline through the given points.
///
/// `step` selects where the step falls.
///
/// # Arguments
///
/// - `&[Point]` - Shared reference to a `[Point]`.
/// - `StepPosition` - A `StepPosition` parameter.
///
/// # Returns
///
/// - `Path` - A `Path` value.
pub(crate) fn step_path(points: &[Point], step: StepPosition) -> Path {
    let mut builder: PathBuilder = Path::builder();
    let count: usize = points.len();
    if count == 0 {
        return builder.build();
    }
    builder = builder.move_to(points[0].x, points[0].y);
    for index in 0..count - 1 {
        let current: Point = points[index];
        let next: Point = points[index + 1];
        let middle: f32 = (current.x + next.x) * 0.5;
        match step {
            StepPosition::Start => {
                builder = builder.line_to(next.x, current.y);
            }
            StepPosition::Middle => {
                builder = builder.line_to(middle, current.y);
                builder = builder.line_to(middle, next.y);
            }
            StepPosition::End => {
                builder = builder.line_to(current.x, next.y);
            }
        }
        builder = builder.line_to(next.x, next.y);
    }
    builder.build()
}

/// Draws a custom series.
///
/// A custom series draws a shape of its own for every data item. The
/// shape is named by the option: a polygon or a polyline over the
/// points of the item, a circle, or a rectangle.
///
/// The points of a polygon are in the *data* space of the series, the
/// way a developer writes them down; the radius of a circle and the
/// size of a rectangle are in pixels, because they are sizes and not
/// positions.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `&ChartLayout` - Shared reference to a `ChartLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_custom(
    list: &mut DisplayList,
    series: &SeriesOption,
    series_layout: &SeriesLayout,
    layout: &ChartLayout,
    color: Color,
    z: i32,
) {
    let options: CustomOptions = series.custom.clone().unwrap_or_default();
    let shape: String = options.renderer.clone().unwrap_or_default();
    let fill: Fill = series_fill(series, color);
    let stroke: Stroke = series_stroke(series, color);
    let style: LineStyle = series_line_style(series, color);
    let axes: Option<(&AxisLayout, &AxisLayout)> = axes_of(layout, series, series_layout);

    for (index, point) in series_layout.points.iter().enumerate() {
        let Some(item) = series.data.get(index) else {
            continue;
        };
        let geometry = item.get("shape");
        let path: Option<Path> = match shape.as_str() {
            "circle" => circle_path(point, geometry),
            CUSTOM_SHAPE_RECT => rect_path(point, geometry),
            "polyline" => custom_polyline_path(point, geometry, axes, false),
            _ => custom_polyline_path(point, geometry, axes, true),
        };
        let Some(path) = path else {
            continue;
        };
        list.fill(path.clone(), fill.clone(), 1.0, z);
        if style.width > 0.0 && !matches!(stroke, Stroke::None) {
            list.stroke(path, stroke.clone(), style.clone(), 1.0, z + 1);
        }
    }
}

/// Returns the axes a custom series is drawn on.
///
/// # Arguments
///
/// - `&'a ChartLayout` - Shared reference to a `'a ChartLayout`.
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
///
/// # Returns
///
/// - `Option<(&'a AxisLayout, &'a AxisLayout)>` - `Some(...)` on success, `None` otherwise.
fn axes_of<'a>(
    layout: &'a ChartLayout,
    series: &SeriesOption,
    series_layout: &SeriesLayout,
) -> Option<(&'a AxisLayout, &'a AxisLayout)> {
    let grid: &GridLayout = layout.grids.get(series_layout.grid_index)?;
    let x_axis: &AxisLayout = grid.x_axis(series.x_axis_index.unwrap_or(0))?;
    let y_axis: &AxisLayout = grid.y_axis(series.y_axis_index.unwrap_or(0))?;
    Some((x_axis, y_axis))
}

/// Builds the path of a circle.
///
/// # Arguments
///
/// - `&SeriesPoint` - Shared reference to a `SeriesPoint`.
/// - `Option<&DataValue>` - A `Option<&DataValue>` parameter.
///
/// # Returns
///
/// - `Option<Path>` - `Some(...)` on success, `None` otherwise.
fn circle_path(point: &SeriesPoint, geometry: Option<&DataValue>) -> Option<Path> {
    let radius: f64 = geometry
        .and_then(|shape: &DataValue| shape.get("r"))
        .and_then(|value: &DataValue| value.to_number())
        .unwrap_or(DEFAULT_CUSTOM_RADIUS)
        .max(0.5);
    Some(
        Path::builder()
            .ellipse(point.x as f32, point.y as f32, radius as f32, radius as f32)
            .build(),
    )
}

/// Builds the path of a rectangle.
///
/// # Arguments
///
/// - `&SeriesPoint` - Shared reference to a `SeriesPoint`.
/// - `Option<&DataValue>` - A `Option<&DataValue>` parameter.
///
/// # Returns
///
/// - `Option<Path>` - `Some(...)` on success, `None` otherwise.
fn rect_path(point: &SeriesPoint, geometry: Option<&DataValue>) -> Option<Path> {
    let width: f64 = geometry
        .and_then(|shape: &DataValue| shape.get("width"))
        .and_then(|value: &DataValue| value.to_number())
        .unwrap_or(DEFAULT_CUSTOM_RADIUS * 2.0)
        .max(0.5);
    let height: f64 = geometry
        .and_then(|shape: &DataValue| shape.get("height"))
        .and_then(|value: &DataValue| value.to_number())
        .unwrap_or(DEFAULT_CUSTOM_RADIUS * 2.0)
        .max(0.5);
    Some(
        Path::builder()
            .rect(
                (point.x - width * 0.5) as f32,
                (point.y - height * 0.5) as f32,
                width as f32,
                height as f32,
            )
            .build(),
    )
}

/// Builds the path of a polygon or of a polyline.
///
/// The points are in the data space of the series, so they are mapped
/// through the axes of the series before they become a path.
///
/// # Arguments
///
/// - `&SeriesPoint` - Shared reference to a `SeriesPoint`.
/// - `Option<&DataValue>` - A `Option<&DataValue>` parameter.
/// - `Option<(&AxisLayout, &AxisLayout)>` - A `Option<(&AxisLayout, &AxisLayout)>` parameter.
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `Option<Path>` - `Some(...)` on success, `None` otherwise.
fn custom_polyline_path(
    point: &SeriesPoint,
    geometry: Option<&DataValue>,
    axes: Option<(&AxisLayout, &AxisLayout)>,
    close: bool,
) -> Option<Path> {
    let points: &[DataValue] = geometry
        .and_then(|shape: &DataValue| shape.get("points"))
        .and_then(|value: &DataValue| value.as_list())?;
    if points.len() < 2 {
        return None;
    }
    let (x_axis, y_axis) = axes?;
    let mut builder: PathBuilder = Path::builder();
    for (index, item) in points.iter().enumerate() {
        let pair: &[DataValue] = item.as_list()?;
        let x: Option<f64> = pair.first().and_then(|value: &DataValue| value.to_number());
        let y: Option<f64> = pair.get(1).and_then(|value: &DataValue| value.to_number());
        let (Some(x), Some(y)) = (x, y) else {
            return None;
        };
        let px: f32 = x_axis.scale.map(x) as f32;
        let py: f32 = y_axis.scale.map(y) as f32;
        if index == 0 {
            builder = builder.move_to(px, py);
        } else {
            builder = builder.line_to(px, py);
        }
    }
    if close {
        builder = builder.close();
    }
    let _: &SeriesPoint = point;
    Some(builder.build())
}

/// Draws a parallel-coordinates series.
///
/// Every item of the series is a polyline that crosses one axis per
/// dimension; the axes are spread evenly across the plot area.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_parallel(
    list: &mut DisplayList,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    let plot: &BBox = &layout.plot_rect;
    let dimensions: Option<usize> = series.data.first().map(|value: &DataValue| match value {
        DataValue::List(items) => items.len(),
        _ => 0,
    });
    let Some(dimensions) = dimensions.filter(|count: _| *count > 0) else {
        return;
    };
    let axis_width: f64 = plot.width() / dimensions as f64;
    let maximum: f64 = series
        .data
        .iter()
        .map(|value: &DataValue| match value {
            DataValue::List(items) => items
                .iter()
                .filter_map(|item: &DataValue| item.to_number())
                .fold(0.0_f64, f64::max),
            _ => 0.0,
        })
        .fold(0.0_f64, f64::max)
        .max(f64::EPSILON);

    let stroke: Stroke = series_stroke(series, color);
    let style: LineStyle = series_line_style(series, color);
    for value in &series.data {
        let DataValue::List(items) = value else {
            continue;
        };
        let mut builder: PathBuilder = Path::builder();
        for (index, item) in items.iter().enumerate() {
            let number: f64 = item.to_number().unwrap_or(0.0);
            let x: f64 = plot.min_x + index as f64 * axis_width + axis_width * 0.5;
            let y: f64 = plot.max_y - (number / maximum).clamp(0.0, 1.0) * plot.height();
            if index == 0 {
                builder = builder.move_to(x as f32, y as f32);
            } else {
                builder = builder.line_to(x as f32, y as f32);
            }
        }
        list.stroke(builder.build(), stroke.clone(), style.clone(), 1.0, z);
    }

    // The axes themselves.
    for index in 0..dimensions {
        let x: f64 = plot.min_x + index as f64 * axis_width + axis_width * 0.5;
        let path: Path = Path::builder()
            .move_to(x as f32, plot.min_y as f32)
            .line_to(x as f32, plot.max_y as f32)
            .build();
        list.stroke(
            path,
            Stroke::Solid(
                Color::parse(DEFAULT_SPLIT_LINE_COLOR).unwrap_or(Color::rgb(217, 217, 217)),
            ),
            LineStyle {
                width: 1.0,
                ..LineStyle::new()
            },
            1.0,
            z - 1,
        );
    }
}

/// Draws a lines series: a polyline between coordinate pairs.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_lines(
    list: &mut DisplayList,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    let plot: &BBox = &layout.plot_rect;
    let coordinates: Vec<Vec<(f64, f64)>> = series
        .data
        .iter()
        .map(|value: &DataValue| match value {
            DataValue::List(items) => items
                .iter()
                .filter_map(|item: &DataValue| match item {
                    DataValue::List(pair) if pair.len() >= 2 => Some((
                        pair[0].to_number().unwrap_or(0.0),
                        pair[1].to_number().unwrap_or(0.0),
                    )),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        })
        .filter(|points: _| points.len() >= 2)
        .collect();
    if coordinates.is_empty() {
        return;
    }

    let extent: (f64, f64) = coordinates
        .iter()
        .flatten()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), (x, _)| {
            (min.min(*x), max.max(*x))
        });
    let span: f64 = (extent.1 - extent.0).max(f64::EPSILON);
    let stroke: Stroke = series_stroke(series, color);
    let style: LineStyle = series_line_style(series, color);
    for points in &coordinates {
        let mut builder: PathBuilder = Path::builder();
        for (index, (x, y)) in points.iter().enumerate() {
            let px: f64 = plot.min_x + (x - extent.0) / span * plot.width();
            let py: f64 = plot.max_y - (y / plot.height().max(1.0)).clamp(0.0, 1.0) * plot.height();
            if index == 0 {
                builder = builder.move_to(px as f32, py as f32);
            } else {
                builder = builder.line_to(px as f32, py as f32);
            }
        }
        list.stroke(builder.build(), stroke.clone(), style.clone(), 1.0, z);
    }
}

/// Draws a theme-river series: one band per category, stacked over
/// time.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_theme_river(
    list: &mut DisplayList,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    if layout.points.is_empty() {
        return;
    }
    let fill: Fill = series_fill(series, color);
    let baseline: f64 = baseline_of(&layout.points);
    let points: Vec<Point> = layout
        .points
        .iter()
        .map(|point: &SeriesPoint| Point::new(point.x as f32, point.y as f32))
        .collect();
    list.strip(points, baseline as f32, fill, 1.0, z);
}

/// Draws a pictorial-bar series: a bar with a symbol on top of it.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_pictorial_bar(
    list: &mut DisplayList,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    if layout.points.is_empty() {
        return;
    }
    let fill: Fill = series_fill(series, color);
    let baseline: f64 = baseline_of(&layout.points);
    let symbol: SymbolType = series
        .pictorial_bar
        .as_ref()
        .and_then(|bar: &PictorialBarOptions| bar.symbol.as_deref())
        .and_then(|symbol: &str| match symbol {
            SYMBOL_RECT => Some(SymbolType::Rect),
            "roundRect" => Some(SymbolType::RoundRect),
            "triangle" => Some(SymbolType::Triangle),
            "diamond" => Some(SymbolType::Diamond),
            "pin" => Some(SymbolType::Pin),
            "arrow" => Some(SymbolType::Arrow),
            "none" => Some(SymbolType::None),
            _ => None,
        })
        .unwrap_or(SymbolType::Rect);
    let size: f64 = series
        .pictorial_bar
        .as_ref()
        .and_then(|bar: &PictorialBarOptions| bar.symbol_size.as_ref())
        .and_then(|sizes: &Vec<String>| sizes.first())
        .and_then(|value: &String| parse_percent(value))
        .map(|value: f64| value * 100.0)
        .unwrap_or(20.0);

    for point in &layout.points {
        let height: f64 = (baseline - point.y).abs();
        let count: usize = ((height / size).ceil() as usize).max(1);
        for index in 0..count {
            let y: f64 = point.y + index as f64 * size;
            let path: Path = symbol_path(symbol, point.x, y, size);
            list.fill(path, fill.clone(), 1.0, z);
        }
    }
}

/// Draws a funnel series: one trapezoid per stage.
///
/// The stages are stacked from the top down, and the width of a stage is
/// proportional to its value, which is what makes the shape read as a
/// funnel.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_funnel(
    list: &mut DisplayList,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    let values: Vec<f64> = series_values(series);
    if values.is_empty() {
        return;
    }
    let plot: &BBox = &layout.plot_rect;
    let funnel: Option<&FunnelOptions> = series.funnel.as_ref();

    let center_x: f64 = funnel
        .and_then(|funnel: &FunnelOptions| funnel.center.as_ref())
        .and_then(|center: &Vec<String>| center.first())
        .and_then(|value: &String| parse_percent(value))
        .map(|value: f64| plot.min_x + value * plot.width())
        .unwrap_or(plot.center_x());
    let total_height: f64 = funnel
        .and_then(|funnel: &FunnelOptions| funnel.height.as_deref())
        .and_then(parse_percent)
        .map(|value: f64| value * plot.height())
        .unwrap_or(plot.height() * 0.8);
    let total_width: f64 = funnel
        .and_then(|funnel: &FunnelOptions| funnel.width.as_deref())
        .and_then(parse_percent)
        .map(|value: f64| value * plot.width())
        .unwrap_or(plot.width() * 0.8);
    let gap: f64 = funnel
        .and_then(|funnel: &FunnelOptions| funnel.gap)
        .unwrap_or(0.2);

    let maximum: f64 = values
        .iter()
        .map(|value: &f64| value.abs())
        .fold(0.0_f64, f64::max)
        .max(f64::EPSILON);
    let stage_height: f64 = (total_height / values.len() as f64) * (1.0 - gap);
    let top: f64 = plot.min_y + (plot.height() - total_height) * 0.5;

    let fill: Fill = series_fill(series, color);
    for (index, value) in values.iter().enumerate() {
        let ratio: f64 = (value.abs() / maximum).clamp(0.0, 1.0);
        let next_ratio: f64 = values
            .get(index + 1)
            .map(|next: &f64| (next.abs() / maximum).clamp(0.0, 1.0))
            .unwrap_or(ratio);
        let top_width: f64 = total_width * ratio;
        let bottom_width: f64 = total_width * next_ratio;
        let y: f64 = top + index as f64 * (total_height / values.len() as f64);
        let path: Path = Path::builder()
            .move_to((center_x - top_width * 0.5) as f32, y as f32)
            .line_to((center_x + top_width * 0.5) as f32, y as f32)
            .line_to(
                (center_x + bottom_width * 0.5) as f32,
                (y + stage_height) as f32,
            )
            .line_to(
                (center_x - bottom_width * 0.5) as f32,
                (y + stage_height) as f32,
            )
            .close()
            .build();
        list.fill(path, fill.clone(), 1.0, z + index as i32);
    }
}

/// Draws a gauge series: the scale, the progress arc, and the pointer.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_gauge(
    list: &mut DisplayList,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    let plot: &BBox = &layout.plot_rect;
    let gauge: Option<&GaugeOptions> = series.gauge.as_ref();
    let maximum: f64 = plot.width().min(plot.height()) * 0.5;
    let center_x: f64 = gauge
        .and_then(|gauge: &GaugeOptions| gauge.center.as_ref())
        .and_then(|center: &Vec<String>| center.first())
        .and_then(|value: &String| parse_percent(value))
        .map(|value: f64| plot.min_x + value * plot.width())
        .unwrap_or(plot.center_x());
    let center_y: f64 = gauge
        .and_then(|gauge: &GaugeOptions| gauge.center.as_ref())
        .and_then(|center: &Vec<String>| center.get(1))
        .and_then(|value: &String| parse_percent(value))
        .map(|value: f64| plot.min_y + value * plot.height())
        .unwrap_or(plot.center_y());
    let radius: f64 = gauge
        .and_then(|gauge: &GaugeOptions| gauge.radius.as_deref())
        .and_then(parse_percent)
        .map(|value: f64| value * maximum)
        .unwrap_or(0.75 * maximum);

    let start_angle: f64 = gauge
        .and_then(|gauge: &GaugeOptions| gauge.start_angle)
        .unwrap_or(225.0)
        .to_radians();
    let end_angle: f64 = gauge
        .and_then(|gauge: &GaugeOptions| gauge.end_angle)
        .unwrap_or(-45.0)
        .to_radians();
    let minimum: f64 = gauge
        .and_then(|gauge: &GaugeOptions| gauge.min)
        .unwrap_or(0.0);
    let maximum_value: f64 = gauge
        .and_then(|gauge: &GaugeOptions| gauge.max)
        .unwrap_or(100.0);
    let value: f64 = layout
        .points
        .first()
        .map(|point: &SeriesPoint| point.value)
        .unwrap_or(0.0);
    let extent: f64 = (maximum_value - minimum).max(f64::EPSILON);
    let ratio: f64 = ((value - minimum) / extent).clamp(0.0, 1.0);
    let sweep: f64 = end_angle - start_angle;
    let value_angle: f64 = start_angle + sweep * ratio;

    // The scale.
    let scale_path: Path = arc_path(
        center_x,
        center_y,
        radius,
        radius * 0.75,
        start_angle,
        end_angle,
    );
    list.stroke(
        scale_path,
        Stroke::Solid(Color::parse(DEFAULT_AXIS_LINE_COLOR).unwrap_or(Color::rgb(217, 217, 217))),
        LineStyle {
            width: 8.0,
            ..LineStyle::new()
        },
        1.0,
        z,
    );

    // The progress.
    let progress: Path = arc_path(
        center_x,
        center_y,
        radius,
        radius * 0.75,
        start_angle,
        value_angle,
    );
    list.stroke(
        progress,
        Stroke::Solid(color),
        LineStyle {
            width: 8.0,
            ..LineStyle::new()
        },
        1.0,
        z + 1,
    );

    // The pointer.
    let pointer_length: f64 = radius * 0.6;
    let pointer: Path = Path::builder()
        .move_to(
            (center_x + pointer_length * value_angle.cos()) as f32,
            (center_y + pointer_length * value_angle.sin()) as f32,
        )
        .line_to(center_x as f32, center_y as f32)
        .build();
    list.stroke(
        pointer,
        Stroke::Solid(color),
        LineStyle {
            width: 2.0,
            ..LineStyle::new()
        },
        1.0,
        z + 2,
    );

    // The hub.
    let hub: Path = Path::builder()
        .ellipse(center_x as f32, center_y as f32, 4.0, 4.0)
        .build();
    list.fill(hub, Fill::Solid(color), 1.0, z + 3);

    // The value.
    let mut style: TextStyle = TextStyle::new();
    style.size = 16.0;
    style.align = TextAlign::Center;
    style.vertical_align = TextVerticalAlign::Middle;
    list.text(
        format_number(value),
        center_x,
        center_y + radius * 0.3,
        style,
        z + 4,
    );
}

/// Builds the path of an arc between two radii.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `Path` - A `Path` value.
pub(crate) fn arc_path(
    center_x: f64,
    center_y: f64,
    outer_radius: f64,
    inner_radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> Path {
    let mut builder: PathBuilder = Path::builder();
    builder = builder.arc(
        center_x as f32,
        center_y as f32,
        outer_radius as f32,
        outer_radius as f32,
        start_angle,
        end_angle,
    );
    let (end_sin, end_cos) = end_angle.sin_cos();
    builder = builder.line_to(
        (center_x + inner_radius * end_cos) as f32,
        (center_y + inner_radius * end_sin) as f32,
    );
    builder = builder.arc(
        center_x as f32,
        center_y as f32,
        inner_radius as f32,
        inner_radius as f32,
        end_angle,
        start_angle,
    );
    builder.close().build()
}

/// Reads the nodes of a graph series.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
///
/// # Returns
///
/// - `Vec<GraphNode>` - A `Vec<GraphNode>` value.
pub(crate) fn graph_nodes(series: &SeriesOption) -> Vec<GraphNode> {
    series
        .data
        .iter()
        .map(|value: &DataValue| {
            let name: String = value
                .get(SERIES_DATA_NAME_KEY)
                .and_then(|item: &DataValue| item.as_text())
                .unwrap_or_default()
                .to_string();
            let number: f64 = value
                .get(SERIES_DATA_VALUE_KEY)
                .and_then(|item: &DataValue| item.to_number())
                .unwrap_or(0.0);
            GraphNode {
                name,
                value: number,
                x: value
                    .get("x")
                    .and_then(|item: &DataValue| item.to_number())
                    .unwrap_or(0.0),
                y: value
                    .get("y")
                    .and_then(|item: &DataValue| item.to_number())
                    .unwrap_or(0.0),
                size: 0.0,
            }
        })
        .collect()
}

/// Reads the links of a graph series.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
///
/// # Returns
///
/// - `Vec<GraphLink>` - A `Vec<GraphLink>` value.
pub(crate) fn graph_links(series: &SeriesOption) -> Vec<GraphLink> {
    let Some(graph) = &series.graph else {
        return Vec::new();
    };
    let Some(links) = &graph.links else {
        return Vec::new();
    };
    links
        .iter()
        .map(|value: &DataValue| GraphLink {
            source: value
                .get(SERIES_DATA_SOURCE_KEY)
                .and_then(|item: &DataValue| item.as_text())
                .unwrap_or_default()
                .to_string(),
            target: value
                .get(SERIES_DATA_TARGET_KEY)
                .and_then(|item: &DataValue| item.as_text())
                .unwrap_or_default()
                .to_string(),
            value: value
                .get(SERIES_DATA_VALUE_KEY)
                .and_then(|item: &DataValue| item.to_number())
                .unwrap_or(1.0),
        })
        .collect()
}

/// Places the nodes of a graph.
///
/// A node that carries a position keeps it; the rest are placed on a
/// circle around the middle of the plot area, which is the layout a
/// graph gets when the option does not ask for a force simulation.
///
/// # Arguments
///
/// - `&mut [GraphNode]` - Mutable reference to a `[GraphNode]` (mutated in place).
/// - `&BBox` - Shared reference to a `BBox`.
/// - `f64` - A 64-bit float (`f64`).
pub(crate) fn place_graph_nodes(nodes: &mut [GraphNode], plot: &BBox, radius: f64) {
    let fixed: bool = nodes
        .iter()
        .all(|node: &GraphNode| node.x != 0.0 || node.y != 0.0);
    if fixed {
        return;
    }
    let count: usize = nodes.len();
    if count == 0 {
        return;
    }
    let center_x: f64 = plot.center_x();
    let center_y: f64 = plot.center_y();
    for (index, node) in nodes.iter_mut().enumerate() {
        let angle: f64 = index as f64 / count as f64 * core::f64::consts::TAU;
        node.x = center_x + radius * angle.cos();
        node.y = center_y + radius * angle.sin();
        node.size = (8.0 + node.value.abs().sqrt() * 4.0).min(24.0);
    }
}

/// Draws a graph series: the links first, then the nodes.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_graph(
    list: &mut DisplayList,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    let mut nodes: Vec<GraphNode> = graph_nodes(series);
    if nodes.is_empty() {
        return;
    }
    let plot: &BBox = &layout.plot_rect;
    let radius: f64 = plot.width().min(plot.height()) * 0.35;
    place_graph_nodes(&mut nodes, plot, radius);

    let links: Vec<GraphLink> = graph_links(series);
    let stroke: Stroke = series_stroke(series, color);
    let style: LineStyle = series_line_style(series, color);
    for link in &links {
        let source: Option<&GraphNode> = nodes
            .iter()
            .find(|node: &&GraphNode| node.name == link.source);
        let target: Option<&GraphNode> = nodes
            .iter()
            .find(|node: &&GraphNode| node.name == link.target);
        let (Some(source), Some(target)) = (source, target) else {
            continue;
        };
        let path: Path = Path::builder()
            .move_to(source.x as f32, source.y as f32)
            .line_to(target.x as f32, target.y as f32)
            .build();
        // A weaker link is drawn fainter, so that the strong ones stand
        // out of a dense graph.
        let weight: f64 = if source.value > 0.0 {
            (link.value / source.value).clamp(0.1, 1.0)
        } else {
            1.0
        };
        list.stroke(path, stroke.clone(), style.clone(), weight, z);
    }

    let fill: Fill = series_fill(series, color);
    for node in &nodes {
        let path: Path = symbol_path(SymbolType::Circle, node.x, node.y, node.size);
        list.fill(path, fill.clone(), 1.0, z + 1);
    }
}

/// Draws a map series.
///
/// A map is a set of regions, and the series' data gives a value to
/// some of them. The regions are filled with the colour the visual map
/// gives their value, and the regions without a value keep the colour
/// of the series.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&ChartOption` - Shared reference to a `ChartOption`.
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `&ChartLayout` - Shared reference to a `ChartLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_map(
    list: &mut DisplayList,
    option: &ChartOption,
    series: &SeriesOption,
    series_layout: &SeriesLayout,
    layout: &ChartLayout,
    color: Color,
    z: i32,
) {
    let Some(name) = map_series_name(series, option) else {
        return;
    };
    let Some(geo) = layout.geo(&name) else {
        return;
    };

    // The values of the regions, by name. A data item names the region
    // it belongs to, the way a map series names it in ECharts.
    let mut values: HashMap<&str, f64> = HashMap::new();
    for point in &series_layout.points {
        values.insert(point.name.as_str(), point.value);
    }

    let visual: VisualMap = resolve_visual_map(option, series_layout.index, &values);
    let fallback: Fill = series_fill(series, color);
    let stroke: Stroke = series_stroke(series, color);
    let style: LineStyle = series_line_style(series, color);

    for region in &geo.regions {
        let paint: Fill = match values.get(region.name.as_str()) {
            Some(value) => Fill::Solid(visual.color_of(*value)),
            None => fallback.clone(),
        };
        list.fill(region.path.clone(), paint, 1.0, z);
        if style.width > 0.0 && !matches!(stroke, Stroke::None) {
            list.stroke(
                region.path.clone(),
                stroke.clone(),
                style.clone(),
                1.0,
                z + 1,
            );
        }
    }

    draw_region_labels(list, series, series_layout, geo, z + 2);
}

/// Draws the names of the regions.
///
/// A map is read by its names, so they are drawn unless the option
/// turns them off.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `&GeoLayout` - Shared reference to a `GeoLayout`.
/// - `i32` - A 32-bit signed integer (`i32`).
fn draw_region_labels(
    list: &mut DisplayList,
    series: &SeriesOption,
    series_layout: &SeriesLayout,
    geo: &GeoLayout,
    z: i32,
) {
    let options: MapOptions = series.map.clone().unwrap_or_default();
    // A map is read by its names, so they are drawn unless the option
    // turns them off.
    let (show, label) = match &options.label {
        Some(label) => (label.show, label.clone()),
        None => (true, LabelOption::default()),
    };
    if !show {
        return;
    }
    let mut style: TextStyle = TextStyle::new();
    style.align = TextAlign::Center;
    style.vertical_align = TextVerticalAlign::Middle;
    if let Some(size) = label.font_size {
        style.size = size;
    }
    if let Some(color) = label.color.as_deref().and_then(Color::parse) {
        style.color = color;
    }
    if let Some(family) = label.font_family.as_deref() {
        style.family = family.to_string();
    }
    for region in &geo.regions {
        if region.name.is_empty() {
            continue;
        }
        list.text(
            region.name.clone(),
            region.center.x as f64,
            region.center.y as f64,
            style.clone(),
            z,
        );
    }
    let _: &SeriesLayout = series_layout;
}

/// Resolves the visual map that colours the regions of a series.
///
/// The first visual map that applies to the series is the one that
/// colours it: a visual map with no series index applies to all of
/// them.
///
/// # Arguments
///
/// - `&ChartOption` - Shared reference to a `ChartOption`.
/// - `usize` - A non-negative integer (`usize`).
/// - `&HashMap<&str, f64>` - Shared reference to a `HashMap<&str, f64>`.
///
/// # Returns
///
/// - `VisualMap` - A `VisualMap` value.
fn resolve_visual_map(
    option: &ChartOption,
    series_index: usize,
    values: &HashMap<&str, f64>,
) -> VisualMap {
    let mut chosen: Option<&VisualMapOption> = None;
    for visual in &option.visual_map {
        match visual.series_index {
            Some(index) if index == series_index => {
                chosen = Some(visual);
                break;
            }
            Some(_) => continue,
            None => {
                if chosen.is_none() {
                    chosen = Some(visual);
                }
            }
        }
    }
    let Some(visual) = chosen else {
        return VisualMap::resolve(
            &VisualMapOption {
                show: true,
                ..Default::default()
            },
            &[],
        );
    };
    let values: Vec<f64> = values.values().copied().collect();
    VisualMap::resolve(visual, &values)
}

/// Returns the geometry of a radar coordinate system.
///
/// The radar takes its shape from the option's `radar` component; the
/// first one is used when the chart has several.
///
/// # Arguments
///
/// - `&ChartOption` - Shared reference to a `ChartOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
///
/// # Returns
///
/// - `RadarGeometry` - A `RadarGeometry` value.
pub(crate) fn radar_geometry(option: &ChartOption, layout: &SeriesLayout) -> RadarGeometry {
    let plot: &BBox = &layout.plot_rect;
    let radar: Option<&RadarOption> = option.radar.first();
    let maximum: f64 = plot.width().min(plot.height()) * 0.5;
    let center_x: f64 = radar
        .and_then(|radar: &RadarOption| radar.center.as_ref())
        .and_then(|center: &Vec<String>| center.first())
        .and_then(|value: &String| parse_percent(value))
        .map(|value: f64| plot.min_x + value * plot.width())
        .unwrap_or(plot.center_x());
    let center_y: f64 = radar
        .and_then(|radar: &RadarOption| radar.center.as_ref())
        .and_then(|center: &Vec<String>| center.get(1))
        .and_then(|value: &String| parse_percent(value))
        .map(|value: f64| plot.min_y + value * plot.height())
        .unwrap_or(plot.center_y());
    let radius: f64 = radar
        .and_then(|radar: &RadarOption| radar.radius.as_deref())
        .and_then(parse_percent)
        .map(|value: f64| value * maximum)
        .unwrap_or(0.75 * maximum);
    let sides: usize = radar
        .map(|radar: &RadarOption| radar.indicator.len())
        .unwrap_or(3)
        .max(3);
    let start_angle: f64 = radar
        .and_then(|radar: &RadarOption| radar.start_angle)
        .unwrap_or(90.0)
        .to_radians();
    RadarGeometry {
        center_x,
        center_y,
        radius,
        sides,
        start_angle,
    }
}

/// Draws a radar series: one polygon per series, over the radar's web.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&ChartOption` - Shared reference to a `ChartOption`.
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_radar(
    list: &mut DisplayList,
    option: &ChartOption,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    let geometry: RadarGeometry = radar_geometry(option, layout);
    let indicator_count: usize = option
        .radar
        .first()
        .map(|radar: &RadarOption| radar.indicator.len())
        .unwrap_or(layout.points.len())
        .max(1);
    let maximum: f64 = series
        .radar
        .as_ref()
        .and_then(|radar: &RadarOptions| radar.split_number)
        .map(|split: usize| split as f64)
        .unwrap_or(1.0)
        .max(1.0);

    // The web: the rings and the spokes.
    for ring in 1..=maximum as usize {
        let ratio: f64 = ring as f64 / maximum;
        let mut builder: PathBuilder = Path::builder();
        for index in 0..indicator_count {
            let (x, y) = geometry.point(index, ratio);
            if index == 0 {
                builder = builder.move_to(x as f32, y as f32);
            } else {
                builder = builder.line_to(x as f32, y as f32);
            }
        }
        let path: Path = builder.close().build();
        list.stroke(
            path,
            Stroke::Solid(
                Color::parse(DEFAULT_SPLIT_LINE_COLOR).unwrap_or(Color::rgb(217, 217, 217)),
            ),
            LineStyle {
                width: 1.0,
                ..LineStyle::new()
            },
            1.0,
            z - 1,
        );
    }
    for index in 0..indicator_count {
        let (x, y) = geometry.point(index, 1.0);
        let path: Path = Path::builder()
            .move_to(geometry.center_x as f32, geometry.center_y as f32)
            .line_to(x as f32, y as f32)
            .build();
        list.stroke(
            path,
            Stroke::Solid(
                Color::parse(DEFAULT_SPLIT_LINE_COLOR).unwrap_or(Color::rgb(217, 217, 217)),
            ),
            LineStyle {
                width: 1.0,
                ..LineStyle::new()
            },
            1.0,
            z - 1,
        );
    }

    // The series themselves.
    let values: Vec<f64> = series_values(series);
    if values.is_empty() {
        return;
    }
    let extent: f64 = values
        .iter()
        .map(|value: &f64| value.abs())
        .fold(0.0_f64, f64::max)
        .max(f64::EPSILON);

    let mut builder: PathBuilder = Path::builder();
    for (index, value) in values.iter().enumerate() {
        let (x, y) = geometry.point(index, (value / extent).clamp(0.0, 1.0));
        if index == 0 {
            builder = builder.move_to(x as f32, y as f32);
        } else {
            builder = builder.line_to(x as f32, y as f32);
        }
    }
    let path: Path = builder.close().build();
    let fill: Fill = series_fill(series, color);
    list.fill(path.clone(), fill, 1.0, z);
    list.stroke(
        path,
        series_stroke(series, color),
        series_line_style(series, color),
        1.0,
        z + 1,
    );
}

/// Returns the numeric values of a series' data.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
///
/// # Returns
///
/// - `Vec<f64>` - A `Vec<f64>` value.
pub(crate) fn series_values(series: &SeriesOption) -> Vec<f64> {
    series
        .data
        .iter()
        .map(|value: &DataValue| match value {
            DataValue::Number(number) => *number,
            DataValue::List(items) => items
                .get(1)
                .and_then(|item: &DataValue| item.to_number())
                .unwrap_or(0.0),
            DataValue::Object(_) => value
                .get(SERIES_DATA_VALUE_KEY)
                .and_then(|item: &DataValue| item.to_number())
                .unwrap_or(0.0),
            _ => 0.0,
        })
        .collect()
}

/// Reads the nodes of a sankey series.
///
/// A node is an object with a `name` and a `value`; a node without a
/// name is a link.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
///
/// # Returns
///
/// - `Vec<SankeyNode>` - A `Vec<SankeyNode>` value.
pub(crate) fn sankey_nodes(series: &SeriesOption) -> Vec<SankeyNode> {
    series
        .data
        .iter()
        .filter(|value: &&DataValue| value.get(SERIES_DATA_NAME_KEY).is_some())
        .map(|value: &DataValue| SankeyNode {
            name: value
                .get(SERIES_DATA_NAME_KEY)
                .and_then(|item: &DataValue| item.as_text())
                .unwrap_or_default()
                .to_string(),
            value: value
                .get(SERIES_DATA_VALUE_KEY)
                .and_then(|item: &DataValue| item.to_number())
                .unwrap_or(0.0),
            column: 0,
            top: 0.0,
            height: 0.0,
        })
        .collect()
}

/// Reads the links of a sankey series.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
///
/// # Returns
///
/// - `Vec<SankeyLink>` - A `Vec<SankeyLink>` value.
pub(crate) fn sankey_links(series: &SeriesOption) -> Vec<SankeyLink> {
    series
        .data
        .iter()
        .filter(|value: &&DataValue| value.get(SERIES_DATA_NAME_KEY).is_none())
        .map(|value: &DataValue| SankeyLink {
            source: value
                .get(SERIES_DATA_SOURCE_KEY)
                .and_then(|item: &DataValue| item.as_text())
                .unwrap_or_default()
                .to_string(),
            target: value
                .get(SERIES_DATA_TARGET_KEY)
                .and_then(|item: &DataValue| item.as_text())
                .unwrap_or_default()
                .to_string(),
            value: value
                .get(SERIES_DATA_VALUE_KEY)
                .and_then(|item: &DataValue| item.to_number())
                .unwrap_or(0.0),
        })
        .collect()
}

/// Places the nodes of a sankey.
///
/// The nodes are put in one column per depth, and their heights are
/// proportional to their values; the links decide which column a node
/// belongs to.
///
/// # Arguments
///
/// - `&mut [SankeyNode]` - Mutable reference to a `[SankeyNode]` (mutated in place).
/// - `&[SankeyLink]` - Shared reference to a `[SankeyLink]`.
/// - `&BBox` - Shared reference to a `BBox`.
pub(crate) fn place_sankey_nodes(nodes: &mut [SankeyNode], links: &[SankeyLink], plot: &BBox) {
    // A node that is the source of a link sits one column to the left
    // of the node it feeds.
    let mut columns: Vec<String> = Vec::new();
    for link in links {
        for name in [&link.source, &link.target] {
            if !columns.contains(name) {
                columns.push(name.clone());
            }
        }
    }
    for node in nodes.iter_mut() {
        node.column = columns
            .iter()
            .position(|name: &String| name == &node.name)
            .unwrap_or(0);
    }
    let column_count: usize = nodes
        .iter()
        .map(|node: &SankeyNode| node.column)
        .max()
        .unwrap_or(0)
        .max(1)
        + 1;
    let gap: f64 = 2.0;

    for column in 0..column_count {
        let members: Vec<usize> = nodes
            .iter()
            .enumerate()
            .filter(|(_, node): &(usize, &SankeyNode)| node.column == column)
            .map(|(index, _): (usize, &SankeyNode)| index)
            .collect();
        if members.is_empty() {
            continue;
        }
        let total: f64 = members
            .iter()
            .map(|index: &usize| nodes[*index].value.max(0.0))
            .sum();
        let available: f64 = plot.height() - gap * (members.len() as f64 - 1.0).max(0.0);
        let mut top: f64 = plot.min_y;
        for index in members {
            let share: f64 = if total > 0.0 {
                nodes[index].value.max(0.0) / total
            } else {
                0.0
            };
            let height: f64 = available * share;
            nodes[index].top = top;
            nodes[index].height = height;
            top += height + gap;
        }
    }
}

/// Draws a sankey series: the links first, then the nodes.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_sankey(
    list: &mut DisplayList,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    let mut nodes: Vec<SankeyNode> = sankey_nodes(series);
    if nodes.is_empty() {
        return;
    }
    let links: Vec<SankeyLink> = sankey_links(series);
    let plot: &BBox = &layout.plot_rect;
    place_sankey_nodes(&mut nodes, &links, plot);

    let column_count: usize = nodes
        .iter()
        .map(|node: &SankeyNode| node.column)
        .max()
        .unwrap_or(0)
        .max(1)
        + 1;
    let column_width: f64 = plot.width() / column_count as f64;
    let sankey: Option<&SankeyOptions> = series.sankey.as_ref();
    let node_width: f64 = sankey
        .and_then(|sankey: &SankeyOptions| sankey.node_width)
        .unwrap_or(16.0)
        .min(column_width * 0.5);

    let fill: Fill = series_fill(series, color);
    let stroke: Stroke = series_stroke(series, color);
    let style: LineStyle = series_line_style(series, color);

    // The links are drawn as ribbons: a quadrilateral between the two
    // ends of the link.
    for link in &links {
        let source: Option<&SankeyNode> = nodes
            .iter()
            .find(|node: &&SankeyNode| node.name == link.source);
        let target: Option<&SankeyNode> = nodes
            .iter()
            .find(|node: &&SankeyNode| node.name == link.target);
        let (Some(source), Some(target)) = (source, target) else {
            continue;
        };
        let source_x: f64 = plot.min_x + source.column as f64 * column_width + node_width;
        let target_x: f64 = plot.min_x + target.column as f64 * column_width;
        let source_height: f64 =
            (source.height * link.value / source.value.max(f64::EPSILON)).max(1.0);
        let target_height: f64 =
            (target.height * link.value / target.value.max(f64::EPSILON)).max(1.0);
        let path: Path = Path::builder()
            .move_to(source_x as f32, source.top as f32)
            .line_to(target_x as f32, target.top as f32)
            .line_to(target_x as f32, (target.top + target_height) as f32)
            .line_to(source_x as f32, (source.top + source_height) as f32)
            .close()
            .build();
        list.fill(path, fill.clone(), 0.6, z);
    }

    for node in &nodes {
        let x: f64 = plot.min_x + node.column as f64 * column_width;
        let path: Path = Path::builder()
            .rect(
                x as f32,
                node.top as f32,
                node_width as f32,
                node.height as f32,
            )
            .build();
        list.fill(path, fill.clone(), 1.0, z + 1);
        let _: Stroke = stroke;
        let _: LineStyle = style;
    }
}

/// Flattens the hierarchy of a sunburst into a list of sectors.
///
/// The root's children cover the whole circle; each child's own children
/// share its angular extent.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `usize` - A non-negative integer (`usize`).
/// - `f64` - A 64-bit float (`f64`).
/// - `&mut Vec<SunburstSector>` - Mutable reference to a `Vec<SunburstSector>` (mutated in place).
pub(crate) fn flatten_sectors(
    series: &SeriesOption,
    depth: usize,
    start_angle: f64,
    end_angle: f64,
    output: &mut Vec<SunburstSector>,
) {
    let total: f64 = series
        .data
        .iter()
        .map(|value: &DataValue| {
            value
                .get(SERIES_DATA_VALUE_KEY)
                .and_then(|item: &DataValue| item.to_number())
                .unwrap_or(0.0)
        })
        .sum();
    if total <= 0.0 {
        return;
    }
    let mut angle: f64 = start_angle;
    for value in &series.data {
        let number: f64 = value
            .get(SERIES_DATA_VALUE_KEY)
            .and_then(|item: &DataValue| item.to_number())
            .unwrap_or(0.0);
        let share: f64 = number.max(0.0) / total;
        let span: f64 = (end_angle - start_angle) * share;
        let name: String = value
            .get(SERIES_DATA_NAME_KEY)
            .and_then(|item: &DataValue| item.as_text())
            .unwrap_or_default()
            .to_string();
        output.push(SunburstSector {
            name,
            value: number,
            depth,
            start_angle: angle,
            end_angle: angle + span,
        });
        if let Some(children) = value
            .get(SERIES_DATA_CHILDREN_KEY)
            .and_then(|item: &DataValue| item.as_list())
        {
            let child_option: SeriesOption = SeriesOption {
                data: children.to_vec(),
                ..series.clone()
            };
            flatten_sectors(&child_option, depth + 1, angle, angle + span, output);
        }
        angle += span;
    }
}

/// Draws a sunburst series.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_sunburst(
    list: &mut DisplayList,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    let plot: &BBox = &layout.plot_rect;
    let sunburst: Option<&SunburstOptions> = series.sunburst.as_ref();
    let maximum: f64 = plot.width().min(plot.height()) * 0.5;
    let center_x: f64 = sunburst
        .and_then(|sunburst: &SunburstOptions| sunburst.center.as_ref())
        .and_then(|center: &Vec<String>| center.first())
        .and_then(|value: &String| parse_percent(value))
        .map(|value: f64| plot.min_x + value * plot.width())
        .unwrap_or(plot.center_x());
    let center_y: f64 = sunburst
        .and_then(|sunburst: &SunburstOptions| sunburst.center.as_ref())
        .and_then(|center: &Vec<String>| center.get(1))
        .and_then(|value: &String| parse_percent(value))
        .map(|value: f64| plot.min_y + value * plot.height())
        .unwrap_or(plot.center_y());
    let start_angle: f64 = sunburst
        .and_then(|sunburst: &SunburstOptions| sunburst.start_angle)
        .unwrap_or(90.0)
        .to_radians();

    let mut sectors: Vec<SunburstSector> = Vec::new();
    flatten_sectors(
        series,
        0,
        start_angle,
        start_angle + core::f64::consts::TAU,
        &mut sectors,
    );
    if sectors.is_empty() {
        return;
    }
    let depth: usize = sectors
        .iter()
        .map(|sector: &SunburstSector| sector.depth)
        .max()
        .unwrap_or(0)
        + 1;
    let ring: f64 = maximum / depth as f64;
    let fill: Fill = series_fill(series, color);

    for sector in &sectors {
        let inner: f64 = ring * sector.depth as f64;
        let outer: f64 = inner + ring;
        let path: Path = arc_path(
            center_x,
            center_y,
            outer,
            inner,
            sector.start_angle,
            sector.end_angle,
        );
        list.fill(path, fill.clone(), 1.0, z + sector.depth as i32);

        // The label sits in the middle of the sector's arc, and is left
        // out when the sector is too narrow to hold it.
        let middle: f64 = (sector.start_angle + sector.end_angle) * 0.5;
        let radius: f64 = (inner + outer) * 0.5;
        let x: f64 = center_x + radius * middle.cos();
        let y: f64 = center_y + radius * middle.sin();
        let arc: f64 = sector.end_angle - sector.start_angle;
        if sector.name.is_empty() || arc * radius < 18.0 {
            continue;
        }
        let mut label_style: TextStyle = TextStyle::new();
        label_style.size = 12.0;
        label_style.align = TextAlign::Center;
        label_style.vertical_align = TextVerticalAlign::Middle;
        list.text(
            format!("{} ({})", sector.name, format_number(sector.value)),
            x,
            y,
            label_style,
            z + sector.depth as i32 + 1,
        );
    }
}

/// Flattens the hierarchy of a treemap into a list of tiles.
///
/// The hierarchy is a list of objects with a `name`, a `value`, and
/// optional `children`; the children of a node are laid out inside the
/// node's own rectangle.
///
/// # Arguments
///
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `BBox` - A `BBox` parameter.
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `Vec<TreeTile>` - A `Vec<TreeTile>` value.
pub(crate) fn flatten_tiles(series: &SeriesOption, rect: BBox, depth: usize) -> Vec<TreeTile> {
    let mut tiles: Vec<TreeTile> = Vec::new();
    for value in &series.data {
        let name: String = value
            .get(SERIES_DATA_NAME_KEY)
            .and_then(|item: &DataValue| item.as_text())
            .unwrap_or_default()
            .to_string();
        let number: f64 = value
            .get(SERIES_DATA_VALUE_KEY)
            .and_then(|item: &DataValue| item.to_number())
            .unwrap_or(0.0);
        let children: Option<Vec<DataValue>> = value
            .get(SERIES_DATA_CHILDREN_KEY)
            .and_then(|item: &DataValue| item.as_list())
            .map(|items: &[DataValue]| items.to_vec());
        tiles.push(TreeTile {
            name,
            value: number,
            rect,
            depth,
        });
        if let Some(children) = children {
            let child_option: SeriesOption = SeriesOption {
                data: children,
                ..series.clone()
            };
            tiles.extend(flatten_tiles(&child_option, rect, depth + 1));
        }
    }
    tiles
}

/// Lays a list of tiles out inside a rectangle, squarified.
///
/// The squarified layout keeps every tile as close to a square as it
/// can, which is what makes a treemap readable: a long thin tile is
/// hard to compare with its neighbours.
///
/// # Arguments
///
/// - `&mut [TreeTile]` - Mutable reference to a `[TreeTile]` (mutated in place).
/// - `BBox` - A `BBox` parameter.
/// - `f64` - A 64-bit float (`f64`).
pub(crate) fn squarify(tiles: &mut [TreeTile], rect: BBox, gap: f64) {
    let total: f64 = tiles
        .iter()
        .map(|tile: &TreeTile| tile.value.max(0.0))
        .sum();
    if total <= 0.0 || tiles.is_empty() {
        return;
    }
    let mut remaining: BBox = rect;
    let mut index: usize = 0;
    while index < tiles.len() {
        // The row grows until the tiles stop being square.
        let mut row: Vec<usize> = Vec::new();
        let mut row_value: f64 = 0.0;
        let mut best_ratio: f64 = f64::INFINITY;
        while index < tiles.len() {
            let value: f64 = tiles[index].value.max(0.0);
            let candidate: f64 = row_value + value;
            let ratio: f64 = row_ratio(&tiles[index..index + row.len() + 1], candidate, remaining);
            if !row.is_empty() && ratio > best_ratio {
                break;
            }
            best_ratio = ratio;
            row.push(index);
            row_value = candidate;
            index += 1;
        }
        if row.is_empty() {
            break;
        }
        let share: f64 = row_value / total;
        let width: f64 = remaining.width();
        let height: f64 = remaining.height();
        // The row is laid out along the shorter side of the rectangle.
        let vertical: bool = width >= height;
        let thickness: f64 = if vertical {
            width * share
        } else {
            height * share
        };
        let mut offset: f64 = 0.0;
        for position in &row {
            let value: f64 = tiles[*position].value.max(0.0);
            let fraction: f64 = if row_value > 0.0 {
                value / row_value
            } else {
                0.0
            };
            let extent: f64 = if vertical {
                remaining.height() * fraction
            } else {
                remaining.width() * fraction
            };
            let tile_rect: BBox = if vertical {
                BBox::from_corners(
                    remaining.min_x,
                    remaining.min_y + offset,
                    remaining.min_x + thickness,
                    remaining.min_y + offset + extent,
                )
            } else {
                BBox::from_corners(
                    remaining.min_x + offset,
                    remaining.min_y,
                    remaining.min_x + offset + extent,
                    remaining.min_y + thickness,
                )
            };
            tiles[*position].rect = shrink(tile_rect, gap);
            offset += extent;
        }
        remaining = if vertical {
            BBox::from_corners(
                remaining.min_x + thickness,
                remaining.min_y,
                remaining.max_x,
                remaining.max_y,
            )
        } else {
            BBox::from_corners(
                remaining.min_x,
                remaining.min_y + thickness,
                remaining.max_x,
                remaining.max_y,
            )
        };
    }
}

/// Returns how far the tiles of a row are from being square.
///
/// The smaller the value, the closer the row is to a square.
///
/// # Arguments
///
/// - `&[TreeTile]` - Shared reference to a `[TreeTile]`.
/// - `f64` - A 64-bit float (`f64`).
/// - `BBox` - A `BBox` parameter.
///
/// # Returns
///
/// - `f64` - A 64-bit float.
fn row_ratio(tiles: &[TreeTile], total: f64, rect: BBox) -> f64 {
    if total <= 0.0 {
        return f64::INFINITY;
    }
    let area: f64 = rect.width() * rect.height();
    if area <= 0.0 {
        return f64::INFINITY;
    }
    let side: f64 = (area * total).sqrt();
    let mut worst: f64 = 0.0_f64;
    for tile in tiles {
        let value: f64 = tile.value.max(0.0);
        if value <= 0.0 {
            continue;
        }
        let length: f64 = side / value.max(f64::EPSILON);
        worst = worst.max(length / side.max(f64::EPSILON));
    }
    worst
}

/// Shrinks a rectangle by the gap on every side.
///
/// # Arguments
///
/// - `BBox` - A `BBox` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `BBox` - A `BBox` value.
fn shrink(rect: BBox, gap: f64) -> BBox {
    BBox::from_corners(
        rect.min_x + gap,
        rect.min_y + gap,
        (rect.max_x - gap).max(rect.min_x + gap),
        (rect.max_y - gap).max(rect.min_y + gap),
    )
}

/// Draws a treemap series.
///
/// # Arguments
///
/// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
/// - `&SeriesOption` - Shared reference to a `SeriesOption`.
/// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
/// - `Color` - A `Color` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
pub(crate) fn draw_treemap(
    list: &mut DisplayList,
    series: &SeriesOption,
    layout: &SeriesLayout,
    color: Color,
    z: i32,
) {
    let plot: &BBox = &layout.plot_rect;
    let treemap: Option<&TreemapOptions> = series.treemap.as_ref();
    let gap: f64 = treemap
        .and_then(|treemap: &TreemapOptions| treemap.left)
        .unwrap_or(2.0);
    let mut tiles: Vec<TreeTile> = flatten_tiles(series, *plot, 0);
    if tiles.is_empty() {
        return;
    }
    squarify(&mut tiles, *plot, gap);
    let fill: Fill = series_fill(series, color);
    let mut label_style: TextStyle = TextStyle::new();
    label_style.size = 12.0;
    label_style.align = TextAlign::Center;
    label_style.vertical_align = TextVerticalAlign::Middle;
    for tile in &tiles {
        let path: Path = Path::builder()
            .rect(
                tile.rect.min_x as f32,
                tile.rect.min_y as f32,
                tile.rect.width() as f32,
                tile.rect.height() as f32,
            )
            .build();
        list.fill(path, fill.clone(), 1.0, z + tile.depth as i32);
        // A tile that is too small for its label is left unlabelled: a
        // label that does not fit is worse than no label.
        if tile.name.is_empty() || tile.rect.width() < 24.0 || tile.rect.height() < 14.0 {
            continue;
        }
        list.text(
            tile.name.clone(),
            tile.rect.center_x(),
            tile.rect.center_y(),
            label_style.clone(),
            z + tile.depth as i32 + 1,
        );
    }
}
