use super::*;

/// Inherent implementation of [`Scale`].
impl Scale {
    /// Returns a linear scale covering the given domain and range.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn linear(domain_min: f64, domain_max: f64, range_min: f64, range_max: f64) -> Self {
        Self {
            kind: ScaleKind::Linear,
            domain_min,
            domain_max,
            range_min,
            range_max,
            categories: Vec::new(),
            log_base: 10.0,
            band: 0.0,
            count: 0,
        }
    }

    /// Returns a logarithmic scale covering the given domain and range.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn log(
        domain_min: f64,
        domain_max: f64,
        range_min: f64,
        range_max: f64,
        base: f64,
    ) -> Self {
        Self {
            kind: ScaleKind::Log,
            domain_min,
            domain_max,
            range_min,
            range_max,
            categories: Vec::new(),
            log_base: base,
            band: 0.0,
            count: 0,
        }
    }

    /// Returns an ordinal scale over the given categories.
    ///
    /// `range_min` and `range_max` are the positions of the *centres* of
    /// the first and the last band, and `band` is the width of one band.
    ///
    /// # Arguments
    ///
    /// - `Vec<String>` - A `Vec<String>` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    pub fn category(categories: Vec<String>, range_min: f64, range_max: f64, band: f64) -> Self {
        let count: usize = categories.len();
        Self {
            kind: ScaleKind::Category,
            domain_min: 0.0,
            domain_max: count.saturating_sub(1) as f64,
            range_min,
            range_max,
            categories,
            log_base: 10.0,
            band,
            count,
        }
    }

    /// Returns an ordinal scale over the given number of categories.
    ///
    /// The scale has no names: it is used for a chart whose points are
    /// not named, where keeping a million empty strings would be pure
    /// waste.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    /// - `f64` - A 64-bit float (`f64`).
    pub fn category_count(count: usize, range_min: f64, range_max: f64, band: f64) -> Self {
        Self {
            kind: ScaleKind::Category,
            domain_min: 0.0,
            domain_max: count.saturating_sub(1) as f64,
            range_min,
            range_max,
            categories: Vec::new(),
            log_base: 10.0,
            band,
            count,
        }
    }

    /// Maps a data value to a pixel position.
    ///
    /// Values outside the domain are clamped to the range, which is what
    /// keeps a series that overflows its axis from drawing outside the
    /// grid.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn map(&self, value: f64) -> f64 {
        match self.get_kind() {
            ScaleKind::Category => self.map_ordinal(value),
            ScaleKind::Log => {
                let base: f64 = self.get_log_base().max(1.000_001);
                let low: f64 = self.get_domain_min().max(f64::MIN_POSITIVE).log(base);
                let high: f64 = self.get_domain_max().max(f64::MIN_POSITIVE).log(base);
                let position: f64 = if (high - low).abs() < f64::EPSILON {
                    self.get_range_min()
                } else {
                    let t: f64 = (value.max(f64::MIN_POSITIVE).log(base) - low) / (high - low);
                    self.get_range_min() + t * (self.get_range_max() - self.get_range_min())
                };
                position.clamp(
                    self.get_range_min().min(self.get_range_max()),
                    self.get_range_max().max(self.get_range_min()),
                )
            }
            _ => {
                let span: f64 = self.get_domain_max() - self.get_domain_min();
                if span.abs() < f64::EPSILON {
                    return (self.get_range_min() + self.get_range_max()) * 0.5;
                }
                let t: f64 = (value - self.get_domain_min()) / span;
                let position: f64 =
                    self.get_range_min() + t * (self.get_range_max() - self.get_range_min());
                position.clamp(
                    self.get_range_min().min(self.get_range_max()),
                    self.get_range_max().max(self.get_range_min()),
                )
            }
        }
    }

    /// Maps an ordinal index to a pixel position.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    fn map_ordinal(&self, value: f64) -> f64 {
        if self.get_count() <= 1 {
            return (self.get_range_min() + self.get_range_max()) * 0.5;
        }
        let span: f64 = self.get_domain_max() - self.get_domain_min();
        if span.abs() < f64::EPSILON {
            return self.get_range_min();
        }
        let t: f64 = (value - self.get_domain_min()) / span;
        let position: f64 =
            self.get_range_min() + t * (self.get_range_max() - self.get_range_min());
        position.clamp(
            self.get_range_min().min(self.get_range_max()),
            self.get_range_max().max(self.get_range_min()),
        )
    }

    /// Maps a pixel position back to a data value.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn invert(&self, pixel: f64) -> f64 {
        match self.get_kind() {
            ScaleKind::Log => {
                let base: f64 = self.get_log_base().max(1.000_001);
                let span: f64 = self.get_range_max() - self.get_range_min();
                if span.abs() < f64::EPSILON {
                    return self.get_domain_min();
                }
                let t: f64 = (pixel - self.get_range_min()) / span;
                let low: f64 = self.get_domain_min().max(f64::MIN_POSITIVE).log(base);
                let high: f64 = self.get_domain_max().max(f64::MIN_POSITIVE).log(base);
                (low + t * (high - low)).powf(base)
            }
            _ => {
                let span: f64 = self.get_range_max() - self.get_range_min();
                if span.abs() < f64::EPSILON {
                    return self.get_domain_min();
                }
                let t: f64 = (pixel - self.get_range_min()) / span;
                self.get_domain_min() + t * (self.get_domain_max() - self.get_domain_min())
            }
        }
    }

    /// Returns the width of one band of an ordinal scale, in pixels.
    ///
    /// Returns `0.0` for a scale that is not ordinal.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn band_width(&self) -> f64 {
        if self.get_kind() != ScaleKind::Category {
            return 0.0;
        }
        self.get_band()
    }

    /// Returns `true` when the scale maps ordinal values.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_ordinal(&self) -> bool {
        self.get_kind() == ScaleKind::Category
    }
}

