use super::*;

/// Inherent implementation of [`DataValue`].
impl DataValue {
    /// Returns the numeric value, when the value is a number.
    ///
    /// # Returns
    ///
    /// - `Option<f64>` - `Some(...)` on success, `None` otherwise.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            DataValue::Number(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the numeric value, parsing a text value when needed.
    ///
    /// # Returns
    ///
    /// - `Option<f64>` - `Some(...)` on success, `None` otherwise.
    pub fn to_number(&self) -> Option<f64> {
        match self {
            DataValue::Number(value) => Some(*value),
            DataValue::Text(text) => text.trim().parse::<f64>().ok(),
            DataValue::Bool(value) => Some(if *value { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    /// Returns the text value, when the value is text.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - `Some(...)` on success, `None` otherwise.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            DataValue::Text(text) => Some(text.as_str()),
            _ => None,
        }
    }

    /// Returns the list value, when the value is a list.
    ///
    /// # Returns
    ///
    /// - `Option<&[DataValue]>` - `Some(...)` on success, `None` otherwise.
    pub fn as_list(&self) -> Option<&[DataValue]> {
        match self {
            DataValue::List(values) => Some(values.as_slice()),
            _ => None,
        }
    }

    /// Returns the object value, when the value is an object.
    ///
    /// # Returns
    ///
    /// - `Option<&[(String, DataValue)]>` - `Some(...)` on success, `None` otherwise.
    pub fn as_object(&self) -> Option<&[(String, DataValue)]> {
        match self {
            DataValue::Object(fields) => Some(fields.as_slice()),
            _ => None,
        }
    }

    /// Looks up a named field of an object value.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<&DataValue>` - `Some(...)` on success, `None` otherwise.
    pub fn get(&self, name: &str) -> Option<&DataValue> {
        self.as_object()?
            .iter()
            .find(|(key, _): &&(String, DataValue)| key == name)
            .map(|(_, value): &(String, DataValue)| value)
    }

    /// Looks up a named field of an object value, mutably.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<&mut DataValue>` - `Some(...)` on success, `None` otherwise.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut DataValue> {
        self.as_object_mut()?
            .iter_mut()
            .find(|(key, _): &&mut (String, DataValue)| key == name)
            .map(|(_, value): &mut (String, DataValue)| value)
    }

    /// Returns the object value, mutably, when the value is an object.
    ///
    /// # Returns
    ///
    /// - `Option<&mut [(String, DataValue)]>` - `Some(...)` on success, `None` otherwise.
    pub fn as_object_mut(&mut self) -> Option<&mut [(String, DataValue)]> {
        match self {
            DataValue::Object(fields) => Some(fields.as_mut_slice()),
            _ => None,
        }
    }

    /// Returns `true` when the value carries no data.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_null(&self) -> bool {
        matches!(self, DataValue::Null)
    }
}

/// `From` conversion into [`DataValue`].
impl From<f64> for DataValue {
    /// Converts a `f64` into [`DataValue`].
    ///
    /// # Arguments
    ///
    /// - `f64` - Input value to convert from.
    fn from(value: f64) -> Self {
        DataValue::Number(value)
    }
}

/// `From` conversion into [`DataValue`].
impl From<i32> for DataValue {
    /// Converts a `i32` into [`DataValue`].
    ///
    /// # Arguments
    ///
    /// - `i32` - Input value to convert from.
    fn from(value: i32) -> Self {
        DataValue::Number(value as f64)
    }
}

/// `From` conversion into [`DataValue`].
impl From<bool> for DataValue {
    /// Converts a `bool` into [`DataValue`].
    ///
    /// # Arguments
    ///
    /// - `bool` - Input value to convert from.
    fn from(value: bool) -> Self {
        DataValue::Bool(value)
    }
}

/// `From` conversion into [`DataValue`].
impl From<&str> for DataValue {
    /// Converts a `&str` into [`DataValue`].
    ///
    /// # Arguments
    ///
    /// - `&str` - Input value to convert from.
    fn from(value: &str) -> Self {
        DataValue::Text(value.to_string())
    }
}

