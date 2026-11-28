use super::*;

fn assert_all_commands_finite(dl: &DisplayList) {
    assert!(!dl.get_commands().is_empty());
    for cmd in dl.get_commands().iter() {
        match cmd {
            DrawCommand::FillRoundedRect { rect, .. } => {
                assert!(rect.get_x().is_finite());
                assert!(rect.get_y().is_finite());
                assert!(rect.get_width().is_finite());
                assert!(rect.get_height().is_finite());
                assert!(rect.get_width() >= 0.0);
                assert!(rect.get_height() >= 0.0);
            }
            DrawCommand::Line {
                from, to, width, ..
            } => {
                assert!(from.get_x().is_finite());
                assert!(from.get_y().is_finite());
                assert!(to.get_x().is_finite());
                assert!(to.get_y().is_finite());
                assert!(width.is_finite() && *width >= 0.0);
            }
            DrawCommand::Circle { center, radius, .. } => {
                assert!(center.get_x().is_finite());
                assert!(center.get_y().is_finite());
                assert!(radius.is_finite() && *radius >= 0.0);
            }
            DrawCommand::Triangle { p0, p1, p2, .. } => {
                assert!(p0.get_x().is_finite() && p0.get_y().is_finite());
                assert!(p1.get_x().is_finite() && p1.get_y().is_finite());
                assert!(p2.get_x().is_finite() && p2.get_y().is_finite());
            }
            DrawCommand::TextRun { rect, .. } => {
                assert!(rect.get_x().is_finite() && rect.get_y().is_finite());
                assert!(rect.get_width().is_finite() && rect.get_height().is_finite());
            }
            DrawCommand::PushClip(rect) => {
                assert!(rect.get_x().is_finite() && rect.get_y().is_finite());
                assert!(rect.get_width().is_finite() && rect.get_height().is_finite());
            }
            _ => {}
        }
    }
}

fn verify_option_across_viewports_and_themes(option: &EuvChartOption, min_commands: usize) {
    let viewports: [Rect; 2] = [
        Rect::new(0.0, 0.0, 360.0, 240.0),
        Rect::new(0.0, 0.0, 800.0, 480.0),
    ];
    let themes: [ThemeTokens; 2] = [
        ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light),
        ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Dark),
    ];
    let state: EuvChartState = EuvChartState::from_option(option);
    for bounds in viewports.into_iter() {
        for tokens in themes.into_iter() {
            let mut dl: DisplayList = DisplayList::default();
            let res: EuvChartRenderResult =
                render_echart_to_display_list(option, &state, bounds, tokens, &mut dl);
            assert!(dl.get_commands().len() >= min_commands);
            assert_all_commands_finite(&dl);
            assert!(res.get_plot_rect().get_width() > 0.0);
            assert!(res.get_plot_rect().get_height() > 0.0);
        }
    }
}