/// Inherent implementation of [`AxisLayout`].
impl AxisLayout {
    /// Returns the pixel position of the given data value.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn position_of(&self, value: f64) -> f64 {
        self.get_scale().map(value)
    }

    /// Returns the index of the category at the given pixel position.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - `Some(...)` on success, `None` otherwise.
    pub fn category_at(&self, pixel: f64) -> Option<usize> {
        if !self.get_scale().is_ordinal() {
            return None;
        }
        let value: f64 = self.get_scale().invert(pixel);
        let index: isize = value.round() as isize;
        // The count is the number of categories whether they are named
        // or not: a chart over a million unnamed points has a million
        // categories and not one name.
        let count: usize = if self.get_scale().get_categories().is_empty() {
            self.get_scale().get_count()
        } else {
            self.get_scale().get_categories().len()
        };
        if index < 0 || index as usize >= count {
            return None;
        }
        Some(index as usize)
    }
}

/// Inherent implementation of [`GridLayout`].
impl GridLayout {
    /// Returns the x-axis at the given index, falling back to the first
    /// one.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    ///
    /// # Returns
    ///
    /// - `Option<&AxisLayout>` - `Some(...)` on success, `None` otherwise.
    pub fn x_axis(&self, index: usize) -> Option<&AxisLayout> {
        self.get_x_axes()
            .get(index)
            .or_else(|| self.get_x_axes().first())
    }

    /// Returns the y-axis at the given index, falling back to the first
    /// one.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    ///
    /// # Returns
    ///
    /// - `Option<&AxisLayout>` - `Some(...)` on success, `None` otherwise.
    pub fn y_axis(&self, index: usize) -> Option<&AxisLayout> {
        self.get_y_axes()
            .get(index)
            .or_else(|| self.get_y_axes().first())
    }
}

/// Inherent implementation of [`ChartLayout`].
impl ChartLayout {
    /// Computes the layout of a chart.
    ///
    /// The layout is the bridge between the option and the display list:
    /// it resolves the sizes, computes the scales and the ticks, and
    /// places every data point in pixel space. The renderer then only has
    /// to turn the result into drawing instructions.
    ///
    /// `measurer` is used to size the axis labels; passing a measurer
    /// that has already measured the same labels costs nothing, because
    /// the measurements are cached.
    ///
    /// # Arguments
    ///
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `&TextMeasurer` - Shared reference to a `TextMeasurer`.
    ///
    /// # Returns
    ///
    /// - `ChartLayout` - A `ChartLayout` value.
    pub fn compute(
        option: &ChartOption,
        width: f64,
        height: f64,
        measurer: &TextMeasurer,
    ) -> ChartLayout {
        Self::compute_with_maps(option, width, height, measurer, &MapRegistry::new())
    }

    /// Computes the layout of a chart that draws maps.
    ///
    /// The geometry of a map is far too large to live in an option: the
    /// same map is drawn by every chart of an application, and it is
    /// registered once. The registry is where the charts look it up.
    ///
    /// # Arguments
    ///
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `&TextMeasurer` - Shared reference to a `TextMeasurer`.
    /// - `&MapRegistry` - Shared reference to a `MapRegistry`.
    ///
    /// # Returns
    ///
    /// - `ChartLayout` - A `ChartLayout` value.
    pub fn compute_with_maps(
        option: &ChartOption,
        width: f64,
        height: f64,
        measurer: &TextMeasurer,
        maps: &MapRegistry,
    ) -> ChartLayout {
        let mut layout: ChartLayout = ChartLayout {
            width,
            height,
            grids: Vec::new(),
            series: Vec::new(),
            geo: Vec::new(),
            title_rect: None,
            legend_rect: None,
            plot_rect: BBox::from_min_size(0.0, 0.0, width, height),
        };

        // The title and the legend take space away from the plot area.
        let title_space: f64 = layout.measure_title(option, measurer);
        let legend_space: f64 = layout.measure_legend(option, measurer);

        layout.plot_rect = BBox::from_min_size(
            0.0,
            title_space,
            width,
            (height - title_space - legend_space).max(0.0),
        );

        layout.grids = layout.measure_grids(option, &layout.plot_rect, measurer);
        layout.series = layout.measure_series(option, &layout.grids);
        layout.geo = layout.measure_geo(option, maps);
        layout
    }