/// `From` conversion into [`DataValue`].
impl From<String> for DataValue {
    /// Converts a `String` into [`DataValue`].
    ///
    /// # Arguments
    ///
    /// - `String` - Input value to convert from.
    fn from(value: String) -> Self {
        DataValue::Text(value)
    }
}

/// `From` conversion into [`DataValue`].
impl From<Vec<DataValue>> for DataValue {
    /// Lifts a `Vec<DataValue>` into [`DataValue`].
    ///
    /// # Arguments
    ///
    /// - `Vec<DataValue>` - Input value to convert from.
    fn from(value: Vec<DataValue>) -> Self {
        DataValue::List(value)
    }
}

/// Inherent implementation of [`SeriesType`].
impl SeriesType {
    /// Returns the ECharts name of the series type.
    ///
    /// # Returns
    ///
    /// - `&'static str` - A `'static str` value.
    pub fn name(&self) -> &'static str {
        match self {
            SeriesType::Line => SERIES_TYPE_LINE,
            SeriesType::Bar => SERIES_TYPE_BAR,
            SeriesType::Pie => SERIES_TYPE_PIE,
            SeriesType::Scatter => SERIES_TYPE_SCATTER,
            SeriesType::EffectScatter => SERIES_TYPE_EFFECT_SCATTER,
            SeriesType::Radar => SERIES_TYPE_RADAR,
            SeriesType::Gauge => SERIES_TYPE_GAUGE,
            SeriesType::Funnel => SERIES_TYPE_FUNNEL,
            SeriesType::Heatmap => SERIES_TYPE_HEATMAP,
            SeriesType::Graph => SERIES_TYPE_GRAPH,
            SeriesType::Treemap => SERIES_TYPE_TREEMAP,
            SeriesType::Sunburst => SERIES_TYPE_SUNBURST,
            SeriesType::Boxplot => SERIES_TYPE_BOXPLOT,
            SeriesType::Candlestick => SERIES_TYPE_CANDLESTICK,
            SeriesType::ThemeRiver => SERIES_TYPE_THEME_RIVER,
            SeriesType::Sankey => SERIES_TYPE_SANKEY,
            SeriesType::Parallel => SERIES_TYPE_PARALLEL,
            SeriesType::PictorialBar => SERIES_TYPE_PICTORIAL_BAR,
            SeriesType::Lines => SERIES_TYPE_LINES,
            SeriesType::Map => SERIES_TYPE_MAP,
            SeriesType::Custom => SERIES_TYPE_CUSTOM,
        }
    }

    /// Parses an ECharts series type name.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    pub fn parse(name: &str) -> Option<Self> {
        Some(match name {
            SERIES_TYPE_LINE => SeriesType::Line,
            SERIES_TYPE_BAR => SeriesType::Bar,
            SERIES_TYPE_PIE => SeriesType::Pie,
            SERIES_TYPE_SCATTER => SeriesType::Scatter,
            SERIES_TYPE_EFFECT_SCATTER => SeriesType::EffectScatter,
            SERIES_TYPE_RADAR => SeriesType::Radar,
            SERIES_TYPE_GAUGE => SeriesType::Gauge,
            SERIES_TYPE_FUNNEL => SeriesType::Funnel,
            SERIES_TYPE_HEATMAP => SeriesType::Heatmap,
            SERIES_TYPE_GRAPH => SeriesType::Graph,
            SERIES_TYPE_TREEMAP => SeriesType::Treemap,
            SERIES_TYPE_SUNBURST => SeriesType::Sunburst,
            SERIES_TYPE_BOXPLOT => SeriesType::Boxplot,
            SERIES_TYPE_CANDLESTICK => SeriesType::Candlestick,
            SERIES_TYPE_THEME_RIVER => SeriesType::ThemeRiver,
            SERIES_TYPE_SANKEY => SeriesType::Sankey,
            SERIES_TYPE_PARALLEL => SeriesType::Parallel,
            SERIES_TYPE_PICTORIAL_BAR => SeriesType::PictorialBar,
            SERIES_TYPE_LINES => SeriesType::Lines,
            SERIES_TYPE_MAP => SeriesType::Map,
            SERIES_TYPE_CUSTOM => SeriesType::Custom,
            _ => return None,
        })
    }
}

