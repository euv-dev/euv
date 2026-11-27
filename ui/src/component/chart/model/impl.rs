use super::*;

/// Default legend configuration positioned at the top edge.
impl Default for EuvChartLegend {
    /// Creates a visible top-docked legend.
    ///
    /// # Returns
    ///
    /// - `Self` - Default legend configuration.
    fn default() -> Self {
        Self {
            show: true,
            position: EuvChartPosition::Top,
            items: Vec::new(),
        }
    }
}

/// Default Cartesian grid margins and label containment.
impl Default for EuvChartGrid {
    /// Creates a standard padded Cartesian grid.
    ///
    /// # Returns
    ///
    /// - `Self` - Default grid margins.
    fn default() -> Self {
        Self {
            left: ECHART_GRID_DEFAULT_LEFT,
            right: ECHART_GRID_DEFAULT_RIGHT,
            top: ECHART_GRID_DEFAULT_TOP,
            bottom: ECHART_GRID_DEFAULT_BOTTOM,
            contain_label: true,
            show_border: false,
        }
    }
}

/// Default continuous numeric value axis configuration.
impl Default for EuvChartAxis {
    /// Creates a default `Value` axis with ticks, labels, and subtle grid split lines.
    ///
    /// # Returns
    ///
    /// - `Self` - Default axis configuration.
    fn default() -> Self {
        Self {
            axis_type: EuvChartAxisType::Value,
            name: String::new(),
            unit_suffix: String::new(),
            categories: Vec::new(),
            grid_index: 0,
            min: None,
            max: None,
            inverse: false,
            split_number: ECHART_DEFAULT_SPLIT_NUMBER,
            show_axis_line: true,
            show_axis_tick: true,
            show_axis_label: true,
            show_split_line: true,
            show_split_area: false,
            position: None,
        }
    }
}

/// Default polar coordinate system.
impl Default for EuvChartPolar {
    /// Creates a centered polar coordinate system with category angle axis and value radius axis.
    ///
    /// # Returns
    ///
    /// - `Self` - Default polar configuration.
    fn default() -> Self {
        Self {
            angle_axis: EuvChartAxis::category(Vec::new()),
            radius_axis: EuvChartAxis::value(),
            radius_ratio: 0.72,
            center_x_ratio: 0.5,
            center_y_ratio: 0.5,
        }
    }
}

/// Default radar coordinate system.
impl Default for EuvChartRadar {
    /// Creates a polygon radar web with four concentric rings.
    ///
    /// # Returns
    ///
    /// - `Self` - Default radar configuration.
    fn default() -> Self {
        Self {
            indicators: Vec::new(),
            shape: EuvChartRadarShape::Polygon,
            split_number: 4,
            radius_ratio: 0.72,
        }
    }
}

/// Default single-axis coordinate system.
impl Default for EuvChartSingleAxis {
    /// Creates a centered horizontal single-axis track.
    ///
    /// # Returns
    ///
    /// - `Self` - Default single-axis configuration.
    fn default() -> Self {
        Self {
            axis: EuvChartAxis::category(Vec::new()),
            center_y_ratio: 0.54,
        }
    }
}

/// Default calendar coordinate system.
impl Default for EuvChartCalendar {
    /// Creates a horizontal 26-week calendar matrix.
    ///
    /// # Returns
    ///
    /// - `Self` - Default calendar configuration.
    fn default() -> Self {
        Self {
            year: 2026,
            weeks: ECHART_CALENDAR_DEFAULT_WEEKS,
            month_labels: Vec::new(),
            cell_gap: 3.0,
            orient: EuvChartOrient::Horizontal,
        }
    }
}

/// Default geographic coordinate system.
impl Default for EuvChartGeo {
    /// Creates a built-in stylized world map coordinate system.
    ///
    /// # Returns
    ///
    /// - `Self` - Default geo configuration.
    fn default() -> Self {
        Self {
            preset: EuvChartGeoMapPreset::World,
            regions: Vec::new(),
            show_labels: true,
            roam: false,
        }
    }
}

/// Default interactive hover tooltip configuration.
impl Default for EuvChartTooltip {
    /// Creates an axis-triggered tooltip with a vertical hairline crosshair.
    ///
    /// # Returns
    ///
    /// - `Self` - Default tooltip configuration.
    fn default() -> Self {
        Self {
            show: true,
            trigger: EuvChartTooltipTrigger::Axis,
            axis_pointer: EuvChartAxisPointerType::Line,
        }
    }
}

/// Default interactive data-zoom slider configuration.
impl Default for EuvChartDataZoom {
    /// Creates a full-range slider data-zoom controller spanning `0%..=100%`.
    ///
    /// # Returns
    ///
    /// - `Self` - Default data-zoom controller.
    fn default() -> Self {
        Self {
            kind: EuvChartDataZoomKind::Slider,
            start_pct: 0.0,
            end_pct: 100.0,
            min_span_pct: ECHART_ZOOM_DEFAULT_MIN_SPAN,
            show_preview: true,
        }
    }
}

/// Default visual-map gradient configuration.
impl Default for EuvChartVisualMap {
    /// Creates a visible continuous visual-map spanning `0.0..=100.0`.
    ///
    /// # Returns
    ///
    /// - `Self` - Default visual-map configuration.
    fn default() -> Self {
        Self {
            show: true,
            kind: EuvChartVisualMapKind::Continuous,
            min: 0.0,
            max: 100.0,
            in_range_colors: Vec::new(),
            pieces: Vec::new(),
            orient: EuvChartOrient::Horizontal,
            position: EuvChartPosition::Bottom,
        }
    }
}

/// Default compact icon-only toolbox configuration.
impl Default for EuvChartToolbox {
    /// Creates a toolbox with Restore, MagicType, DataView, and Fullscreen icons enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Default toolbox configuration.
    fn default() -> Self {
        Self {
            show: true,
            restore: true,
            magic_type: true,
            data_view: true,
            fullscreen: true,
        }
    }
}

/// Default line series configuration.
impl Default for EuvChartLineSeries {
    /// Creates a crisp solid line series with circular point markers.
    ///
    /// # Returns
    ///
    /// - `Self` - Default line series.
    fn default() -> Self {
        Self {
            values: Vec::new(),
            xy_points: Vec::new(),
            smooth: false,
            step: EuvChartStepMode::None,
            area_opacity: 0.0,
            area_gradient: true,
            stack: None,
            dashed: false,
            line_width: 2.2,
            symbol: EuvChartSymbol::Circle,
            symbol_size: 3.5,
            connect_nulls: false,
            show_labels: false,
            coordinate_system: EuvChartCoordinateSystem::Cartesian2d,
            x_axis_index: 0,
            y_axis_index: 0,
            annotations: EuvChartAnnotations::default(),
            encode: None,
        }
    }
}

/// Default bar series configuration.
impl Default for EuvChartBarSeries {
    /// Creates a vertical grouped bar series with gently rounded caps.
    ///
    /// # Returns
    ///
    /// - `Self` - Default bar series.
    fn default() -> Self {
        Self {
            values: Vec::new(),
            stack: None,
            bar_radius: 4.0,
            bar_width_ratio: 0.62,
            show_background: false,
            show_labels: false,
            coordinate_system: EuvChartCoordinateSystem::Cartesian2d,
            x_axis_index: 0,
            y_axis_index: 0,
            annotations: EuvChartAnnotations::default(),
            encode: None,
        }
    }
}

/// Default pie series configuration.
impl Default for EuvChartPieSeries {
    /// Creates a standard pie chart with external leader lines and labels.
    ///
    /// # Returns
    ///
    /// - `Self` - Default pie series.
    fn default() -> Self {
        Self {
            slices: Vec::new(),
            inner_radius_ratio: 0.0,
            outer_radius_ratio: ECHART_PIE_DEFAULT_OUTER_RATIO,
            rose_type: EuvChartRoseType::None,
            pad_angle_deg: 1.5,
            start_angle_deg: ECHART_PIE_DEFAULT_START_ANGLE,
            show_labels: true,
            show_leader_lines: true,
            selected_offset: ECHART_PIE_DEFAULT_POPOUT,
            center_x_ratio: 0.5,
            center_y_ratio: 0.52,
        }
    }
}

/// Default scatter series configuration.
impl Default for EuvChartScatterSeries {
    /// Creates a Cartesian circular scatter series.
    ///
    /// # Returns
    ///
    /// - `Self` - Default scatter series.
    fn default() -> Self {
        Self {
            points: Vec::new(),
            symbol: EuvChartSymbol::Circle,
            default_size: 6.0,
            ripple_rings: 3,
            ripple_scale: 2.6,
            show_labels: false,
            coordinate_system: EuvChartCoordinateSystem::Cartesian2d,
            x_axis_index: 0,
            y_axis_index: 0,
            annotations: EuvChartAnnotations::default(),
            encode: None,
        }
    }
}

/// Default radar series configuration.
impl Default for EuvChartRadarSeries {
    /// Creates a semi-transparent filled radar polygon series with vertex dots.
    ///
    /// # Returns
    ///
    /// - `Self` - Default radar series.
    fn default() -> Self {
        Self {
            items: Vec::new(),
            area_opacity: 0.24,
            line_width: 2.0,
            show_vertices: true,
        }
    }
}

/// Default candlestick series configuration.
impl Default for EuvChartCandlestickSeries {
    /// Creates an empty OHLC candlestick series on axis 0.
    ///
    /// # Returns
    ///
    /// - `Self` - Default candlestick series.
    fn default() -> Self {
        Self {
            candles: Vec::new(),
            bull_color: None,
            bear_color: None,
            x_axis_index: 0,
            y_axis_index: 0,
            annotations: EuvChartAnnotations::default(),
        }
    }
}

/// Default boxplot series configuration.
impl Default for EuvChartBoxplotSeries {
    /// Creates an empty five-number boxplot series on axis 0.
    ///
    /// # Returns
    ///
    /// - `Self` - Default boxplot series.
    fn default() -> Self {
        Self {
            items: Vec::new(),
            x_axis_index: 0,
            y_axis_index: 0,
            annotations: EuvChartAnnotations::default(),
        }
    }
}

/// Default heatmap series configuration.
impl Default for EuvChartHeatmapSeries {
    /// Creates a Cartesian matrix heatmap with cell labels enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Default heatmap series.
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            show_labels: true,
            cell_gap: 2.0,
            coordinate_system: EuvChartCoordinateSystem::Cartesian2d,
            x_axis_index: 0,
            y_axis_index: 0,
        }
    }
}

/// Default treemap series configuration.
impl Default for EuvChartTreemapSeries {
    /// Creates a squarified treemap with group headers and subtle rounded corners.
    ///
    /// # Returns
    ///
    /// - `Self` - Default treemap series.
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            show_headers: true,
            cell_gap: 3.0,
            corner_radius: 4.0,
        }
    }
}

/// Default sunburst series configuration.
impl Default for EuvChartSunburstSeries {
    /// Creates a concentric hierarchical sunburst series.
    ///
    /// # Returns
    ///
    /// - `Self` - Default sunburst series.
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            inner_radius_ratio: 0.16,
            outer_radius_ratio: 0.84,
            pad_angle_deg: 1.0,
            show_labels: true,
        }
    }
}

/// Default Sankey flow diagram configuration.
impl Default for EuvChartSankeySeries {
    /// Creates a layered Sankey series with translucent flow ribbons.
    ///
    /// # Returns
    ///
    /// - `Self` - Default Sankey series.
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            node_width: 14.0,
            node_gap: 10.0,
            ribbon_opacity: 0.36,
        }
    }
}