    /// Measures the space the title needs.
    ///
    /// # Arguments
    ///
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&TextMeasurer` - Shared reference to a `TextMeasurer`.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    fn measure_title(&mut self, option: &ChartOption, measurer: &TextMeasurer) -> f64 {
        let Some(title) = &option.title else {
            return 0.0;
        };
        if !title.show || title.text.is_empty() {
            return 0.0;
        }
        let mut space: f64 = 0.0;
        let font_size: f64 = title
            .text_style
            .as_ref()
            .and_then(|style: &LabelOption| style.font_size)
            .unwrap_or(DEFAULT_TITLE_FONT_SIZE);
        let metrics: TextMetrics = measurer.measure(&title.text, &font_string(font_size));
        space += metrics.height;
        if !title.subtext.is_empty() {
            let subtext_size: f64 = title
                .subtext_style
                .as_ref()
                .and_then(|style: &LabelOption| style.font_size)
                .unwrap_or(DEFAULT_SUBTITLE_FONT_SIZE);
            let subtext: TextMetrics = measurer.measure(&title.subtext, &font_string(subtext_size));
            space += subtext.height + title.item_gap.unwrap_or(4.0);
        }
        space += 8.0;
        self.set_title_rect(Some(BBox::from_min_size(0.0, 0.0, self.get_width(), space)));
        space
    }

    /// Measures the space the legend needs.
    ///
    /// # Arguments
    ///
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&TextMeasurer` - Shared reference to a `TextMeasurer`.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    fn measure_legend(&mut self, option: &ChartOption, measurer: &TextMeasurer) -> f64 {
        let Some(legend) = &option.legend else {
            return 0.0;
        };
        if !legend.show {
            return 0.0;
        }
        let font_size: f64 = legend
            .text_style
            .as_ref()
            .and_then(|style: &LabelOption| style.font_size)
            .unwrap_or(DEFAULT_LEGEND_FONT_SIZE);
        let mut widest: f64 = 0.0_f64;
        for name in &legend.data {
            let metrics: TextMetrics = measurer.measure(name, &font_string(font_size));
            widest = widest.max(metrics.width);
        }
        let item_width: f64 = legend.item_width.unwrap_or(25.0);
        let item_gap: f64 = legend.item_gap.unwrap_or(10.0);
        let item_height: f64 = legend.item_height.unwrap_or(14.0);
        let space: f64 = (item_height.max(font_size) + 8.0).min(self.get_height() * 0.5);
        self.set_legend_rect(Some(BBox::from_min_size(
            0.0,
            0.0,
            widest + item_width + item_gap,
            space,
        )));
        space
    }

    /// Computes the layout of every grid of the chart.
    ///
    /// # Arguments
    ///
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&BBox` - Shared reference to a `BBox`.
    /// - `&TextMeasurer` - Shared reference to a `TextMeasurer`.
    ///
    /// # Returns
    ///
    /// - `Vec<GridLayout>` - A `Vec<GridLayout>` value.
    fn measure_grids(
        &self,
        option: &ChartOption,
        plot: &BBox,
        measurer: &TextMeasurer,
    ) -> Vec<GridLayout> {
        let grid_option: GridOption = option.grid.clone().unwrap_or_default();
        let mut grids: Vec<GridLayout> = Vec::new();

        // A chart without an explicit grid still gets one: the cartesian
        // series need somewhere to live.
        let count: usize = 1;
        for index in 0..count {
            let rect: BBox = self.resolve_grid_rect(&grid_option, plot);
            let mut x_axes: Vec<AxisLayout> = Vec::new();
            let mut y_axes: Vec<AxisLayout> = Vec::new();

            for (axis_index, axis) in option.x_axis.iter().enumerate() {
                if axis.grid_index.unwrap_or(0) != index {
                    continue;
                }
                x_axes.push(self.measure_x_axis(axis, axis_index, &rect, &option.series, measurer));
            }
            for (axis_index, axis) in option.y_axis.iter().enumerate() {
                if axis.grid_index.unwrap_or(0) != index {
                    continue;
                }
                y_axes.push(self.measure_y_axis(axis, axis_index, &rect, &option.series, measurer));
            }

            // Without an explicit axis, the chart falls back to a value
            // axis on the left and a category axis on the bottom.
            if x_axes.is_empty() {
                x_axes.push(self.default_x_axis(&rect, &option.series, measurer));
            }
            if y_axes.is_empty() {
                y_axes.push(self.default_y_axis(&rect, &option.series));
            }

            grids.push(GridLayout {
                index,
                rect,
                x_axes,
                y_axes,
            });
        }
        grids
    }

    /// Projects the maps of the chart into the plot area.
    ///
    /// A map is projected once, however many series draw it: the
    /// projection is the expensive part, and every series that draws
    /// the same map shares it.
    ///
    /// # Arguments
    ///
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&MapRegistry` - Shared reference to a `MapRegistry`.
    ///
    /// # Returns
    ///
    /// - `Vec<GeoLayout>` - A `Vec<GeoLayout>` value.
    fn measure_geo(&self, option: &ChartOption, maps: &MapRegistry) -> Vec<GeoLayout> {
        let mut names: Vec<(String, f64)> = Vec::new();
        let remember: fn(&mut Vec<(String, f64)>, &str, f64) =
            |names: &mut Vec<(String, f64)>, name: &str, aspect: f64| {
                if !names.iter().any(|(known, _): &(String, f64)| known == name) {
                    names.push((name.to_string(), aspect));
                }
            };
        for geo in &option.geo {
            let Some(name) = &geo.map_name else {
                continue;
            };
            remember(
                &mut names,
                name,
                geo.aspect_scale.unwrap_or(DEFAULT_ASPECT_SCALE),
            );
        }
        for series in &option.series {
            if series.kind != SeriesType::Map {
                continue;
            }
            let Some(name) = map_series_name(series, option) else {
                continue;
            };
            remember(&mut names, &name, map_series_aspect(series, option));
        }
        let mut layouts: Vec<GeoLayout> = Vec::new();
        for (name, aspect) in names {
            let Some(data) = maps.get(&name) else {
                continue;
            };
            layouts.push(GeoLayout::compute(data, self.get_plot_rect(), aspect));
        }
        layouts
    }