#[test]
fn all_twenty_echarts_series_types_render_valid_finite_vector_commands() {
    let cats: Vec<String> = vec![
        "Mon".to_string(),
        "Tue".to_string(),
        "Wed".to_string(),
        "Thu".to_string(),
        "Fri".to_string(),
        "Sat".to_string(),
    ];
    let line_opt: EuvChartOption = EuvChartOption::new()
        .title(EuvChartTitle::new("Smooth Stacked Area Line").subtext("Weekly traffic"))
        .legend(EuvChartLegend::at(EuvChartPosition::Top))
        .x_axis(EuvChartAxis::category(cats.clone()))
        .y_axis(EuvChartAxis::value().unit_suffix("k"))
        .add_series(EuvChartSeries::new(
            "Direct",
            EuvChartSeriesData::Line(
                EuvChartLineSeries::from_nullable(vec![
                    Some(120.0),
                    Some(180.0),
                    None,
                    Some(210.0),
                    Some(260.0),
                    Some(290.0),
                ])
                .smooth(true)
                .area(0.28)
                .connect_nulls(true)
                .stack("total"),
            ),
        ))
        .add_series(EuvChartSeries::new(
            "Step Search",
            EuvChartSeriesData::Line(
                EuvChartLineSeries::from_values(vec![60.0, 90.0, 110.0, 95.0, 130.0, 150.0])
                    .step(EuvChartStepMode::Middle)
                    .dashed(true)
                    .symbol(EuvChartSymbol::Diamond, 4.0),
            ),
        ));
    verify_option_across_viewports_and_themes(&line_opt, 24);
    let bar_opt: EuvChartOption = EuvChartOption::new()
        .title(EuvChartTitle::new("Grouped & Stacked Bars"))
        .x_axis(EuvChartAxis::category(cats.clone()))
        .y_axis(EuvChartAxis::value())
        .add_series(EuvChartSeries::new(
            "Q1",
            EuvChartSeriesData::Bar(
                EuvChartBarSeries::from_values(vec![45.0, 62.0, 58.0, 74.0, 90.0, 82.0])
                    .stack("s1")
                    .show_background(true)
                    .show_labels(true),
            ),
        ))
        .add_series(EuvChartSeries::new(
            "Q2",
            EuvChartSeriesData::Bar(
                EuvChartBarSeries::from_values(vec![20.0, 28.0, 31.0, 26.0, 35.0, 40.0])
                    .stack("s1"),
            ),
        ));
    verify_option_across_viewports_and_themes(&bar_opt, 20);
    let hbar_opt: EuvChartOption = EuvChartOption::new()
        .x_axis(EuvChartAxis::value())
        .y_axis(EuvChartAxis::category(vec![
            "Web".to_string(),
            "iOS".to_string(),
            "Android".to_string(),
        ]))
        .add_series(EuvChartSeries::new(
            "Share",
            EuvChartSeriesData::Bar(EuvChartBarSeries::from_values(vec![88.0, 72.0, 94.0])),
        ));
    verify_option_across_viewports_and_themes(&hbar_opt, 12);
    let pie_opt: EuvChartOption = EuvChartOption::new()
        .title(EuvChartTitle::new("Nightingale Rose Donut"))
        .add_series(EuvChartSeries::new(
            "Traffic",
            EuvChartSeriesData::Pie(
                EuvChartPieSeries::new(vec![
                    EuvChartPieSlice::new("Search", 420.0).selected(true),
                    EuvChartPieSlice::new("Direct", 280.0),
                    EuvChartPieSlice::new("Referral", 190.0),
                    EuvChartPieSlice::new("Social", 140.0),
                ])
                .donut(0.35)
                .rose(EuvChartRoseType::Radius)
                .pad_angle(2.0),
            ),
        ));
    verify_option_across_viewports_and_themes(&pie_opt, 24);
    let scatter_opt: EuvChartOption = EuvChartOption::new()
        .x_axis(EuvChartAxis::value())
        .y_axis(EuvChartAxis::value())
        .add_series(EuvChartSeries::new(
            "Clusters",
            EuvChartSeriesData::Scatter(
                EuvChartScatterSeries::new(vec![
                    EuvChartScatterPoint::new(12.0, 32.0).size(7.0).label("A"),
                    EuvChartScatterPoint::new(28.0, 54.0).size(11.0).label("B"),
                    EuvChartScatterPoint::new(45.0, 41.0).size(9.0).label("C"),
                ])
                .symbol(EuvChartSymbol::Triangle, 6.0)
                .show_labels(true),
            ),
        ))
        .add_series(EuvChartSeries::new(
            "Hotspots",
            EuvChartSeriesData::EffectScatter(EuvChartScatterSeries::new(vec![
                EuvChartScatterPoint::new(62.0, 78.0).size(8.0),
                EuvChartScatterPoint::new(84.0, 66.0).size(10.0),
            ])),
        ));
    verify_option_across_viewports_and_themes(&scatter_opt, 24);
    let radar_opt: EuvChartOption = EuvChartOption::new()
        .radar(
            EuvChartRadar::new(vec![
                EuvChartRadarIndicator::new("FPS".to_string(), 0.0, 120.0),
                EuvChartRadarIndicator::new("Memory".to_string(), 0.0, 100.0),
                EuvChartRadarIndicator::new("Startup".to_string(), 0.0, 100.0),
                EuvChartRadarIndicator::new("Binary".to_string(), 0.0, 100.0),
                EuvChartRadarIndicator::new("Battery".to_string(), 0.0, 100.0),
            ])
            .shape(EuvChartRadarShape::Polygon),
        )
        .add_series(EuvChartSeries::new(
            "Engine",
            EuvChartSeriesData::Radar(EuvChartRadarSeries::new(vec![
                EuvChartRadarItem::new("euv", vec![120.0, 92.0, 96.0, 94.0, 90.0]),
                EuvChartRadarItem::new("Legacy", vec![60.0, 52.0, 48.0, 40.0, 55.0]),
            ])),
        ));
    verify_option_across_viewports_and_themes(&radar_opt, 24);
    let candle_opt: EuvChartOption = EuvChartOption::new()
        .x_axis(EuvChartAxis::category(cats.clone()))
        .y_axis(EuvChartAxis::value())
        .add_series(EuvChartSeries::new(
            "KLine",
            EuvChartSeriesData::Candlestick(EuvChartCandlestickSeries::new(vec![
                EuvChartOhlcPoint::new(100.0, 112.0, 96.0, 118.0),
                EuvChartOhlcPoint::new(112.0, 105.0, 101.0, 116.0),
                EuvChartOhlcPoint::new(105.0, 124.0, 103.0, 128.0),
                EuvChartOhlcPoint::new(124.0, 120.0, 117.0, 130.0),
                EuvChartOhlcPoint::new(120.0, 136.0, 118.0, 141.0),
                EuvChartOhlcPoint::new(136.0, 132.0, 129.0, 140.0),
            ])),
        ))
        .add_series(EuvChartSeries::new(
            "MA5",
            EuvChartSeriesData::Line(
                EuvChartLineSeries::from_values(vec![106.0, 109.0, 114.0, 118.0, 124.0, 129.0])
                    .smooth(true),
            ),
        ));
    verify_option_across_viewports_and_themes(&candle_opt, 24);
    let box_opt: EuvChartOption = EuvChartOption::new()
        .x_axis(EuvChartAxis::category(vec![
            "CPU".to_string(),
            "GPU".to_string(),
            "Hybrid".to_string(),
        ]))
        .y_axis(EuvChartAxis::value())
        .add_series(EuvChartSeries::new(
            "Frame Latency",
            EuvChartSeriesData::Boxplot(EuvChartBoxplotSeries::new(vec![
                EuvChartBoxplotItem::new(4.0, 6.0, 8.0, 10.5, 13.0).outliers(vec![16.2]),
                EuvChartBoxplotItem::new(1.5, 2.2, 3.0, 4.1, 5.4),
                EuvChartBoxplotItem::new(2.0, 3.1, 4.0, 5.2, 6.8).outliers(vec![9.5]),
            ])),
        ));
    verify_option_across_viewports_and_themes(&box_opt, 20);
    let heat_opt: EuvChartOption = EuvChartOption::new()
        .x_axis(EuvChartAxis::category(vec![
            "00h".to_string(),
            "06h".to_string(),
            "12h".to_string(),
            "18h".to_string(),
        ]))
        .y_axis(EuvChartAxis::category(vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
        ]))
        .visual_map(EuvChartVisualMap::continuous(0.0, 100.0))
        .add_series(EuvChartSeries::new(
            "Load",
            EuvChartSeriesData::Heatmap(EuvChartHeatmapSeries::new(vec![
                EuvChartHeatmapCell::new(0, 0, 12.0),
                EuvChartHeatmapCell::new(1, 0, 48.0),
                EuvChartHeatmapCell::new(2, 1, 86.0),
                EuvChartHeatmapCell::new(3, 2, 95.0),
            ])),
        ));
    verify_option_across_viewports_and_themes(&heat_opt, 20);
    let cal_opt: EuvChartOption = EuvChartOption::new()
        .calendar(EuvChartCalendar::new(2026, 16))
        .visual_map(EuvChartVisualMap::continuous(0.0, 20.0))
        .add_series(EuvChartSeries::new(
            "Commits",
            EuvChartSeriesData::Heatmap(
                EuvChartHeatmapSeries::new(vec![
                    EuvChartHeatmapCell::new(0, 1, 4.0),
                    EuvChartHeatmapCell::new(3, 2, 12.0),
                    EuvChartHeatmapCell::new(8, 4, 19.0),
                    EuvChartHeatmapCell::new(12, 0, 9.0),
                ])
                .coordinate_system(EuvChartCoordinateSystem::Calendar),
            ),
        ));
    verify_option_across_viewports_and_themes(&cal_opt, 40);
    let treemap_opt: EuvChartOption = EuvChartOption::new().add_series(EuvChartSeries::new(
        "Bundle Size",
        EuvChartSeriesData::Treemap(EuvChartTreemapSeries::new(vec![
            EuvChartHierarchyNode::branch(
                "Core",
                vec![
                    EuvChartHierarchyNode::leaf("Layout", 140.0),
                    EuvChartHierarchyNode::leaf("Render", 220.0),
                ],
            ),
            EuvChartHierarchyNode::branch(
                "UI",
                vec![
                    EuvChartHierarchyNode::leaf("Charts", 160.0),
                    EuvChartHierarchyNode::leaf("Markdown", 95.0),
                ],
            ),
        ])),
    ));
    verify_option_across_viewports_and_themes(&treemap_opt, 10);
    let sunburst_opt: EuvChartOption = EuvChartOption::new().add_series(EuvChartSeries::new(
        "Taxonomy",
        EuvChartSeriesData::Sunburst(EuvChartSunburstSeries::new(vec![
            EuvChartHierarchyNode::branch(
                "Native",
                vec![
                    EuvChartHierarchyNode::leaf("Metal", 45.0),
                    EuvChartHierarchyNode::leaf("Vulkan", 35.0),
                ],
            ),
            EuvChartHierarchyNode::branch(
                "Web",
                vec![
                    EuvChartHierarchyNode::leaf("WebGPU", 30.0),
                    EuvChartHierarchyNode::leaf("Canvas2D", 20.0),
                ],
            ),
        ])),
    ));
    verify_option_across_viewports_and_themes(&sunburst_opt, 24);
    let sankey_opt: EuvChartOption = EuvChartOption::new().add_series(EuvChartSeries::new(
        "Traffic Flow",
        EuvChartSeriesData::Sankey(EuvChartSankeySeries::new(
            vec![
                EuvChartSankeyNode::new("Organic"),
                EuvChartSankeyNode::new("Ads"),
                EuvChartSankeyNode::new("Landing"),
                EuvChartSankeyNode::new("Signup"),
                EuvChartSankeyNode::new("Paid"),
            ],
            vec![
                EuvChartSankeyLink::new("Organic".to_string(), "Landing".to_string(), 60.0),
                EuvChartSankeyLink::new("Ads".to_string(), "Landing".to_string(), 40.0),
                EuvChartSankeyLink::new("Landing".to_string(), "Signup".to_string(), 55.0),
                EuvChartSankeyLink::new("Signup".to_string(), "Paid".to_string(), 28.0),
            ],
        )),
    ));
    verify_option_across_viewports_and_themes(&sankey_opt, 20);
    let funnel_opt: EuvChartOption = EuvChartOption::new().add_series(EuvChartSeries::new(
        "Conversion",
        EuvChartSeriesData::Funnel(
            EuvChartFunnelSeries::new(vec![
                EuvChartFunnelStage::new("Visit", 100.0),
                EuvChartFunnelStage::new("Explore", 72.0),
                EuvChartFunnelStage::new("Cart", 46.0),
                EuvChartFunnelStage::new("Order", 24.0),
            ])
            .sort(EuvChartFunnelSort::Descending),
        ),
    ));
    verify_option_across_viewports_and_themes(&funnel_opt, 12);
    let gauge_opt: EuvChartOption = EuvChartOption::new().add_series(EuvChartSeries::new(
        "Frame Budget",
        EuvChartSeriesData::Gauge(
            EuvChartGaugeSeries::new(78.5, 0.0, 100.0)
                .unit("%")
                .title("GPU Efficiency"),
        ),
    ));
    verify_option_across_viewports_and_themes(&gauge_opt, 30);
    let graph_opt: EuvChartOption = EuvChartOption::new().add_series(EuvChartSeries::new(
        "Cluster",
        EuvChartSeriesData::Graph(
            EuvChartGraphSeries::new(
                vec![
                    EuvChartGraphNode::new("a", "API", 10.0, 0),
                    EuvChartGraphNode::new("b", "Auth", 8.0, 1),
                    EuvChartGraphNode::new("c", "Cache", 9.0, 1),
                    EuvChartGraphNode::new("d", "DB", 12.0, 2),
                ],
                vec![
                    EuvChartGraphEdge::new("a", "b"),
                    EuvChartGraphEdge::new("a", "c"),
                    EuvChartGraphEdge::new("b", "d"),
                    EuvChartGraphEdge::new("c", "d").curveness(0.0),
                ],
            )
            .layout(EuvChartGraphLayout::Force),
        ),
    ));
    verify_option_across_viewports_and_themes(&graph_opt, 20);
    let tree_opt: EuvChartOption = EuvChartOption::new().add_series(EuvChartSeries::new(
        "Scene Tree",
        EuvChartSeriesData::Tree(
            EuvChartTreeSeries::new(EuvChartHierarchyNode::branch(
                "Root",
                vec![
                    EuvChartHierarchyNode::branch(
                        "Shell",
                        vec![
                            EuvChartHierarchyNode::leaf("Nav", 1.0),
                            EuvChartHierarchyNode::leaf("Outlet", 1.0),
                        ],
                    ),
                    EuvChartHierarchyNode::leaf("Portal", 1.0),
                ],
            ))
            .layout(EuvChartTreeLayout::Radial),
        ),
    ));
    verify_option_across_viewports_and_themes(&tree_opt, 12);
    let parallel_opt: EuvChartOption = EuvChartOption::new().add_series(EuvChartSeries::new(
        "Benchmarks",
        EuvChartSeriesData::Parallel(EuvChartParallelSeries::new(
            vec![
                EuvChartParallelAxis::new("TTFF").range(0.0, 100.0),
                EuvChartParallelAxis::new("FPS").range(0.0, 120.0),
                EuvChartParallelAxis::new("RAM").range(0.0, 256.0),
                EuvChartParallelAxis::new("Score").range(0.0, 100.0),
            ],
            vec![
                EuvChartParallelRow::new("euv", vec![12.0, 120.0, 24.0, 96.0], 0),
                EuvChartParallelRow::new("WebView", vec![85.0, 60.0, 190.0, 54.0], 1),
            ],
        )),
    ));
    verify_option_across_viewports_and_themes(&parallel_opt, 16);
    let lines_opt: EuvChartOption = EuvChartOption::new()
        .x_axis(EuvChartAxis::value().min(0.0).max(100.0))
        .y_axis(EuvChartAxis::value().min(0.0).max(100.0))
        .add_series(EuvChartSeries::new(
            "Routes",
            EuvChartSeriesData::Lines(EuvChartLinesSeries::new(vec![
                EuvChartFlowArc::new("Node A", "Node B", (15.0, 20.0), (85.0, 75.0), 42.0),
                EuvChartFlowArc::new("Node B", "Node C", (85.0, 75.0), (40.0, 88.0), 19.0),
            ])),
        ));
    verify_option_across_viewports_and_themes(&lines_opt, 20);
    let river_opt: EuvChartOption = EuvChartOption::new().add_series(EuvChartSeries::new(
        "Streams",
        EuvChartSeriesData::ThemeRiver(EuvChartThemeRiverSeries::new(vec![
            EuvChartThemeRiverPoint::new(1.0, 10.0, "Rust".to_string()),
            EuvChartThemeRiverPoint::new(2.0, 18.0, "Rust".to_string()),
            EuvChartThemeRiverPoint::new(3.0, 26.0, "Rust".to_string()),
            EuvChartThemeRiverPoint::new(1.0, 15.0, "WASM".to_string()),
            EuvChartThemeRiverPoint::new(2.0, 14.0, "WASM".to_string()),
            EuvChartThemeRiverPoint::new(3.0, 22.0, "WASM".to_string()),
        ])),
    ));
    verify_option_across_viewports_and_themes(&river_opt, 18);
    let custom_invoked: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let custom_flag: Rc<Cell<bool>> = Rc::clone(&custom_invoked);
    let pic_custom_opt: EuvChartOption = EuvChartOption::new()
        .x_axis(EuvChartAxis::category(vec![
            "Alpha".to_string(),
            "Beta".to_string(),
            "Gamma".to_string(),
        ]))
        .y_axis(EuvChartAxis::value())
        .add_series(EuvChartSeries::new(
            "Capsules",
            EuvChartSeriesData::PictorialBar(EuvChartPictorialBarSeries::new(
                vec![40.0, 75.0, 58.0],
                EuvChartSymbol::RoundRect,
            )),
        ))
        .add_series(EuvChartSeries::new(
            "CustomOverlay",
            EuvChartSeriesData::Custom(EuvChartCustomSeries::new(
                vec![20.0, 50.0, 90.0],
                move |rect: Rect, _ctx: &EuvChartCustomDrawContext, dl: &mut DisplayList| {
                    custom_flag.set(true);
                    dl.push(DrawCommand::Circle {
                        center: Point::new(
                            rect.get_x() + rect.get_width() * 0.5,
                            rect.get_y() + rect.get_height() * 0.5,
                        ),
                        radius: 6.0,
                        color: Color::hex(0x2563EB),
                    });
                },
            )),
        ));
    verify_option_across_viewports_and_themes(&pic_custom_opt, 18);
    assert!(custom_invoked.get());
    let geo_opt: EuvChartOption = EuvChartOption::new()
        .geo(EuvChartGeo::world())
        .visual_map(EuvChartVisualMap::continuous(0.0, 100.0))
        .add_series(EuvChartSeries::new(
            "Users",
            EuvChartSeriesData::GeoMap(EuvChartGeoMapSeries::new(vec![
                EuvChartGeoDatum::new("China", 94.0),
                EuvChartGeoDatum::new("United States", 88.0),
                EuvChartGeoDatum::new("Europe", 76.0),
                EuvChartGeoDatum::new("Brazil", 52.0),
            ])),
        ))
        .add_series(EuvChartSeries::new(
            "Edges",
            EuvChartSeriesData::Lines(
                EuvChartLinesSeries::new(vec![EuvChartFlowArc::new(
                    "SFO",
                    "SHA",
                    (-122.4, 37.7),
                    (121.4, 31.2),
                    80.0,
                )])
                .coordinate_system(EuvChartCoordinateSystem::Geo),
            ),
        ))
        .add_series(EuvChartSeries::new(
            "Hubs",
            EuvChartSeriesData::EffectScatter(
                EuvChartScatterSeries::new(vec![
                    EuvChartScatterPoint::new(121.4, 31.2)
                        .size(6.0)
                        .label("SHA"),
                    EuvChartScatterPoint::new(-122.4, 37.7)
                        .size(6.0)
                        .label("SFO"),
                ])
                .coordinate_system(EuvChartCoordinateSystem::Geo)
                .show_labels(true),
            ),
        ));
    verify_option_across_viewports_and_themes(&geo_opt, 40);
}