/// Default conversion funnel configuration.
impl Default for EuvChartFunnelSeries {
    /// Creates a descending conversion funnel with percentage labels.
    ///
    /// # Returns
    ///
    /// - `Self` - Default funnel series.
    fn default() -> Self {
        Self {
            stages: Vec::new(),
            sort: EuvChartFunnelSort::Descending,
            gap: 3.0,
            show_labels: true,
            show_percent: true,
        }
    }
}

/// Default circular dial gauge configuration.
impl Default for EuvChartGaugeSeries {
    /// Creates a 270-degree dial gauge spanning `0..=100` with progress arc and pointer needle.
    ///
    /// # Returns
    ///
    /// - `Self` - Default gauge series.
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            unit: String::new(),
            title: String::new(),
            start_angle_deg: ECHART_GAUGE_DEFAULT_START_ANGLE,
            end_angle_deg: ECHART_GAUGE_DEFAULT_END_ANGLE,
            split_number: 5,
            show_progress: true,
            show_pointer: true,
            segments: vec![
                EuvChartGaugeSegment::new(0.35, Color::hex(ECHART_GAUGE_LOW_HEX)),
                EuvChartGaugeSegment::new(0.75, Color::hex(ECHART_GAUGE_MID_HEX)),
                EuvChartGaugeSegment::new(1.0, Color::hex(ECHART_GAUGE_HIGH_HEX)),
            ],
        }
    }
}

/// Default relational network graph configuration.
impl Default for EuvChartGraphSeries {
    /// Creates a force-directed graph series with labels enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Default graph series.
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            categories: Vec::new(),
            layout: EuvChartGraphLayout::Force,
            directed: true,
            show_labels: true,
            iterations: ECHART_GRAPH_DEFAULT_ITERS,
        }
    }
}

/// Default hierarchical tree series configuration.
impl Default for EuvChartTreeSeries {
    /// Creates a left-to-right orthogonal tree series.
    ///
    /// # Returns
    ///
    /// - `Self` - Default tree series.
    fn default() -> Self {
        Self {
            root: EuvChartHierarchyNode::leaf(String::new(), 1.0),
            layout: EuvChartTreeLayout::OrthogonalLeftRight,
            symbol_size: 5.5,
            show_labels: true,
        }
    }
}

/// Default parallel coordinates series configuration.
impl Default for EuvChartParallelSeries {
    /// Creates a parallel coordinates series with semi-transparent polylines.
    ///
    /// # Returns
    ///
    /// - `Self` - Default parallel series.
    fn default() -> Self {
        Self {
            axes: Vec::new(),
            rows: Vec::new(),
            line_width: 1.8,
            line_opacity: 0.72,
        }
    }
}

/// Default curved directional flow arcs configuration.
impl Default for EuvChartLinesSeries {
    /// Creates a Cartesian curved lines series with arrowheads enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Default lines series.
    fn default() -> Self {
        Self {
            arcs: Vec::new(),
            curveness: 0.22,
            line_width: 2.0,
            show_arrows: true,
            coordinate_system: EuvChartCoordinateSystem::Cartesian2d,
            x_axis_index: 0,
            y_axis_index: 0,
        }
    }
}

/// Default ThemeRiver streamgraph configuration.
impl Default for EuvChartThemeRiverSeries {
    /// Creates a smooth centered ThemeRiver streamgraph with labels enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Default ThemeRiver series.
    fn default() -> Self {
        Self {
            points: Vec::new(),
            show_labels: true,
            smooth: true,
        }
    }
}

/// Default pictorial bar series configuration.
impl Default for EuvChartPictorialBarSeries {
    /// Creates a repeated rounded-capsule pictorial bar series.
    ///
    /// # Returns
    ///
    /// - `Self` - Default pictorial bar series.
    fn default() -> Self {
        Self {
            values: Vec::new(),
            symbol: EuvChartSymbol::RoundRect,
            symbol_repeat: true,
            symbol_size: 10.0,
            x_axis_index: 0,
            y_axis_index: 0,
        }
    }
}

/// Default custom series configuration.
impl Default for EuvChartCustomSeries {
    /// Creates an empty custom painter series on axis 0.
    ///
    /// # Returns
    ///
    /// - `Self` - Default custom series.
    fn default() -> Self {
        Self {
            values: Vec::new(),
            painter: None,
            x_axis_index: 0,
            y_axis_index: 0,
        }
    }
}

/// Default geographic choropleth map series configuration.
impl Default for EuvChartGeoMapSeries {
    /// Creates a GeoMap series with labels and crisp borders enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Default GeoMap series.
    fn default() -> Self {
        Self {
            data: Vec::new(),
            show_labels: true,
            border_width: 1.0,
        }
    }
}

/// Constructors and fluent builders for `EuvChartTitle`.
impl EuvChartTitle {
    /// Constructs a chart title with left alignment.
    ///
    /// # Arguments
    ///
    /// - `S` - Primary chart title string.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed title block.
    #[must_use]
    pub fn new<S>(text: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            text: text.into(),
            subtext: String::new(),
            align: EuvChartAlign::Left,
        }
    }

    /// Sets an optional subtitle line fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source title block.
    /// - `S` - Subtitle copy.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated title block.
    #[must_use]
    pub fn subtext<S>(mut self, subtext: S) -> Self
    where
        S: Into<String>,
    {
        self.set_subtext(subtext.into());
        self
    }

    /// Sets horizontal alignment fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source title block.
    /// - `EuvChartAlign` - Title horizontal alignment.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated title block.
    #[must_use]
    pub fn align(mut self, align: EuvChartAlign) -> Self {
        self.set_align(align);
        self
    }
}

/// Fluent builders for `EuvChartLegend`.
impl EuvChartLegend {
    /// Creates a legend docked at `position`.
    ///
    /// # Arguments
    ///
    /// - `EuvChartPosition` - Docked legend edge.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured legend.
    #[must_use]
    pub fn at(position: EuvChartPosition) -> Self {
        let mut legend: Self = Self::default();
        legend.set_position(position);
        legend
    }

    /// Controls whether the legend strip is shown.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source legend.
    /// - `bool` - Visibility flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated legend.
    #[must_use]
    pub fn show(mut self, show: bool) -> Self {
        self.set_show(show);
        self
    }

    /// Sets custom explicit legend item labels.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source legend.
    /// - `Vec<String>` - Explicit item labels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated legend.
    #[must_use]
    pub fn items(mut self, items: Vec<String>) -> Self {
        self.set_items(items);
        self
    }
}

/// Fluent builders for `EuvChartGrid`.
impl EuvChartGrid {
    /// Creates a Cartesian subplot grid with explicit logical margins.
    ///
    /// # Arguments
    ///
    /// - `f32` - Left margin.
    /// - `f32` - Right margin.
    /// - `f32` - Top margin.
    /// - `f32` - Bottom margin.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured grid.
    #[must_use]
    pub fn margins(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
            contain_label: true,
            show_border: false,
        }
    }

    /// Sets whether axis labels expand the grid margin automatically.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source grid.
    /// - `bool` - Label containment flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated grid.
    #[must_use]
    pub fn contain_label(mut self, contain_label: bool) -> Self {
        self.set_contain_label(contain_label);
        self
    }

    /// Sets whether a frame border is drawn around the plot area.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source grid.
    /// - `bool` - Border visibility flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated grid.
    #[must_use]
    pub fn show_border(mut self, show_border: bool) -> Self {
        self.set_show_border(show_border);
        self
    }
}

/// Constructors, fluent builders, and safe option accessors for `EuvChartAxis`.
impl EuvChartAxis {
    /// Creates a discrete `Category` axis with the provided labels.
    ///
    /// # Arguments
    ///
    /// - `Vec<String>` - Ordered category labels.
    ///
    /// # Returns
    ///
    /// - `Self` - Category axis configuration.
    #[must_use]
    pub fn category(categories: Vec<String>) -> Self {
        let mut axis: Self = Self::default();
        axis.set_axis_type(EuvChartAxisType::Category);
        axis.set_categories(categories);
        axis.set_show_split_line(false);
        axis
    }

    /// Creates a continuous linear `Value` axis.
    ///
    /// # Returns
    ///
    /// - `Self` - Value axis configuration.
    #[must_use]
    pub fn value() -> Self {
        Self::default()
    }

    /// Creates a continuous `Time` axis formatting numeric timestamps.
    ///
    /// # Returns
    ///
    /// - `Self` - Time axis configuration.
    #[must_use]
    pub fn time() -> Self {
        let mut axis: Self = Self::default();
        axis.set_axis_type(EuvChartAxisType::Time);
        axis
    }

    /// Creates a base-10 logarithmic `Log` axis.
    ///
    /// # Returns
    ///
    /// - `Self` - Logarithmic axis configuration.
    #[must_use]
    pub fn log() -> Self {
        let mut axis: Self = Self::default();
        axis.set_axis_type(EuvChartAxisType::Log);
        axis
    }

    /// Sets the axis caption name fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source axis.
    /// - `S` - Axis name text.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated axis.
    #[must_use]
    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.set_name(name.into());
        self
    }

    /// Sets a unit suffix appended to numeric tick labels.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source axis.
    /// - `S` - Unit suffix (e.g. `"%"` or `"ms"`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated axis.
    #[must_use]
    pub fn unit_suffix<S>(mut self, suffix: S) -> Self
    where
        S: Into<String>,
    {
        self.set_unit_suffix(suffix.into());
        self
    }

    /// Sets an explicit lower domain bound.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source axis.
    /// - `f32` - Explicit minimum domain value.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated axis.
    #[must_use]
    pub fn min(mut self, min: f32) -> Self {
        self.set_min(Some(min));
        self
    }

    /// Sets an explicit upper domain bound.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source axis.
    /// - `f32` - Explicit maximum domain value.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated axis.
    #[must_use]
    pub fn max(mut self, max: f32) -> Self {
        self.set_max(Some(max));
        self
    }

    /// Reverses axis direction when `inverse` is `true`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source axis.
    /// - `bool` - Inverse direction flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated axis.
    #[must_use]
    pub fn inverse(mut self, inverse: bool) -> Self {
        self.set_inverse(inverse);
        self
    }

    /// Sets the target tick split count.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source axis.
    /// - `usize` - Split interval count.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated axis.
    #[must_use]
    pub fn split_number(mut self, count: usize) -> Self {
        self.set_split_number(count.max(1));
        self
    }

    /// Sets whether grid split lines are rendered.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source axis.
    /// - `bool` - Split line visibility flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated axis.
    #[must_use]
    pub fn show_split_line(mut self, show: bool) -> Self {
        self.set_show_split_line(show);
        self
    }

    /// Sets whether alternating split background bands are rendered.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source axis.
    /// - `bool` - Split area visibility flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated axis.
    #[must_use]
    pub fn show_split_area(mut self, show: bool) -> Self {
        self.set_show_split_area(show);
        self
    }

    /// Assigns this axis to a specific subplot grid index.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source axis.
    /// - `usize` - Subplot grid index.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated axis.
    #[must_use]
    pub fn grid_index(mut self, idx: usize) -> Self {
        self.set_grid_index(idx);
        self
    }

    /// Overrides the axis side placement for dual-axis layouts.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source axis.
    /// - `EuvChartPosition` - Explicit axis side.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated axis.
    #[must_use]
    pub fn position(mut self, pos: EuvChartPosition) -> Self {
        self.set_position(Some(pos));
        self
    }

    /// Returns the optional explicit minimum domain bound.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Axis configuration.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Explicit lower bound when configured.
    #[must_use]
    pub fn min_opt(&self) -> Option<f32> {
        let Self { min, .. } = *self;
        min
    }

    /// Returns the optional explicit maximum domain bound.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Axis configuration.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Explicit upper bound when configured.
    #[must_use]
    pub fn max_opt(&self) -> Option<f32> {
        let Self { max, .. } = *self;
        max
    }

    /// Returns the optional explicit side override.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Axis configuration.
    ///
    /// # Returns
    ///
    /// - `Option<EuvChartPosition>` - Explicit side when set.
    #[must_use]
    pub fn position_opt(&self) -> Option<EuvChartPosition> {
        let Self { position, .. } = *self;
        position
    }
}