    /// Resolves the rectangle of the grid.
    ///
    /// # Arguments
    ///
    /// - `&GridOption` - Shared reference to a `GridOption`.
    /// - `&BBox` - Shared reference to a `BBox`.
    ///
    /// # Returns
    ///
    /// - `BBox` - A `BBox` value.
    fn resolve_grid_rect(&self, option: &GridOption, plot: &BBox) -> BBox {
        let left: f64 = resolve_edge(
            option.left.as_ref(),
            DEFAULT_GRID_LEFT * plot.width(),
            plot.width(),
        );
        let right: f64 = resolve_edge(
            option.right.as_ref(),
            DEFAULT_GRID_RIGHT * plot.width(),
            plot.width(),
        );
        let top: f64 = resolve_edge(option.top.as_ref(), DEFAULT_GRID_TOP, plot.height());
        let bottom: f64 = resolve_edge(option.bottom.as_ref(), DEFAULT_GRID_BOTTOM, plot.height());
        let min_x: f64 = plot.min_x + left;
        let min_y: f64 = plot.min_y + top;
        let max_x: f64 = plot.max_x - right;
        let max_y: f64 = plot.max_y - bottom;
        BBox::from_corners(min_x, min_y, max_x.max(min_x), max_y.max(min_y))
    }

    /// Measures one x-axis.
    ///
    /// # Arguments
    ///
    /// - `&AxisOption` - Shared reference to a `AxisOption`.
    /// - `usize` - A non-negative integer (`usize`).
    /// - `&BBox` - Shared reference to a `BBox`.
    /// - `&[SeriesOption]` - Shared reference to a `[SeriesOption]`.
    /// - `&TextMeasurer` - Shared reference to a `TextMeasurer`.
    ///
    /// # Returns
    ///
    /// - `AxisLayout` - A `AxisLayout` value.
    fn measure_x_axis(
        &self,
        axis: &AxisOption,
        index: usize,
        rect: &BBox,
        series: &[SeriesOption],
        measurer: &TextMeasurer,
    ) -> AxisLayout {
        let side: AxisSide = match axis.position {
            Some(Position::Top) => AxisSide::Top,
            _ => AxisSide::Bottom,
        };
        let offset: f64 = match side {
            AxisSide::Top => rect.min_y,
            _ => rect.max_y,
        };
        let (scale, ticks) = self.build_scale(axis, ScaleKind::Linear, rect, series, true);
        let (label_space, _, label_interval) =
            self.measure_label_space(&ticks, axis, measurer, false, rect.width());
        let name_space: f64 = if axis.name.is_some() { 20.0 } else { 0.0 };
        AxisLayout {
            index,
            kind: axis.axis_type,
            scale,
            ticks,
            side,
            offset,
            label_space,
            name: axis.name.clone(),
            name_space,
            show: axis.show,
            labels: Vec::new(),
            label_interval,
        }
    }

    /// Measures one y-axis.
    ///
    /// # Arguments
    ///
    /// - `&AxisOption` - Shared reference to a `AxisOption`.
    /// - `usize` - A non-negative integer (`usize`).
    /// - `&BBox` - Shared reference to a `BBox`.
    /// - `&[SeriesOption]` - Shared reference to a `[SeriesOption]`.
    /// - `&TextMeasurer` - Shared reference to a `TextMeasurer`.
    ///
    /// # Returns
    ///
    /// - `AxisLayout` - A `AxisLayout` value.
    fn measure_y_axis(
        &self,
        axis: &AxisOption,
        index: usize,
        rect: &BBox,
        series: &[SeriesOption],
        measurer: &TextMeasurer,
    ) -> AxisLayout {
        let side: AxisSide = match axis.position {
            Some(Position::Right) => AxisSide::Right,
            _ => AxisSide::Left,
        };
        let offset: f64 = match side {
            AxisSide::Right => rect.max_x,
            _ => rect.min_x,
        };
        let (scale, ticks) = self.build_scale(axis, ScaleKind::Linear, rect, series, false);
        let (label_space, _, label_interval) =
            self.measure_label_space(&ticks, axis, measurer, true, rect.height());
        let name_space: f64 = if axis.name.is_some() { 20.0 } else { 0.0 };
        AxisLayout {
            index,
            kind: axis.axis_type,
            scale,
            ticks,
            side,
            offset,
            label_space,
            name: axis.name.clone(),
            name_space,
            show: axis.show,
            labels: Vec::new(),
            label_interval,
        }
    }

