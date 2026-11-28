use super::*;

/// Converts a static string slice array into an owned `Vec<String>`.
///
/// # Arguments
///
/// - `&[&str]` - Static string slice labels.
///
/// # Returns
///
/// - `Vec<String>` - Owned category labels.
fn owned_labels(labels: &[&str]) -> Vec<String> {
    labels.iter().map(|s: &&str| (*s).to_string()).collect()
}

/// Builds Card 1 option: Line series with smooth stacked area, dashed step line, Marks, DataZoom, Cross AxisPointer, and full Toolbox.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Line chart option.
fn build_line_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_LINE_CHART).subtext(SUB_LINE_CHART))
        .legend(EuvChartLegend::at(EuvChartPosition::Top))
        .tooltip(EuvChartTooltip::axis(EuvChartAxisPointerType::Cross))
        .toolbox(EuvChartToolbox::full())
        .data_zoom(EuvChartDataZoom::inside())
        .data_zoom(EuvChartDataZoom::slider().range(0.0, 100.0))
        .x_axis(EuvChartAxis::category(owned_labels(&DAYS_EIGHT)))
        .y_axis(EuvChartAxis::value().unit_suffix("k"))
        .add_series(EuvChartSeries::new(
            SER_DIRECT,
            EuvChartSeriesData::Line(
                EuvChartLineSeries::from_values(vec![
                    120.0, 165.0, 148.0, 224.0, 286.0, 260.0, 318.0, 342.0,
                ])
                .smooth(true)
                .area(0.28)
                .stack(STACK_TRAFFIC)
                .mark_point(EuvChartMarkPoint::new(
                    EuvChartMarkPointKind::Max,
                    MARK_PEAK,
                ))
                .mark_line(EuvChartMarkLine::new(
                    EuvChartMarkLineKind::Average,
                    MARK_AVG,
                ))
                .mark_area(EuvChartMarkArea::x_range(3.0, 5.0, MARK_PROMO)),
            ),
        ))
        .add_series(EuvChartSeries::new(
            SER_ORGANIC,
            EuvChartSeriesData::Line(
                EuvChartLineSeries::from_values(vec![
                    64.0, 82.0, 96.0, 112.0, 128.0, 118.0, 146.0, 160.0,
                ])
                .smooth(true)
                .area(0.22)
                .stack(STACK_TRAFFIC),
            ),
        ))
        .add_series(EuvChartSeries::new(
            SER_SEARCH,
            EuvChartSeriesData::Line(
                EuvChartLineSeries::from_values(vec![
                    90.0, 110.0, 104.0, 152.0, 178.0, 162.0, 205.0, 226.0,
                ])
                .step(EuvChartStepMode::Middle)
                .dashed(true)
                .symbol(EuvChartSymbol::Diamond, 4.0),
            ),
        ))
}

/// Builds Card 2 option: Grouped/stacked Bar + dual-axis Margin Line backed by tabular `Dataset` & `Encode`.
///
/// # Returns
///
/// - `EuvChartOption` - Configured dual-axis Dataset chart option.
fn build_bar_dataset_option() -> EuvChartOption {
    let rev_vals: [f32; 6] = [120.0, 152.0, 188.0, 174.0, 232.0, 276.0];
    let prof_vals: [f32; 6] = [34.0, 44.0, 58.0, 52.0, 76.0, 94.0];
    let margin_vals: [f32; 6] = [28.0, 29.0, 31.0, 30.0, 33.0, 34.0];
    let mut rows: Vec<Vec<EuvChartCell>> = Vec::with_capacity(6);
    for idx in 0..6usize {
        rows.push(vec![
            EuvChartCell::Text(MONTHS_SIX[idx].to_string()),
            EuvChartCell::Number(rev_vals[idx]),
            EuvChartCell::Number(prof_vals[idx]),
            EuvChartCell::Number(margin_vals[idx]),
        ]);
    }
    let ds: EuvChartDataset = EuvChartDataset::new(
        vec![
            DIM_MONTH.to_string(),
            DIM_REVENUE.to_string(),
            DIM_PROFIT.to_string(),
            DIM_MARGIN.to_string(),
        ],
        rows,
    );
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_BAR_CHART))
        .legend(EuvChartLegend::at(EuvChartPosition::Top))
        .tooltip(EuvChartTooltip::axis(EuvChartAxisPointerType::Shadow))
        .toolbox(EuvChartToolbox::full())
        .dataset(ds)
        .y_axis(EuvChartAxis::value().name(DIM_REVENUE).unit_suffix("k"))
        .y_axis(
            EuvChartAxis::value()
                .name(DIM_MARGIN)
                .unit_suffix("%")
                .position(EuvChartPosition::Right),
        )
        .add_series(EuvChartSeries::new(
            DIM_REVENUE,
            EuvChartSeriesData::Bar(
                EuvChartBarSeries::default()
                    .axes(0, 0)
                    .stack(STACK_REV)
                    .show_background(true)
                    .encode(EuvChartEncode::default().x(DIM_MONTH).y(DIM_REVENUE)),
            ),
        ))
        .add_series(EuvChartSeries::new(
            DIM_PROFIT,
            EuvChartSeriesData::Bar(
                EuvChartBarSeries::default()
                    .axes(0, 0)
                    .stack(STACK_REV)
                    .encode(EuvChartEncode::default().x(DIM_MONTH).y(DIM_PROFIT)),
            ),
        ))
        .add_series(EuvChartSeries::new(
            DIM_MARGIN,
            EuvChartSeriesData::Line(
                EuvChartLineSeries::default()
                    .axes(0, 1)
                    .smooth(true)
                    .encode(EuvChartEncode::default().x(DIM_MONTH).y(DIM_MARGIN)),
            ),
        ))
}