/// Fluent constructors for `EuvChartPolar`.
impl EuvChartPolar {
    /// Creates a polar coordinate system from angle and radius axes.
    ///
    /// # Arguments
    ///
    /// - `EuvChartAxis` - Angular axis definition.
    /// - `EuvChartAxis` - Radial axis definition.
    ///
    /// # Returns
    ///
    /// - `Self` - Polar coordinate system.
    #[must_use]
    pub fn new(angle_axis: EuvChartAxis, radius_axis: EuvChartAxis) -> Self {
        Self {
            angle_axis,
            radius_axis,
            radius_ratio: 0.72,
            center_x_ratio: 0.5,
            center_y_ratio: 0.52,
        }
    }
}

/// Fluent constructors for `EuvChartRadar`.
impl EuvChartRadar {
    /// Creates a radar web from ordered indicator definitions.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartRadarIndicator>` - Multivariate indicator spokes.
    ///
    /// # Returns
    ///
    /// - `Self` - Radar coordinate system.
    #[must_use]
    pub fn new(indicators: Vec<EuvChartRadarIndicator>) -> Self {
        let mut radar: Self = Self::default();
        radar.set_indicators(indicators);
        radar
    }

    /// Sets polygon or circular web ring geometry.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source radar configuration.
    /// - `EuvChartRadarShape` - Web ring geometry.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated radar configuration.
    #[must_use]
    pub fn shape(mut self, shape: EuvChartRadarShape) -> Self {
        self.set_shape(shape);
        self
    }

    /// Sets the number of concentric split rings.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source radar configuration.
    /// - `usize` - Ring count.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated radar configuration.
    #[must_use]
    pub fn split_number(mut self, split_number: usize) -> Self {
        self.set_split_number(split_number.max(1));
        self
    }
}

/// Fluent constructors for `EuvChartSingleAxis`.
impl EuvChartSingleAxis {
    /// Creates a single-axis coordinate system wrapping `axis`.
    ///
    /// # Arguments
    ///
    /// - `EuvChartAxis` - Underlying 1D axis definition.
    ///
    /// # Returns
    ///
    /// - `Self` - Single-axis coordinate system.
    #[must_use]
    pub fn new(axis: EuvChartAxis) -> Self {
        Self {
            axis,
            center_y_ratio: 0.54,
        }
    }
}

/// Fluent constructors for `EuvChartCalendar`.
impl EuvChartCalendar {
    /// Creates a calendar coordinate system for `year` and `weeks`.
    ///
    /// # Arguments
    ///
    /// - `u32` - Calendar year.
    /// - `usize` - Number of week columns.
    ///
    /// # Returns
    ///
    /// - `Self` - Calendar coordinate system.
    #[must_use]
    pub fn new(year: u32, weeks: usize) -> Self {
        let mut cal: Self = Self::default();
        cal.set_year(year);
        cal.set_weeks(weeks.max(1));
        cal
    }

    /// Sets custom month header labels along the week span.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source calendar.
    /// - `Vec<String>` - Ordered month labels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated calendar.
    #[must_use]
    pub fn month_labels(mut self, labels: Vec<String>) -> Self {
        self.set_month_labels(labels);
        self
    }
}

/// Constructors and accessors for `EuvChartGeoRegion` and `EuvChartGeo`.
impl EuvChartGeoRegion {
    /// Creates a geographic polygon region from `(lon, lat)` rings.
    ///
    /// # Arguments
    ///
    /// - `S` - Region name.
    /// - `C` - Region ISO or short code.
    /// - `Vec<Vec<(f32, f32)>>` - Closed exterior polygon rings.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed region.
    #[must_use]
    pub fn new<S, C>(name: S, code: C, polygons: Vec<Vec<(f32, f32)>>) -> Self
    where
        S: Into<String>,
        C: Into<String>,
    {
        Self {
            name: name.into(),
            code: code.into(),
            polygons,
            label_center: None,
        }
    }

    /// Sets an explicit `(lon, lat)` label anchor point.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source region.
    /// - `(f32, f32)` - Label `(longitude, latitude)` coordinate.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated region.
    #[must_use]
    pub fn label_center(mut self, center: (f32, f32)) -> Self {
        self.set_label_center(Some(center));
        self
    }

    /// Returns the optional explicit label anchor.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source region.
    ///
    /// # Returns
    ///
    /// - `Option<(f32, f32)>` - Explicit `(lon, lat)` label anchor.
    #[must_use]
    pub fn label_center_opt(&self) -> Option<(f32, f32)> {
        let Self { label_center, .. } = *self;
        label_center
    }
}

/// Fluent builders for `EuvChartGeo`.
impl EuvChartGeo {
    /// Creates a built-in stylized world map coordinate system.
    ///
    /// # Returns
    ///
    /// - `Self` - World geo coordinate system.
    #[must_use]
    pub fn world() -> Self {
        Self::default()
    }

    /// Creates a custom geo coordinate system from caller-supplied regions.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartGeoRegion>` - Custom vector polygon regions.
    ///
    /// # Returns
    ///
    /// - `Self` - Custom geo coordinate system.
    #[must_use]
    pub fn custom(regions: Vec<EuvChartGeoRegion>) -> Self {
        Self {
            preset: EuvChartGeoMapPreset::Custom,
            regions,
            show_labels: true,
            roam: false,
        }
    }

    /// Toggles region caption visibility on the map.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source geo coordinate system.
    /// - `bool` - Label visibility flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated geo coordinate system.
    #[must_use]
    pub fn show_labels(mut self, show: bool) -> Self {
        self.set_show_labels(show);
        self
    }
}

/// Fluent constructors for `EuvChartTooltip`.
impl EuvChartTooltip {
    /// Creates a tooltip with `trigger` and `axis_pointer`.
    ///
    /// # Arguments
    ///
    /// - `EuvChartTooltipTrigger` - Tooltip trigger mode.
    /// - `EuvChartAxisPointerType` - Crosshair axis pointer style.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured tooltip.
    #[must_use]
    pub fn new(trigger: EuvChartTooltipTrigger, axis_pointer: EuvChartAxisPointerType) -> Self {
        Self {
            show: trigger != EuvChartTooltipTrigger::None,
            trigger,
            axis_pointer,
        }
    }

    /// Creates an item-snapping tooltip without crosshair bars.
    ///
    /// # Returns
    ///
    /// - `Self` - Item tooltip configuration.
    #[must_use]
    pub fn item() -> Self {
        Self::new(EuvChartTooltipTrigger::Item, EuvChartAxisPointerType::None)
    }

    /// Creates an axis-snapping tooltip with a chosen crosshair style.
    ///
    /// # Arguments
    ///
    /// - `EuvChartAxisPointerType` - Crosshair pointer style.
    ///
    /// # Returns
    ///
    /// - `Self` - Axis tooltip configuration.
    #[must_use]
    pub fn axis(pointer: EuvChartAxisPointerType) -> Self {
        Self::new(EuvChartTooltipTrigger::Axis, pointer)
    }
}

/// Fluent constructors for `EuvChartDataZoom`.
impl EuvChartDataZoom {
    /// Creates an `Inside` wheel-zoom and drag-pan controller.
    ///
    /// # Returns
    ///
    /// - `Self` - Inside data-zoom controller.
    #[must_use]
    pub fn inside() -> Self {
        let mut dz: Self = Self::default();
        dz.set_kind(EuvChartDataZoomKind::Inside);
        dz
    }

    /// Creates a bottom interactive `Slider` range scrubber.
    ///
    /// # Returns
    ///
    /// - `Self` - Slider data-zoom controller.
    #[must_use]
    pub fn slider() -> Self {
        Self::default()
    }

    /// Sets the initial percentage window `(start_pct, end_pct)`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source data-zoom controller.
    /// - `f32` - Start percentage in `[0.0, 100.0]`.
    /// - `f32` - End percentage in `[0.0, 100.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated data-zoom controller.
    #[must_use]
    pub fn range(mut self, start_pct: f32, end_pct: f32) -> Self {
        let lo: f32 = start_pct.clamp(0.0, 99.0);
        let hi: f32 = end_pct.clamp(lo + 1.0, 100.0);
        self.set_start_pct(lo);
        self.set_end_pct(hi);
        self
    }
}

/// Constructors and accessors for `EuvChartVisualPiece`.
impl EuvChartVisualPiece {
    /// Creates a discrete piecewise color bucket.
    ///
    /// # Arguments
    ///
    /// - `Option<f32>` - Optional inclusive lower threshold.
    /// - `Option<f32>` - Optional inclusive upper threshold.
    /// - `S` - Display label.
    /// - `Color` - Bucket fill color.
    ///
    /// # Returns
    ///
    /// - `Self` - Visual-map piece.
    #[must_use]
    pub fn new<S>(min: Option<f32>, max: Option<f32>, label: S, color: Color) -> Self
    where
        S: Into<String>,
    {
        Self {
            min,
            max,
            label: label.into(),
            color,
        }
    }

    /// Returns the optional lower bound.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Piece definition.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Lower threshold.
    #[must_use]
    pub fn min_opt(&self) -> Option<f32> {
        let Self { min, .. } = *self;
        min
    }

    /// Returns the optional upper bound.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Piece definition.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Upper threshold.
    #[must_use]
    pub fn max_opt(&self) -> Option<f32> {
        let Self { max, .. } = *self;
        max
    }
}

/// Fluent constructors for `EuvChartVisualMap`.
impl EuvChartVisualMap {
    /// Creates a continuous gradient visual-map spanning `[min, max]`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Minimum mapped value.
    /// - `f32` - Maximum mapped value.
    ///
    /// # Returns
    ///
    /// - `Self` - Continuous visual-map.
    #[must_use]
    pub fn continuous(min: f32, max: f32) -> Self {
        let mut vm: Self = Self::default();
        vm.set_kind(EuvChartVisualMapKind::Continuous);
        vm.set_min(min);
        vm.set_max(max);
        vm
    }

    /// Creates a piecewise discrete range visual-map.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartVisualPiece>` - Ordered discrete buckets.
    ///
    /// # Returns
    ///
    /// - `Self` - Piecewise visual-map.
    #[must_use]
    pub fn piecewise(pieces: Vec<EuvChartVisualPiece>) -> Self {
        let mut vm: Self = Self::default();
        vm.set_kind(EuvChartVisualMapKind::Piecewise);
        vm.set_pieces(pieces);
        vm
    }

    /// Overrides the ordered color stops used by continuous interpolation.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source visual-map.
    /// - `Vec<Color>` - Ordered gradient color stops.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated visual-map.
    #[must_use]
    pub fn colors(mut self, colors: Vec<Color>) -> Self {
        self.set_in_range_colors(colors);
        self
    }