/// Inherent implementation of [`ChartOption`].
impl ChartOption {
    /// Returns an empty configuration.
    pub fn new() -> Self {
        Self {
            animation: true,
            ..Self::default()
        }
    }

    /// Sets the title.
    ///
    /// # Arguments
    ///
    /// - `TitleOption` - A `TitleOption` parameter.
    pub fn title(mut self, title: TitleOption) -> Self {
        self.set_title(Some(title));
        self
    }

    /// Sets the legend.
    ///
    /// # Arguments
    ///
    /// - `LegendOption` - A `LegendOption` parameter.
    pub fn legend(mut self, legend: LegendOption) -> Self {
        self.set_legend(Some(legend));
        self
    }

    /// Sets the cartesian grid.
    ///
    /// # Arguments
    ///
    /// - `GridOption` - A `GridOption` parameter.
    pub fn grid(mut self, grid: GridOption) -> Self {
        self.set_grid(Some(grid));
        self
    }

    /// Adds an x-axis.
    ///
    /// # Arguments
    ///
    /// - `AxisOption` - A `AxisOption` parameter.
    pub fn x_axis(mut self, axis: AxisOption) -> Self {
        self.get_mut_x_axis().push(axis);
        self
    }

    /// Adds a y-axis.
    ///
    /// # Arguments
    ///
    /// - `AxisOption` - A `AxisOption` parameter.
    pub fn y_axis(mut self, axis: AxisOption) -> Self {
        self.get_mut_y_axis().push(axis);
        self
    }

    /// Adds a polar coordinate system.
    ///
    /// # Arguments
    ///
    /// - `PolarOption` - A `PolarOption` parameter.
    pub fn polar(mut self, polar: PolarOption) -> Self {
        self.get_mut_polar().push(polar);
        self
    }

    /// Adds a radar coordinate system.
    ///
    /// # Arguments
    ///
    /// - `RadarOption` - A `RadarOption` parameter.
    pub fn radar(mut self, radar: RadarOption) -> Self {
        self.get_mut_radar().push(radar);
        self
    }

    /// Adds a geo component.
    ///
    /// # Arguments
    ///
    /// - `GeoOption` - A `GeoOption` parameter.
    pub fn geo(mut self, geo: GeoOption) -> Self {
        self.get_mut_geo().push(geo);
        self
    }

    /// Adds a calendar component.
    ///
    /// # Arguments
    ///
    /// - `CalendarOption` - A `CalendarOption` parameter.
    pub fn calendar(mut self, calendar: CalendarOption) -> Self {
        self.get_mut_calendar().push(calendar);
        self
    }

    /// Sets the dataset.
    ///
    /// # Arguments
    ///
    /// - `DatasetOption` - A `DatasetOption` parameter.
    pub fn dataset(mut self, dataset: DatasetOption) -> Self {
        self.set_dataset(Some(dataset));
        self
    }

    /// Adds a series.
    ///
    /// # Arguments
    ///
    /// - `SeriesOption` - A `SeriesOption` parameter.
    pub fn series(mut self, series: SeriesOption) -> Self {
        self.get_mut_series().push(series);
        self
    }

    /// Sets the tooltip.
    ///
    /// # Arguments
    ///
    /// - `TooltipOption` - A `TooltipOption` parameter.
    pub fn tooltip(mut self, tooltip: TooltipOption) -> Self {
        self.set_tooltip(Some(tooltip));
        self
    }