    /// Builds the default x-axis of a chart without explicit axes.
    ///
    /// The axis behaves exactly like an explicit category axis: it falls
    /// back to the number of points when the data carry no names, and it
    /// insets the band so that the first and the last point do not sit
    /// on the edge of the grid.
    ///
    /// # Arguments
    ///
    /// - `&BBox` - Shared reference to a `BBox`.
    /// - `&[SeriesOption]` - Shared reference to a `[SeriesOption]`.
    /// - `&TextMeasurer` - Shared reference to a `TextMeasurer`.
    ///
    /// # Returns
    ///
    /// - `AxisLayout` - A `AxisLayout` value.
    fn default_x_axis(
        &self,
        rect: &BBox,
        series: &[SeriesOption],
        measurer: &TextMeasurer,
    ) -> AxisLayout {
        let categories: Vec<String> = collect_categories(series);
        let count: usize = if categories.is_empty() {
            category_count(series)
        } else {
            categories.len()
        }
        .max(1);
        let band: f64 = rect.width() / count as f64;
        let inset: f64 = band * 0.5;
        let scale: Scale = if categories.is_empty() {
            Scale::category_count(count, rect.min_x + inset, rect.max_x - inset, band)
        } else {
            Scale::category(categories, rect.min_x + inset, rect.max_x - inset, band)
        };
        let ticks: Vec<AxisTick> = category_ticks(&scale);
        // The labels of a dense axis are sampled: the interval tells the
        // renderer how many ticks it may skip between two labels.
        let default_axis: AxisOption = AxisOption::category();
        let (label_space, _, label_interval) =
            self.measure_label_space(&ticks, &default_axis, measurer, false, rect.width());
        AxisLayout {
            index: 0,
            kind: AxisType::Category,
            scale,
            ticks,
            side: AxisSide::Bottom,
            offset: rect.max_y,
            label_space,
            name: None,
            name_space: 0.0,
            show: true,
            labels: Vec::new(),
            label_interval,
        }
    }

    /// Builds the default y-axis of a chart without explicit axes.
    ///
    /// # Arguments
    ///
    /// - `&BBox` - Shared reference to a `BBox`.
    /// - `&[SeriesOption]` - Shared reference to a `[SeriesOption]`.
    ///
    /// # Returns
    ///
    /// - `AxisLayout` - A `AxisLayout` value.
    fn default_y_axis(&self, rect: &BBox, series: &[SeriesOption]) -> AxisLayout {
        let extent: (f64, f64) = value_extent(series, None);
        let (domain_min, domain_max) = nice_extent(extent.0, extent.1, DEFAULT_SPLIT_NUMBER);
        let scale: Scale = Scale::linear(domain_min, domain_max, rect.max_y, rect.min_y);
        let ticks: Vec<AxisTick> = place_ticks(
            &scale,
            value_ticks(domain_min, domain_max, DEFAULT_SPLIT_NUMBER),
        );
        AxisLayout {
            index: 0,
            kind: AxisType::Value,
            scale,
            ticks,
            side: AxisSide::Left,
            offset: rect.min_x,
            label_space: 40.0,
            name: None,
            name_space: 0.0,
            show: true,
            labels: Vec::new(),
            label_interval: 1,
        }
    }

    /// Measures the space an axis' labels need.
    ///
    /// The method returns the width and the height of the widest label,
    /// and the interval between two labels: an axis that has more than
    /// categories than fit skips the labels in between.
    ///
    /// # Arguments
    ///
    /// - `&[AxisTick]` - Shared reference to a `[AxisTick]`.
    /// - `&AxisOption` - Shared reference to a `AxisOption`.
    /// - `&TextMeasurer` - Shared reference to a `TextMeasurer`.
    /// - `bool` - A boolean (`bool`).
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `(f64, f64, usize)` - A `(f64, f64, usize)` value.
    fn measure_label_space(
        &self,
        ticks: &[AxisTick],
        axis: &AxisOption,
        measurer: &TextMeasurer,
        vertical: bool,
        extent: f64,
    ) -> (f64, f64, usize) {
        let font_size: f64 = axis
            .axis_label
            .as_ref()
            .and_then(|label: &LabelOption| label.font_size)
            .unwrap_or(DEFAULT_AXIS_LABEL_FONT_SIZE);
        let font: String = font_string(font_size);
        let mut widest: f64 = 0.0_f64;
        let mut tallest: f64 = 0.0_f64;
        for tick in ticks {
            let metrics: TextMetrics = measurer.measure(&tick.label, &font);
            widest = widest.max(metrics.width);
            tallest = tallest.max(metrics.height);
        }
        let tick_length: f64 = if axis
            .axis_tick
            .as_ref()
            .map(|tick: &AxisTickOption| tick.show)
            .unwrap_or(false)
        {
            DEFAULT_AXIS_TICK_LENGTH
        } else {
            0.0
        };
        let space: f64 =
            (if vertical { widest } else { tallest }) + tick_length + DEFAULT_AXIS_LABEL_GAP;

        // The interval between two labels: the label has to fit in the
        // space one tick leaves.
        let label_extent: f64 = if vertical { tallest } else { widest };
        let spacing: f64 = if ticks.is_empty() {
            extent
        } else {
            extent / ticks.len() as f64
        };
        let interval = if label_extent > 0.0 && spacing > 0.0 {
            ((label_extent / spacing).ceil() as usize).max(1)
        } else {
            1
        };
        (space, interval as f64, interval)
    }