    /// Sets the visual-map bar dock position.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source visual-map.
    /// - `EuvChartPosition` - Docked edge.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated visual-map.
    #[must_use]
    pub fn position(mut self, position: EuvChartPosition) -> Self {
        self.set_position(position);
        self
    }
}

/// Fluent builders for `EuvChartToolbox`.
impl EuvChartToolbox {
    /// Creates a toolbox with all standard actions enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Full toolbox configuration.
    #[must_use]
    pub fn full() -> Self {
        Self::default()
    }

    /// Toggles MagicType switcher availability.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source toolbox.
    /// - `bool` - Enabled flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated toolbox.
    #[must_use]
    pub fn magic_type(mut self, enabled: bool) -> Self {
        self.set_magic_type(enabled);
        self
    }

    /// Toggles DataView tabular inspection availability.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source toolbox.
    /// - `bool` - Enabled flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated toolbox.
    #[must_use]
    pub fn data_view(mut self, enabled: bool) -> Self {
        self.set_data_view(enabled);
        self
    }

    /// Toggles Fullscreen modal availability.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source toolbox.
    /// - `bool` - Enabled flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated toolbox.
    #[must_use]
    pub fn fullscreen(mut self, enabled: bool) -> Self {
        self.set_fullscreen(enabled);
        self
    }
}

/// Constructors and helpers for `EuvChartDataset` and `EuvChartEncode`.
impl EuvChartDataset {
    /// Constructs a tabular dataset from column names and rows.
    ///
    /// # Arguments
    ///
    /// - `Vec<String>` - Dimension column names.
    /// - `Vec<Vec<EuvChartCell>>` - Ordered data rows.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed dataset.
    #[must_use]
    pub fn new(dimensions: Vec<String>, rows: Vec<Vec<EuvChartCell>>) -> Self {
        Self { dimensions, rows }
    }

    /// Resolves a dimension name (or decimal column index string) to its column index.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source dataset.
    /// - `&str` - Dimension name or numeric column index.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - Matched column index.
    #[must_use]
    pub fn column_index(&self, dim: &str) -> Option<usize> {
        if let Some(pos) = self
            .get_dimensions()
            .iter()
            .position(|d: &String| d.as_str() == dim)
        {
            return Some(pos);
        }
        dim.parse::<usize>().ok()
    }
}

/// Fluent builders and accessors for `EuvChartEncode`.
impl EuvChartEncode {
    /// Maps the X channel to dimension `dim`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source encode mapping.
    /// - `S` - Dimension name or column index.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated encode mapping.
    #[must_use]
    pub fn x<S>(mut self, dim: S) -> Self
    where
        S: Into<String>,
    {
        self.set_x(Some(dim.into()));
        self
    }

    /// Maps the Y channel to dimension `dim`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source encode mapping.
    /// - `S` - Dimension name or column index.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated encode mapping.
    #[must_use]
    pub fn y<S>(mut self, dim: S) -> Self
    where
        S: Into<String>,
    {
        self.set_y(Some(dim.into()));
        self
    }

    /// Maps the scalar value channel to dimension `dim`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source encode mapping.
    /// - `S` - Dimension name or column index.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated encode mapping.
    #[must_use]
    pub fn value<S>(mut self, dim: S) -> Self
    where
        S: Into<String>,
    {
        self.set_value(Some(dim.into()));
        self
    }

    /// Maps the bubble size channel to dimension `dim`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source encode mapping.
    /// - `S` - Dimension name or column index.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated encode mapping.
    #[must_use]
    pub fn size<S>(mut self, dim: S) -> Self
    where
        S: Into<String>,
    {
        self.set_size(Some(dim.into()));
        self
    }

    /// Maps the item caption channel to dimension `dim`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source encode mapping.
    /// - `S` - Dimension name or column index.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated encode mapping.
    #[must_use]
    pub fn item_name<S>(mut self, dim: S) -> Self
    where
        S: Into<String>,
    {
        self.set_item_name(Some(dim.into()));
        self
    }

    /// Returns the X dimension name if mapped.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Encode mapping.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Mapped X dimension.
    #[must_use]
    pub fn x_opt(&self) -> Option<&str> {
        let Self { x, .. } = self;
        x.as_deref()
    }

    /// Returns the Y dimension name if mapped.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Encode mapping.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Mapped Y dimension.
    #[must_use]
    pub fn y_opt(&self) -> Option<&str> {
        let Self { y, .. } = self;
        y.as_deref()
    }

    /// Returns the value dimension name if mapped.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Encode mapping.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Mapped value dimension.
    #[must_use]
    pub fn value_opt(&self) -> Option<&str> {
        let Self { value, .. } = self;
        value.as_deref()
    }

    /// Returns the size dimension name if mapped.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Encode mapping.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Mapped size dimension.
    #[must_use]
    pub fn size_opt(&self) -> Option<&str> {
        let Self { size, .. } = self;
        size.as_deref()
    }

    /// Returns the item name dimension if mapped.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Encode mapping.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Mapped item name dimension.
    #[must_use]
    pub fn item_name_opt(&self) -> Option<&str> {
        let Self { item_name, .. } = self;
        item_name.as_deref()
    }
}

/// Constructors and accessors for `EuvChartMarkPoint`, `EuvChartMarkLine`, and `EuvChartMarkArea`.
impl EuvChartMarkPoint {
    /// Creates a mark-point annotation pin.
    ///
    /// # Arguments
    ///
    /// - `EuvChartMarkPointKind` - Target datum rule.
    /// - `S` - Callout label.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed mark point.
    #[must_use]
    pub fn new<S>(kind: EuvChartMarkPointKind, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            kind,
            name: name.into(),
            color: None,
        }
    }

    /// Overrides the pin tint color.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source mark point.
    /// - `Color` - Pin fill color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated mark point.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.set_color(Some(color));
        self
    }

    /// Returns the optional color override.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source mark point.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit pin color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Constructors and accessors for `EuvChartMarkLine`.
impl EuvChartMarkLine {
    /// Creates a dashed reference mark line.
    ///
    /// # Arguments
    ///
    /// - `EuvChartMarkLineKind` - Target reference rule.
    /// - `S` - Reference label text.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed mark line.
    #[must_use]
    pub fn new<S>(kind: EuvChartMarkLineKind, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            kind,
            name: name.into(),
            color: None,
            dashed: true,
        }
    }

    /// Overrides the mark line stroke color.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source mark line.
    /// - `Color` - Stroke color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated mark line.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.set_color(Some(color));
        self
    }

    /// Returns the optional color override.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source mark line.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit stroke color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Constructors and accessors for `EuvChartMarkArea`.
impl EuvChartMarkArea {
    /// Creates a vertical highlight band spanning `start_x..=end_x`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Lower X bound or category index.
    /// - `f32` - Upper X bound or category index.
    /// - `S` - Band caption.
    ///
    /// # Returns
    ///
    /// - `Self` - Vertical mark area.
    #[must_use]
    pub fn x_range<S>(start_x: f32, end_x: f32, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            start_x: Some(start_x),
            end_x: Some(end_x),
            start_y: None,
            end_y: None,
            name: name.into(),
            color: None,
        }
    }

    /// Creates a horizontal highlight band spanning `start_y..=end_y`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Lower Y value.
    /// - `f32` - Upper Y value.
    /// - `S` - Band caption.
    ///
    /// # Returns
    ///
    /// - `Self` - Horizontal mark area.
    #[must_use]
    pub fn y_range<S>(start_y: f32, end_y: f32, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            start_x: None,
            end_x: None,
            start_y: Some(start_y),
            end_y: Some(end_y),
            name: name.into(),
            color: None,
        }
    }

    /// Overrides the highlight band tint color.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source mark area.
    /// - `Color` - Band tint color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated mark area.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.set_color(Some(color));
        self
    }

    /// Returns `(start_x, end_x, start_y, end_y)` bounds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source mark area.
    ///
    /// # Returns
    ///
    /// - `(Option<f32>, Option<f32>, Option<f32>, Option<f32>)` - Optional data bounds.
    #[must_use]
    pub fn bounds_opt(&self) -> (Option<f32>, Option<f32>, Option<f32>, Option<f32>) {
        let Self {
            start_x,
            end_x,
            start_y,
            end_y,
            ..
        } = *self;
        (start_x, end_x, start_y, end_y)
    }

    /// Returns the optional tint override.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source mark area.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit band tint.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Fluent builders and accessors for `EuvChartLineSeries`.
impl EuvChartLineSeries {
    /// Creates a line series from dense numeric values.
    ///
    /// # Arguments
    ///
    /// - `Vec<f32>` - Ordered Y values.
    ///
    /// # Returns
    ///
    /// - `Self` - Line series.
    #[must_use]
    pub fn from_values(values: Vec<f32>) -> Self {
        let mut s: Self = Self::default();
        let mapped: Vec<Option<f32>> = values.into_iter().map(Some).collect();
        s.set_values(mapped);
        s
    }

    /// Creates a line series supporting `None` null gaps.
    ///
    /// # Arguments
    ///
    /// - `Vec<Option<f32>>` - Nullable Y values.
    ///
    /// # Returns
    ///
    /// - `Self` - Line series.
    #[must_use]
    pub fn from_nullable(values: Vec<Option<f32>>) -> Self {
        let mut s: Self = Self::default();
        s.set_values(values);
        s
    }

    /// Creates a line series from explicit `(x, y)` coordinate pairs.
    ///
    /// # Arguments
    ///
    /// - `Vec<(f32, f32)>` - Explicit `(x, y)` coordinates.
    ///
    /// # Returns
    ///
    /// - `Self` - Line series.
    #[must_use]
    pub fn from_xy(points: Vec<(f32, f32)>) -> Self {
        let mut s: Self = Self::default();
        s.set_xy_points(points);
        s
    }