/// Builds Card 3 option: Horizontal Category-Y Bar chart with background tracks and value labels.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Horizontal Bar chart option.
fn build_horizontal_bar_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_HBAR_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .x_axis(EuvChartAxis::value().max(100.0).unit_suffix("%"))
        .y_axis(EuvChartAxis::category(owned_labels(&PLATFORMS_FIVE)))
        .add_series(EuvChartSeries::new(
            SER_SHARE,
            EuvChartSeriesData::Bar(
                EuvChartBarSeries::from_values(vec![94.0, 91.0, 88.0, 85.0, 79.0])
                    .show_background(true)
                    .show_labels(true),
            ),
        ))
}

/// Builds Card 4 option: Scatter & EffectScatter bubble clusters shaded via Continuous VisualMap.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Scatter + EffectScatter chart option.
fn build_scatter_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_SCATTER_CHART))
        .legend(EuvChartLegend::at(EuvChartPosition::Top))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .visual_map(EuvChartVisualMap::continuous(10.0, 100.0))
        .x_axis(EuvChartAxis::value().min(0.0).max(100.0))
        .y_axis(EuvChartAxis::value().min(0.0).max(100.0))
        .add_series(EuvChartSeries::new(
            SER_CLUSTERS,
            EuvChartSeriesData::Scatter(
                EuvChartScatterSeries::new(vec![
                    EuvChartScatterPoint::new(14.0, 26.0)
                        .size(7.0)
                        .label(PT_EDGE_A),
                    EuvChartScatterPoint::new(28.0, 44.0).size(9.5),
                    EuvChartScatterPoint::new(42.0, 58.0)
                        .size(11.0)
                        .label(PT_EDGE_B),
                    EuvChartScatterPoint::new(56.0, 49.0).size(8.5),
                    EuvChartScatterPoint::new(70.0, 74.0)
                        .size(12.0)
                        .label(PT_CORE_C),
                ])
                .symbol(EuvChartSymbol::Circle, 7.0)
                .show_labels(true),
            ),
        ))
        .add_series(EuvChartSeries::new(
            SER_HOTSPOTS,
            EuvChartSeriesData::EffectScatter(
                EuvChartScatterSeries::new(vec![
                    EuvChartScatterPoint::new(82.0, 88.0)
                        .size(10.0)
                        .label(PT_BURST_D),
                    EuvChartScatterPoint::new(34.0, 76.0).size(8.0),
                ])
                .show_labels(true),
            ),
        ))
}

/// Builds Card 5 option: OHLC Candlestick (K-Line) overlaid with smooth MA5 Line and DataZoom Slider.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Candlestick + MA5 chart option.
fn build_candlestick_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_CANDLE_CHART))
        .legend(EuvChartLegend::at(EuvChartPosition::Top))
        .tooltip(EuvChartTooltip::axis(EuvChartAxisPointerType::Cross))
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .data_zoom(EuvChartDataZoom::inside())
        .data_zoom(EuvChartDataZoom::slider().range(0.0, 100.0))
        .x_axis(EuvChartAxis::category(owned_labels(&DAYS_EIGHT)))
        .y_axis(EuvChartAxis::value())
        .add_series(EuvChartSeries::new(
            SER_KLINE,
            EuvChartSeriesData::Candlestick(EuvChartCandlestickSeries::new(vec![
                EuvChartOhlcPoint::new(102.0, 114.0, 98.0, 119.0),
                EuvChartOhlcPoint::new(114.0, 108.0, 103.0, 117.0),
                EuvChartOhlcPoint::new(108.0, 124.0, 106.0, 129.0),
                EuvChartOhlcPoint::new(124.0, 119.0, 115.0, 128.0),
                EuvChartOhlcPoint::new(119.0, 134.0, 117.0, 139.0),
                EuvChartOhlcPoint::new(134.0, 142.0, 130.0, 147.0),
                EuvChartOhlcPoint::new(142.0, 136.0, 131.0, 145.0),
                EuvChartOhlcPoint::new(136.0, 149.0, 134.0, 154.0),
            ])),
        ))
        .add_series(EuvChartSeries::new(
            SER_MA5,
            EuvChartSeriesData::Line(
                EuvChartLineSeries::from_values(vec![
                    108.0, 110.0, 114.0, 117.0, 122.0, 129.0, 134.0, 140.0,
                ])
                .smooth(true),
            ),
        ))
}

/// Builds Card 6 option: 5-number Boxplot distribution with outlier points.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Boxplot chart option.
fn build_boxplot_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_BOXPLOT_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .x_axis(EuvChartAxis::category(owned_labels(&BACKENDS_THREE)))
        .y_axis(EuvChartAxis::value().unit_suffix("ms"))
        .add_series(EuvChartSeries::new(
            SER_LATENCY,
            EuvChartSeriesData::Boxplot(EuvChartBoxplotSeries::new(vec![
                EuvChartBoxplotItem::new(3.6, 5.4, 7.2, 9.8, 12.4).outliers(vec![15.6]),
                EuvChartBoxplotItem::new(1.8, 2.9, 3.8, 5.1, 6.9).outliers(vec![9.2]),
                EuvChartBoxplotItem::new(0.8, 1.4, 2.1, 3.0, 4.2),
            ])),
        ))
}