    /// Builds the scale and the ticks of an axis.
    ///
    /// # Arguments
    ///
    /// - `&AxisOption` - Shared reference to a `AxisOption`.
    /// - `ScaleKind` - A `ScaleKind` parameter.
    /// - `&BBox` - Shared reference to a `BBox`.
    /// - `&[SeriesOption]` - Shared reference to a `[SeriesOption]`.
    /// - `bool` - A boolean (`bool`).
    ///
    /// # Returns
    ///
    /// - `(Scale, Vec<AxisTick>)` - A `(Scale, Vec<AxisTick>)` value.
    fn build_scale(
        &self,
        axis: &AxisOption,
        default_kind: ScaleKind,
        rect: &BBox,
        series: &[SeriesOption],
        horizontal: bool,
    ) -> (Scale, Vec<AxisTick>) {
        let (range_min, range_max) = if horizontal {
            (rect.min_x, rect.max_x)
        } else {
            (rect.max_y, rect.min_y)
        };

        let kind: ScaleKind = match axis.axis_type {
            AxisType::Category => ScaleKind::Category,
            AxisType::Value => ScaleKind::Linear,
            AxisType::Time => ScaleKind::Time,
            AxisType::Log => ScaleKind::Log,
        };
        let _: ScaleKind = default_kind;

        match kind {
            ScaleKind::Category => {
                let categories: Vec<String> = if !axis.data.is_empty() {
                    axis.data
                        .iter()
                        .map(|value: &DataValue| value.as_text().unwrap_or_default().to_string())
                        .collect()
                } else {
                    collect_categories(series)
                };
                let count: usize = if categories.is_empty() {
                    category_count(series)
                } else {
                    categories.len()
                };
                let band: f64 = (range_max - range_min).abs() / count.max(1) as f64;
                let inset: f64 = band * 0.5;
                let scale: Scale = if categories.is_empty() {
                    Scale::category_count(count, range_min + inset, range_max - inset, band)
                } else {
                    Scale::category(categories, range_min + inset, range_max - inset, band)
                };
                let ticks: Vec<AxisTick> = category_ticks(&scale);
                (scale, ticks)
            }
            ScaleKind::Log => {
                let extent: (f64, f64) = value_extent(series, Some(axis));
                let (domain_min, domain_max) =
                    nice_extent(extent.0, extent.1, DEFAULT_SPLIT_NUMBER);
                let base: f64 = axis.log_base.unwrap_or(10.0);
                let scale: Scale = Scale::log(
                    domain_min.max(f64::MIN_POSITIVE),
                    domain_max.max(f64::MIN_POSITIVE),
                    range_min,
                    range_max,
                    base,
                );
                let ticks: Vec<AxisTick> =
                    log_ticks(domain_min, domain_max, base, range_min, range_max);
                (scale, ticks)
            }
            _ => {
                let extent: (f64, f64) = value_extent(series, Some(axis));
                let mut domain_min: f64 = axis.min.unwrap_or(extent.0);
                let mut domain_max: f64 = axis.max.unwrap_or(extent.1);
                if domain_min == domain_max {
                    domain_min -= 1.0;
                    domain_max += 1.0;
                }
                // The interval is decided once, here: the ticks are
                // spaced by the very interval the bounds were snapped
                // to, so that the last tick lands on the last bound.
                let (nice_min, nice_max, interval) =
                    nice_scale(domain_min, domain_max, DEFAULT_SPLIT_NUMBER);
                if axis.min.is_none() {
                    domain_min = nice_min;
                }
                if axis.max.is_none() {
                    domain_max = nice_max;
                }
                let scale: Scale = Scale::linear(domain_min, domain_max, range_min, range_max);
                let ticks: Vec<AxisTick> =
                    place_ticks(&scale, ticks_at_interval(domain_min, domain_max, interval));
                (scale, ticks)
            }
        }
    }

