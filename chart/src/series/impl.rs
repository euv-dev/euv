use super::*;

/// Inherent implementation of [`SeriesRenderer`].
impl SeriesRenderer {
    /// Returns a renderer.
    pub fn new() -> Self {
        Self {}
    }

    /// Draws every series of the chart.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&ChartLayout` - Shared reference to a `ChartLayout`.
    pub fn draw(&mut self, list: &mut DisplayList, option: &ChartOption, layout: &ChartLayout) {
        for (index, series) in option.series.iter().enumerate() {
            if !series.show {
                continue;
            }
            let color: Color = palette_color(option, index);
            let series_layout: Option<&SeriesLayout> = layout
                .series
                .iter()
                .find(|entry: &&SeriesLayout| entry.index == index);
            let Some(series_layout) = series_layout else {
                continue;
            };
            let z: i32 = Z_SERIES + index as i32 * 10;
            match series.kind {
                SeriesType::Line => {
                    self.draw_line(list, series, series_layout, color, z);
                }
                SeriesType::ThemeRiver => {
                    draw_theme_river(list, series, series_layout, color, z);
                }
                SeriesType::Bar => {
                    self.draw_bar(list, series, series_layout, color, z);
                }
                SeriesType::PictorialBar => {
                    draw_pictorial_bar(list, series, series_layout, color, z);
                }
                SeriesType::Pie => {
                    self.draw_pie(list, series, series_layout, color, z);
                }
                SeriesType::Funnel => {
                    draw_funnel(list, series, series_layout, color, z);
                }
                SeriesType::Radar => {
                    draw_radar(list, option, series, series_layout, color, z);
                }
                SeriesType::Gauge => {
                    draw_gauge(list, series, series_layout, color, z);
                }
                SeriesType::Scatter | SeriesType::EffectScatter => {
                    self.draw_scatter(list, series, series_layout, color, z);
                }
                SeriesType::Candlestick => {
                    self.draw_candlestick(list, series, series_layout, color, z);
                }
                SeriesType::Boxplot => {
                    self.draw_boxplot(list, series, series_layout, color, z);
                }
                SeriesType::Heatmap => {
                    self.draw_heatmap(list, series, series_layout, color, z);
                }
                SeriesType::Graph => {
                    draw_graph(list, series, series_layout, color, z);
                }
                SeriesType::Treemap => {
                    draw_treemap(list, series, series_layout, color, z);
                }
                SeriesType::Sunburst => {
                    draw_sunburst(list, series, series_layout, color, z);
                }
                SeriesType::Sankey => {
                    draw_sankey(list, series, series_layout, color, z);
                }
                SeriesType::Parallel => {
                    draw_parallel(list, series, series_layout, color, z);
                }
                SeriesType::Lines => {
                    draw_lines(list, series, series_layout, color, z);
                }
                SeriesType::Map => {
                    draw_map(list, option, series, series_layout, layout, color, z);
                }
                SeriesType::Custom => {
                    draw_custom(list, series, series_layout, layout, color, z);
                }
            }
        }
    }