    /// Enables or disables smooth Catmull-Rom curve interpolation.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `bool` - Smooth curve flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.set_smooth(smooth);
        self
    }

    /// Sets step-line interpolation mode.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `EuvChartStepMode` - Step mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn step(mut self, step: EuvChartStepMode) -> Self {
        self.set_step(step);
        self
    }

    /// Enables translucent area shading under the line.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `f32` - Area fill opacity in `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn area(mut self, opacity: f32) -> Self {
        self.set_area_opacity(opacity.clamp(0.0, 1.0));
        self
    }

    /// Assigns a stack group name for stacked line/area rendering.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `S` - Stack group key.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn stack<S>(mut self, stack: S) -> Self
    where
        S: Into<String>,
    {
        self.set_stack(Some(stack.into()));
        self
    }

    /// Enables dashed stroke rendering.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `bool` - Dashed stroke flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn dashed(mut self, dashed: bool) -> Self {
        self.set_dashed(dashed);
        self
    }

    /// Configures vertex symbol shape and radius.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `EuvChartSymbol` - Symbol shape.
    /// - `f32` - Symbol radius in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn symbol(mut self, symbol: EuvChartSymbol, size: f32) -> Self {
        self.set_symbol(symbol);
        self.set_symbol_size(size);
        self
    }

    /// Controls whether `None` gaps are bridged continuously.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `bool` - Connect-nulls flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn connect_nulls(mut self, connect: bool) -> Self {
        self.set_connect_nulls(connect);
        self
    }

    /// Binds the series to a specific `(x_axis_index, y_axis_index)` pair.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `usize` - X axis index.
    /// - `usize` - Y axis index.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn axes(mut self, x_axis_index: usize, y_axis_index: usize) -> Self {
        self.set_x_axis_index(x_axis_index);
        self.set_y_axis_index(y_axis_index);
        self
    }

    /// Binds this line series to a non-default coordinate system (e.g. `Polar`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `EuvChartCoordinateSystem` - Target coordinate system.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn coordinate_system(mut self, coord: EuvChartCoordinateSystem) -> Self {
        self.set_coordinate_system(coord);
        self
    }

    /// Attaches a `MarkPoint` annotation.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `EuvChartMarkPoint` - Mark point descriptor.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn mark_point(mut self, mark: EuvChartMarkPoint) -> Self {
        self.get_mut_annotations().get_mut_mark_points().push(mark);
        self
    }

    /// Attaches a `MarkLine` annotation.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `EuvChartMarkLine` - Mark line descriptor.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn mark_line(mut self, mark: EuvChartMarkLine) -> Self {
        self.get_mut_annotations().get_mut_mark_lines().push(mark);
        self
    }

    /// Attaches a `MarkArea` annotation.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `EuvChartMarkArea` - Mark area descriptor.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn mark_area(mut self, mark: EuvChartMarkArea) -> Self {
        self.get_mut_annotations().get_mut_mark_areas().push(mark);
        self
    }

    /// Attaches a `Dataset` column encoding mapping.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source line series.
    /// - `EuvChartEncode` - Column mapping.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated line series.
    #[must_use]
    pub fn encode(mut self, encode: EuvChartEncode) -> Self {
        self.set_encode(Some(encode));
        self
    }

    /// Returns the optional stack group identifier.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Line series.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Stack group key.
    #[must_use]
    pub fn stack_opt(&self) -> Option<&str> {
        let Self { stack, .. } = self;
        stack.as_deref()
    }

    /// Returns the optional dataset encode mapping.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Line series.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartEncode>` - Dataset column mapping.
    #[must_use]
    pub fn encode_opt(&self) -> Option<&EuvChartEncode> {
        let Self { encode, .. } = self;
        encode.as_ref()
    }
}

/// Fluent builders and accessors for `EuvChartBarSeries`.
impl EuvChartBarSeries {
    /// Creates a bar series from numeric values.
    ///
    /// # Arguments
    ///
    /// - `Vec<f32>` - Ordered bar magnitudes.
    ///
    /// # Returns
    ///
    /// - `Self` - Bar series.
    #[must_use]
    pub fn from_values(values: Vec<f32>) -> Self {
        let mut s: Self = Self::default();
        let mapped: Vec<Option<f32>> = values.into_iter().map(Some).collect();
        s.set_values(mapped);
        s
    }

    /// Assigns a stack group key for stacked bars.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bar series.
    /// - `S` - Stack group key.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bar series.
    #[must_use]
    pub fn stack<S>(mut self, stack: S) -> Self
    where
        S: Into<String>,
    {
        self.set_stack(Some(stack.into()));
        self
    }

    /// Sets the bar cap corner radius.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bar series.
    /// - `f32` - Corner radius in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bar series.
    #[must_use]
    pub fn bar_radius(mut self, radius: f32) -> Self {
        self.set_bar_radius(radius.max(0.0));
        self
    }

    /// Enables subtle background tracks behind each bar.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bar series.
    /// - `bool` - Background track flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bar series.
    #[must_use]
    pub fn show_background(mut self, show: bool) -> Self {
        self.set_show_background(show);
        self
    }

    /// Enables numeric value labels on bars.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bar series.
    /// - `bool` - Value label flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bar series.
    #[must_use]
    pub fn show_labels(mut self, show: bool) -> Self {
        self.set_show_labels(show);
        self
    }

    /// Binds this bar series to a coordinate system (`Cartesian2d` or `Polar`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bar series.
    /// - `EuvChartCoordinateSystem` - Coordinate system.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bar series.
    #[must_use]
    pub fn coordinate_system(mut self, coord: EuvChartCoordinateSystem) -> Self {
        self.set_coordinate_system(coord);
        self
    }

    /// Binds the series to a specific `(x_axis_index, y_axis_index)` pair.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bar series.
    /// - `usize` - X axis index.
    /// - `usize` - Y axis index.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bar series.
    #[must_use]
    pub fn axes(mut self, x_axis_index: usize, y_axis_index: usize) -> Self {
        self.set_x_axis_index(x_axis_index);
        self.set_y_axis_index(y_axis_index);
        self
    }

    /// Attaches a `MarkLine` annotation.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bar series.
    /// - `EuvChartMarkLine` - Mark line descriptor.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bar series.
    #[must_use]
    pub fn mark_line(mut self, mark: EuvChartMarkLine) -> Self {
        self.get_mut_annotations().get_mut_mark_lines().push(mark);
        self
    }

    /// Attaches a `Dataset` encode mapping.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bar series.
    /// - `EuvChartEncode` - Column mapping.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bar series.
    #[must_use]
    pub fn encode(mut self, encode: EuvChartEncode) -> Self {
        self.set_encode(Some(encode));
        self
    }

    /// Returns the optional stack group key.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bar series.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Stack group key.
    #[must_use]
    pub fn stack_opt(&self) -> Option<&str> {
        let Self { stack, .. } = self;
        stack.as_deref()
    }

    /// Returns the optional dataset encode mapping.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bar series.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartEncode>` - Dataset column mapping.
    #[must_use]
    pub fn encode_opt(&self) -> Option<&EuvChartEncode> {
        let Self { encode, .. } = self;
        encode.as_ref()
    }
}

/// Constructors and accessors for `EuvChartPieSlice` and `EuvChartPieSeries`.
impl EuvChartPieSlice {
    /// Creates a labeled pie slice.
    ///
    /// # Arguments
    ///
    /// - `S` - Slice label.
    /// - `f32` - Positive numeric weight.
    ///
    /// # Returns
    ///
    /// - `Self` - Pie slice.
    #[must_use]
    pub fn new<S>(name: S, value: f32) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            value,
            color: None,
            selected: false,
        }
    }

    /// Sets an explicit fill color for this slice.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source slice.
    /// - `Color` - Fill color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated slice.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.set_color(Some(color));
        self
    }

    /// Marks this slice as popped out radially.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source slice.
    /// - `bool` - Selected pop-out state.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated slice.
    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.set_selected(selected);
        self
    }

    /// Returns the optional explicit slice color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source slice.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit fill color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Fluent builders for `EuvChartPieSeries`.
impl EuvChartPieSeries {
    /// Creates a pie series from slices.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartPieSlice>` - Ordered pie slices.
    ///
    /// # Returns
    ///
    /// - `Self` - Pie series.
    #[must_use]
    pub fn new(slices: Vec<EuvChartPieSlice>) -> Self {
        let mut s: Self = Self::default();
        s.set_slices(slices);
        s
    }

    /// Converts the pie into a donut chart with the given inner cutout ratio.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source pie series.
    /// - `f32` - Inner radius ratio in `[0.0, 0.85]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated pie series.
    #[must_use]
    pub fn donut(mut self, inner_ratio: f32) -> Self {
        self.set_inner_radius_ratio(inner_ratio.clamp(0.0, 0.85));
        self
    }

    /// Sets Nightingale Rose mode (`Radius` or `Area`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source pie series.
    /// - `EuvChartRoseType` - Rose variation mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated pie series.
    #[must_use]
    pub fn rose(mut self, rose_type: EuvChartRoseType) -> Self {
        self.set_rose_type(rose_type);
        self
    }

    /// Sets angular gap between adjacent wedges in degrees.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source pie series.
    /// - `f32` - Pad angle in degrees.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated pie series.
    #[must_use]
    pub fn pad_angle(mut self, deg: f32) -> Self {
        self.set_pad_angle_deg(deg.clamp(0.0, 15.0));
        self
    }
}

/// Constructors and accessors for `EuvChartScatterPoint` and `EuvChartScatterSeries`.
impl EuvChartScatterPoint {
    /// Creates a 2D scatter point `(x, y)` using the series default size.
    ///
    /// # Arguments
    ///
    /// - `f32` - X coordinate.
    /// - `f32` - Y coordinate.
    ///
    /// # Returns
    ///
    /// - `Self` - Scatter point.
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            size: 0.0,
            value: None,
            label: String::new(),
        }
    }

    /// Sets an explicit bubble radius in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source scatter point.
    /// - `f32` - Bubble radius.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated scatter point.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.set_size(size);
        self
    }

    /// Sets a scalar value for `VisualMap` coloring.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source scatter point.
    /// - `f32` - Mapped numeric value.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated scatter point.
    #[must_use]
    pub fn value(mut self, value: f32) -> Self {
        self.set_value(Some(value));
        self
    }

    /// Sets a point caption label.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source scatter point.
    /// - `S` - Caption text.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated scatter point.
    #[must_use]
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: Into<String>,
    {
        self.set_label(label.into());
        self
    }

    /// Returns the optional visual-map scalar value.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source scatter point.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Mapped numeric value.
    #[must_use]
    pub fn value_opt(&self) -> Option<f32> {
        let Self { value, .. } = *self;
        value
    }
}

/// Fluent builders for `EuvChartScatterSeries`.
impl EuvChartScatterSeries {
    /// Creates a scatter series from `(x, y)` points.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartScatterPoint>` - Scatter points.
    ///
    /// # Returns
    ///
    /// - `Self` - Scatter series.
    #[must_use]
    pub fn new(points: Vec<EuvChartScatterPoint>) -> Self {
        let mut s: Self = Self::default();
        s.set_points(points);
        s
    }

    /// Sets marker glyph shape and default radius.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source scatter series.
    /// - `EuvChartSymbol` - Symbol shape.
    /// - `f32` - Default radius in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated scatter series.
    #[must_use]
    pub fn symbol(mut self, symbol: EuvChartSymbol, size: f32) -> Self {
        self.set_symbol(symbol);
        self.set_default_size(size.max(1.0));
        self
    }

    /// Binds this scatter series to a coordinate system (`Cartesian2d`, `Polar`, `SingleAxis`, or `Geo`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source scatter series.
    /// - `EuvChartCoordinateSystem` - Hosting coordinate system.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated scatter series.
    #[must_use]
    pub fn coordinate_system(mut self, coord: EuvChartCoordinateSystem) -> Self {
        self.set_coordinate_system(coord);
        self
    }

    /// Enables point caption labels.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source scatter series.
    /// - `bool` - Label visibility flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated scatter series.
    #[must_use]
    pub fn show_labels(mut self, show: bool) -> Self {
        self.set_show_labels(show);
        self
    }

    /// Attaches a dataset column encoding mapping.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source scatter series.
    /// - `EuvChartEncode` - Column mapping.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated scatter series.
    #[must_use]
    pub fn encode(mut self, encode: EuvChartEncode) -> Self {
        self.set_encode(Some(encode));
        self
    }

    /// Returns the optional dataset encode mapping.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source scatter series.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartEncode>` - Dataset column mapping.
    #[must_use]
    pub fn encode_opt(&self) -> Option<&EuvChartEncode> {
        let Self { encode, .. } = self;
        encode.as_ref()
    }
}

/// Constructors and accessors for `EuvChartRadarItem` and `EuvChartRadarSeries`.
impl EuvChartRadarItem {
    /// Creates a radar polygon record.
    ///
    /// # Arguments
    ///
    /// - `S` - Item display name.
    /// - `Vec<f32>` - Values aligned to radar indicators.
    ///
    /// # Returns
    ///
    /// - `Self` - Radar item.
    #[must_use]
    pub fn new<S>(name: S, values: Vec<f32>) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            values,
            color: None,
        }
    }

    /// Sets an explicit polygon color.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source radar item.
    /// - `Color` - Stroke and tint color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated radar item.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.set_color(Some(color));
        self
    }

    /// Returns the optional explicit color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source radar item.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit polygon color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Fluent constructors for `EuvChartRadarSeries`.