    /// Computes the layout of every series.
    ///
    /// # Arguments
    ///
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&[GridLayout]` - Shared reference to a `[GridLayout]`.
    ///
    /// # Returns
    ///
    /// - `Vec<SeriesLayout>` - A `Vec<SeriesLayout>` value.
    fn measure_series(&self, option: &ChartOption, grids: &[GridLayout]) -> Vec<SeriesLayout> {
        let mut layouts: Vec<SeriesLayout> = Vec::with_capacity(option.series.len());
        for (index, series) in option.series.iter().enumerate() {
            if !series.show {
                continue;
            }
            let grid_index: usize = series
                .x_axis_index
                .or(series.y_axis_index)
                .map(|_| 0)
                .unwrap_or(0);
            let grid: Option<&GridLayout> = grids.get(grid_index);
            let (points, stack_index, stack_size, band_width) = match series.kind {
                // The non-cartesian series place themselves: all they
                // need from the layout is their value.
                SeriesType::Pie
                | SeriesType::Funnel
                | SeriesType::Radar
                | SeriesType::Gauge
                | SeriesType::Map => (self.place_values(series), 0, 1, 0.0),
                _ => match grid {
                    Some(grid) => {
                        let x_axis: Option<&AxisLayout> =
                            grid.x_axis(series.x_axis_index.unwrap_or(0));
                        let y_axis: Option<&AxisLayout> =
                            grid.y_axis(series.y_axis_index.unwrap_or(0));
                        match (x_axis, y_axis) {
                            (Some(x_axis), Some(y_axis)) => {
                                let points: Vec<SeriesPoint> =
                                    self.place_points(series, x_axis, y_axis);
                                let (stack_index, stack_size) = stack_position(option, index);
                                (points, stack_index, stack_size, x_axis.scale.band_width())
                            }
                            _ => (Vec::new(), 0, 1, 0.0),
                        }
                    }
                    None => (Vec::new(), 0, 1, 0.0),
                },
            };
            layouts.push(SeriesLayout {
                index,
                kind: series.kind,
                name: series.name.clone(),
                grid_index,
                points,
                stack: series.stack.clone(),
                stack_index,
                stack_size,
                plot_rect: self.get_plot_rect(),
                band_width,
            });
        }
        layouts
    }

    /// Places the values of a non-cartesian series.
    ///
    /// A pie, a funnel, a radar, and a gauge all decide for themselves
    /// where their values go; the layout only carries the values.    ///
    /// # Arguments
    ///
    /// - `&SeriesOption` - Shared reference to a `SeriesOption`.
    ///
    /// # Returns
    ///
    /// - `Vec<SeriesPoint>` - A `Vec<SeriesPoint>` value.
    fn place_values(&self, series: &SeriesOption) -> Vec<SeriesPoint> {
        series
            .data
            .iter()
            .enumerate()
            .map(|(data_index, value): (usize, &DataValue)| {
                let (name, number) = match value {
                    DataValue::List(items) => (
                        items
                            .first()
                            .and_then(|item: &DataValue| item.as_text())
                            .unwrap_or_default()
                            .to_string(),
                        items
                            .get(1)
                            .and_then(|item: &DataValue| item.to_number())
                            .unwrap_or(0.0),
                    ),
                    DataValue::Object(_) => (
                        value
                            .get(DATA_FIELD_NAME)
                            .and_then(|item: &DataValue| item.as_text())
                            .unwrap_or_default()
                            .to_string(),
                        value
                            .get(DATA_FIELD_VALUE)
                            .and_then(|item: &DataValue| item.to_number())
                            .unwrap_or(0.0),
                    ),
                    other => (
                        other.as_text().unwrap_or_default().to_string(),
                        other.to_number().unwrap_or(0.0),
                    ),
                };
                SeriesPoint {
                    x: 0.0,
                    y: 0.0,
                    value: number,
                    name,
                    data_index,
                }
            })
            .collect()
    }

    /// Places the data points of a cartesian series in pixel space.
    ///
    /// # Arguments
    ///
    /// - `&SeriesOption` - Shared reference to a `SeriesOption`.
    /// - `&AxisLayout` - Shared reference to a `AxisLayout`.
    ///
    /// # Returns
    ///
    /// - `Vec<SeriesPoint>` - A `Vec<SeriesPoint>` value.
    fn place_points(
        &self,
        series: &SeriesOption,
        x_axis: &AxisLayout,
        y_axis: &AxisLayout,
    ) -> Vec<SeriesPoint> {
        let mut points: Vec<SeriesPoint> = Vec::with_capacity(series.data.len());
        let category_axis: bool = x_axis.scale.is_ordinal();
        for (data_index, value) in series.data.iter().enumerate() {
            let (name, x_value, y_value) = match value {
                DataValue::List(items) => {
                    let first: Option<f64> =
                        items.first().and_then(|item: &DataValue| item.to_number());
                    let second: Option<f64> =
                        items.get(1).and_then(|item: &DataValue| item.to_number());
                    let label: String = items
                        .first()
                        .and_then(|item: &DataValue| item.as_text())
                        .map(|text: &str| text.to_string())
                        .unwrap_or_default();
                    // The list holds the category and the value, the two
                    // values, or only the value. A list whose first
                    // element is a name has no number in front of the
                    // value, and the point then belongs to the category
                    // of its own position in the data.
                    match (first, second) {
                        (Some(x), Some(y)) => (label, x, y),
                        (None, Some(y)) => (label, data_index as f64, y),
                        (Some(y), None) => (label, data_index as f64, y),
                        (None, None) => continue,
                    }
                }
                DataValue::Object(_) => {
                    let name: String = value
                        .get(DATA_FIELD_NAME)
                        .and_then(|item: &DataValue| item.as_text())
                        .unwrap_or_default()
                        .to_string();
                    // The value of a custom series is a pair: the
                    // point it is drawn at. Every other series has a
                    // single value, and takes its position from the
                    // order of the data.
                    let (x_value, y_value) = match value.get(DATA_FIELD_VALUE) {
                        Some(DataValue::List(items)) => (
                            items.first().and_then(|item: &DataValue| item.to_number()),
                            items.get(1).and_then(|item: &DataValue| item.to_number()),
                        ),
                        other => (None, other.and_then(|item: &DataValue| item.to_number())),
                    };
                    let Some(y_value) = y_value else {
                        continue;
                    };
                    let x_value: f64 = match x_value {
                        Some(x_value) => x_value,
                        None => data_index as f64,
                    };
                    (name, x_value, y_value)
                }
                other => {
                    let y: f64 = match other.to_number() {
                        Some(y) => y,
                        None => continue,
                    };
                    (String::new(), data_index as f64, y)
                }
            };
            let x: f64 = if category_axis {
                x_axis.scale.map(data_index as f64)
            } else {
                x_axis.scale.map(x_value)
            };
            let y: f64 = y_axis.scale.map(y_value);
            points.push(SeriesPoint {
                x,
                y,
                value: y_value,
                name,
                data_index,
            });
        }
        points
    }
    /// Returns the map with the given name.
    ///
    /// A chart can hold more than one map: a geo component and a map
    /// series may draw different maps, and two map series may draw the
    /// same one twice.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<&GeoLayout>` - `Some(...)` on success, `None` otherwise.
    pub fn geo(&self, name: &str) -> Option<&GeoLayout> {
        self.get_geo()
            .iter()
            .find(|layout: &&GeoLayout| layout.name == name)
    }
}