#[test]
fn polar_and_single_axis_coordinate_systems_render_cleanly() {
    let polar_opt: EuvChartOption = EuvChartOption::new()
        .polar(EuvChartPolar::new(
            EuvChartAxis::category(vec![
                "N".to_string(),
                "E".to_string(),
                "S".to_string(),
                "W".to_string(),
            ]),
            EuvChartAxis::value(),
        ))
        .add_series(EuvChartSeries::new(
            "Wind",
            EuvChartSeriesData::Bar(
                EuvChartBarSeries::from_values(vec![32.0, 58.0, 44.0, 70.0])
                    .coordinate_system(EuvChartCoordinateSystem::Polar),
            ),
        ));
    verify_option_across_viewports_and_themes(&polar_opt, 20);
    let single_opt: EuvChartOption = EuvChartOption::new()
        .single_axis(EuvChartSingleAxis::new(
            EuvChartAxis::value().min(0.0).max(24.0),
        ))
        .add_series(EuvChartSeries::new(
            "Events",
            EuvChartSeriesData::EffectScatter(
                EuvChartScatterSeries::new(vec![
                    EuvChartScatterPoint::new(4.0, 10.0)
                        .size(7.0)
                        .label("04:00"),
                    EuvChartScatterPoint::new(12.0, 22.0)
                        .size(11.0)
                        .label("12:00"),
                    EuvChartScatterPoint::new(20.0, 15.0)
                        .size(9.0)
                        .label("20:00"),
                ])
                .coordinate_system(EuvChartCoordinateSystem::SingleAxis),
            ),
        ));
    verify_option_across_viewports_and_themes(&single_opt, 16);
}