    /// Sets the toolbox.
    ///
    /// # Arguments
    ///
    /// - `ToolboxOption` - A `ToolboxOption` parameter.
    pub fn toolbox(mut self, toolbox: ToolboxOption) -> Self {
        self.set_toolbox(Some(toolbox));
        self
    }

    /// Adds a data-zoom control.
    ///
    /// # Arguments
    ///
    /// - `DataZoomOption` - A `DataZoomOption` parameter.
    pub fn data_zoom(mut self, data_zoom: DataZoomOption) -> Self {
        self.get_mut_data_zoom().push(data_zoom);
        self
    }

    /// Adds a visual map.
    ///
    /// # Arguments
    ///
    /// - `VisualMapOption` - A `VisualMapOption` parameter.
    pub fn visual_map(mut self, visual_map: VisualMapOption) -> Self {
        self.get_mut_visual_map().push(visual_map);
        self
    }

    /// Adds a graphic element.
    ///
    /// # Arguments
    ///
    /// - `GraphicOption` - A `GraphicOption` parameter.
    pub fn graphic(mut self, graphic: GraphicOption) -> Self {
        self.get_mut_graphic().push(graphic);
        self
    }

    /// Sets the brush.
    ///
    /// # Arguments
    ///
    /// - `BrushOption` - A `BrushOption` parameter.
    pub fn brush(mut self, brush: BrushOption) -> Self {
        self.set_brush(Some(brush));
        self
    }

    /// Sets the axis pointer.
    ///
    /// # Arguments
    ///
    /// - `AxisPointerOption` - A `AxisPointerOption` parameter.
    pub fn axis_pointer(mut self, axis_pointer: AxisPointerOption) -> Self {
        self.set_axis_pointer(Some(axis_pointer));
        self
    }

    /// Sets the palette the series take their colours from.
    ///
    /// # Arguments
    ///
    /// - `Vec<String>` - A `Vec<String>` parameter.
    pub fn color(mut self, color: Vec<String>) -> Self {
        self.set_color(color);
        self
    }

    /// Adds one colour to the palette.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub fn add_color(mut self, color: &str) -> Self {
        self.get_mut_color().push(color.to_string());
        self
    }

    /// Sets the background colour of the chart.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub fn background_color(mut self, color: &str) -> Self {
        self.set_background_color(Some(color.to_string()));
        self
    }

    /// Sets the default text style of the chart.
    ///
    /// # Arguments
    ///
    /// - `LabelOption` - A `LabelOption` parameter.
    pub fn text_style(mut self, text_style: LabelOption) -> Self {
        self.set_text_style(Some(text_style));
        self
    }

    /// Sets whether the chart animates.
    ///
    /// # Arguments
    ///
    /// - `bool` - A boolean (`bool`).
    pub fn animation(mut self, animation: bool) -> Self {
        self.set_animation(animation);
        self
    }

    /// Sets the duration of the chart's animation, in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn animation_duration(mut self, duration: f64) -> Self {
        self.set_animation_duration(Some(duration));
        self
    }
}