/// Inherent implementation of [`VisualMap`].
impl VisualMap {
    /// Resolves a visual map option against the values it applies to.
    ///
    /// The range of the map is the range of the values, unless the
    /// option names one. The colours are the ones the option names, or
    /// the ones ECharts falls back to.
    ///
    /// # Arguments
    ///
    /// - `&VisualMapOption` - Shared reference to a `VisualMapOption`.
    /// - `&[f64]` - Shared reference to a `[f64]`.
    pub fn resolve(option: &VisualMapOption, values: &[f64]) -> Self {
        let mut min: f64 = option.min.unwrap_or(f64::INFINITY);
        let mut max: f64 = option.max.unwrap_or(f64::NEG_INFINITY);
        for value in values {
            min = min.min(*value);
            max = max.max(*value);
        }
        if !min.is_finite() {
            min = 0.0;
        }
        if !max.is_finite() {
            max = 0.0;
        }
        if max <= min {
            // A map of a single value still has a range, or the value
            // would sit at the very edge of the palette.
            max = min + 1.0;
        }
        let colors: Vec<Color> = parse_colors(&option.colors);
        Self {
            visual_type: option.visual_type,
            min,
            max,
            colors,
            split_number: option
                .split_number
                .unwrap_or(DEFAULT_VISUAL_MAP_SPLIT)
                .max(1),
            pieces: parse_pieces(&option.pieces),
            out_of_range: option.out_of_range.as_deref().and_then(Color::parse),
        }
    }

    /// Returns the colour of a value.
    ///
    /// A continuous map mixes the colours of its palette, and a
    /// piecewise one picks the band the value falls into. The bands the
    /// option names itself win over both.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `Color` - A `Color` value.
    pub fn color_of(&self, value: f64) -> Color {
        if !self.get_pieces().is_empty() {
            for piece in self.get_pieces() {
                if value >= piece.min && value < piece.max {
                    return piece.color;
                }
            }
            if let Some(color) = self.get_out_of_range() {
                return color;
            }
        }
        if self.get_colors().is_empty() {
            return Color::rgb(191, 68, 76);
        }
        let span: f64 = self.get_max() - self.get_min();
        let position: f64 = if span.abs() < f64::EPSILON {
            0.0
        } else {
            ((value - self.get_min()) / span).clamp(0.0, 1.0)
        };
        match self.get_visual_type() {
            VisualMapType::Piecewise => {
                let bands: usize = self.get_split_number().max(1);
                let index: usize = ((position * bands as f64).floor() as usize).min(bands - 1);
                self.get_colors()[index % self.get_colors().len()]
            }
            VisualMapType::Continuous => {
                let place: f64 = position * (self.get_colors().len() - 1) as f64;
                let low: usize = place.floor() as usize;
                let high: usize = place.ceil() as usize;
                if low == high {
                    return self.get_colors()[low];
                }
                self.get_colors()[low].lerp(self.get_colors()[high], place - low as f64)
            }
        }
    }
}