/// Builds Card 7 option: Cartesian 2D Heatmap shaded via Continuous VisualMap.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Cartesian Heatmap chart option.
fn build_heatmap_option() -> EuvChartOption {
    let vals: [[f32; 6]; 4] = [
        [14.0, 26.0, 52.0, 74.0, 66.0, 38.0],
        [18.0, 34.0, 68.0, 88.0, 79.0, 46.0],
        [12.0, 28.0, 62.0, 95.0, 84.0, 52.0],
        [20.0, 40.0, 72.0, 91.0, 76.0, 44.0],
    ];
    let mut cells: Vec<EuvChartHeatmapCell> = Vec::with_capacity(24);
    for (y_i, row) in vals.iter().enumerate() {
        for (x_i, &v) in row.iter().enumerate() {
            cells.push(EuvChartHeatmapCell::new(x_i, y_i, v));
        }
    }
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_HEATMAP_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .x_axis(EuvChartAxis::category(owned_labels(&HOURS_SIX)))
        .y_axis(EuvChartAxis::category(owned_labels(&WEEKDAYS_FOUR)))
        .visual_map(EuvChartVisualMap::continuous(0.0, 100.0))
        .add_series(EuvChartSeries::new(
            SER_LOAD,
            EuvChartSeriesData::Heatmap(EuvChartHeatmapSeries::new(cells)),
        ))
}

/// Builds Card 8 option: Calendar date-grid Heatmap shaded via Piecewise VisualMap.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Calendar Heatmap chart option.
fn build_calendar_heatmap_option() -> EuvChartOption {
    let mut cells: Vec<EuvChartHeatmapCell> = Vec::with_capacity(96);
    for w in 0..16usize {
        for d in 0..7usize {
            let v: f32 = (((w * 7 + d * 5 + 3) % 21) + 1) as f32;
            cells.push(EuvChartHeatmapCell::new(w, d, v));
        }
    }
    let piece_vm: EuvChartVisualMap = EuvChartVisualMap::piecewise(vec![
        EuvChartVisualPiece::new(None, Some(7.0), VM_QUIET, Color::hex(0x93C5FD)),
        EuvChartVisualPiece::new(Some(7.0), Some(14.0), VM_ACTIVE, Color::hex(0x3B82F6)),
        EuvChartVisualPiece::new(Some(14.0), None, VM_PEAK, Color::hex(0x1D4ED8)),
    ]);
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_CALENDAR_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .calendar(EuvChartCalendar::new(2026, 16))
        .visual_map(piece_vm)
        .add_series(EuvChartSeries::new(
            SER_COMMITS,
            EuvChartSeriesData::Heatmap(
                EuvChartHeatmapSeries::new(cells)
                    .coordinate_system(EuvChartCoordinateSystem::Calendar),
            ),
        ))
}

/// Builds Card 9 option: Donut Nightingale Rose Pie chart with interactive Legend and pop-out slice.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Pie/Rose chart option.
fn build_pie_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_PIE_CHART))
        .legend(EuvChartLegend::at(EuvChartPosition::Bottom))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .add_series(EuvChartSeries::new(
            SER_TRAFFIC,
            EuvChartSeriesData::Pie(
                EuvChartPieSeries::new(vec![
                    EuvChartPieSlice::new(SLICE_SEARCH, 420.0).selected(true),
                    EuvChartPieSlice::new(SER_DIRECT, 310.0),
                    EuvChartPieSlice::new(SLICE_REFERRAL, 230.0),
                    EuvChartPieSlice::new(SLICE_SOCIAL, 175.0),
                    EuvChartPieSlice::new(SLICE_EMAIL, 120.0),
                ])
                .donut(0.32)
                .rose(EuvChartRoseType::Radius)
                .pad_angle(2.0),
            ),
        ))
}

/// Builds Card 10 option: Multi-series Radar polygon web comparing native euv vs WebView.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Radar chart option.
fn build_radar_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_RADAR_CHART))
        .legend(EuvChartLegend::at(EuvChartPosition::Top))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .radar(
            EuvChartRadar::new(vec![
                EuvChartRadarIndicator::new(IND_FPS.to_string(), 0.0, 120.0),
                EuvChartRadarIndicator::new(IND_MEM.to_string(), 0.0, 100.0),
                EuvChartRadarIndicator::new(IND_STARTUP.to_string(), 0.0, 100.0),
                EuvChartRadarIndicator::new(IND_BINARY.to_string(), 0.0, 100.0),
                EuvChartRadarIndicator::new(IND_BATTERY.to_string(), 0.0, 100.0),
            ])
            .shape(EuvChartRadarShape::Polygon),
        )
        .add_series(EuvChartSeries::new(
            SER_PROFILE,
            EuvChartSeriesData::Radar(EuvChartRadarSeries::new(vec![
                EuvChartRadarItem::new(ITEM_EUV_NATIVE, vec![120.0, 94.0, 97.0, 93.0, 92.0]),
                EuvChartRadarItem::new(ITEM_WEBVIEW, vec![60.0, 48.0, 44.0, 38.0, 52.0]),
            ])),
        ))
}

/// Builds Card 11 option: Multi-zone telemetry Gauge dial.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Gauge chart option.
fn build_gauge_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_GAUGE_CHART))
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .add_series(EuvChartSeries::new(
            SER_BUDGET,
            EuvChartSeriesData::Gauge(
                EuvChartGaugeSeries::new(84.5, 0.0, 100.0)
                    .unit("%")
                    .title(GAUGE_EFFICIENCY),
            ),
        ))
}