    /// Draws the labels of a series' data points.
    ///
    /// A label shows the point's name when it has one, and its value
    /// otherwise — which is what a reader expects from a chart whose
    /// points are not named.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&SeriesOption` - Shared reference to a `SeriesOption`.
    /// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
    /// - `Color` - A `Color` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    fn draw_labels(
        &mut self,
        list: &mut DisplayList,
        series: &SeriesOption,
        layout: &SeriesLayout,
        color: Color,
        z: i32,
    ) {
        let Some(label) = &series.label else {
            return;
        };
        if !label.show {
            return;
        }
        let style: TextStyle = series_label_style(series, color);
        let distance: f64 = label.distance.unwrap_or(0.0);
        for point in &layout.points {
            let text: String = if point.name.is_empty() {
                format_number(point.value)
            } else {
                point.name.clone()
            };
            if text.is_empty() {
                continue;
            }
            let mut label_style: TextStyle = style.clone();
            if let Some(align) = label.align {
                label_style.align = align;
            }
            if let Some(vertical_align) = label.vertical_align {
                label_style.vertical_align = vertical_align;
            }
            list.text(text, point.x, point.y - distance, label_style, z);
        }
    }

    /// Draws a line series, with its area fill and its symbols.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&SeriesOption` - Shared reference to a `SeriesOption`.
    /// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
    /// - `Color` - A `Color` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    fn draw_line(
        &mut self,
        list: &mut DisplayList,
        series: &SeriesOption,
        layout: &SeriesLayout,
        color: Color,
        z: i32,
    ) {
        if layout.points.is_empty() {
            return;
        }
        let points: Vec<Point> = layout
            .points
            .iter()
            .map(|point: &SeriesPoint| Point::new(point.x as f32, point.y as f32))
            .collect();

        // The area fill comes first: it sits under the line.
        if let Some(line_options) = &series.line
            && let Some(area_style) = &line_options.area_style
        {
            let mut area: SeriesOption = series.clone();
            area.item_style = Some(area_style.clone());
            let area_color: Color = area_style
                .color
                .as_ref()
                .and_then(|value: &String| Color::parse(value))
                .unwrap_or(color);
            let fill: Fill = series_fill(&area, area_color);
            let baseline: f64 = baseline_of(&layout.points);
            list.strip(points.clone(), baseline as f32, fill, 1.0, z);
        }

        let path: Path = match series
            .line
            .as_ref()
            .and_then(|line: &LineOptions| line.smooth)
        {
            Some(LineSmooth::Smooth) => smooth_path(&points),
            Some(LineSmooth::Step) => step_path(&points, StepPosition::Middle),
            _ => polyline_path(&points),
        };
        let stroke: Stroke = series_stroke(series, color);
        let style: LineStyle = series_line_style(series, color);
        list.stroke(path, stroke, style, 1.0, z + 1);

        // The symbols.
        let show_symbol: bool = series
            .line
            .as_ref()
            .and_then(|line: &LineOptions| line.show_symbol)
            .unwrap_or(true)
            && series
                .label
                .as_ref()
                .map(|label: &LabelOption| label.show)
                .unwrap_or(false);
        let symbol: SymbolType = series
            .line
            .as_ref()
            .and_then(|line: &LineOptions| line.symbol)
            .unwrap_or(SymbolType::Circle);
        let size: f64 = series
            .line
            .as_ref()
            .and_then(|line: &LineOptions| line.symbol_size)
            .map(|size: SymbolSize| match size {
                SymbolSize::Uniform(value) => value,
                SymbolSize::NonUniform(width, _) => width,
            })
            .unwrap_or(DEFAULT_SYMBOL_SIZE);
        // The symbols are only drawn when the points are far enough
        // apart for them to be told apart.
        let dense: bool = average_spacing(&layout.points) < size;
        let show_all: bool = series
            .line
            .as_ref()
            .and_then(|line: &LineOptions| line.show_all_symbol)
            .unwrap_or(false);
        if show_symbol && (!dense || show_all) {
            let fill: Fill = series_fill(series, color);
            for point in &layout.points {
                let path: Path = symbol_path(symbol, point.x, point.y, size);
                list.fill(path, fill.clone(), 1.0, z + 2);
            }
        }

        self.draw_labels(list, series, layout, color, z + 3);
    }

    /// Draws a bar series.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&SeriesOption` - Shared reference to a `SeriesOption`.
    /// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
    /// - `Color` - A `Color` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    fn draw_bar(
        &mut self,
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
        // A bar is bordered by its *item* style, and not by its line
        // style: ECharts draws no border at all unless the option asks
        // for one. A border on every one of a million bars would double
        // the work of the frame for a line nobody asked for.
        let item_style: ItemStyleOption = series.item_style.clone().unwrap_or_default();
        let border_width: f64 = item_style.border_width.unwrap_or(0.0);
        let border_color: Color = item_style
            .border_color
            .as_deref()
            .and_then(Color::parse)
            .unwrap_or(color);

        let bar_options: BarOptions = series.bar.clone().unwrap_or_default();
        let horizontal: bool = bar_options.horizontal;
        let category_gap: f64 = bar_options
            .bar_category_gap
            .as_deref()
            .and_then(parse_percent)
            .unwrap_or(DEFAULT_BAR_CATEGORY_GAP);
        let group_gap: f64 = bar_options
            .bar_gap
            .as_deref()
            .and_then(parse_percent)
            .unwrap_or(DEFAULT_BAR_GAP);

        // The width of one bar: the band, minus the gaps.
        let group_size: f64 = layout.stack_size.max(1) as f64;
        let available: f64 = (layout.band_width * (1.0 - category_gap)).max(1.0);
        let bar_width: f64 = match bar_options.bar_width {
            Some(width) => width,
            None => (available / group_size * (1.0 - group_gap)).max(1.0),
        };
        let clamped: f64 = bar_options.bar_max_width.unwrap_or(f64::INFINITY);
        let minimum: f64 = bar_options.bar_min_width.unwrap_or(0.0);
        let bar_width: f64 = bar_width.clamp(minimum, clamped);

        // The baseline the bars grow from.
        let baseline: f64 = baseline_of(&layout.points);

        for (index, point) in layout.points.iter().enumerate() {
            let offset: f64 = (index % group_size as usize) as f64
                * (bar_width / (1.0 - group_gap).max(f64::EPSILON));
            let center: f64 = point.x;
            let value: f64 = point.value;
            let path: Path = if horizontal {
                let left: f64 = baseline.min(point.y);
                let right: f64 = baseline.max(point.y);
                Path::builder()
                    .rect(
                        (left + offset) as f32,
                        (center - bar_width * 0.5) as f32,
                        (right - left).max(0.0) as f32,
                        bar_width as f32,
                    )
                    .build()
            } else {
                let top: f64 = baseline.min(point.y);
                let height: f64 = (baseline - point.y).abs();
                Path::builder()
                    .rect(
                        (center - bar_width * 0.5 + offset) as f32,
                        top as f32,
                        bar_width as f32,
                        height.max(0.0) as f32,
                    )
                    .build()
            };
            let _: f64 = value;
            list.fill(path.clone(), fill.clone(), 1.0, z);
            if border_width > 0.0 {
                let mut style: LineStyle = LineStyle::new();
                style.width = border_width;
                style.color = border_color;
                list.stroke(path, Stroke::Solid(border_color), style, 1.0, z + 1);
            }
        }
    }

    /// Draws a pie or a funnel series.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&SeriesOption` - Shared reference to a `SeriesOption`.
    /// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
    /// - `Color` - A `Color` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    fn draw_pie(
        &mut self,
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
        let total: f64 = layout
            .points
            .iter()
            .map(|point: &SeriesPoint| point.value.max(0.0))
            .sum();
        if total <= 0.0 {
            return;
        }
        let (center_x, center_y, radius) = pie_geometry(series, layout);
        let inner_radius: f64 = series
            .pie
            .as_ref()
            .and_then(|pie: &PieOptions| pie.inner_radius.as_deref())
            .and_then(parse_percent)
            .map(|value: f64| value * radius)
            .unwrap_or(0.0);
        let start_angle: f64 = series
            .pie
            .as_ref()
            .and_then(|pie: &PieOptions| pie.start_angle)
            .unwrap_or(90.0)
            .to_radians();
        let mut angle: f64 = start_angle;
        for (index, point) in layout.points.iter().enumerate() {
            let share: f64 = point.value.max(0.0) / total;
            let sweep: f64 = share * core::f64::consts::TAU;
            let path: Path = sector_path(
                center_x,
                center_y,
                radius,
                inner_radius,
                angle,
                angle + sweep,
            );
            list.fill(path, fill.clone(), 1.0, z + index as i32);
            angle += sweep;
        }
    }

    /// Draws a scatter series.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&SeriesOption` - Shared reference to a `SeriesOption`.
    /// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
    /// - `Color` - A `Color` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    fn draw_scatter(
        &mut self,
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
        let symbol: SymbolType = series
            .scatter
            .as_ref()
            .and_then(|scatter: &ScatterOptions| scatter.symbol)
            .unwrap_or(SymbolType::Circle);
        let size: f64 = series
            .scatter
            .as_ref()
            .and_then(|scatter: &ScatterOptions| scatter.symbol_size)
            .map(|size: SymbolSize| match size {
                SymbolSize::Uniform(value) => value,
                SymbolSize::NonUniform(width, _) => width,
            })
            .unwrap_or(DEFAULT_SYMBOL_SIZE);
        // The symbols are dropped when the points are too close to be
        // told apart; a dense scatter is better read as a cloud.
        let dense: bool = average_spacing(&layout.points) < size;
        if dense {
            return;
        }
        for point in &layout.points {
            let path: Path = symbol_path(symbol, point.x, point.y, size);
            list.fill(path, fill.clone(), 1.0, z);
        }
        self.draw_labels(list, series, layout, color, z + 1);
    }

    /// Draws a candlestick series.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&SeriesOption` - Shared reference to a `SeriesOption`.
    /// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
    /// - `Color` - A `Color` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    fn draw_candlestick(
        &mut self,
        list: &mut DisplayList,
        series: &SeriesOption,
        layout: &SeriesLayout,
        color: Color,
        z: i32,
    ) {
        if layout.points.is_empty() {
            return;
        }
        let rising: Color = series
            .candlestick
            .as_ref()
            .and_then(|candle: &CandlestickOptions| candle.color.as_deref())
            .and_then(Color::parse)
            .unwrap_or(color);
        let falling: Color = series
            .candlestick
            .as_ref()
            .and_then(|candle: &CandlestickOptions| candle.color0.as_deref())
            .and_then(Color::parse)
            .unwrap_or(Color::rgb(30, 130, 220));
        let width: f64 = series
            .candlestick
            .as_ref()
            .and_then(|candle: &CandlestickOptions| candle.bar_width)
            .unwrap_or(DEFAULT_BAR_WIDTH * 0.5);
        let baseline: f64 = baseline_of(&layout.points);

        for point in &layout.points {
            let body: f64 = series
                .candlestick
                .as_ref()
                .and_then(|candle: &CandlestickOptions| candle.bar_max_width)
                .unwrap_or(width);
            let fill: Color = if point.value >= 0.0 { rising } else { falling };
            let top: f64 = baseline.min(point.y);
            let height: f64 = (baseline - point.y).abs();
            let body_path: Path = Path::builder()
                .rect(
                    (point.x - body * 0.5) as f32,
                    top as f32,
                    body as f32,
                    height.max(1.0) as f32,
                )
                .build();
            list.fill(body_path, Fill::Solid(fill), 1.0, z);
        }
    }

    /// Draws a boxplot series.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&SeriesOption` - Shared reference to a `SeriesOption`.
    /// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
    /// - `Color` - A `Color` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    fn draw_boxplot(
        &mut self,
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
        let width: f64 = series
            .boxplot
            .as_ref()
            .and_then(|boxplot: &BoxplotOptions| boxplot.box_width)
            .unwrap_or(DEFAULT_BAR_WIDTH);
        for point in &layout.points {
            let path: Path = Path::builder()
                .rect(
                    (point.x - width * 0.5) as f32,
                    (point.y - width * 0.5) as f32,
                    width as f32,
                    width as f32,
                )
                .build();
            list.fill(path, fill.clone(), 1.0, z);
        }
    }

    /// Draws a heatmap series.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&SeriesOption` - Shared reference to a `SeriesOption`.
    /// - `&SeriesLayout` - Shared reference to a `SeriesLayout`.
    /// - `Color` - A `Color` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    fn draw_heatmap(
        &mut self,
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
        let size: f64 = series
            .heatmap
            .as_ref()
            .and_then(|heatmap: &HeatmapOptions| heatmap.min_size)
            .unwrap_or(1.0);
        for point in &layout.points {
            let path: Path = Path::builder()
                .rect(
                    (point.x - size * 0.5) as f32,
                    (point.y - size * 0.5) as f32,
                    size as f32,
                    size as f32,
                )
                .build();
            list.fill(path, fill.clone(), 1.0, z);
        }
    }
}

/// Default construction for [`SeriesRenderer`].
impl Default for SeriesRenderer {
    /// Constructs a default [`SeriesRenderer`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`RadarGeometry`].
impl RadarGeometry {
    /// Returns the position of a point on the radar.
    ///
    /// `index` is the index of the side, `ratio` is how far along that
    /// side the point is, from `0.0` at the centre to `1.0` at the edge.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `(f64, f64)` - A `(f64, f64)` value.
    pub fn point(&self, index: usize, ratio: f64) -> (f64, f64) {
        let angle: f64 = self.get_start_angle()
            + index as f64 / self.get_sides().max(1) as f64 * core::f64::consts::TAU;
        (
            self.get_center_x() + self.get_radius() * ratio * angle.cos(),
            self.get_center_y() + self.get_radius() * ratio * angle.sin(),
        )
    }
}