impl EuvChartRadarSeries {
    /// Creates a radar series from multivariate items.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartRadarItem>` - Radar polygon records.
    ///
    /// # Returns
    ///
    /// - `Self` - Radar series.
    #[must_use]
    pub fn new(items: Vec<EuvChartRadarItem>) -> Self {
        let mut s: Self = Self::default();
        s.set_items(items);
        s
    }
}

/// Fluent builders and accessors for `EuvChartCandlestickSeries`.
impl EuvChartCandlestickSeries {
    /// Creates a candlestick series from OHLC records.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartOhlcPoint>` - Ordered OHLC candles.
    ///
    /// # Returns
    ///
    /// - `Self` - Candlestick series.
    #[must_use]
    pub fn new(candles: Vec<EuvChartOhlcPoint>) -> Self {
        let mut s: Self = Self::default();
        s.set_candles(candles);
        s
    }

    /// Overrides the bullish and bearish candle fill colors.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source candlestick series.
    /// - `Color` - Bullish color (`close >= open`).
    /// - `Color` - Bearish color (`close < open`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated candlestick series.
    #[must_use]
    pub fn colors(mut self, bull: Color, bear: Color) -> Self {
        self.set_bull_color(Some(bull));
        self.set_bear_color(Some(bear));
        self
    }

    /// Returns the resolved bullish candle color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Candlestick series.
    ///
    /// # Returns
    ///
    /// - `Color` - Bullish fill color.
    #[must_use]
    pub fn resolved_bull_color(&self) -> Color {
        let Self { bull_color, .. } = *self;
        bull_color.unwrap_or_else(|| Color::hex(ECHART_CANDLE_BULL_HEX))
    }

    /// Returns the resolved bearish candle color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Candlestick series.
    ///
    /// # Returns
    ///
    /// - `Color` - Bearish fill color.
    #[must_use]
    pub fn resolved_bear_color(&self) -> Color {
        let Self { bear_color, .. } = *self;
        bear_color.unwrap_or_else(|| Color::hex(ECHART_CANDLE_BEAR_HEX))
    }
}

/// Constructors for `EuvChartBoxplotItem` and `EuvChartBoxplotSeries`.
impl EuvChartBoxplotItem {
    /// Creates a five-number boxplot summary without outliers.
    ///
    /// # Arguments
    ///
    /// - `f32` - Lower whisker minimum.
    /// - `f32` - First quartile.
    /// - `f32` - Median value.
    /// - `f32` - Third quartile.
    /// - `f32` - Upper whisker maximum.
    ///
    /// # Returns
    ///
    /// - `Self` - Boxplot summary item.
    #[must_use]
    pub fn new(min: f32, q1: f32, median: f32, q3: f32, max: f32) -> Self {
        Self {
            min,
            q1,
            median,
            q3,
            max,
            outliers: Vec::new(),
        }
    }

    /// Attaches outlier scalar values plotted above/below the whiskers.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source boxplot item.
    /// - `Vec<f32>` - Outlier observations.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated boxplot item.
    #[must_use]
    pub fn outliers(mut self, outliers: Vec<f32>) -> Self {
        self.set_outliers(outliers);
        self
    }
}

/// Constructors for `EuvChartBoxplotSeries`.
impl EuvChartBoxplotSeries {
    /// Creates a boxplot series from summary items.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartBoxplotItem>` - Ordered boxplot summaries.
    ///
    /// # Returns
    ///
    /// - `Self` - Boxplot series.
    #[must_use]
    pub fn new(items: Vec<EuvChartBoxplotItem>) -> Self {
        let mut s: Self = Self::default();
        s.set_items(items);
        s
    }
}

/// Constructors for `EuvChartHeatmapCell` and `EuvChartHeatmapSeries`.
impl EuvChartHeatmapCell {
    /// Creates a heatmap cell at `(x_index, y_index)` with `value`.
    ///
    /// # Arguments
    ///
    /// - `usize` - Column index.
    /// - `usize` - Row index.
    /// - `f32` - Numeric intensity.
    ///
    /// # Returns
    ///
    /// - `Self` - Heatmap cell.
    #[must_use]
    pub fn new(x_index: usize, y_index: usize, value: f32) -> Self {
        Self {
            x_index,
            y_index,
            value,
            label: String::new(),
        }
    }
}

/// Fluent constructors for `EuvChartHeatmapSeries`.
impl EuvChartHeatmapSeries {
    /// Creates a Cartesian matrix heatmap series.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartHeatmapCell>` - Matrix cells.
    ///
    /// # Returns
    ///
    /// - `Self` - Heatmap series.
    #[must_use]
    pub fn new(cells: Vec<EuvChartHeatmapCell>) -> Self {
        let mut s: Self = Self::default();
        s.set_cells(cells);
        s
    }

    /// Binds this heatmap to `Calendar` or `Cartesian2d` coordinates.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source heatmap series.
    /// - `EuvChartCoordinateSystem` - Coordinate system.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated heatmap series.
    #[must_use]
    pub fn coordinate_system(mut self, coord: EuvChartCoordinateSystem) -> Self {
        self.set_coordinate_system(coord);
        self
    }

    /// Toggles numeric cell value captions.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source heatmap series.
    /// - `bool` - Cell label flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated heatmap series.
    #[must_use]
    pub fn show_labels(mut self, show: bool) -> Self {
        self.set_show_labels(show);
        self
    }
}

/// Constructors and recursive aggregation helpers for `EuvChartHierarchyNode`.
impl EuvChartHierarchyNode {
    /// Creates a leaf hierarchy node with numeric `value`.
    ///
    /// # Arguments
    ///
    /// - `S` - Node display label.
    /// - `f32` - Positive numeric weight.
    ///
    /// # Returns
    ///
    /// - `Self` - Leaf node.
    #[must_use]
    pub fn leaf<S>(name: S, value: f32) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            value,
            color: None,
            children: Vec::new(),
        }
    }

    /// Creates a parent branch hierarchy node wrapping `children`.
    ///
    /// # Arguments
    ///
    /// - `S` - Branch display label.
    /// - `Vec<Self>` - Nested child nodes.
    ///
    /// # Returns
    ///
    /// - `Self` - Branch node whose weight sums its children.
    #[must_use]
    pub fn branch<S>(name: S, children: Vec<Self>) -> Self
    where
        S: Into<String>,
    {
        let total: f32 = children.iter().map(Self::total_value).sum();
        Self {
            name: name.into(),
            value: total,
            color: None,
            children,
        }
    }

    /// Overrides the node tint color.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hierarchy node.
    /// - `Color` - Explicit node color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hierarchy node.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.set_color(Some(color));
        self
    }

    /// Returns the optional explicit color tint.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source hierarchy node.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit node color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }

    /// Computes total positive weight recursively for treemap and sunburst proportionality.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hierarchy node.
    ///
    /// # Returns
    ///
    /// - `f32` - Positive aggregated weight.
    #[must_use]
    pub fn total_value(&self) -> f32 {
        let children: &[Self] = self.get_children().as_slice();
        if children.is_empty() {
            self.get_value().max(0.0)
        } else {
            let sum: f32 = children.iter().map(Self::total_value).sum();
            sum.max(self.get_value().max(0.0))
        }
    }

    /// Returns the maximum subtree depth (`1` for a leaf).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hierarchy node.
    ///
    /// # Returns
    ///
    /// - `usize` - Subtree depth.
    #[must_use]
    pub fn max_depth(&self) -> usize {
        let children: &[Self] = self.get_children().as_slice();
        if children.is_empty() {
            1
        } else {
            let deepest: usize = children.iter().map(Self::max_depth).max().unwrap_or(0);
            1 + deepest
        }
    }
}

/// Constructors for `EuvChartTreemapSeries`, `EuvChartSunburstSeries`, `EuvChartSankeySeries`, `EuvChartFunnelSeries`, `EuvChartGaugeSeries`, `EuvChartGraphSeries`, `EuvChartTreeSeries`, `EuvChartParallelSeries`, `EuvChartLinesSeries`, `EuvChartThemeRiverSeries`, `EuvChartPictorialBarSeries`, `EuvChartCustomSeries`, and `EuvChartGeoMapSeries`.
impl EuvChartTreemapSeries {
    /// Creates a treemap series from top-level nodes.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartHierarchyNode>` - Top-level hierarchy nodes.
    ///
    /// # Returns
    ///
    /// - `Self` - Treemap series.
    #[must_use]
    pub fn new(nodes: Vec<EuvChartHierarchyNode>) -> Self {
        let mut s: Self = Self::default();
        s.set_nodes(nodes);
        s
    }
}

/// Constructors for `EuvChartSunburstSeries`.
impl EuvChartSunburstSeries {
    /// Creates a sunburst series from top-level nodes.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartHierarchyNode>` - Top-level hierarchy nodes.
    ///
    /// # Returns
    ///
    /// - `Self` - Sunburst series.
    #[must_use]
    pub fn new(nodes: Vec<EuvChartHierarchyNode>) -> Self {
        let mut s: Self = Self::default();
        s.set_nodes(nodes);
        s
    }
}

/// Constructors and accessors for `EuvChartSankeyNode` and `EuvChartSankeySeries`.
impl EuvChartSankeyNode {
    /// Creates a named Sankey node.
    ///
    /// # Arguments
    ///
    /// - `S` - Node identifier name.
    ///
    /// # Returns
    ///
    /// - `Self` - Sankey node.
    #[must_use]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            color: None,
            depth: None,
        }
    }

    /// Overrides the node fill color.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source node.
    /// - `Color` - Fill color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated node.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.set_color(Some(color));
        self
    }

    /// Returns the optional explicit node color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source node.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit node color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }

    /// Returns the optional explicit column depth.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source node.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - Explicit column depth.
    #[must_use]
    pub fn depth_opt(&self) -> Option<usize> {
        let Self { depth, .. } = *self;
        depth
    }
}

/// Constructors for `EuvChartSankeySeries`.
impl EuvChartSankeySeries {
    /// Creates a Sankey diagram from nodes and weighted links.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartSankeyNode>` - Sankey nodes.
    /// - `Vec<EuvChartSankeyLink>` - Weighted flow links.
    ///
    /// # Returns
    ///
    /// - `Self` - Sankey series.
    #[must_use]
    pub fn new(nodes: Vec<EuvChartSankeyNode>, links: Vec<EuvChartSankeyLink>) -> Self {
        let mut s: Self = Self::default();
        s.set_nodes(nodes);
        s.set_links(links);
        s
    }
}

/// Constructors and accessors for `EuvChartFunnelStage` and `EuvChartFunnelSeries`.
impl EuvChartFunnelStage {
    /// Creates a conversion funnel stage.
    ///
    /// # Arguments
    ///
    /// - `S` - Stage display name.
    /// - `f32` - Positive stage weight.
    ///
    /// # Returns
    ///
    /// - `Self` - Funnel stage.
    #[must_use]
    pub fn new<S>(name: S, value: f32) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            value,
            color: None,
        }
    }

    /// Returns the optional explicit stage color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source stage.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit stage color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Constructors for `EuvChartFunnelSeries`.