/// Builds Card 12 option: Squarified nested Treemap.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Treemap chart option.
fn build_treemap_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_TREEMAP_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .add_series(EuvChartSeries::new(
            SER_FOOTPRINT,
            EuvChartSeriesData::Treemap(EuvChartTreemapSeries::new(vec![
                EuvChartHierarchyNode::branch(
                    H_CORE,
                    vec![
                        EuvChartHierarchyNode::leaf(H_LAYOUT, 145.0),
                        EuvChartHierarchyNode::leaf(H_RENDER, 240.0),
                        EuvChartHierarchyNode::leaf(H_SIGNALS, 88.0),
                    ],
                ),
                EuvChartHierarchyNode::branch(
                    H_UI_SUITE,
                    vec![
                        EuvChartHierarchyNode::leaf(H_CHARTS, 175.0),
                        EuvChartHierarchyNode::leaf(H_MARKDOWN, 110.0),
                        EuvChartHierarchyNode::leaf(H_ROUTER, 72.0),
                    ],
                ),
            ])),
        ))
}

/// Builds Card 13 option: Concentric multi-ring Sunburst hierarchy.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Sunburst chart option.
fn build_sunburst_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_SUNBURST_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .add_series(EuvChartSeries::new(
            SER_TAXONOMY,
            EuvChartSeriesData::Sunburst(EuvChartSunburstSeries::new(vec![
                EuvChartHierarchyNode::branch(
                    H_NATIVE,
                    vec![
                        EuvChartHierarchyNode::leaf(H_METAL, 48.0),
                        EuvChartHierarchyNode::leaf(H_VULKAN, 42.0),
                        EuvChartHierarchyNode::leaf(H_DX12, 26.0),
                    ],
                ),
                EuvChartHierarchyNode::branch(
                    H_BROWSER,
                    vec![
                        EuvChartHierarchyNode::leaf(PLATFORMS_FIVE[0], 34.0),
                        EuvChartHierarchyNode::leaf(H_CANVAS2D, 22.0),
                    ],
                ),
            ])),
        ))
}

/// Builds Card 14 option: Multi-column Sankey flow diagram.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Sankey chart option.
fn build_sankey_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_SANKEY_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .add_series(EuvChartSeries::new(
            SER_FLOW,
            EuvChartSeriesData::Sankey(EuvChartSankeySeries::new(
                vec![
                    EuvChartSankeyNode::new(SER_ORGANIC),
                    EuvChartSankeyNode::new(SK_PAID_ADS),
                    EuvChartSankeyNode::new(SK_LANDING),
                    EuvChartSankeyNode::new(SK_TRIAL),
                    EuvChartSankeyNode::new(SK_SIGNUP),
                    EuvChartSankeyNode::new(SK_ENTERPRISE),
                ],
                vec![
                    EuvChartSankeyLink::new(SER_ORGANIC.to_string(), SK_LANDING.to_string(), 64.0),
                    EuvChartSankeyLink::new(SK_PAID_ADS.to_string(), SK_LANDING.to_string(), 36.0),
                    EuvChartSankeyLink::new(SK_LANDING.to_string(), SK_TRIAL.to_string(), 58.0),
                    EuvChartSankeyLink::new(SK_LANDING.to_string(), SK_SIGNUP.to_string(), 32.0),
                    EuvChartSankeyLink::new(SK_TRIAL.to_string(), SK_ENTERPRISE.to_string(), 28.0),
                    EuvChartSankeyLink::new(SK_SIGNUP.to_string(), SK_ENTERPRISE.to_string(), 18.0),
                ],
            )),
        ))
}

/// Builds Card 15 option: Descending conversion Funnel.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Funnel chart option.
fn build_funnel_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_FUNNEL_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .add_series(EuvChartSeries::new(
            SER_CONVERSION,
            EuvChartSeriesData::Funnel(
                EuvChartFunnelSeries::new(vec![
                    EuvChartFunnelStage::new(FN_VISIT, 100.0),
                    EuvChartFunnelStage::new(FN_EXPLORE, 74.0),
                    EuvChartFunnelStage::new(FN_EVAL, 48.0),
                    EuvChartFunnelStage::new(FN_ADOPT, 26.0),
                ])
                .sort(EuvChartFunnelSort::Descending),
            ),
        ))
}

/// Builds Card 16 option: Relational network Graph in either Force or Circular layout.
///
/// # Arguments
///
/// - `EuvChartGraphLayout` - Active graph layout algorithm (`Force` or `Circular`).
///
/// # Returns
///
/// - `EuvChartOption` - Configured Graph chart option.
fn build_graph_option(layout: EuvChartGraphLayout) -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_GRAPH_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .add_series(EuvChartSeries::new(
            SER_CLUSTER,
            EuvChartSeriesData::Graph(
                EuvChartGraphSeries::new(
                    vec![
                        EuvChartGraphNode::new("gw", GR_GATEWAY, 13.0, 0),
                        EuvChartGraphNode::new("au", GR_AUTH, 10.0, 1),
                        EuvChartGraphNode::new("ca", GR_CACHE, 10.0, 1),
                        EuvChartGraphNode::new("wk", GR_WORKER, 11.0, 2),
                        EuvChartGraphNode::new("db", GR_STORAGE, 12.0, 2),
                        EuvChartGraphNode::new("mt", GR_METRICS, 9.0, 3),
                    ],
                    vec![
                        EuvChartGraphEdge::new("gw", "au"),
                        EuvChartGraphEdge::new("gw", "ca"),
                        EuvChartGraphEdge::new("gw", "wk"),
                        EuvChartGraphEdge::new("au", "db"),
                        EuvChartGraphEdge::new("wk", "db"),
                        EuvChartGraphEdge::new("wk", "mt").curveness(0.15),
                        EuvChartGraphEdge::new("ca", "mt").curveness(0.0),
                    ],
                )
                .layout(layout),
            ),
        ))
}