#[test]
fn all_four_axis_types_dual_axis_and_nice_ticks_behave_deterministically() {
    assert!((compute_nice_step(93.0, 5) - 20.0).abs() < 1e-4);
    assert!((compute_nice_step(4.7, 5) - 1.0).abs() < 1e-4);
    let cat_scale: EuvChartAxisScale = build_axis_scale(
        &EuvChartAxis::category(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        0.0,
        2.0,
        false,
        None,
    );
    assert_eq!(cat_scale.get_categories().len(), 3);
    assert_eq!(cat_scale.get_ticks().len(), 3);
    let val_scale: EuvChartAxisScale = build_axis_scale(
        &EuvChartAxis::value().unit_suffix("%"),
        12.0,
        87.0,
        true,
        None,
    );
    assert!(val_scale.get_min() <= 0.0);
    assert!(val_scale.get_max() >= 87.0);
    assert!(
        val_scale
            .get_labels()
            .iter()
            .all(|l: &String| l.ends_with('%'))
    );
    let time_scale: EuvChartAxisScale =
        build_axis_scale(&EuvChartAxis::time(), 0.0, 24.0, false, None);
    assert!(
        time_scale
            .get_labels()
            .iter()
            .any(|l: &String| l.contains(':'))
    );
    let log_scale: EuvChartAxisScale =
        build_axis_scale(&EuvChartAxis::log(), 1.0, 10_000.0, false, None);
    assert!(log_scale.get_ticks().len() >= 5);
    assert!((log_scale.normalize(100.0) - 0.5).abs() < 0.05);
    let dual_opt: EuvChartOption = EuvChartOption::new()
        .x_axis(EuvChartAxis::category(vec![
            "Jan".to_string(),
            "Feb".to_string(),
            "Mar".to_string(),
        ]))
        .y_axis(EuvChartAxis::value().name("Revenue").unit_suffix("k"))
        .y_axis(
            EuvChartAxis::value()
                .name("Margin")
                .unit_suffix("%")
                .position(EuvChartPosition::Right),
        )
        .add_series(EuvChartSeries::new(
            "Revenue",
            EuvChartSeriesData::Bar(
                EuvChartBarSeries::from_values(vec![120.0, 155.0, 190.0]).axes(0, 0),
            ),
        ))
        .add_series(EuvChartSeries::new(
            "Margin",
            EuvChartSeriesData::Line(
                EuvChartLineSeries::from_values(vec![18.0, 22.0, 27.0]).axes(0, 1),
            ),
        ));
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let mut dl: DisplayList = DisplayList::default();
    let res: EuvChartRenderResult = render_echart_to_display_list(
        &dual_opt,
        &EuvChartState::from_option(&dual_opt),
        Rect::new(0.0, 0.0, 720.0, 380.0),
        tokens,
        &mut dl,
    );
    assert_eq!(res.get_y_scales().len(), 2);
    assert!(res.get_y_scales()[0].get_max() >= 190.0);
    assert!(res.get_y_scales()[1].get_max() <= 40.0);
}

#[test]
fn interactive_legend_toggle_removes_series_and_rescales_y_axis() {
    let opt: EuvChartOption = EuvChartOption::new()
        .legend(EuvChartLegend::at(EuvChartPosition::Top))
        .x_axis(EuvChartAxis::category(vec![
            "Q1".to_string(),
            "Q2".to_string(),
            "Q3".to_string(),
        ]))
        .y_axis(EuvChartAxis::value())
        .add_series(EuvChartSeries::new(
            "Small",
            EuvChartSeriesData::Bar(EuvChartBarSeries::from_values(vec![10.0, 20.0, 30.0])),
        ))
        .add_series(EuvChartSeries::new(
            "Huge",
            EuvChartSeriesData::Bar(EuvChartBarSeries::from_values(vec![500.0, 800.0, 1000.0])),
        ));
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let bounds: Rect = Rect::new(0.0, 0.0, 640.0, 340.0);
    let mut state: EuvChartState = EuvChartState::from_option(&opt);
    let mut dl_before: DisplayList = DisplayList::default();
    let res_before: EuvChartRenderResult =
        render_echart_to_display_list(&opt, &state, bounds, tokens, &mut dl_before);
    assert_eq!(res_before.get_legend_hits().len(), 2);
    assert_eq!(res_before.get_hits().len(), 6);
    let y_max_before: f32 = res_before.get_y_scales()[0].get_max();
    assert!(y_max_before >= 1000.0);
    let now_vis: bool = state.toggle_series("Huge");
    assert!(!now_vis);
    let mut dl_after: DisplayList = DisplayList::default();
    let res_after: EuvChartRenderResult =
        render_echart_to_display_list(&opt, &state, bounds, tokens, &mut dl_after);
    assert_eq!(res_after.get_hits().len(), 3);
    assert!(
        res_after
            .get_hits()
            .iter()
            .all(|h: &EuvChartDatumHit| h.get_series_name() == "Small")
    );
    let y_max_after: f32 = res_after.get_y_scales()[0].get_max();
    assert!(y_max_after < 100.0);
    assert!(dl_after.get_commands().len() < dl_before.get_commands().len());
}

#[test]
fn data_zoom_slicing_visual_map_tooltip_annotations_dataset_and_toolbox_work() {
    let ds: EuvChartDataset = EuvChartDataset::new(
        vec![
            "Month".to_string(),
            "Sales".to_string(),
            "Profit".to_string(),
        ],
        vec![
            vec![
                EuvChartCell::Text("Jan".to_string()),
                EuvChartCell::Number(100.0),
                EuvChartCell::Number(24.0),
            ],
            vec![
                EuvChartCell::Text("Feb".to_string()),
                EuvChartCell::Number(140.0),
                EuvChartCell::Number(36.0),
            ],
            vec![
                EuvChartCell::Text("Mar".to_string()),
                EuvChartCell::Number(210.0),
                EuvChartCell::Number(58.0),
            ],
            vec![
                EuvChartCell::Text("Apr".to_string()),
                EuvChartCell::Number(180.0),
                EuvChartCell::Number(45.0),
            ],
            vec![
                EuvChartCell::Text("May".to_string()),
                EuvChartCell::Number(260.0),
                EuvChartCell::Number(72.0),
            ],
            vec![
                EuvChartCell::Text("Jun".to_string()),
                EuvChartCell::Number(310.0),
                EuvChartCell::Number(88.0),
            ],
        ],
    );
    let opt: EuvChartOption = EuvChartOption::new()
        .dataset(ds)
        .tooltip(EuvChartTooltip::axis(EuvChartAxisPointerType::Cross))
        .toolbox(EuvChartToolbox::full())
        .data_zoom(EuvChartDataZoom::inside())
        .data_zoom(EuvChartDataZoom::slider().range(0.0, 100.0))
        .add_series(EuvChartSeries::new(
            "Sales",
            EuvChartSeriesData::Line(
                EuvChartLineSeries::default()
                    .encode(EuvChartEncode::default().x("Month").y("Sales"))
                    .mark_point(EuvChartMarkPoint::new(EuvChartMarkPointKind::Max, "Peak"))
                    .mark_line(EuvChartMarkLine::new(EuvChartMarkLineKind::Average, "Avg"))
                    .mark_area(EuvChartMarkArea::x_range(1.0, 3.0, "Promo")),
            ),
        ))
        .add_series(EuvChartSeries::new(
            "Profit",
            EuvChartSeriesData::Bar(
                EuvChartBarSeries::default()
                    .encode(EuvChartEncode::default().x("Month").y("Profit")),
            ),
        ));
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let bounds: Rect = Rect::new(0.0, 0.0, 760.0, 420.0);
    let mut state: EuvChartState = EuvChartState::from_option(&opt);
    let mut dl_full: DisplayList = DisplayList::default();
    let res_full: EuvChartRenderResult =
        render_echart_to_display_list(&opt, &state, bounds, tokens, &mut dl_full);
    assert!(res_full.slider_rects_opt().is_some());
    assert_eq!(res_full.get_x_scales()[0].get_categories().len(), 6);
    assert!(
        dl_full
            .get_commands()
            .iter()
            .any(|c: &DrawCommand| matches!(
                c,
                DrawCommand::TextRun { text, .. } if text.contains("Peak") || text.contains("Promo")
            ))
    );
    let sample_anchor: Point = res_full.get_hits()[2].get_anchor();
    let item_hits: Vec<EuvChartDatumHit> = hit_test_chart(
        res_full.get_hits(),
        sample_anchor,
        EuvChartTooltipTrigger::Item,
    );
    assert_eq!(item_hits.len(), 1);
    let axis_hits: Vec<EuvChartDatumHit> = hit_test_chart(
        res_full.get_hits(),
        sample_anchor,
        EuvChartTooltipTrigger::Axis,
    );
    assert_eq!(axis_hits.len(), 2);
    state.set_hover_pos(Some(sample_anchor));
    let mut dl_tip: DisplayList = DisplayList::default();
    let res_tip: EuvChartRenderResult =
        render_echart_to_display_list(&opt, &state, bounds, tokens, &mut dl_tip);
    assert_eq!(res_tip.get_active_tooltip_hits().len(), 2);
    assert!(dl_tip.get_commands().len() > dl_full.get_commands().len());
    assert!(state.zoom_by_wheel(-1.0, 0.5, 10.0));
    state.set_zoom(25.0, 75.0, 10.0);
    let mut dl_zoom: DisplayList = DisplayList::default();
    let res_zoom: EuvChartRenderResult =
        render_echart_to_display_list(&opt, &state, bounds, tokens, &mut dl_zoom);
    assert!(res_zoom.get_x_scales()[0].get_categories().len() < 6);
    assert!(state.pan_by_pct(15.0));
    assert_eq!(state.cycle_magic_type(), EuvChartMagicType::Line);
    assert_eq!(state.cycle_magic_type(), EuvChartMagicType::Bar);
    assert_eq!(state.cycle_magic_type(), EuvChartMagicType::Stack);
    assert!(state.toggle_data_view());
    let dv: EuvChartDataViewTable = extract_data_view_table(&opt, &state);
    assert_eq!(dv.get_headers().len(), 3);
    assert_eq!(dv.get_rows().len(), 6);
    state.restore(&opt);
    assert_eq!(state.get_magic_type(), EuvChartMagicType::Default);
    assert!(!state.get_data_view_open());
    assert!((state.get_zoom_start_pct() - 0.0).abs() < 1e-3);
    assert!((state.get_zoom_end_pct() - 100.0).abs() < 1e-3);
    let cont_vm: EuvChartVisualMap = EuvChartVisualMap::continuous(0.0, 100.0)
        .colors(vec![Color::hex(0x0000FF), Color::hex(0xFF0000)]);
    let cont_eval: EuvChartVisualEvaluator =
        build_visual_evaluator(Some(&cont_vm), 0.0, 100.0, tokens);
    let c_lo: Color = cont_eval.map_color(0.0);
    let c_mid: Color = cont_eval.map_color(50.0);
    let c_hi: Color = cont_eval.map_color(100.0);
    assert_eq!(c_lo, Color::hex(0x0000FF));
    assert_eq!(c_hi, Color::hex(0xFF0000));
    assert!(c_mid.get_r() > 100 && c_mid.get_b() > 100);
    let piece_vm: EuvChartVisualMap = EuvChartVisualMap::piecewise(vec![
        EuvChartVisualPiece::new(None, Some(30.0), "Low", Color::hex(0x22C55E)),
        EuvChartVisualPiece::new(Some(30.0), Some(70.0), "Mid", Color::hex(0xF59E0B)),
        EuvChartVisualPiece::new(Some(70.0), None, "High", Color::hex(0xEF4444)),
    ]);
    let piece_eval: EuvChartVisualEvaluator =
        build_visual_evaluator(Some(&piece_vm), 0.0, 100.0, tokens);
    assert_eq!(piece_eval.map_color(12.0), Color::hex(0x22C55E));
    assert_eq!(piece_eval.map_color(50.0), Color::hex(0xF59E0B));
    assert_eq!(piece_eval.map_color(95.0), Color::hex(0xEF4444));
}

#[test]
fn euv_echart_widget_mounts_reactively_in_headless_app() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let opt_a: EuvChartOption = EuvChartOption::new()
        .title(EuvChartTitle::new("Live Telemetry"))
        .legend(EuvChartLegend::at(EuvChartPosition::Top))
        .toolbox(EuvChartToolbox::full())
        .tooltip(EuvChartTooltip::axis(EuvChartAxisPointerType::Line))
        .data_zoom(EuvChartDataZoom::slider())
        .x_axis(EuvChartAxis::category(vec![
            "T1".to_string(),
            "T2".to_string(),
            "T3".to_string(),
            "T4".to_string(),
        ]))
        .y_axis(EuvChartAxis::value())
        .add_series(EuvChartSeries::new(
            "FPS",
            EuvChartSeriesData::Line(EuvChartLineSeries::from_values(vec![
                60.0, 90.0, 118.0, 120.0,
            ])),
        ));
    let opt_sig: Signal<EuvChartOption> = Signal::new(opt_a.clone());
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(800, 520).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                EuvEChart {
                    option_signal: opt_sig
                    tokens: tokens
                    width: Length::Percent(100.0)
                    height: Length::Px(320.0)
                }
            }
        }
    });
    let frame1: FrameStats = app.step_frame().unwrap();
    assert_ne!(frame1.get_frame_hash(), 0);
    assert!(frame1.get_draw_commands() >= 20);
    let init_props: EuvEChart = <EuvEChart as ComponentProps>::init_props();
    assert_eq!(init_props.get_width(), Length::Percent(100.0));
    assert_eq!(init_props.get_height(), Length::Px(340.0));
    assert!(init_props.get_interactive());
    let opt_b: EuvChartOption = opt_a.add_series(EuvChartSeries::new(
        "Target",
        EuvChartSeriesData::Bar(EuvChartBarSeries::from_values(vec![
            120.0, 120.0, 120.0, 120.0,
        ])),
    ));
    let _: bool = opt_sig.set(opt_b);
    let frame2: FrameStats = app.step_frame().unwrap();
    assert_ne!(frame2.get_frame_hash(), frame1.get_frame_hash());
}