impl EuvChartFunnelSeries {
    /// Creates a funnel series from conversion stages.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartFunnelStage>` - Ordered conversion stages.
    ///
    /// # Returns
    ///
    /// - `Self` - Funnel series.
    #[must_use]
    pub fn new(stages: Vec<EuvChartFunnelStage>) -> Self {
        let mut s: Self = Self::default();
        s.set_stages(stages);
        s
    }

    /// Sets vertical funnel sorting order.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source funnel series.
    /// - `EuvChartFunnelSort` - Sorting mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated funnel series.
    #[must_use]
    pub fn sort(mut self, sort: EuvChartFunnelSort) -> Self {
        self.set_sort(sort);
        self
    }
}

/// Fluent constructors for `EuvChartGaugeSeries`.
impl EuvChartGaugeSeries {
    /// Creates a gauge reading `value` over `[min, max]`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Active numeric reading.
    /// - `f32` - Lower dial bound.
    /// - `f32` - Upper dial bound.
    ///
    /// # Returns
    ///
    /// - `Self` - Gauge series.
    #[must_use]
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        let mut g: Self = Self::default();
        g.set_value(value);
        g.set_min(min);
        g.set_max(max);
        g
    }

    /// Sets the center unit suffix string.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source gauge.
    /// - `S` - Unit text.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated gauge.
    #[must_use]
    pub fn unit<S>(mut self, unit: S) -> Self
    where
        S: Into<String>,
    {
        self.set_unit(unit.into());
        self
    }

    /// Sets the dial caption title.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source gauge.
    /// - `S` - Caption text.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated gauge.
    #[must_use]
    pub fn title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.set_title(title.into());
        self
    }

    /// Sets custom arc start and end angles in degrees.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source gauge.
    /// - `f32` - Start angle in degrees.
    /// - `f32` - End angle in degrees.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated gauge.
    #[must_use]
    pub fn angles(mut self, start_deg: f32, end_deg: f32) -> Self {
        self.set_start_angle_deg(start_deg);
        self.set_end_angle_deg(end_deg);
        self
    }
}

/// Constructors and accessors for `EuvChartGraphNode`, `EuvChartGraphEdge`, and `EuvChartGraphSeries`.
impl EuvChartGraphNode {
    /// Creates a graph vertex with `id`, `name`, radius `size`, and `category`.
    ///
    /// # Arguments
    ///
    /// - `I` - Unique node identifier.
    /// - `N` - Display caption.
    /// - `f32` - Visual radius in logical pixels.
    /// - `usize` - Category index.
    ///
    /// # Returns
    ///
    /// - `Self` - Graph node.
    #[must_use]
    pub fn new<I, N>(id: I, name: N, size: f32, category: usize) -> Self
    where
        I: Into<String>,
        N: Into<String>,
    {
        Self {
            id: id.into(),
            name: name.into(),
            value: size,
            size: size.max(4.0),
            category,
            x: None,
            y: None,
            color: None,
        }
    }

    /// Returns optional pre-assigned `(x, y)` normalized coordinates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source graph node.
    ///
    /// # Returns
    ///
    /// - `(Option<f32>, Option<f32>)` - Optional preset position.
    #[must_use]
    pub fn pos_opt(&self) -> (Option<f32>, Option<f32>) {
        let Self { x, y, .. } = *self;
        (x, y)
    }

    /// Returns optional explicit vertex color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source graph node.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit node color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Constructors for `EuvChartGraphEdge`.
impl EuvChartGraphEdge {
    /// Creates an edge connecting `source` and `target`.
    ///
    /// # Arguments
    ///
    /// - `S` - Source node id.
    /// - `T` - Target node id.
    ///
    /// # Returns
    ///
    /// - `Self` - Graph edge.
    #[must_use]
    pub fn new<S, T>(source: S, target: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            source: source.into(),
            target: target.into(),
            weight: 1.5,
            label: String::new(),
            curveness: 0.14,
        }
    }

    /// Sets edge curve bow ratio (`0.0` = straight).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source edge.
    /// - `f32` - Curveness ratio.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated edge.
    #[must_use]
    pub fn curveness(mut self, curveness: f32) -> Self {
        self.set_curveness(curveness);
        self
    }
}

/// Constructors for `EuvChartGraphSeries`.
impl EuvChartGraphSeries {
    /// Creates a network graph series from vertices and edges.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartGraphNode>` - Vertices.
    /// - `Vec<EuvChartGraphEdge>` - Edges.
    ///
    /// # Returns
    ///
    /// - `Self` - Graph series.
    #[must_use]
    pub fn new(nodes: Vec<EuvChartGraphNode>, edges: Vec<EuvChartGraphEdge>) -> Self {
        let mut s: Self = Self::default();
        s.set_nodes(nodes);
        s.set_edges(edges);
        s
    }

    /// Selects the graph layout algorithm (`Force`, `Circular`, or `None`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source graph series.
    /// - `EuvChartGraphLayout` - Layout strategy.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated graph series.
    #[must_use]
    pub fn layout(mut self, layout: EuvChartGraphLayout) -> Self {
        self.set_layout(layout);
        self
    }
}

/// Constructors for `EuvChartTreeSeries`.
impl EuvChartTreeSeries {
    /// Creates a tree series rooted at `root`.
    ///
    /// # Arguments
    ///
    /// - `EuvChartHierarchyNode` - Tree root node.
    ///
    /// # Returns
    ///
    /// - `Self` - Tree series.
    #[must_use]
    pub fn new(root: EuvChartHierarchyNode) -> Self {
        let mut s: Self = Self::default();
        s.set_root(root);
        s
    }

    /// Selects tree layout geometry (`OrthogonalLeftRight`, `OrthogonalTopBottom`, or `Radial`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source tree series.
    /// - `EuvChartTreeLayout` - Tree layout mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated tree series.
    #[must_use]
    pub fn layout(mut self, layout: EuvChartTreeLayout) -> Self {
        self.set_layout(layout);
        self
    }
}

/// Constructors and accessors for `EuvChartParallelAxis`, `EuvChartParallelRow`, and `EuvChartParallelSeries`.
impl EuvChartParallelAxis {
    /// Creates a numeric parallel dimension axis.
    ///
    /// # Arguments
    ///
    /// - `S` - Dimension caption.
    ///
    /// # Returns
    ///
    /// - `Self` - Parallel axis.
    #[must_use]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            min: None,
            max: None,
            categories: Vec::new(),
        }
    }

    /// Sets explicit lower and upper numeric bounds for this dimension.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source parallel axis.
    /// - `f32` - Minimum bound.
    /// - `f32` - Maximum bound.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated parallel axis.
    #[must_use]
    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.set_min(Some(min));
        self.set_max(Some(max));
        self
    }

    /// Returns optional `(min, max)` bounds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source parallel axis.
    ///
    /// # Returns
    ///
    /// - `(Option<f32>, Option<f32>)` - Optional domain bounds.
    #[must_use]
    pub fn bounds_opt(&self) -> (Option<f32>, Option<f32>) {
        let Self { min, max, .. } = *self;
        (min, max)
    }
}

/// Constructors and accessors for `EuvChartParallelRow`.
impl EuvChartParallelRow {
    /// Creates a parallel coordinates observation row.
    ///
    /// # Arguments
    ///
    /// - `S` - Observation label.
    /// - `Vec<f32>` - Dimension values.
    /// - `usize` - Category index.
    ///
    /// # Returns
    ///
    /// - `Self` - Parallel row.
    #[must_use]
    pub fn new<S>(name: S, values: Vec<f32>, category: usize) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            values,
            category,
            color: None,
        }
    }

    /// Returns optional explicit stroke color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source row.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit stroke color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Constructors for `EuvChartParallelSeries`.
impl EuvChartParallelSeries {
    /// Creates a parallel coordinates series from axes and observation rows.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartParallelAxis>` - Dimension axes.
    /// - `Vec<EuvChartParallelRow>` - Observation rows.
    ///
    /// # Returns
    ///
    /// - `Self` - Parallel series.
    #[must_use]
    pub fn new(axes: Vec<EuvChartParallelAxis>, rows: Vec<EuvChartParallelRow>) -> Self {
        let mut s: Self = Self::default();
        s.set_axes(axes);
        s.set_rows(rows);
        s
    }
}

/// Constructors and accessors for `EuvChartFlowArc` and `EuvChartLinesSeries`.
impl EuvChartFlowArc {
    /// Creates a directed curved flow arc between `(from_x, from_y)` and `(to_x, to_y)`.
    ///
    /// # Arguments
    ///
    /// - `A` - Origin label.
    /// - `B` - Destination label.
    /// - `(f32, f32)` - Origin `(x, y)` or `(lon, lat)`.
    /// - `(f32, f32)` - Destination `(x, y)` or `(lon, lat)`.
    /// - `f32` - Flow magnitude value.
    ///
    /// # Returns
    ///
    /// - `Self` - Flow arc descriptor.
    #[must_use]
    pub fn new<A, B>(from_name: A, to_name: B, from: (f32, f32), to: (f32, f32), value: f32) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            from_name: from_name.into(),
            to_name: to_name.into(),
            from_x: from.0,
            from_y: from.1,
            to_x: to.0,
            to_y: to.1,
            value,
            color: None,
        }
    }

    /// Returns optional explicit arc color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source flow arc.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit arc stroke color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Constructors for `EuvChartLinesSeries`.
impl EuvChartLinesSeries {
    /// Creates a curved directional lines series from arcs.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartFlowArc>` - Directed arcs.
    ///
    /// # Returns
    ///
    /// - `Self` - Lines series.
    #[must_use]
    pub fn new(arcs: Vec<EuvChartFlowArc>) -> Self {
        let mut s: Self = Self::default();
        s.set_arcs(arcs);
        s
    }

    /// Binds this lines series to `Cartesian2d` or `Geo` coordinates.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source lines series.
    /// - `EuvChartCoordinateSystem` - Coordinate system.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated lines series.
    #[must_use]
    pub fn coordinate_system(mut self, coord: EuvChartCoordinateSystem) -> Self {
        self.set_coordinate_system(coord);
        self
    }
}

/// Constructors for `EuvChartThemeRiverSeries`, `EuvChartPictorialBarSeries`, `EuvChartCustomSeries`, `EuvChartCustomDrawContext`, `EuvChartGeoDatum`, and `EuvChartGeoMapSeries`.
impl EuvChartThemeRiverSeries {
    /// Creates a ThemeRiver streamgraph from `(time, value, theme)` observations.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartThemeRiverPoint>` - Stream observations.
    ///
    /// # Returns
    ///
    /// - `Self` - ThemeRiver series.
    #[must_use]
    pub fn new(points: Vec<EuvChartThemeRiverPoint>) -> Self {
        let mut s: Self = Self::default();
        s.set_points(points);
        s
    }
}

/// Constructors for `EuvChartPictorialBarSeries`.
impl EuvChartPictorialBarSeries {
    /// Creates a stacked symbol pictorial bar series.
    ///
    /// # Arguments
    ///
    /// - `Vec<f32>` - Category values.
    /// - `EuvChartSymbol` - Repeated glyph symbol.
    ///
    /// # Returns
    ///
    /// - `Self` - Pictorial bar series.
    #[must_use]
    pub fn new(values: Vec<f32>, symbol: EuvChartSymbol) -> Self {
        let mut s: Self = Self::default();
        s.set_values(values);
        s.set_symbol(symbol);
        s
    }
}