/// Builds Card 17 option: Hierarchical node-link Tree in Orthogonal or Radial layout.
///
/// # Arguments
///
/// - `EuvChartTreeLayout` - Active tree layout algorithm.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Tree chart option.
fn build_tree_option(layout: EuvChartTreeLayout) -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_TREE_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .add_series(EuvChartSeries::new(
            SER_SCENE_TREE,
            EuvChartSeriesData::Tree(
                EuvChartTreeSeries::new(EuvChartHierarchyNode::branch(
                    TR_ROOT,
                    vec![
                        EuvChartHierarchyNode::branch(
                            TR_SHELL,
                            vec![
                                EuvChartHierarchyNode::leaf(TR_SIDEBAR, 1.0),
                                EuvChartHierarchyNode::leaf(TR_OUTLET, 1.0),
                            ],
                        ),
                        EuvChartHierarchyNode::branch(
                            TR_PORTAL,
                            vec![
                                EuvChartHierarchyNode::leaf(TR_FULLSCREEN, 1.0),
                                EuvChartHierarchyNode::leaf(TR_TOOLTIP, 1.0),
                            ],
                        ),
                    ],
                ))
                .layout(layout),
            ),
        ))
}

/// Builds Card 18 option: World GeoMap choropleth overlaid with curved directional `Lines` and pulsing `EffectScatter` hubs.
///
/// # Returns
///
/// - `EuvChartOption` - Configured GeoMap + Lines + EffectScatter chart option.
fn build_geo_lines_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_GEO_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .geo(EuvChartGeo::world())
        .visual_map(EuvChartVisualMap::continuous(0.0, 100.0))
        .add_series(EuvChartSeries::new(
            SER_USERS,
            EuvChartSeriesData::GeoMap(EuvChartGeoMapSeries::new(vec![
                EuvChartGeoDatum::new(GEO_CHINA, 96.0),
                EuvChartGeoDatum::new(GEO_USA, 90.0),
                EuvChartGeoDatum::new(GEO_EUROPE, 82.0),
                EuvChartGeoDatum::new(GEO_INDIA, 74.0),
                EuvChartGeoDatum::new(GEO_BRAZIL, 58.0),
                EuvChartGeoDatum::new(GEO_AUSTRALIA, 64.0),
            ])),
        ))
        .add_series(EuvChartSeries::new(
            SER_EDGE_ROUTES,
            EuvChartSeriesData::Lines(
                EuvChartLinesSeries::new(vec![
                    EuvChartFlowArc::new("SFO", "SHA", (-122.4, 37.7), (121.4, 31.2), 92.0),
                    EuvChartFlowArc::new("FRA", "SHA", (8.6, 50.1), (121.4, 31.2), 78.0),
                    EuvChartFlowArc::new("SFO", "FRA", (-122.4, 37.7), (8.6, 50.1), 84.0),
                    EuvChartFlowArc::new("SHA", "SYD", (121.4, 31.2), (151.2, -33.8), 64.0),
                ])
                .coordinate_system(EuvChartCoordinateSystem::Geo),
            ),
        ))
        .add_series(EuvChartSeries::new(
            SER_EDGE_HUBS,
            EuvChartSeriesData::EffectScatter(
                EuvChartScatterSeries::new(vec![
                    EuvChartScatterPoint::new(121.4, 31.2)
                        .size(6.0)
                        .label("SHA"),
                    EuvChartScatterPoint::new(-122.4, 37.7)
                        .size(6.0)
                        .label("SFO"),
                    EuvChartScatterPoint::new(8.6, 50.1).size(5.5).label("FRA"),
                    EuvChartScatterPoint::new(151.2, -33.8)
                        .size(5.5)
                        .label("SYD"),
                ])
                .coordinate_system(EuvChartCoordinateSystem::Geo)
                .show_labels(true),
            ),
        ))
}

/// Builds Card 19 option: Multi-axis Parallel Coordinates chart.
///
/// # Returns
///
/// - `EuvChartOption` - Configured Parallel Coordinates chart option.
fn build_parallel_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_PARALLEL_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .add_series(EuvChartSeries::new(
            SER_BENCHMARKS,
            EuvChartSeriesData::Parallel(EuvChartParallelSeries::new(
                vec![
                    EuvChartParallelAxis::new(AX_TTFF).range(0.0, 120.0),
                    EuvChartParallelAxis::new(IND_FPS).range(0.0, 120.0),
                    EuvChartParallelAxis::new(AX_RAM).range(0.0, 256.0),
                    EuvChartParallelAxis::new(AX_SCORE).range(0.0, 100.0),
                ],
                vec![
                    EuvChartParallelRow::new(ITEM_EUV_NATIVE, vec![12.0, 120.0, 24.0, 97.0], 0),
                    EuvChartParallelRow::new(ITEM_FLUTTER, vec![38.0, 90.0, 92.0, 78.0], 1),
                    EuvChartParallelRow::new(ITEM_WEBVIEW, vec![94.0, 58.0, 210.0, 48.0], 2),
                ],
            )),
        ))
}