fn find_chart_a11y_center(app: &App, label: &str) -> Option<Point> {
    app.get_scene().with_read(
        |tree: &SceneTree| {
            for (idx, n) in tree.get_nodes().iter().enumerate() {
                if !n.get_alive() || n.style_ref().get_layout().get_display() == Display::None {
                    continue;
                }
                if n.a11y_props_ref()
                    .is_some_and(|a: &A11yProps| a.get_label() == label)
                {
                    let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                    if let Some(b) = tree.node_screen_bounds(nid)
                        && b.get_width() > 0.0
                        && b.get_height() > 0.0
                    {
                        return Some(Point::new(
                            b.get_x() + b.get_width() * 0.5,
                            b.get_y() + b.get_height() * 0.5,
                        ));
                    }
                }
            }
            None
        },
        None,
    )
}

fn click_chart_point(app: &mut App, pt: Point) {
    let wid: WindowId = app.get_window_state().get_id();
    let ev: PointerEvent = PointerEvent::mouse(pt.get_x(), pt.get_y());
    app.dispatch_window_event(WindowEvent::PointerMove(wid, ev));
    app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
    let _: FrameStats = app.step_frame().unwrap();
}

#[test]
fn euv_echart_fullscreen_edge_to_edge_hides_app_shell_chrome_and_restores_on_exit_and_dispose() {
    reset_reactive_runtime();
    EuvOverlayStack::clear();
    reset_fullscreen_state();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let route_sig: Signal<String> = Signal::new("/".to_string());
    let clicks: Signal<u32> = Signal::new(0);
    let show_chart: Signal<bool> = Signal::new(true);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(390, 844).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let clicks_cb: Signal<u32> = clicks;
    let _: NodeId = app.mount(move || {
        let on_ping = move |_: &UiEvent| {
            let _: bool = clicks_cb.update(|c: &mut u32| *c += 1);
        };
        let sidebar: NodeId = view! {
            EuvSidebarNav {
                items: vec![EuvNavItem::new("/".to_string(), "Home".to_string())]
                active_route: route_sig
                tokens: tokens
            }
        };
        let bottom_nav: NodeId = view! {
            EuvBottomNav {
                items: vec![
                    EuvNavItem::new("/".to_string(), "Home".to_string()).section("H"),
                    EuvNavItem::new("/charts".to_string(), "Charts".to_string()).section("C"),
                ]
                active_route: route_sig
                tokens: tokens
            }
        };
        let build_chart_slot = move || -> Vec<NodeId> {
            if !show_chart.get() {
                return Vec::new();
            }
            let opt: EuvChartOption = EuvChartOption::new()
                .title(EuvChartTitle::new("Immersive Line"))
                .legend(EuvChartLegend::at(EuvChartPosition::Top))
                .toolbox(EuvChartToolbox::full())
                .tooltip(EuvChartTooltip::axis(EuvChartAxisPointerType::Cross))
                .data_zoom(EuvChartDataZoom::inside())
                .data_zoom(EuvChartDataZoom::slider())
                .x_axis(EuvChartAxis::category(vec![
                    "Mon".to_string(),
                    "Tue".to_string(),
                    "Wed".to_string(),
                    "Thu".to_string(),
                ]))
                .y_axis(EuvChartAxis::value())
                .add_series(EuvChartSeries::new(
                    "Throughput",
                    EuvChartSeriesData::Line(
                        EuvChartLineSeries::from_values(vec![42.0, 68.0, 54.0, 92.0]).smooth(true),
                    ),
                ));
            let node: NodeId = view! {
                EuvEChart {
                    option: opt
                    tokens: tokens
                    width: Length::Percent(100.0)
                    height: Length::Px(260.0)
                }
            };
            vec![node]
        };
        let main_col: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                EuvButton {
                    label: "ChartUnderlyingPing".to_string()
                    tokens: tokens
                    on_click: on_ping
                }
                column {
                    width: Length::Percent(100.0)
                    bind_children: build_chart_slot
                }
            }
        };
        view! {
            EuvAppShell {
                brand: "Euv Studio".to_string()
                sidebar: sidebar
                main_content: main_col
                tokens: tokens
                bottom_nav: bottom_nav
                show_fps: true
            }
        }
    });
    let _: FrameStats = app.step_frame().unwrap();
    assert!(find_chart_a11y_center(&app, "Restore").is_some());
    assert!(find_chart_a11y_center(&app, "Magic Type").is_some());
    assert!(find_chart_a11y_center(&app, "Data View").is_some());
    let fs_btn: Point =
        find_chart_a11y_center(&app, "Fullscreen").expect("chart fullscreen toolbox icon");
    click_chart_point(&mut app, fs_btn);
    assert!(is_fullscreen_active());
    assert!(app.is_immersive_fullscreen());
    assert_eq!(EuvOverlayStack::active_count(), 1);
    let (portal_bounds, chrome_hidden): (Option<Rect>, bool) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut fs_rect: Option<Rect> = None;
            let mut any_visible_chrome: bool = false;
            for (idx, n) in tree.get_nodes().iter().enumerate() {
                if !n.get_alive() {
                    continue;
                }
                let lay: LayoutStyle = n.style_ref().get_layout();
                let paint: PaintStyle = n.style_ref().get_paint();
                if lay.get_z_index() == Z_INDEX_CHROME && lay.get_display() != Display::None {
                    any_visible_chrome = true;
                }
                if matches!(n.get_kind(), NodeKind::Portal)
                    && lay.get_z_index() == Z_INDEX_FULLSCREEN
                    && lay.get_display() == Display::Flex
                {
                    assert_eq!(lay.get_position(), Position::Fixed);
                    assert_eq!(lay.get_padding(), Edges::all(0.0));
                    assert_eq!(lay.get_margin(), Edges::all(0.0));
                    assert_eq!(paint.get_radius(), CornerRadius::all(0.0));
                    assert_eq!(paint.get_border().get_width(), 0.0);
                    let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                    fs_rect = tree.node_screen_bounds(nid);
                }
            }
            (fs_rect, !any_visible_chrome)
        },
        (None, false),
    );
    let fs_rect: Rect = portal_bounds.expect("edge-to-edge chart fullscreen portal");
    assert!((fs_rect.get_x() - 0.0).abs() < 0.5);
    assert!((fs_rect.get_y() - 0.0).abs() < 0.5);
    assert!((fs_rect.get_width() - 390.0).abs() < 0.5);
    assert!((fs_rect.get_height() - 844.0).abs() < 0.5);
    assert!(chrome_hidden);
    let close_btn: Point =
        find_chart_a11y_center(&app, "Close").expect("chart fullscreen close icon");
    click_chart_point(&mut app, close_btn);
    assert!(!is_fullscreen_active());
    assert!(!app.is_immersive_fullscreen());
    assert_eq!(EuvOverlayStack::active_count(), 0);
    let (chrome_restored, any_open_fs_portal): (bool, bool) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut visible_chrome: usize = 0;
            let mut open_fs: bool = false;
            for n in tree.get_nodes().iter() {
                if !n.get_alive() {
                    continue;
                }
                let lay: LayoutStyle = n.style_ref().get_layout();
                if lay.get_z_index() == Z_INDEX_CHROME && lay.get_display() == Display::Flex {
                    visible_chrome += 1;
                }
                if matches!(n.get_kind(), NodeKind::Portal)
                    && lay.get_z_index() == Z_INDEX_FULLSCREEN
                    && lay.get_display() != Display::None
                {
                    open_fs = true;
                }
            }
            (visible_chrome >= 2, open_fs)
        },
        (false, true),
    );
    assert!(chrome_restored);
    assert!(!any_open_fs_portal);
    let ping_pt: Point = app
        .get_scene()
        .with_read(
            |tree: &SceneTree| {
                for (idx, n) in tree.get_nodes().iter().enumerate() {
                    if n.get_alive()
                        && matches!(n.get_kind(), NodeKind::Text(t) if t == "ChartUnderlyingPing")
                    {
                        let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                        if let Some(b) = tree.node_screen_bounds(nid) {
                            return Some(Point::new(
                                b.get_x() + b.get_width() * 0.5,
                                b.get_y() + b.get_height() * 0.5,
                            ));
                        }
                    }
                }
                None
            },
            None,
        )
        .expect("underlying button must remain clickable after fullscreen exit");
    click_chart_point(&mut app, ping_pt);
    assert_eq!(clicks.get_untracked(), 1);
    let fs_btn_again: Point =
        find_chart_a11y_center(&app, "Fullscreen").expect("chart fullscreen icon on reopen");
    click_chart_point(&mut app, fs_btn_again);
    assert!(is_fullscreen_active());
    assert!(app.is_immersive_fullscreen());
    assert_eq!(EuvOverlayStack::active_count(), 1);
    let _: bool = show_chart.set(false);
    let _: FrameStats = app.step_frame().unwrap();
    assert!(!is_fullscreen_active());
    assert!(!app.is_immersive_fullscreen());
    assert_eq!(EuvOverlayStack::active_count(), 0);
}