/// Inherent implementation of [`SeriesOption`].
impl SeriesOption {
    /// Returns a line series.
    pub fn line() -> Self {
        Self {
            kind: SeriesType::Line,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a bar series.
    pub fn bar() -> Self {
        Self {
            kind: SeriesType::Bar,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a pie series.
    pub fn pie() -> Self {
        Self {
            kind: SeriesType::Pie,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a scatter series.
    pub fn scatter() -> Self {
        Self {
            kind: SeriesType::Scatter,
            show: true,
            ..Self::default()
        }
    }

    /// Returns an effect-scatter series.
    pub fn effect_scatter() -> Self {
        Self {
            kind: SeriesType::EffectScatter,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a radar series.
    pub fn radar() -> Self {
        Self {
            kind: SeriesType::Radar,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a gauge series.
    pub fn gauge() -> Self {
        Self {
            kind: SeriesType::Gauge,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a funnel series.
    pub fn funnel() -> Self {
        Self {
            kind: SeriesType::Funnel,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a heatmap series.
    pub fn heatmap() -> Self {
        Self {
            kind: SeriesType::Heatmap,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a graph series.
    pub fn graph() -> Self {
        Self {
            kind: SeriesType::Graph,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a treemap series.
    pub fn treemap() -> Self {
        Self {
            kind: SeriesType::Treemap,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a sunburst series.
    pub fn sunburst() -> Self {
        Self {
            kind: SeriesType::Sunburst,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a boxplot series.
    pub fn boxplot() -> Self {
        Self {
            kind: SeriesType::Boxplot,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a candlestick series.
    pub fn candlestick() -> Self {
        Self {
            kind: SeriesType::Candlestick,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a theme-river series.
    pub fn theme_river() -> Self {
        Self {
            kind: SeriesType::ThemeRiver,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a sankey series.
    pub fn sankey() -> Self {
        Self {
            kind: SeriesType::Sankey,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a parallel-coordinates series.
    pub fn parallel() -> Self {
        Self {
            kind: SeriesType::Parallel,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a pictorial-bar series.
    pub fn pictorial_bar() -> Self {
        Self {
            kind: SeriesType::PictorialBar,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a lines series.
    pub fn lines() -> Self {
        Self {
            kind: SeriesType::Lines,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a map series.
    pub fn map() -> Self {
        Self {
            kind: SeriesType::Map,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a custom series.
    pub fn custom() -> Self {
        Self {
            kind: SeriesType::Custom,
            show: true,
            ..Self::default()
        }
    }

    /// Sets the name of the series.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub fn name(mut self, name: &str) -> Self {
        self.set_name(name.to_string());
        self
    }

    /// Sets the data of the series.
    ///
    /// # Arguments
    ///
    /// - `Vec<DataValue>` - A `Vec<DataValue>` parameter.
    pub fn data(mut self, data: Vec<DataValue>) -> Self {
        self.set_data(data);
        self
    }

    /// Adds one data point to the series.
    ///
    /// The method borrows the series instead of consuming it, so that a
    /// long series can be built in a loop without moving the data
    /// around on every step.
    ///
    /// # Arguments
    ///
    /// - `DataValue` - A `DataValue` parameter.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn add_data(&mut self, value: DataValue) -> &mut Self {
        self.get_mut_data().push(value);
        self
    }

    /// Sets the stack the series is stacked in.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub fn stack(mut self, stack: &str) -> Self {
        self.set_stack(Some(stack.to_string()));
        self
    }

    /// Binds the series to an x-axis.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    pub fn x_axis_index(mut self, index: usize) -> Self {
        self.set_x_axis_index(Some(index));
        self
    }

    /// Binds the series to a y-axis.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    pub fn y_axis_index(mut self, index: usize) -> Self {
        self.set_y_axis_index(Some(index));
        self
    }

    /// Sets the fill of the series' shapes.
    ///
    /// # Arguments
    ///
    /// - `ItemStyleOption` - A `ItemStyleOption` parameter.
    pub fn item_style(mut self, item_style: ItemStyleOption) -> Self {
        self.set_item_style(Some(item_style));
        self
    }

    /// Sets the stroke of the series' shapes.
    ///
    /// # Arguments
    ///
    /// - `LineStyleOption` - A `LineStyleOption` parameter.
    pub fn line_style(mut self, line_style: LineStyleOption) -> Self {
        self.set_line_style(Some(line_style));
        self
    }

    /// Sets the label of the series' data points.
    ///
    /// # Arguments
    ///
    /// - `LabelOption` - A `LabelOption` parameter.
    pub fn label(mut self, label: LabelOption) -> Self {
        self.set_label(Some(label));
        self
    }

    /// Sets the style applied when a data point is highlighted.
    ///
    /// # Arguments
    ///
    /// - `EmphasisOption` - A `EmphasisOption` parameter.
    pub fn emphasis(mut self, emphasis: EmphasisOption) -> Self {
        self.set_emphasis(Some(emphasis));
        self
    }

    /// Sets the options specific to a bar series.
    ///
    /// # Arguments
    ///
    /// - `BarOptions` - A `BarOptions` parameter.
    pub fn bar_options(mut self, options: BarOptions) -> Self {
        self.set_bar(Some(options));
        self
    }

    /// Sets the options specific to a line series.
    ///
    /// # Arguments
    ///
    /// - `LineOptions` - A `LineOptions` parameter.
    pub fn line_options(mut self, options: LineOptions) -> Self {
        self.set_line(Some(options));
        self
    }

    /// Sets the options specific to a pie series.
    ///
    /// # Arguments
    ///
    /// - `PieOptions` - A `PieOptions` parameter.
    pub fn pie_options(mut self, options: PieOptions) -> Self {
        self.set_pie(Some(options));
        self
    }

    /// Sets the options specific to a scatter series.
    ///
    /// # Arguments
    ///
    /// - `ScatterOptions` - A `ScatterOptions` parameter.
    pub fn scatter_options(mut self, options: ScatterOptions) -> Self {
        self.set_scatter(Some(options));
        self
    }

    /// Sets the options specific to a radar series.
    ///
    /// # Arguments
    ///
    /// - `RadarOptions` - A `RadarOptions` parameter.
    pub fn radar_options(mut self, options: RadarOptions) -> Self {
        self.set_radar(Some(options));
        self
    }

    /// Sets the options specific to a gauge series.
    ///
    /// # Arguments
    ///
    /// - `GaugeOptions` - A `GaugeOptions` parameter.
    pub fn gauge_options(mut self, options: GaugeOptions) -> Self {
        self.set_gauge(Some(options));
        self
    }

    /// Sets the options specific to a funnel series.
    ///
    /// # Arguments
    ///
    /// - `FunnelOptions` - A `FunnelOptions` parameter.
    pub fn funnel_options(mut self, options: FunnelOptions) -> Self {
        self.set_funnel(Some(options));
        self
    }

    /// Sets the options specific to a heatmap series.
    ///
    /// # Arguments
    ///
    /// - `HeatmapOptions` - A `HeatmapOptions` parameter.
    pub fn heatmap_options(mut self, options: HeatmapOptions) -> Self {
        self.set_heatmap(Some(options));
        self
    }

    /// Returns `true` when the series draws a line between its points.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_line_like(&self) -> bool {
        matches!(
            self.get_kind(),
            SeriesType::Line | SeriesType::EffectScatter | SeriesType::ThemeRiver
        )
    }

    /// Returns `true` when the series draws an area under its points.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn has_area(&self) -> bool {
        self.get_line()
            .as_ref()
            .and_then(|line: &LineOptions| line.area_style.as_ref())
            .is_some()
    }
}

/// Inherent implementation of [`AxisOption`].
impl AxisOption {
    /// Returns a category axis.
    pub fn category() -> Self {
        Self {
            axis_type: AxisType::Category,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a value axis.
    pub fn value() -> Self {
        Self {
            axis_type: AxisType::Value,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a time axis.
    pub fn time() -> Self {
        Self {
            axis_type: AxisType::Time,
            show: true,
            ..Self::default()
        }
    }

    /// Returns a logarithmic axis.
    pub fn log() -> Self {
        Self {
            axis_type: AxisType::Log,
            show: true,
            ..Self::default()
        }
    }

    /// Sets the categories or the tick values of the axis.
    ///
    /// # Arguments
    ///
    /// - `Vec<DataValue>` - A `Vec<DataValue>` parameter.
    pub fn data(mut self, data: Vec<DataValue>) -> Self {
        self.set_data(data);
        self
    }

    /// Sets the name of the axis.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub fn name(mut self, name: &str) -> Self {
        self.set_name(Some(name.to_string()));
        self
    }

    /// Sets the labels of the axis.
    ///
    /// # Arguments
    ///
    /// - `LabelOption` - A `LabelOption` parameter.
    pub fn axis_label(mut self, label: LabelOption) -> Self {
        self.set_axis_label(Some(label));
        self
    }

    /// Sets the line of the axis.
    ///
    /// # Arguments
    ///
    /// - `AxisLineOption` - A `AxisLineOption` parameter.
    pub fn axis_line(mut self, line: AxisLineOption) -> Self {
        self.set_axis_line(Some(line));
        self
    }

    /// Sets the tick marks of the axis.
    ///
    /// # Arguments
    ///
    /// - `AxisTickOption` - A `AxisTickOption` parameter.
    pub fn axis_tick(mut self, tick: AxisTickOption) -> Self {
        self.set_axis_tick(Some(tick));
        self
    }

    /// Sets the split lines of the axis.
    ///
    /// # Arguments
    ///
    /// - `SplitLineOption` - A `SplitLineOption` parameter.
    pub fn split_line(mut self, line: SplitLineOption) -> Self {
        self.set_split_line(Some(line));
        self
    }

    /// Sets the minimum value of the axis.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn min(mut self, min: f64) -> Self {
        self.set_min(Some(min));
        self
    }

    /// Sets the maximum value of the axis.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn max(mut self, max: f64) -> Self {
        self.set_max(Some(max));
        self
    }
}

/// Inherent implementation of [`TitleOption`].
impl TitleOption {
    /// Returns a title with the given text.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            show: true,
            ..Self::default()
        }
    }

    /// Sets the subtitle.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub fn subtext(mut self, subtext: &str) -> Self {
        self.set_subtext(subtext.to_string());
        self
    }
}

/// Inherent implementation of [`LegendOption`].
impl LegendOption {
    /// Returns a legend showing the given series names.
    ///
    /// # Arguments
    ///
    /// - `Vec<String>` - A `Vec<String>` parameter.
    pub fn new(data: Vec<String>) -> Self {
        Self {
            data,
            show: true,
            ..Self::default()
        }
    }
}

/// Inherent implementation of [`GridOption`].
impl GridOption {
    /// Returns a visible grid.
    pub fn new() -> Self {
        Self {
            show: true,
            ..Self::default()
        }
    }
}

/// Inherent implementation of [`TooltipOption`].
impl TooltipOption {
    /// Returns a visible tooltip.
    pub fn new() -> Self {
        Self {
            show: true,
            ..Self::default()
        }
    }

    /// Sets what the tooltip describes.
    ///
    /// # Arguments
    ///
    /// - `TriggerType` - A `TriggerType` parameter.
    pub fn trigger(mut self, trigger: TriggerType) -> Self {
        self.set_trigger(trigger);
        self
    }
}

/// Inherent implementation of [`LabelOption`].
impl LabelOption {
    /// Returns a visible label.
    pub fn new() -> Self {
        Self {
            show: true,
            ..Self::default()
        }
    }

    /// Sets the colour of the label.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub fn color(mut self, color: &str) -> Self {
        self.set_color(Some(color.to_string()));
        self
    }

    /// Sets the font size of the label.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn font_size(mut self, size: f64) -> Self {
        self.set_font_size(Some(size));
        self
    }

    /// Sets the font family of the label.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub fn font_family(mut self, family: &str) -> Self {
        self.set_font_family(Some(family.to_string()));
        self
    }
}

/// Inherent implementation of [`ItemStyleOption`].
impl ItemStyleOption {
    /// Returns a style filled with the given colour.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub fn filled(color: &str) -> Self {
        Self {
            color: Some(color.to_string()),
            ..Self::default()
        }
    }
}

/// Inherent implementation of [`LineStyleOption`].
impl LineStyleOption {
    /// Returns a line style of the given colour and width.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `f64` - A 64-bit float (`f64`).
    pub fn new(color: &str, width: f64) -> Self {
        Self {
            color: Some(color.to_string()),
            width: Some(width),
            ..Self::default()
        }
    }
}