/// Builds Card 20 option: Centered ThemeRiver streamgraph over time.
///
/// # Returns
///
/// - `EuvChartOption` - Configured ThemeRiver chart option.
fn build_themeriver_option() -> EuvChartOption {
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_RIVER_CHART))
        .tooltip(EuvChartTooltip::item())
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .add_series(EuvChartSeries::new(
            SER_STREAMS,
            EuvChartSeriesData::ThemeRiver(EuvChartThemeRiverSeries::new(vec![
                EuvChartThemeRiverPoint::new(1.0, 14.0, STREAM_RUST.to_string()),
                EuvChartThemeRiverPoint::new(2.0, 22.0, STREAM_RUST.to_string()),
                EuvChartThemeRiverPoint::new(3.0, 32.0, STREAM_RUST.to_string()),
                EuvChartThemeRiverPoint::new(4.0, 40.0, STREAM_RUST.to_string()),
                EuvChartThemeRiverPoint::new(1.0, 18.0, STREAM_WASM.to_string()),
                EuvChartThemeRiverPoint::new(2.0, 24.0, STREAM_WASM.to_string()),
                EuvChartThemeRiverPoint::new(3.0, 20.0, STREAM_WASM.to_string()),
                EuvChartThemeRiverPoint::new(4.0, 28.0, STREAM_WASM.to_string()),
                EuvChartThemeRiverPoint::new(1.0, 12.0, STREAM_MOBILE.to_string()),
                EuvChartThemeRiverPoint::new(2.0, 19.0, STREAM_MOBILE.to_string()),
                EuvChartThemeRiverPoint::new(3.0, 29.0, STREAM_MOBILE.to_string()),
                EuvChartThemeRiverPoint::new(4.0, 36.0, STREAM_MOBILE.to_string()),
            ])),
        ))
}

/// Builds Card 21 option: Repeated capsule `PictorialBar` paired with a `Custom` vector `DrawCommand` overlay.
///
/// # Returns
///
/// - `EuvChartOption` - Configured PictorialBar + Custom chart option.
fn build_pictorial_custom_option() -> EuvChartOption {
    let paint_target = move |rect: Rect, ctx: &EuvChartCustomDrawContext, dl: &mut DisplayList| {
        let cx: f32 = rect.get_x() + rect.get_width() * 0.5;
        let cy: f32 = rect.get_y() + 6.0;
        let col: Color = ctx
            .get_palette()
            .get(1)
            .copied()
            .unwrap_or_else(|| ctx.get_tokens().get_palette().get_primary());
        dl.push(DrawCommand::Line {
            from: Point::new(cx - 12.0, cy),
            to: Point::new(cx + 12.0, cy),
            color: col,
            width: 2.0,
        });
        dl.push(DrawCommand::Circle {
            center: Point::new(cx, cy),
            radius: 4.5,
            color: col,
        });
    };
    EuvChartOption::new()
        .title(EuvChartTitle::new(TITLE_PIC_CUSTOM_CHART))
        .legend(EuvChartLegend::at(EuvChartPosition::Top))
        .tooltip(EuvChartTooltip::axis(EuvChartAxisPointerType::Line))
        .toolbox(EuvChartToolbox::full().magic_type(false))
        .x_axis(EuvChartAxis::category(owned_labels(&TIERS_FOUR)))
        .y_axis(EuvChartAxis::value().max(100.0))
        .add_series(EuvChartSeries::new(
            SER_CAPSULES,
            EuvChartSeriesData::PictorialBar(EuvChartPictorialBarSeries::new(
                vec![46.0, 78.0, 64.0, 92.0],
                EuvChartSymbol::RoundRect,
            )),
        ))
        .add_series(EuvChartSeries::new(
            SER_CUSTOM_TARGET,
            EuvChartSeriesData::Custom(EuvChartCustomSeries::new(
                vec![55.0, 82.0, 74.0, 95.0],
                paint_target,
            )),
        ))
}

/// Wraps a static `EuvChartOption` inside a responsive `EuvCard` wired with optional live interaction callbacks.
///
/// # Arguments
///
/// - `&'static str` - Card heading title.
/// - `&'static str` - Card descriptive subtitle.
/// - `EuvChartOption` - Declarative ECharts option document.
/// - `f32` - Chart canvas height in logical pixels.
/// - `ThemeTokens` - Active UI theme tokens.
/// - `Signal<String>` - Live interaction log signal updated on datum click, legend toggle, and DataZoom.
///
/// # Returns
///
/// - `NodeId` - Mounted chart showcase card node.
fn build_static_chart_card(
    title: &'static str,
    subtitle: &'static str,
    option: EuvChartOption,
    height: f32,
    tokens: ThemeTokens,
    event_log: Signal<String>,
) -> NodeId {
    let hit_log: Signal<String> = event_log;
    let on_hit: EuvChartHitCallback = Rc::new(move |hit: &EuvChartDatumHit| {
        let msg: String = format!(
            "Clicked {} [{}] = {:.1}",
            hit.get_series_name(),
            hit.get_category_label(),
            hit.get_value()
        );
        let _: bool = hit_log.set_if_changed(msg);
    });
    let leg_log: Signal<String> = event_log;
    let on_legend: EuvChartLegendCallback = Rc::new(move |name: &str, visible: bool| {
        let msg: String = format!("Legend {name}: visible={visible}");
        let _: bool = leg_log.set_if_changed(msg);
    });
    let zoom_log: Signal<String> = event_log;
    let on_zoom: EuvChartZoomCallback = Rc::new(move |start_pct: f32, end_pct: f32| {
        let msg: String = format!("DataZoom range: {start_pct:.0}% .. {end_pct:.0}%");
        let _: bool = zoom_log.set_if_changed(msg);
    });
    view! {
        EuvCard {
            title: title.to_string()
            tokens: tokens
            subtitle: subtitle
            EuvEChart {
                option: option
                tokens: tokens
                width: Length::Percent(100.0)
                height: Length::Px(height)
                on_point_click: on_hit
                on_legend_change: on_legend
                on_zoom_change: on_zoom
            }
        }
    }
}