/// Accessors for `EuvChartCustomDrawContext`.
impl EuvChartCustomDrawContext {
    /// Returns the active pointer coordinates when hovering the plot area.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Draw context.
    ///
    /// # Returns
    ///
    /// - `Option<Point>` - Pointer coordinates.
    #[must_use]
    pub fn hover_pos_opt(&self) -> Option<Point> {
        let Self { hover_pos, .. } = *self;
        hover_pos
    }
}

/// Constructors and accessors for `EuvChartCustomSeries`.
impl EuvChartCustomSeries {
    /// Creates a custom vector painter series.
    ///
    /// # Arguments
    ///
    /// - `Vec<f32>` - Numeric reference values used for auto-scaling.
    /// - `F` - Custom `DrawCommand` painter closure.
    ///
    /// # Returns
    ///
    /// - `Self` - Custom series.
    #[must_use]
    pub fn new<F>(values: Vec<f32>, painter: F) -> Self
    where
        F: Fn(Rect, &EuvChartCustomDrawContext, &mut DisplayList) + 'static,
    {
        let mut s: Self = Self::default();
        s.set_values(values);
        s.set_painter(Some(Rc::new(painter)));
        s
    }

    /// Returns the optional custom painter handle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Custom series.
    ///
    /// # Returns
    ///
    /// - `Option<EuvChartCustomPainter>` - Cloned painter handle.
    #[must_use]
    pub fn painter_opt(&self) -> Option<EuvChartCustomPainter> {
        let Self { painter, .. } = self;
        painter.clone()
    }
}

/// Constructors and accessors for `EuvChartGeoDatum` and `EuvChartGeoMapSeries`.
impl EuvChartGeoDatum {
    /// Creates a region choropleth metric entry.
    ///
    /// # Arguments
    ///
    /// - `S` - Region name.
    /// - `f32` - Numeric intensity.
    ///
    /// # Returns
    ///
    /// - `Self` - Geo datum.
    #[must_use]
    pub fn new<S>(region_name: S, value: f32) -> Self
    where
        S: Into<String>,
    {
        Self {
            region_name: region_name.into(),
            value,
            color: None,
        }
    }

    /// Returns optional explicit region color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Geo datum.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit fill color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Constructors for `EuvChartGeoMapSeries`.
impl EuvChartGeoMapSeries {
    /// Creates a geographic choropleth map series.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvChartGeoDatum>` - Region metric entries.
    ///
    /// # Returns
    ///
    /// - `Self` - GeoMap series.
    #[must_use]
    pub fn new(data: Vec<EuvChartGeoDatum>) -> Self {
        let mut s: Self = Self::default();
        s.set_data(data);
        s
    }
}

/// Constructors and accessors for `EuvChartSeries`.
impl EuvChartSeries {
    /// Creates a visible named series wrapping `data`.
    ///
    /// # Arguments
    ///
    /// - `S` - Legend and tooltip series name.
    /// - `EuvChartSeriesData` - Typed series payload.
    ///
    /// # Returns
    ///
    /// - `Self` - Named series entry.
    #[must_use]
    pub fn new<S>(name: S, data: EuvChartSeriesData) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            color: None,
            visible: true,
            data,
        }
    }

    /// Overrides the series color fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source series.
    /// - `Color` - Explicit tint color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated series.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.set_color(Some(color));
        self
    }

    /// Sets initial series visibility.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source series.
    /// - `bool` - Initial visibility flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated series.
    #[must_use]
    pub fn visible(mut self, visible: bool) -> Self {
        self.set_visible(visible);
        self
    }

    /// Returns the optional explicit series color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source series.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit series color.
    #[must_use]
    pub fn color_opt(&self) -> Option<Color> {
        let Self { color, .. } = *self;
        color
    }
}

/// Fluent declarative builders and safe option accessors for `EuvChartOption`.
impl EuvChartOption {
    /// Creates an empty declarative chart option ready for fluent chaining.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty chart option.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the chart title block fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartTitle` - Title configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn title(mut self, title: EuvChartTitle) -> Self {
        self.set_title(Some(title));
        self
    }

    /// Sets the interactive legend configuration fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartLegend` - Legend configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn legend(mut self, legend: EuvChartLegend) -> Self {
        self.set_legend(Some(legend));
        self
    }

    /// Appends a Cartesian subplot grid fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartGrid` - Subplot grid specification.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn add_grid(mut self, grid: EuvChartGrid) -> Self {
        self.get_mut_grid().push(grid);
        self
    }

    /// Appends an X axis fluently (supporting dual or multi-axis layouts).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartAxis` - X axis configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn x_axis(mut self, axis: EuvChartAxis) -> Self {
        self.get_mut_x_axes().push(axis);
        self
    }

    /// Appends a Y axis fluently (supporting dual or multi-axis layouts).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartAxis` - Y axis configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn y_axis(mut self, axis: EuvChartAxis) -> Self {
        self.get_mut_y_axes().push(axis);
        self
    }

    /// Sets the polar coordinate system fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartPolar` - Polar coordinate configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn polar(mut self, polar: EuvChartPolar) -> Self {
        self.set_polar(Some(polar));
        self
    }

    /// Sets the radar coordinate system fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartRadar` - Radar coordinate configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn radar(mut self, radar: EuvChartRadar) -> Self {
        self.set_radar(Some(radar));
        self
    }

    /// Sets the single-axis coordinate system fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartSingleAxis` - Single-axis configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn single_axis(mut self, single_axis: EuvChartSingleAxis) -> Self {
        self.set_single_axis(Some(single_axis));
        self
    }

    /// Sets the calendar coordinate system fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartCalendar` - Calendar configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn calendar(mut self, calendar: EuvChartCalendar) -> Self {
        self.set_calendar(Some(calendar));
        self
    }

    /// Sets the geographic coordinate system fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartGeo` - Geo configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn geo(mut self, geo: EuvChartGeo) -> Self {
        self.set_geo(Some(geo));
        self
    }

    /// Sets the interactive hover tooltip configuration fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartTooltip` - Tooltip configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn tooltip(mut self, tooltip: EuvChartTooltip) -> Self {
        self.set_tooltip(Some(tooltip));
        self
    }

    /// Appends a data-zoom controller (`Inside` or `Slider`) fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartDataZoom` - Data-zoom controller.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn data_zoom(mut self, zoom: EuvChartDataZoom) -> Self {
        self.get_mut_data_zoom().push(zoom);
        self
    }

    /// Sets the continuous or piecewise visual-map controller fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartVisualMap` - Visual-map configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn visual_map(mut self, visual_map: EuvChartVisualMap) -> Self {
        self.set_visual_map(Some(visual_map));
        self
    }

    /// Sets the compact icon-only toolbox toolbar fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartToolbox` - Toolbox configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn toolbox(mut self, toolbox: EuvChartToolbox) -> Self {
        self.set_toolbox(Some(toolbox));
        self
    }

    /// Sets a shared tabular dataset fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartDataset` - Shared dataset.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn dataset(mut self, dataset: EuvChartDataset) -> Self {
        self.set_dataset(Some(dataset));
        self
    }

    /// Overrides the categorical color palette fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `Vec<Color>` - Custom ordered palette colors.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn color_palette(mut self, colors: Vec<Color>) -> Self {
        self.set_color_palette(Some(colors));
        self
    }

    /// Appends a typed series fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `EuvChartSeries` - Named series entry.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn add_series(mut self, series: EuvChartSeries) -> Self {
        self.get_mut_series().push(series);
        self
    }

    /// Overrides the canvas background color fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart option.
    /// - `Color` - Explicit background color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart option.
    #[must_use]
    pub fn background(mut self, color: Color) -> Self {
        self.set_background(Some(color));
        self
    }

    /// Resolves the active categorical color palette for `tokens`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source chart option.
    /// - `ThemeTokens` - Active UI theme tokens.
    ///
    /// # Returns
    ///
    /// - `Vec<Color>` - Non-empty high-contrast categorical color palette.
    #[must_use]
    pub fn resolved_palette(&self, tokens: ThemeTokens) -> Vec<Color> {
        let Self { color_palette, .. } = self;
        if let Some(custom) = color_palette.as_ref()
            && !custom.is_empty()
        {
            return custom.clone();
        }
        let hexes: &[u32; 10] = match tokens.get_mode() {
            ThemeMode::Light => &ECHART_PALETTE_LIGHT,
            ThemeMode::Dark => &ECHART_PALETTE_DARK,
        };
        let mut out: Vec<Color> = Vec::with_capacity(hexes.len());
        for &hex in hexes.iter() {
            out.push(Color::hex(hex));
        }
        out
    }

    /// Returns the optional chart title reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartTitle>` - Title configuration.
    #[must_use]
    pub fn title_opt(&self) -> Option<&EuvChartTitle> {
        let Self { title, .. } = self;
        title.as_ref()
    }

    /// Returns the optional legend reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartLegend>` - Legend configuration.
    #[must_use]
    pub fn legend_opt(&self) -> Option<&EuvChartLegend> {
        let Self { legend, .. } = self;
        legend.as_ref()
    }

    /// Returns the optional polar coordinate system reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartPolar>` - Polar coordinate configuration.
    #[must_use]
    pub fn polar_opt(&self) -> Option<&EuvChartPolar> {
        let Self { polar, .. } = self;
        polar.as_ref()
    }

    /// Returns the optional radar coordinate system reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartRadar>` - Radar coordinate configuration.
    #[must_use]
    pub fn radar_opt(&self) -> Option<&EuvChartRadar> {
        let Self { radar, .. } = self;
        radar.as_ref()
    }

    /// Returns the optional single-axis coordinate system reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartSingleAxis>` - Single-axis configuration.
    #[must_use]
    pub fn single_axis_opt(&self) -> Option<&EuvChartSingleAxis> {
        let Self { single_axis, .. } = self;
        single_axis.as_ref()
    }

    /// Returns the optional calendar coordinate system reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartCalendar>` - Calendar configuration.
    #[must_use]
    pub fn calendar_opt(&self) -> Option<&EuvChartCalendar> {
        let Self { calendar, .. } = self;
        calendar.as_ref()
    }

    /// Returns the optional geographic coordinate system reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartGeo>` - Geo coordinate configuration.
    #[must_use]
    pub fn geo_opt(&self) -> Option<&EuvChartGeo> {
        let Self { geo, .. } = self;
        geo.as_ref()
    }

    /// Returns the optional tooltip configuration reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartTooltip>` - Tooltip configuration.
    #[must_use]
    pub fn tooltip_opt(&self) -> Option<&EuvChartTooltip> {
        let Self { tooltip, .. } = self;
        tooltip.as_ref()
    }

    /// Returns the optional visual-map configuration reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartVisualMap>` - Visual-map configuration.
    #[must_use]
    pub fn visual_map_opt(&self) -> Option<&EuvChartVisualMap> {
        let Self { visual_map, .. } = self;
        visual_map.as_ref()
    }

    /// Returns the optional toolbox configuration reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartToolbox>` - Toolbox configuration.
    #[must_use]
    pub fn toolbox_opt(&self) -> Option<&EuvChartToolbox> {
        let Self { toolbox, .. } = self;
        toolbox.as_ref()
    }

    /// Returns the optional shared dataset reference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<&EuvChartDataset>` - Shared dataset.
    #[must_use]
    pub fn dataset_opt(&self) -> Option<&EuvChartDataset> {
        let Self { dataset, .. } = self;
        dataset.as_ref()
    }

    /// Returns the optional explicit background color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart option.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Explicit background color.
    #[must_use]
    pub fn background_opt(&self) -> Option<Color> {
        let Self { background, .. } = *self;
        background
    }
}