/// Builds Card 16 (Graph) with a live Force / Circular reactive layout switcher.
///
/// # Arguments
///
/// - `ThemeTokens` - Active UI theme tokens.
/// - `Signal<String>` - Live event telemetry readout signal.
///
/// # Returns
///
/// - `NodeId` - Mounted interactive Graph showcase card node.
fn build_graph_card(tokens: ThemeTokens, event_log: Signal<String>) -> NodeId {
    let opt_sig: Signal<EuvChartOption> =
        Signal::new(build_graph_option(EuvChartGraphLayout::Force));
    let on_force = move |_: &UiEvent| {
        let _: bool = opt_sig.set(build_graph_option(EuvChartGraphLayout::Force));
        let _: bool = event_log.set_if_changed(BTN_GRAPH_FORCE.to_string());
    };
    let on_circular = move |_: &UiEvent| {
        let _: bool = opt_sig.set(build_graph_option(EuvChartGraphLayout::Circular));
        let _: bool = event_log.set_if_changed(BTN_GRAPH_CIRCULAR.to_string());
    };
    view! {
        EuvCard {
            title: CARD_GRAPH_TITLE.to_string()
            tokens: tokens
            subtitle: CARD_GRAPH_SUB
            EuvActionRow {
                EuvButton {
                    label: BTN_GRAPH_FORCE.to_string()
                    tokens: tokens
                    secondary()
                    sm()
                    on_click: on_force
                }
                EuvButton {
                    label: BTN_GRAPH_CIRCULAR.to_string()
                    tokens: tokens
                    outline()
                    sm()
                    on_click: on_circular
                }
            }
            EuvEChart {
                option_signal: opt_sig
                tokens: tokens
                width: Length::Percent(100.0)
                height: Length::Px(CHART_CARD_TALL_HEIGHT)
            }
        }
    }
}

/// Builds Card 17 (Tree) with a live Orthogonal / Radial reactive layout switcher.
///
/// # Arguments
///
/// - `ThemeTokens` - Active UI theme tokens.
/// - `Signal<String>` - Live event telemetry readout signal.
///
/// # Returns
///
/// - `NodeId` - Mounted interactive Tree showcase card node.
fn build_tree_card(tokens: ThemeTokens, event_log: Signal<String>) -> NodeId {
    let opt_sig: Signal<EuvChartOption> =
        Signal::new(build_tree_option(EuvChartTreeLayout::OrthogonalLeftRight));
    let on_ortho = move |_: &UiEvent| {
        let _: bool = opt_sig.set(build_tree_option(EuvChartTreeLayout::OrthogonalLeftRight));
        let _: bool = event_log.set_if_changed(BTN_TREE_ORTHO.to_string());
    };
    let on_radial = move |_: &UiEvent| {
        let _: bool = opt_sig.set(build_tree_option(EuvChartTreeLayout::Radial));
        let _: bool = event_log.set_if_changed(BTN_TREE_RADIAL.to_string());
    };
    view! {
        EuvCard {
            title: CARD_TREE_TITLE.to_string()
            tokens: tokens
            subtitle: CARD_TREE_SUB
            EuvActionRow {
                EuvButton {
                    label: BTN_TREE_ORTHO.to_string()
                    tokens: tokens
                    secondary()
                    sm()
                    on_click: on_ortho
                }
                EuvButton {
                    label: BTN_TREE_RADIAL.to_string()
                    tokens: tokens
                    outline()
                    sm()
                    on_click: on_radial
                }
            }
            EuvEChart {
                option_signal: opt_sig
                tokens: tokens
                width: Length::Percent(100.0)
                height: Length::Px(CHART_CARD_TALL_HEIGHT)
            }
        }
    }
}

/// Builds the wrapped auto-fill category filter bar (`EuvActionRow`) reflecting `cat_sig`.
///
/// # Arguments
///
/// - `Signal<usize>` - Reactive selected category index (`0..=5`).
/// - `ThemeTokens` - Active UI theme tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted wrapped category filter row with zero trailing empty space.
fn build_category_filter_row(cat_sig: Signal<usize>, tokens: ThemeTokens) -> NodeId {
    let active_idx: usize = cat_sig.get();
    let cats: [(usize, &'static str); 6] = [
        (0, CHARTS_CAT_ALL),
        (1, CHARTS_CAT_CARTESIAN),
        (2, CHARTS_CAT_FINANCIAL),
        (3, CHARTS_CAT_RADIAL),
        (4, CHARTS_CAT_HIERARCHY),
        (5, CHARTS_CAT_GEO_FLOW),
    ];
    let mut row: EuvActionRow = EuvActionRow::default();
    for (idx, label) in cats.into_iter() {
        let on_pick = move |_: &UiEvent| {
            let _: bool = cat_sig.set_if_changed(idx);
        };
        let btn: NodeId = if active_idx == idx {
            view! {
                EuvButton {
                    label: label.to_string()
                    tokens: tokens
                    sm()
                    on_click: on_pick
                }
            }
        } else {
            view! {
                EuvButton {
                    label: label.to_string()
                    tokens: tokens
                    outline()
                    sm()
                    on_click: on_pick
                }
            }
        };
        row = row.child(btn);
    }
    row.mount()
}

/// Builds the filtered list of ECharts showcase cards for category `cat_idx`.
///
/// # Arguments
///
/// - `usize` - Active category filter index (`0=All`, `1=Cartesian`, `2=Financial`, `3=Radial`, `4=Hierarchy`, `5=Geo & Flow`).
/// - `ThemeTokens` - Active resolved UI theme tokens.
/// - `Signal<String>` - Live interaction telemetry signal.
///
/// # Returns
///
/// - `Vec<NodeId>` - Ordered list of mounted showcase chart cards.
fn build_filtered_chart_cards(
    cat_idx: usize,
    tokens: ThemeTokens,
    event_log: Signal<String>,
) -> Vec<NodeId> {
    let mut cards: Vec<NodeId> = Vec::with_capacity(21);
    if cat_idx == 0 || cat_idx == 1 {
        cards.push(build_static_chart_card(
            CARD_LINE_TITLE,
            CARD_LINE_SUB,
            build_line_option(),
            CHART_CARD_TALL_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_BAR_TITLE,
            CARD_BAR_SUB,
            build_bar_dataset_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_HBAR_TITLE,
            CARD_HBAR_SUB,
            build_horizontal_bar_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_SCATTER_TITLE,
            CARD_SCATTER_SUB,
            build_scatter_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
    }
    if cat_idx == 0 || cat_idx == 2 {
        cards.push(build_static_chart_card(
            CARD_CANDLE_TITLE,
            CARD_CANDLE_SUB,
            build_candlestick_option(),
            CHART_CARD_TALL_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_BOXPLOT_TITLE,
            CARD_BOXPLOT_SUB,
            build_boxplot_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_HEATMAP_TITLE,
            CARD_HEATMAP_SUB,
            build_heatmap_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_CALENDAR_TITLE,
            CARD_CALENDAR_SUB,
            build_calendar_heatmap_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
    }
    if cat_idx == 0 || cat_idx == 3 {
        cards.push(build_static_chart_card(
            CARD_PIE_TITLE,
            CARD_PIE_SUB,
            build_pie_option(),
            CHART_CARD_TALL_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_RADAR_TITLE,
            CARD_RADAR_SUB,
            build_radar_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_GAUGE_TITLE,
            CARD_GAUGE_SUB,
            build_gauge_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
    }
    if cat_idx == 0 || cat_idx == 4 {
        cards.push(build_static_chart_card(
            CARD_TREEMAP_TITLE,
            CARD_TREEMAP_SUB,
            build_treemap_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_SUNBURST_TITLE,
            CARD_SUNBURST_SUB,
            build_sunburst_option(),
            CHART_CARD_TALL_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_SANKEY_TITLE,
            CARD_SANKEY_SUB,
            build_sankey_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_FUNNEL_TITLE,
            CARD_FUNNEL_SUB,
            build_funnel_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_graph_card(tokens, event_log));
        cards.push(build_tree_card(tokens, event_log));
    }
    if cat_idx == 0 || cat_idx == 5 {
        cards.push(build_static_chart_card(
            CARD_GEO_TITLE,
            CARD_GEO_SUB,
            build_geo_lines_option(),
            CHART_CARD_TALL_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_PARALLEL_TITLE,
            CARD_PARALLEL_SUB,
            build_parallel_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_RIVER_TITLE,
            CARD_RIVER_SUB,
            build_themeriver_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
        cards.push(build_static_chart_card(
            CARD_PIC_CUSTOM_TITLE,
            CARD_PIC_CUSTOM_SUB,
            build_pictorial_custom_option(),
            CHART_CARD_HEIGHT,
            tokens,
            event_log,
        ));
    }
    cards
}

/// Mounts the lazily hydrated ECharts Studio (`/charts`) showcase page demonstrating all 20 Apache ECharts series types.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active resolved design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted ECharts Studio page node.
pub fn mount_charts_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    state.ensure_charts_loaded();
    let p: ThemePalette = tokens.get_palette();
    let cat_sig: Signal<usize> = state.get_charts_category_idx();
    let event_log: Signal<String> = state.get_charts_event_log();
    let build_filter_bar = move || -> Vec<NodeId> {
        let live_tok: ThemeTokens = live_tokens_untracked(tokens);
        vec![build_category_filter_row(cat_sig, live_tok)]
    };
    let status_text = move || -> String { event_log.get() };
    let build_gallery = move || -> Vec<NodeId> {
        let cat_idx: usize = cat_sig.get();
        let live_tok: ThemeTokens = live_tokens_untracked(tokens);
        build_filtered_chart_cards(cat_idx, live_tok, event_log)
    };
    view! {
        EuvPage {
            title: CHARTS_PAGE_TITLE.to_string()
            subtitle: CHARTS_PAGE_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: CHARTS_FILTER_TITLE.to_string()
                tokens: tokens
                subtitle: CHARTS_FILTER_SUBTITLE
                column {
                    width: Length::Percent(100.0)
                    bind_children: build_filter_bar
                }
                text {
                    width: Length::Percent(100.0)
                    font_size: 12.5
                    font_weight: FontWeight::Medium
                    color: p.get_primary()
                    { status_text }
                }
            }
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                bind_children: build_gallery
            }
        }
    }
}
