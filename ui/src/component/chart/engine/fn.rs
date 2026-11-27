use super::*;

/// Renders a complete declarative `EuvChartOption` and interactive `EuvChartState` into `dl` inside `bounds`.
///
/// # Arguments
///
/// - `&EuvChartOption` - Declarative Apache ECharts option document.
/// - `&EuvChartState` - Active interactive runtime state.
/// - `Rect` - Target logical canvas bounds.
/// - `ThemeTokens` - Active UI theme tokens.
/// - `&mut DisplayList` - Destination vector draw command buffer.
///
/// # Returns
///
/// - `EuvChartRenderResult` - Resolved plot geometry, axis scales, datum hits, legend boxes, and slider hit rects.
#[must_use]
pub fn render_echart_to_display_list(
    option: &EuvChartOption,
    state: &EuvChartState,
    bounds: Rect,
    tokens: ThemeTokens,
    dl: &mut DisplayList,
) -> EuvChartRenderResult {
    let mut result: EuvChartRenderResult = EuvChartRenderResult::default();
    if bounds.get_width() < 16.0 || bounds.get_height() < 16.0 {
        return result;
    }
    let p: ThemePalette = tokens.get_palette();
    let bg: Color = option.background_opt().unwrap_or_else(|| p.get_card());
    dl.push(DrawCommand::FillRoundedRect {
        rect: bounds,
        color: bg,
        radius: CornerRadius::all(0.0),
        border: Border::default(),
    });
    let palette: Vec<Color> = option.resolved_palette(tokens);
    let effective_series: Vec<EuvChartSeries> =
        apply_magic_type(option.get_series(), state.get_magic_type());
    let visible: Vec<bool> = effective_series
        .iter()
        .map(|s: &EuvChartSeries| state.is_series_visible(s.get_name().as_str()))
        .collect();
    let mut content: Rect = Rect::new(
        bounds.get_x() + ENGINE_CANVAS_PAD,
        bounds.get_y() + ENGINE_CANVAS_PAD,
        (bounds.get_width() - ENGINE_CANVAS_PAD * 2.0).max(20.0),
        (bounds.get_height() - ENGINE_CANVAS_PAD * 2.0).max(20.0),
    );
    if let Some(title) = option.title_opt()
        && (!title.get_text().is_empty() || !title.get_subtext().is_empty())
    {
        let align: TextAlign = match title.get_align() {
            EuvChartAlign::Left => TextAlign::Left,
            EuvChartAlign::Center => TextAlign::Center,
            EuvChartAlign::Right => TextAlign::Right,
        };
        let title_w: f32 = if option.toolbox_opt().is_some_and(EuvChartToolbox::get_show)
            && title.get_align() != EuvChartAlign::Right
        {
            (content.get_width() - 96.0).max(80.0)
        } else {
            content.get_width()
        };
        draw_label(
            dl,
            Rect::new(content.get_x(), content.get_y(), title_w, 16.0),
            title.get_text().clone(),
            13.5,
            FontWeight::SemiBold,
            align,
            p.get_foreground(),
        );
        let mut used_h: f32 = 19.0;
        if !title.get_subtext().is_empty() {
            draw_label(
                dl,
                Rect::new(content.get_x(), content.get_y() + 17.0, title_w, 13.0),
                title.get_subtext().clone(),
                10.5,
                FontWeight::Normal,
                align,
                p.get_muted_foreground(),
            );
            used_h = 33.0;
        }
        content = Rect::new(
            content.get_x(),
            content.get_y() + used_h,
            content.get_width(),
            (content.get_height() - used_h).max(20.0),
        );
    }
    if let Some(legend) = option.legend_opt()
        && legend.get_show()
    {
        content = render_legend_chrome(
            dl,
            content,
            legend,
            (&effective_series, &visible),
            (&palette, tokens),
            result.get_mut_legend_hits(),
        );
    }
    let has_slider: bool = option
        .get_data_zoom()
        .iter()
        .any(|dz: &EuvChartDataZoom| dz.get_kind() == EuvChartDataZoomKind::Slider);
    let slider_area: Option<Rect> = if has_slider && content.get_height() > 96.0 {
        let sh: f32 = ENGINE_SLIDER_HEIGHT;
        let area: Rect = Rect::new(
            content.get_x() + 12.0,
            content.get_y() + content.get_height() - sh,
            (content.get_width() - 24.0).max(40.0),
            sh,
        );
        content = Rect::new(
            content.get_x(),
            content.get_y(),
            content.get_width(),
            (content.get_height() - sh - 6.0).max(20.0),
        );
        Some(area)
    } else {
        None
    };
    let (vm_min, vm_max): (f32, f32) = compute_visual_data_range(&effective_series, &visible);
    let visual_eval: Option<EuvChartVisualEvaluator> =
        if option.visual_map_opt().is_some() || needs_implicit_visual_map(&effective_series) {
            Some(build_visual_evaluator(
                option.visual_map_opt(),
                vm_min,
                vm_max,
                tokens,
            ))
        } else {
            None
        };
    if let Some(vm) = option.visual_map_opt()
        && vm.get_show()
        && let Some(eval) = visual_eval.as_ref()
    {
        content = render_visual_map_chrome(dl, content, vm, eval, tokens);
    }
    let frame: ChartDrawFrame<'_> = ChartDrawFrame {
        tokens,
        palette: palette.as_slice(),
        visual_eval: visual_eval.as_ref(),
        dataset: option.dataset_opt(),
        hover_pos: state.hover_pos_opt(),
        selected_pie_slice: state.selected_pie_slice_opt(),
    };
    let mut plot_rect: Rect = content;
    let mut cartesian_rendered: bool = false;
    if let Some(geo) = option.geo_opt() {
        render_geo_system(
            dl,
            plot_rect,
            geo,
            &effective_series,
            &visible,
            &frame,
            result.get_mut_hits(),
        );
    } else if effective_series
        .iter()
        .any(|s: &EuvChartSeries| matches!(s.get_data(), EuvChartSeriesData::GeoMap(_)))
    {
        let default_geo: EuvChartGeo = EuvChartGeo::world();
        render_geo_system(
            dl,
            plot_rect,
            &default_geo,
            &effective_series,
            &visible,
            &frame,
            result.get_mut_hits(),
        );
    }
    if let Some(radar) = option.radar_opt() {
        render_radar_system(
            dl,
            plot_rect,
            radar,
            &effective_series,
            &visible,
            &frame,
            result.get_mut_hits(),
        );
    }
    if let Some(polar) = option.polar_opt() {
        render_polar_system(
            dl,
            plot_rect,
            polar,
            &effective_series,
            &visible,
            &frame,
            result.get_mut_hits(),
        );
    }
    if let Some(single) = option.single_axis_opt() {
        render_single_axis_system(
            dl,
            plot_rect,
            single,
            &effective_series,
            &visible,
            &frame,
            result.get_mut_hits(),
        );
    }
    if let Some(cal) = option.calendar_opt() {
        let eval_fallback: EuvChartVisualEvaluator = visual_eval
            .clone()
            .unwrap_or_else(|| build_visual_evaluator(None, vm_min, vm_max, tokens));
        for (s_i, s) in effective_series.iter().enumerate() {
            if !visible[s_i] {
                continue;
            }
            if let EuvChartSeriesData::Heatmap(hm) = s.get_data()
                && hm.get_coordinate_system() == EuvChartCoordinateSystem::Calendar
            {
                let meta: SeriesMeta<'_> = SeriesMeta {
                    index: s_i,
                    name: s.get_name().as_str(),
                    color: s.color_opt().unwrap_or(palette[s_i % palette.len().max(1)]),
                };
                render_calendar_heatmap(
                    dl,
                    plot_rect,
                    cal,
                    meta,
                    hm,
                    &eval_fallback,
                    result.get_mut_hits(),
                );
            }
        }
    }
    for (s_i, s) in effective_series.iter().enumerate() {
        if !visible[s_i] {
            continue;
        }
        let meta: SeriesMeta<'_> = SeriesMeta {
            index: s_i,
            name: s.get_name().as_str(),
            color: s.color_opt().unwrap_or(palette[s_i % palette.len().max(1)]),
        };
        match s.get_data() {
            EuvChartSeriesData::Pie(pie) => {
                render_pie_series(dl, plot_rect, meta, pie, &frame, result.get_mut_hits());
            }
            EuvChartSeriesData::Gauge(g) => {
                render_gauge_series(dl, plot_rect, meta, g, &frame, result.get_mut_hits());
            }
            EuvChartSeriesData::Sunburst(sun) => {
                render_sunburst_series(dl, plot_rect, meta, sun, &frame, result.get_mut_hits());
            }
            EuvChartSeriesData::Funnel(fun) => {
                render_funnel_series(dl, plot_rect, meta, fun, &frame, result.get_mut_hits());
            }
            EuvChartSeriesData::Treemap(tm) => {
                render_treemap_series(dl, plot_rect, meta, tm, &frame, result.get_mut_hits());
            }
            EuvChartSeriesData::Sankey(sk) => {
                render_sankey_series(dl, plot_rect, meta, sk, &frame, result.get_mut_hits());
            }
            EuvChartSeriesData::Graph(gr) => {
                render_graph_series(dl, plot_rect, meta, gr, &frame, result.get_mut_hits());
            }
            EuvChartSeriesData::Tree(tr) => {
                render_tree_series(dl, plot_rect, meta, tr, &frame, result.get_mut_hits());
            }
            EuvChartSeriesData::Parallel(pr) => {
                render_parallel_series(dl, plot_rect, meta, pr, &frame, result.get_mut_hits());
            }
            EuvChartSeriesData::ThemeRiver(tr) => {
                render_themeriver_series(dl, plot_rect, meta, tr, &frame, result.get_mut_hits());
            }
            _ => {}
        }
    }
    if has_cartesian_series(&effective_series) {
        cartesian_rendered = true;
        let sub_in: SubplotRenderInput<'_> = SubplotRenderInput {
            option,
            state,
            series: &effective_series,
            visible: &visible,
        };
        plot_rect = render_cartesian_subplots(dl, content, sub_in, &frame, &mut result);
    }
    result.set_plot_rect(plot_rect);
    if let Some(s_rect) = slider_area {
        let preview_vals: Vec<f32> =
            extract_primary_preview_values(&effective_series, &visible, option.dataset_opt());
        let slider_geom: EuvChartSliderRects =
            render_data_zoom_slider(dl, s_rect, state, &preview_vals, tokens);
        result.set_slider_rects(Some(slider_geom));
    }
    if let Some(hover) = state.hover_pos_opt()
        && let Some(tip) = option.tooltip_opt()
        && tip.get_show()
        && tip.get_trigger() != EuvChartTooltipTrigger::None
    {
        let matched: Vec<EuvChartDatumHit> =
            hit_test_chart(result.get_hits(), hover, tip.get_trigger());
        if !matched.is_empty() {
            if cartesian_rendered {
                render_axis_pointer(
                    dl,
                    plot_rect,
                    hover,
                    &matched,
                    tip.get_axis_pointer(),
                    tokens,
                );
            }
            render_tooltip_card(dl, bounds, hover, &matched, tokens);
            result.set_active_tooltip_hits(matched);
        }
    }
    result
}

/// Applies the active toolbox `MagicType` transformation (`Line`, `Bar`, `Stack`) onto Cartesian line/bar series.
///
/// # Arguments
///
/// - `&[EuvChartSeries]` - Original authored series list.
/// - `EuvChartMagicType` - Active MagicType override.
///
/// # Returns
///
/// - `Vec<EuvChartSeries>` - Transformed series list ready for rendering.
#[must_use]
pub fn apply_magic_type(
    series: &[EuvChartSeries],
    magic: EuvChartMagicType,
) -> Vec<EuvChartSeries> {
    if magic == EuvChartMagicType::Default {
        return series.to_vec();
    }
    let mut out: Vec<EuvChartSeries> = Vec::with_capacity(series.len());
    for s in series.iter() {
        let mut next: EuvChartSeries = s.clone();
        match (magic, s.get_data()) {
            (EuvChartMagicType::Line, EuvChartSeriesData::Bar(b))
                if b.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                let mut line: EuvChartLineSeries =
                    EuvChartLineSeries::from_nullable(b.get_values().clone())
                        .axes(b.get_x_axis_index(), b.get_y_axis_index());
                if let Some(enc) = b.encode_opt() {
                    line = line.encode(enc.clone());
                }
                next.set_data(EuvChartSeriesData::Line(line));
            }
            (EuvChartMagicType::Bar, EuvChartSeriesData::Line(l))
                if l.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d
                    && l.get_xy_points().is_empty() =>
            {
                let mut bar: EuvChartBarSeries = EuvChartBarSeries::default();
                bar.set_values(l.get_values().clone());
                bar.set_x_axis_index(l.get_x_axis_index());
                bar.set_y_axis_index(l.get_y_axis_index());
                if let Some(enc) = l.encode_opt() {
                    bar = bar.encode(enc.clone());
                }
                next.set_data(EuvChartSeriesData::Bar(bar));
            }
            (EuvChartMagicType::Stack, EuvChartSeriesData::Line(l))
                if l.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                let mut stacked: EuvChartLineSeries = l.clone();
                if stacked.stack_opt().is_none() {
                    stacked.set_stack(Some(ENGINE_MAGIC_STACK_KEY.to_string()));
                }
                if stacked.get_area_opacity() < 0.05 {
                    stacked.set_area_opacity(0.24);
                }
                next.set_data(EuvChartSeriesData::Line(stacked));
            }
            (EuvChartMagicType::Stack, EuvChartSeriesData::Bar(b))
                if b.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                let mut stacked: EuvChartBarSeries = b.clone();
                if stacked.stack_opt().is_none() {
                    stacked.set_stack(Some(ENGINE_MAGIC_STACK_KEY.to_string()));
                }
                next.set_data(EuvChartSeriesData::Bar(stacked));
            }
            _ => {}
        }
        out.push(next);
    }
    out
}

/// Runs deterministic tooltip/click hit-testing against emitted `hits` under `Item` or `Axis` trigger semantics.
///
/// # Arguments
///
/// - `&[EuvChartDatumHit]` - Emitted chart datum hits.
/// - `Point` - Probe pointer coordinate in logical canvas space.
/// - `EuvChartTooltipTrigger` - Active trigger snapping mode.
///
/// # Returns
///
/// - `Vec<EuvChartDatumHit>` - Matched datum hits (single best for `Item`, full category column for `Axis`).
#[must_use]
pub fn hit_test_chart(
    hits: &[EuvChartDatumHit],
    pos: Point,
    trigger: EuvChartTooltipTrigger,
) -> Vec<EuvChartDatumHit> {
    if hits.is_empty() || trigger == EuvChartTooltipTrigger::None {
        return Vec::new();
    }
    match trigger {
        EuvChartTooltipTrigger::None => Vec::new(),
        EuvChartTooltipTrigger::Item => {
            let mut best: Option<(f32, &EuvChartDatumHit)> = None;
            for hit in hits.iter() {
                let inside: bool = hit.contains(pos);
                let dx: f32 = pos.get_x() - hit.get_anchor().get_x();
                let dy: f32 = pos.get_y() - hit.get_anchor().get_y();
                let dist_sq: f32 = dx * dx + dy * dy;
                let max_d: f32 = (hit.get_hit_radius() * 1.45).max(18.0);
                if inside || dist_sq <= max_d * max_d {
                    let score: f32 = if inside { dist_sq * 0.1 } else { dist_sq };
                    if best.is_none_or(|(b_score, _): (f32, &EuvChartDatumHit)| score < b_score) {
                        best = Some((score, hit));
                    }
                }
            }
            best.map(|(_, h): (f32, &EuvChartDatumHit)| vec![h.clone()])
                .unwrap_or_default()
        }
        EuvChartTooltipTrigger::Axis => {
            let mut best_dx: f32 = f32::INFINITY;
            let mut target_cat: Option<&str> = None;
            let mut target_idx: usize = 0;
            for hit in hits.iter() {
                let dx: f32 = (pos.get_x() - hit.get_anchor().get_x()).abs();
                if dx < best_dx {
                    best_dx = dx;
                    target_cat = Some(hit.get_category_label().as_str());
                    target_idx = hit.get_data_index();
                }
            }
            if best_dx > 64.0 {
                return Vec::new();
            }
            let Some(cat) = target_cat else {
                return Vec::new();
            };
            let mut column: Vec<EuvChartDatumHit> = Vec::new();
            for hit in hits.iter() {
                if hit.get_category_label().as_str() == cat
                    || (cat.is_empty() && hit.get_data_index() == target_idx)
                {
                    column.push(hit.clone());
                }
            }
            column
        }
    }
}

/// Extracts a human-readable tabular `EuvChartDataViewTable` for the toolbox `DataView` mode across all series types.
///
/// # Arguments
///
/// - `&EuvChartOption` - Declarative chart option.
/// - `&EuvChartState` - Active interactive state (honors series visibility and MagicType).
///
/// # Returns
///
/// - `EuvChartDataViewTable` - Structured headers and rows.
#[must_use]
pub fn extract_data_view_table(
    option: &EuvChartOption,
    state: &EuvChartState,
) -> EuvChartDataViewTable {
    if let Some(ds) = option.dataset_opt()
        && !ds.get_rows().is_empty()
    {
        let headers: Vec<String> = ds.get_dimensions().clone();
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(ds.get_rows().len());
        for row in ds.get_rows().iter() {
            let cells: Vec<String> = row
                .iter()
                .map(|c: &EuvChartCell| match c {
                    EuvChartCell::Text(s) => s.clone(),
                    EuvChartCell::Number(v) => format_chart_number(*v, ""),
                    EuvChartCell::Null => "-".to_string(),
                })
                .collect();
            rows.push(cells);
        }
        return EuvChartDataViewTable { headers, rows };
    }
    let series: Vec<EuvChartSeries> = apply_magic_type(option.get_series(), state.get_magic_type());
    let mut headers: Vec<String> = vec![ENGINE_COL_CATEGORY.to_string()];
    let mut rows: Vec<Vec<String>> = Vec::new();
    let x_cats: Vec<String> = option
        .get_x_axes()
        .first()
        .map(|ax: &EuvChartAxis| ax.get_categories().clone())
        .unwrap_or_default();
    let mut cat_series_indices: Vec<usize> = Vec::new();
    for (i, s) in series.iter().enumerate() {
        if !state.is_series_visible(s.get_name().as_str()) {
            continue;
        }
        if matches!(
            s.get_data(),
            EuvChartSeriesData::Line(_)
                | EuvChartSeriesData::Bar(_)
                | EuvChartSeriesData::PictorialBar(_)
        ) {
            headers.push(s.get_name().clone());
            cat_series_indices.push(i);
        }
    }
    if !cat_series_indices.is_empty() {
        let mut max_len: usize = x_cats.len();
        for &si in cat_series_indices.iter() {
            let len: usize = match series[si].get_data() {
                EuvChartSeriesData::Line(l) => l.get_values().len().max(l.get_xy_points().len()),
                EuvChartSeriesData::Bar(b) => b.get_values().len(),
                EuvChartSeriesData::PictorialBar(p) => p.get_values().len(),
                _ => 0,
            };
            max_len = max_len.max(len);
        }
        for r_i in 0..max_len {
            let mut row: Vec<String> = Vec::with_capacity(headers.len());
            row.push(
                x_cats
                    .get(r_i)
                    .cloned()
                    .unwrap_or_else(|| format!("{}", r_i + 1)),
            );
            for &si in cat_series_indices.iter() {
                let cell: String = match series[si].get_data() {
                    EuvChartSeriesData::Line(l) => l
                        .get_values()
                        .get(r_i)
                        .and_then(|v: &Option<f32>| *v)
                        .or_else(|| l.get_xy_points().get(r_i).map(|p: &(f32, f32)| p.1))
                        .map(|v: f32| format_chart_number(v, ""))
                        .unwrap_or_else(|| "-".to_string()),
                    EuvChartSeriesData::Bar(b) => b
                        .get_values()
                        .get(r_i)
                        .and_then(|v: &Option<f32>| *v)
                        .map(|v: f32| format_chart_number(v, ""))
                        .unwrap_or_else(|| "-".to_string()),
                    EuvChartSeriesData::PictorialBar(p) => p
                        .get_values()
                        .get(r_i)
                        .map(|v: &f32| format_chart_number(*v, ""))
                        .unwrap_or_else(|| "-".to_string()),
                    _ => "-".to_string(),
                };
                row.push(cell);
            }
            rows.push(row);
        }
        return EuvChartDataViewTable { headers, rows };
    }
    headers = vec![
        ENGINE_COL_SERIES.to_string(),
        ENGINE_COL_CATEGORY.to_string(),
        ENGINE_COL_VALUE.to_string(),
    ];
    for s in series.iter() {
        if !state.is_series_visible(s.get_name().as_str()) {
            continue;
        }
        match s.get_data() {
            EuvChartSeriesData::Pie(pie) => {
                for sl in pie.get_slices().iter() {
                    rows.push(vec![
                        s.get_name().clone(),
                        sl.get_name().clone(),
                        format_chart_number(sl.get_value(), ""),
                    ]);
                }
            }
            EuvChartSeriesData::Funnel(fun) => {
                for st in fun.get_stages().iter() {
                    rows.push(vec![
                        s.get_name().clone(),
                        st.get_name().clone(),
                        format_chart_number(st.get_value(), ""),
                    ]);
                }
            }
            EuvChartSeriesData::Gauge(g) => {
                rows.push(vec![
                    s.get_name().clone(),
                    g.get_title().clone(),
                    format!("{}{}", format_chart_number(g.get_value(), ""), g.get_unit()),
                ]);
            }
            EuvChartSeriesData::GeoMap(gm) => {
                for d in gm.get_data().iter() {
                    rows.push(vec![
                        s.get_name().clone(),
                        d.get_region_name().clone(),
                        format_chart_number(d.get_value(), ""),
                    ]);
                }
            }
            EuvChartSeriesData::Candlestick(cs) => {
                for (i, c) in cs.get_candles().iter().enumerate() {
                    let lbl: String = x_cats
                        .get(i)
                        .cloned()
                        .unwrap_or_else(|| format!("{}", i + 1));
                    rows.push(vec![
                        s.get_name().clone(),
                        lbl,
                        format!(
                            "O:{} C:{} L:{} H:{}",
                            format_chart_number(c.get_open(), ""),
                            format_chart_number(c.get_close(), ""),
                            format_chart_number(c.get_low(), ""),
                            format_chart_number(c.get_high(), ""),
                        ),
                    ]);
                }
            }
            EuvChartSeriesData::Scatter(sc) | EuvChartSeriesData::EffectScatter(sc) => {
                for (i, pt) in sc.get_points().iter().enumerate() {
                    let lbl: String = if pt.get_label().is_empty() {
                        format!("#{}", i + 1)
                    } else {
                        pt.get_label().clone()
                    };
                    rows.push(vec![
                        s.get_name().clone(),
                        lbl,
                        format!(
                            "({}, {})",
                            format_chart_number(pt.get_x(), ""),
                            format_chart_number(pt.get_y(), "")
                        ),
                    ]);
                }
            }
            EuvChartSeriesData::Radar(rs) => {
                for item in rs.get_items().iter() {
                    let sum: f32 = item.get_values().iter().copied().sum();
                    rows.push(vec![
                        s.get_name().clone(),
                        item.get_name().clone(),
                        format_chart_number(sum, ""),
                    ]);
                }
            }
            EuvChartSeriesData::Treemap(tm) => {
                for node in tm.get_nodes().iter() {
                    rows.push(vec![
                        s.get_name().clone(),
                        node.get_name().clone(),
                        format_chart_number(node.total_value(), ""),
                    ]);
                }
            }
            EuvChartSeriesData::Sunburst(sb) => {
                for node in sb.get_nodes().iter() {
                    rows.push(vec![
                        s.get_name().clone(),
                        node.get_name().clone(),
                        format_chart_number(node.total_value(), ""),
                    ]);
                }
            }
            EuvChartSeriesData::Sankey(sk) => {
                for lk in sk.get_links().iter() {
                    rows.push(vec![
                        s.get_name().clone(),
                        format!("{} -> {}", lk.get_source(), lk.get_target()),
                        format_chart_number(lk.get_value(), ""),
                    ]);
                }
            }
            _ => {
                rows.push(vec![s.get_name().clone(), "-".to_string(), "-".to_string()]);
            }
        }
    }
    EuvChartDataViewTable { headers, rows }
}

/// Checks whether any series in `series` targets a 2D Cartesian subplot grid.
///
/// # Arguments
///
/// - `&[EuvChartSeries]` - Effective chart series slice.
///
/// # Returns
///
/// - `bool` - True when Cartesian axes should be computed and rendered.
#[must_use]
fn has_cartesian_series(series: &[EuvChartSeries]) -> bool {
    for s in series.iter() {
        let is_cart: bool = match s.get_data() {
            EuvChartSeriesData::Line(l) => {
                l.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d
            }
            EuvChartSeriesData::Bar(b) => {
                b.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d
            }
            EuvChartSeriesData::Scatter(sc) | EuvChartSeriesData::EffectScatter(sc) => {
                sc.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d
            }
            EuvChartSeriesData::Heatmap(hm) => {
                hm.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d
            }
            EuvChartSeriesData::Lines(ln) => {
                ln.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d
            }
            EuvChartSeriesData::Candlestick(_)
            | EuvChartSeriesData::Boxplot(_)
            | EuvChartSeriesData::PictorialBar(_)
            | EuvChartSeriesData::Custom(_) => true,
            _ => false,
        };
        if is_cart {
            return true;
        }
    }
    false
}

/// Returns whether any series inherently relies on a `VisualMap` gradient when none is explicitly set.
///
/// # Arguments
///
/// - `&[EuvChartSeries]` - Effective chart series slice.
///
/// # Returns
///
/// - `bool` - True when an implicit default visual-map ramp should be built.
#[must_use]
fn needs_implicit_visual_map(series: &[EuvChartSeries]) -> bool {
    series.iter().any(|s: &EuvChartSeries| {
        matches!(
            s.get_data(),
            EuvChartSeriesData::Heatmap(_) | EuvChartSeriesData::GeoMap(_)
        )
    })
}

/// Scans visible series to derive fallback `(min, max)` bounds for visual-map color ramps.
///
/// # Arguments
///
/// - `&[EuvChartSeries]` - Effective chart series.
/// - `&[bool]` - Visibility flags.
///
/// # Returns
///
/// - `(f32, f32)` - Observed `(min, max)` scalar domain.
#[must_use]
fn compute_visual_data_range(series: &[EuvChartSeries], visible: &[bool]) -> (f32, f32) {
    let mut lo: f32 = f32::INFINITY;
    let mut hi: f32 = f32::NEG_INFINITY;
    for (i, s) in series.iter().enumerate() {
        if !visible.get(i).copied().unwrap_or(true) {
            continue;
        }
        match s.get_data() {
            EuvChartSeriesData::Heatmap(hm) => {
                for c in hm.get_cells().iter() {
                    lo = lo.min(c.get_value());
                    hi = hi.max(c.get_value());
                }
            }
            EuvChartSeriesData::GeoMap(gm) => {
                for d in gm.get_data().iter() {
                    lo = lo.min(d.get_value());
                    hi = hi.max(d.get_value());
                }
            }
            EuvChartSeriesData::Scatter(sc) | EuvChartSeriesData::EffectScatter(sc) => {
                for pt in sc.get_points().iter() {
                    let v: f32 = pt.value_opt().unwrap_or(pt.get_y());
                    lo = lo.min(v);
                    hi = hi.max(v);
                }
            }
            EuvChartSeriesData::Parallel(pr) => {
                for r in pr.get_rows().iter() {
                    if let Some(&v) = r.get_values().last() {
                        lo = lo.min(v);
                        hi = hi.max(v);
                    }
                }
            }
            _ => {}
        }
    }
    if !lo.is_finite() {
        (0.0, 100.0)
    } else if (hi - lo).abs() < 1e-5 {
        (lo, lo + 1.0)
    } else {
        (lo, hi)
    }
}

/// Renders interactive legend chips along the requested dock edge and returns the remaining inner rect.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Available content rectangle.
/// - `&EuvChartLegend` - Legend configuration.
/// - `(&[EuvChartSeries], &[bool])` - Effective chart series and visibility flags.
/// - `(&[Color], ThemeTokens)` - Resolved categorical palette and UI theme tokens.
/// - `&mut Vec<EuvChartLegendHit>` - Output clickable legend chip boxes.
///
/// # Returns
///
/// - `Rect` - Remaining content area after reserving legend chrome.
#[must_use]
fn render_legend_chrome(
    dl: &mut DisplayList,
    area: Rect,
    legend: &EuvChartLegend,
    series_vis: (&[EuvChartSeries], &[bool]),
    style: (&[Color], ThemeTokens),
    out_hits: &mut Vec<EuvChartLegendHit>,
) -> Rect {
    let (series, visible): (&[EuvChartSeries], &[bool]) = series_vis;
    let (palette, tokens): (&[Color], ThemeTokens) = style;
    let entries: Vec<(usize, String, Color, bool)> =
        build_legend_entries(legend, series, visible, palette);
    if entries.is_empty() {
        return area;
    }
    let p: ThemePalette = tokens.get_palette();
    let fg: Color = p.get_foreground();
    let muted: Color = p.get_muted_foreground();
    let pos: EuvChartPosition = legend.get_position();
    match pos {
        EuvChartPosition::Top | EuvChartPosition::Bottom => {
            let mut row_x: f32 = area.get_x();
            let mut rows: usize = 1;
            let mut layout: Vec<(f32, usize, f32)> = Vec::with_capacity(entries.len());
            for (_, name, _, _) in entries.iter() {
                let chip_w: f32 = ((name.chars().count() as f32) * 6.5 + 24.0).clamp(44.0, 156.0);
                if row_x + chip_w > area.get_x() + area.get_width() && row_x > area.get_x() {
                    row_x = area.get_x();
                    rows += 1;
                }
                layout.push((row_x, rows - 1, chip_w));
                row_x += chip_w + 8.0;
            }
            let total_h: f32 = (rows as f32) * ENGINE_LEGEND_ROW_H + 4.0;
            let start_y: f32 = if pos == EuvChartPosition::Top {
                area.get_y()
            } else {
                area.get_y() + area.get_height() - total_h + 2.0
            };
            for (idx, &(s_idx, ref name, col, is_vis)) in entries.iter().enumerate() {
                let (cx, row_i, cw): (f32, usize, f32) = layout[idx];
                let cy: f32 = start_y + (row_i as f32) * ENGINE_LEGEND_ROW_H;
                let chip_rect: Rect = Rect::new(cx, cy, cw, 18.0);
                let sw_col: Color = if is_vis {
                    col
                } else {
                    muted.multiply_alpha(0.35)
                };
                let txt_col: Color = if is_vis {
                    fg
                } else {
                    muted.multiply_alpha(0.55)
                };
                dl.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(cx + 2.0, cy + 5.0, 10.0, 8.0),
                    color: sw_col,
                    radius: CornerRadius::all(2.5),
                    border: Border::default(),
                });
                draw_label(
                    dl,
                    Rect::new(cx + 16.0, cy + 3.0, cw - 18.0, 13.0),
                    name.clone(),
                    10.0,
                    FontWeight::Medium,
                    TextAlign::Left,
                    txt_col,
                );
                if !is_vis {
                    dl.push(DrawCommand::Line {
                        from: Point::new(cx + 16.0, cy + 9.0),
                        to: Point::new(cx + cw - 4.0, cy + 9.0),
                        color: muted.multiply_alpha(0.6),
                        width: 1.0,
                    });
                }
                out_hits.push(EuvChartLegendHit {
                    series_index: s_idx,
                    name: name.clone(),
                    color: col,
                    visible: is_vis,
                    rect: chip_rect,
                });
            }
            if pos == EuvChartPosition::Top {
                Rect::new(
                    area.get_x(),
                    area.get_y() + total_h,
                    area.get_width(),
                    (area.get_height() - total_h).max(20.0),
                )
            } else {
                Rect::new(
                    area.get_x(),
                    area.get_y(),
                    area.get_width(),
                    (area.get_height() - total_h).max(20.0),
                )
            }
        }
        EuvChartPosition::Left | EuvChartPosition::Right => {
            let side_w: f32 = ENGINE_LEGEND_SIDE_W.min(area.get_width() * 0.34);
            let start_x: f32 = if pos == EuvChartPosition::Left {
                area.get_x()
            } else {
                area.get_x() + area.get_width() - side_w
            };
            for (idx, &(s_idx, ref name, col, is_vis)) in entries.iter().enumerate() {
                let cy: f32 = area.get_y() + (idx as f32) * ENGINE_LEGEND_ROW_H;
                if cy + 16.0 > area.get_y() + area.get_height() {
                    break;
                }
                let chip_rect: Rect = Rect::new(start_x, cy, side_w, 18.0);
                let sw_col: Color = if is_vis {
                    col
                } else {
                    muted.multiply_alpha(0.35)
                };
                let txt_col: Color = if is_vis {
                    fg
                } else {
                    muted.multiply_alpha(0.55)
                };
                dl.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(start_x + 2.0, cy + 5.0, 10.0, 8.0),
                    color: sw_col,
                    radius: CornerRadius::all(2.5),
                    border: Border::default(),
                });
                draw_label(
                    dl,
                    Rect::new(start_x + 16.0, cy + 3.0, (side_w - 18.0).max(20.0), 13.0),
                    name.clone(),
                    10.0,
                    FontWeight::Medium,
                    TextAlign::Left,
                    txt_col,
                );
                out_hits.push(EuvChartLegendHit {
                    series_index: s_idx,
                    name: name.clone(),
                    color: col,
                    visible: is_vis,
                    rect: chip_rect,
                });
            }
            if pos == EuvChartPosition::Left {
                Rect::new(
                    area.get_x() + side_w + 6.0,
                    area.get_y(),
                    (area.get_width() - side_w - 6.0).max(20.0),
                    area.get_height(),
                )
            } else {
                Rect::new(
                    area.get_x(),
                    area.get_y(),
                    (area.get_width() - side_w - 6.0).max(20.0),
                    area.get_height(),
                )
            }
        }
    }
}

/// Builds ordered legend entries `(series_idx, name, color, visible)` from `legend` and `series`.
///
/// # Arguments
///
/// - `&EuvChartLegend` - Legend configuration.
/// - `&[EuvChartSeries]` - Effective chart series.
/// - `&[bool]` - Visibility flags.
/// - `&[Color]` - Resolved categorical palette.
///
/// # Returns
///
/// - `Vec<(usize, String, Color, bool)>` - Legend display descriptors.
#[must_use]
fn build_legend_entries(
    legend: &EuvChartLegend,
    series: &[EuvChartSeries],
    visible: &[bool],
    palette: &[Color],
) -> Vec<(usize, String, Color, bool)> {
    let pal_len: usize = palette.len().max(1);
    if !legend.get_items().is_empty() {
        return legend
            .get_items()
            .iter()
            .enumerate()
            .map(|(i, label): (usize, &String)| {
                let vis: bool = series
                    .iter()
                    .enumerate()
                    .find(|(_, s): &(usize, &EuvChartSeries)| s.get_name() == label)
                    .map(|(si, _): (usize, &EuvChartSeries)| {
                        visible.get(si).copied().unwrap_or(true)
                    })
                    .unwrap_or(true);
                (i, label.clone(), palette[i % pal_len], vis)
            })
            .collect();
    }
    let mut out: Vec<(usize, String, Color, bool)> = Vec::new();
    for (i, s) in series.iter().enumerate() {
        if !s.get_name().is_empty() {
            let col: Color = s.color_opt().unwrap_or(palette[i % pal_len]);
            out.push((
                i,
                s.get_name().clone(),
                col,
                visible.get(i).copied().unwrap_or(true),
            ));
        }
    }
    out
}

/// Renders a continuous gradient bar or piecewise range legend for `EuvChartVisualMap`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Available content area.
/// - `&EuvChartVisualMap` - Visual-map configuration.
/// - `&EuvChartVisualEvaluator` - Resolved color stops and pieces.
/// - `ThemeTokens` - Active UI theme tokens.
///
/// # Returns
///
/// - `Rect` - Remaining content area after reserving visual-map chrome.
#[must_use]
fn render_visual_map_chrome(
    dl: &mut DisplayList,
    area: Rect,
    vm: &EuvChartVisualMap,
    eval: &EuvChartVisualEvaluator,
    tokens: ThemeTokens,
) -> Rect {
    let p: ThemePalette = tokens.get_palette();
    let bar_h: f32 = ENGINE_VISUAL_BAR_HEIGHT;
    let strip_y: f32 = if vm.get_position() == EuvChartPosition::Top {
        area.get_y()
    } else {
        area.get_y() + area.get_height() - bar_h
    };
    match vm.get_kind() {
        EuvChartVisualMapKind::Continuous => {
            let grad_w: f32 = (area.get_width() * 0.42).clamp(90.0, 200.0);
            let grad_x: f32 = area.get_x() + (area.get_width() - grad_w) * 0.5;
            let grad_y: f32 = strip_y + 6.0;
            let steps: usize = 24;
            let seg_w: f32 = grad_w / (steps as f32);
            for i in 0..steps {
                let t: f32 = (i as f32) / ((steps - 1) as f32);
                let v: f32 = eval.get_min() + (eval.get_max() - eval.get_min()) * t;
                let col: Color = eval.map_color(v);
                dl.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(grad_x + (i as f32) * seg_w, grad_y, seg_w + 0.6, 10.0),
                    color: col,
                    radius: CornerRadius::all(0.0),
                    border: Border::default(),
                });
            }
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(grad_x, grad_y, grad_w, 10.0),
                color: COLOR_TRANSPARENT,
                radius: CornerRadius::all(3.0),
                border: Border::new(1.0, p.get_border()),
            });
            draw_label(
                dl,
                Rect::new((grad_x - 42.0).max(area.get_x()), grad_y - 1.0, 38.0, 12.0),
                format_chart_number(eval.get_min(), ""),
                9.0,
                FontWeight::Medium,
                TextAlign::Right,
                p.get_muted_foreground(),
            );
            draw_label(
                dl,
                Rect::new(grad_x + grad_w + 4.0, grad_y - 1.0, 42.0, 12.0),
                format_chart_number(eval.get_max(), ""),
                9.0,
                FontWeight::Medium,
                TextAlign::Left,
                p.get_muted_foreground(),
            );
        }
        EuvChartVisualMapKind::Piecewise => {
            let mut cx: f32 = area.get_x() + 4.0;
            let cy: f32 = strip_y + 4.0;
            for piece in eval.get_pieces().iter() {
                let lbl: String = if piece.get_label().is_empty() {
                    format!(
                        "{}..{}",
                        piece
                            .min_opt()
                            .map(|v: f32| format_chart_number(v, ""))
                            .unwrap_or_default(),
                        piece
                            .max_opt()
                            .map(|v: f32| format_chart_number(v, ""))
                            .unwrap_or_default()
                    )
                } else {
                    piece.get_label().clone()
                };
                let chip_w: f32 = ((lbl.chars().count() as f32) * 6.0 + 20.0).clamp(40.0, 120.0);
                if cx + chip_w > area.get_x() + area.get_width() {
                    break;
                }
                dl.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(cx, cy + 3.0, 10.0, 10.0),
                    color: piece.get_color(),
                    radius: CornerRadius::all(2.0),
                    border: Border::default(),
                });
                draw_label(
                    dl,
                    Rect::new(cx + 14.0, cy + 2.0, chip_w - 14.0, 12.0),
                    lbl,
                    9.0,
                    FontWeight::Medium,
                    TextAlign::Left,
                    p.get_muted_foreground(),
                );
                cx += chip_w + 8.0;
            }
        }
    }
    if vm.get_position() == EuvChartPosition::Top {
        Rect::new(
            area.get_x(),
            area.get_y() + bar_h + 2.0,
            area.get_width(),
            (area.get_height() - bar_h - 2.0).max(20.0),
        )
    } else {
        Rect::new(
            area.get_x(),
            area.get_y(),
            area.get_width(),
            (area.get_height() - bar_h - 2.0).max(20.0),
        )
    }
}

/// Renders one or more Cartesian subplot grids, computes dual/multi-axis scales, and dispatches all Cartesian series.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Available content area.
/// - `SubplotRenderInput<'_>` - Declarative option, interactive state, series, and visibility flags.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut EuvChartRenderResult` - Output render result collector.
///
/// # Returns
///
/// - `Rect` - Primary plot area rectangle.
#[must_use]
fn render_cartesian_subplots(
    dl: &mut DisplayList,
    area: Rect,
    input: SubplotRenderInput<'_>,
    frame: &ChartDrawFrame<'_>,
    result: &mut EuvChartRenderResult,
) -> Rect {
    let option: &EuvChartOption = input.option;
    let state: &EuvChartState = input.state;
    let series: &[EuvChartSeries] = input.series;
    let visible: &[bool] = input.visible;
    let grids: Vec<EuvChartGrid> = if option.get_grid().is_empty() {
        vec![EuvChartGrid::default()]
    } else {
        option.get_grid().clone()
    };
    let grid_count: usize = grids.len();
    let x_axes: Vec<EuvChartAxis> = build_effective_x_axes(option, series);
    let y_axes: Vec<EuvChartAxis> = build_effective_y_axes(option);
    let total_cats: usize = x_axes
        .first()
        .map(|ax: &EuvChartAxis| ax.get_categories().len())
        .unwrap_or(0)
        .max(max_series_category_len(series, option.dataset_opt()));
    let zoom: EuvChartZoomSlice = compute_zoom_slice(
        total_cats,
        state.get_zoom_start_pct(),
        state.get_zoom_end_pct(),
    );
    let mut x_scales: Vec<EuvChartAxisScale> = Vec::with_capacity(x_axes.len());
    for (ax_i, ax) in x_axes.iter().enumerate() {
        let (d_min, d_max, anchor_zero): (f32, f32, bool) =
            compute_axis_domain(ax_i, true, ax, series, visible, option.dataset_opt(), zoom);
        let z_opt: Option<EuvChartZoomSlice> = if ax.get_axis_type() == EuvChartAxisType::Category
            || !option.get_data_zoom().is_empty()
        {
            Some(zoom)
        } else {
            None
        };
        x_scales.push(build_axis_scale(ax, d_min, d_max, anchor_zero, z_opt));
    }
    let mut y_scales: Vec<EuvChartAxisScale> = Vec::with_capacity(y_axes.len());
    for (ax_i, ax) in y_axes.iter().enumerate() {
        let (d_min, d_max, anchor_zero): (f32, f32, bool) =
            compute_axis_domain(ax_i, false, ax, series, visible, option.dataset_opt(), zoom);
        y_scales.push(build_axis_scale(ax, d_min, d_max, anchor_zero, None));
    }
    let mut plot_rects: Vec<Rect> = Vec::with_capacity(grid_count);
    let band_h: f32 = area.get_height() / (grid_count as f32);
    let has_right_y: bool = y_axes.len() > 1
        || y_axes
            .iter()
            .any(|a: &EuvChartAxis| a.position_opt() == Some(EuvChartPosition::Right));
    for (g_i, g) in grids.iter().enumerate() {
        let sub_y: f32 = area.get_y() + (g_i as f32) * band_h;
        let left_m: f32 = if g.get_contain_label() {
            g.get_left().max(42.0)
        } else {
            g.get_left()
        }
        .min(area.get_width() * 0.28);
        let right_m: f32 = if has_right_y {
            g.get_right().max(44.0)
        } else {
            g.get_right()
        }
        .min(area.get_width() * 0.25);
        let top_m: f32 = g.get_top().min(band_h * 0.28);
        let bot_m: f32 = g.get_bottom().min(band_h * 0.3);
        let pr: Rect = Rect::new(
            area.get_x() + left_m,
            sub_y + top_m,
            (area.get_width() - left_m - right_m).max(32.0),
            (band_h - top_m - bot_m).max(32.0),
        );
        plot_rects.push(pr);
        let g_x_axes: Vec<EuvChartAxis> = x_axes
            .iter()
            .filter(|a: &&EuvChartAxis| a.get_grid_index() == g_i)
            .cloned()
            .collect();
        let g_x_scales: Vec<EuvChartAxisScale> = x_axes
            .iter()
            .enumerate()
            .filter(|(_, a): &(usize, &EuvChartAxis)| a.get_grid_index() == g_i)
            .map(|(i, _): (usize, &EuvChartAxis)| x_scales[i].clone())
            .collect();
        let g_y_axes: Vec<EuvChartAxis> = y_axes
            .iter()
            .filter(|a: &&EuvChartAxis| a.get_grid_index() == g_i)
            .cloned()
            .collect();
        let g_y_scales: Vec<EuvChartAxisScale> = y_axes
            .iter()
            .enumerate()
            .filter(|(_, a): &(usize, &EuvChartAxis)| a.get_grid_index() == g_i)
            .map(|(i, _): (usize, &EuvChartAxis)| y_scales[i].clone())
            .collect();
        let bundle: CartesianAxesBundle<'_> = CartesianAxesBundle {
            x_axes: &g_x_axes,
            x_scales: &g_x_scales,
            y_axes: &g_y_axes,
            y_scales: &g_y_scales,
            show_border: g.get_show_border(),
        };
        render_cartesian_axes(dl, pr, &bundle, frame.tokens);
    }
    let mut bar_slots: BarSlotLayout = BarSlotLayout::default();
    for (s_i, s) in series.iter().enumerate() {
        if !visible.get(s_i).copied().unwrap_or(true) {
            continue;
        }
        if let EuvChartSeriesData::Bar(b) = s.get_data()
            && b.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d
        {
            let key: String = b
                .stack_opt()
                .map(str::to_string)
                .unwrap_or_else(|| format!("s_{}", s_i));
            if !bar_slots.slot_keys.contains(&key) {
                bar_slots.slot_keys.push(key);
            }
        }
    }
    let mut line_stacks: HashMap<(String, usize), f32> = HashMap::new();
    let pal_len: usize = frame.palette.len().max(1);
    for (s_i, s) in series.iter().enumerate() {
        if !visible.get(s_i).copied().unwrap_or(true) {
            continue;
        }
        let meta: SeriesMeta<'_> = SeriesMeta {
            index: s_i,
            name: s.get_name().as_str(),
            color: s.color_opt().unwrap_or(frame.palette[s_i % pal_len]),
        };
        let (xi, yi): (usize, usize) = series_axis_indices(s.get_data());
        let x_idx: usize = xi.min(x_scales.len().saturating_sub(1));
        let y_idx: usize = yi.min(y_scales.len().saturating_sub(1));
        let g_idx: usize = x_axes
            .get(x_idx)
            .map(EuvChartAxis::get_grid_index)
            .unwrap_or(0)
            .min(plot_rects.len().saturating_sub(1));
        let ctx: CartesianRenderCtx<'_> = CartesianRenderCtx {
            plot_rect: plot_rects[g_idx],
            x_scale: &x_scales[x_idx],
            y_scale: &y_scales[y_idx],
            zoom,
            frame,
        };
        match s.get_data() {
            EuvChartSeriesData::Bar(b)
                if b.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                render_bar_series(dl, meta, b, &ctx, &mut bar_slots, result.get_mut_hits());
            }
            EuvChartSeriesData::Candlestick(cs) => {
                render_candlestick_series(dl, meta, cs, &ctx, result.get_mut_hits());
            }
            EuvChartSeriesData::Boxplot(bx) => {
                render_boxplot_series(dl, meta, bx, &ctx, result.get_mut_hits());
            }
            EuvChartSeriesData::Heatmap(hm)
                if hm.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                let fallback: EuvChartVisualEvaluator = frame
                    .visual_eval
                    .cloned()
                    .unwrap_or_else(|| build_visual_evaluator(None, 0.0, 100.0, frame.tokens));
                render_cartesian_heatmap(dl, meta, hm, &ctx, &fallback, result.get_mut_hits());
            }
            EuvChartSeriesData::PictorialBar(pb) => {
                render_pictorial_bar_series(dl, meta, pb, &ctx, result.get_mut_hits());
            }
            EuvChartSeriesData::Line(l)
                if l.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                render_line_series(dl, meta, l, &ctx, &mut line_stacks, result.get_mut_hits());
            }
            EuvChartSeriesData::Scatter(sc)
                if sc.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                render_scatter_series(dl, meta, sc, false, &ctx, result.get_mut_hits());
            }
            EuvChartSeriesData::EffectScatter(sc)
                if sc.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                render_scatter_series(dl, meta, sc, true, &ctx, result.get_mut_hits());
            }
            EuvChartSeriesData::Lines(ln)
                if ln.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                render_cartesian_lines_series(dl, meta, ln, &ctx, result.get_mut_hits());
            }
            EuvChartSeriesData::Custom(cs) => {
                render_custom_series(dl, cs, &ctx);
            }
            _ => {}
        }
    }
    result.set_x_scales(x_scales);
    result.set_y_scales(y_scales);
    plot_rects.first().copied().unwrap_or(area)
}

/// Returns `(x_axis_index, y_axis_index)` for a Cartesian series variant.
///
/// # Arguments
///
/// - `&EuvChartSeriesData` - Series payload.
///
/// # Returns
///
/// - `(usize, usize)` - Bound `(x_axis_index, y_axis_index)`.
#[must_use]
fn series_axis_indices(data: &EuvChartSeriesData) -> (usize, usize) {
    match data {
        EuvChartSeriesData::Line(l) => (l.get_x_axis_index(), l.get_y_axis_index()),
        EuvChartSeriesData::Bar(b) => (b.get_x_axis_index(), b.get_y_axis_index()),
        EuvChartSeriesData::Scatter(sc) | EuvChartSeriesData::EffectScatter(sc) => {
            (sc.get_x_axis_index(), sc.get_y_axis_index())
        }
        EuvChartSeriesData::Candlestick(c) => (c.get_x_axis_index(), c.get_y_axis_index()),
        EuvChartSeriesData::Boxplot(b) => (b.get_x_axis_index(), b.get_y_axis_index()),
        EuvChartSeriesData::Heatmap(h) => (h.get_x_axis_index(), h.get_y_axis_index()),
        EuvChartSeriesData::Lines(l) => (l.get_x_axis_index(), l.get_y_axis_index()),
        EuvChartSeriesData::PictorialBar(p) => (p.get_x_axis_index(), p.get_y_axis_index()),
        EuvChartSeriesData::Custom(c) => (c.get_x_axis_index(), c.get_y_axis_index()),
        _ => (0, 0),
    }
}

/// Builds effective X axes, inferring default categories from `Dataset` or series lengths when omitted.
///
/// # Arguments
///
/// - `&EuvChartOption` - Declarative chart option.
/// - `&[EuvChartSeries]` - Effective chart series.
///
/// # Returns
///
/// - `Vec<EuvChartAxis>` - Non-empty X axis list.
#[must_use]
fn build_effective_x_axes(option: &EuvChartOption, series: &[EuvChartSeries]) -> Vec<EuvChartAxis> {
    if !option.get_x_axes().is_empty() {
        let mut axes: Vec<EuvChartAxis> = option.get_x_axes().clone();
        if let Some(first) = axes.first_mut()
            && first.get_axis_type() == EuvChartAxisType::Category
            && first.get_categories().is_empty()
            && let Some(ds) = option.dataset_opt()
            && let Some(dim0) = ds.get_dimensions().first()
        {
            first.set_categories(dataset_extract_labels(ds, dim0.as_str()));
        }
        return axes;
    }
    let has_only_xy: bool = series.iter().all(|s: &EuvChartSeries| match s.get_data() {
        EuvChartSeriesData::Scatter(_)
        | EuvChartSeriesData::EffectScatter(_)
        | EuvChartSeriesData::Lines(_) => true,
        EuvChartSeriesData::Line(l) => !l.get_xy_points().is_empty() && l.get_values().is_empty(),
        _ => false,
    });
    if has_only_xy {
        return vec![EuvChartAxis::value()];
    }
    if let Some(ds) = option.dataset_opt()
        && let Some(dim0) = ds.get_dimensions().first()
    {
        return vec![EuvChartAxis::category(dataset_extract_labels(
            ds,
            dim0.as_str(),
        ))];
    }
    let len: usize = max_series_category_len(series, option.dataset_opt()).max(1);
    let cats: Vec<String> = (1..=len).map(|i: usize| format!("{}", i)).collect();
    vec![EuvChartAxis::category(cats)]
}

/// Builds effective Y axes (defaulting to a single continuous `Value` axis when omitted).
///
/// # Arguments
///
/// - `&EuvChartOption` - Declarative chart option.
///
/// # Returns
///
/// - `Vec<EuvChartAxis>` - Non-empty Y axis list.
#[must_use]
fn build_effective_y_axes(option: &EuvChartOption) -> Vec<EuvChartAxis> {
    if !option.get_y_axes().is_empty() {
        return option.get_y_axes().clone();
    }
    vec![EuvChartAxis::value()]
}

/// Returns the maximum category sample count across Cartesian series.
///
/// # Arguments
///
/// - `&[EuvChartSeries]` - Effective chart series.
/// - `Option<&EuvChartDataset>` - Optional shared dataset.
///
/// # Returns
///
/// - `usize` - Maximum category length.
#[must_use]
fn max_series_category_len(series: &[EuvChartSeries], dataset: Option<&EuvChartDataset>) -> usize {
    let mut max_len: usize = 0;
    for s in series.iter() {
        let len: usize = match s.get_data() {
            EuvChartSeriesData::Line(l) => resolve_line_values(l, dataset).len(),
            EuvChartSeriesData::Bar(b) => resolve_bar_values(b, dataset).len(),
            EuvChartSeriesData::Candlestick(c) => c.get_candles().len(),
            EuvChartSeriesData::Boxplot(b) => b.get_items().len(),
            EuvChartSeriesData::PictorialBar(p) => p.get_values().len(),
            EuvChartSeriesData::Custom(c) => c.get_values().len(),
            _ => 0,
        };
        max_len = max_len.max(len);
    }
    max_len
}

/// Computes `(data_min, data_max, anchor_zero)` for one X or Y axis across all visible bound series (accounting for stacks and zoom!).
///
/// # Arguments
///
/// - `usize` - Axis index.
/// - `bool` - True for X axis, false for Y axis.
/// - `&EuvChartAxis` - Axis descriptor.
/// - `&[EuvChartSeries]` - Effective series slice.
/// - `&[bool]` - Visibility flags.
/// - `Option<&EuvChartDataset>` - Optional shared dataset.
/// - `EuvChartZoomSlice` - Active data-zoom slice.
///
/// # Returns
///
/// - `(f32, f32, bool)` - `(observed_min, observed_max, anchor_zero)`.
#[must_use]
fn compute_axis_domain(
    axis_idx: usize,
    is_x: bool,
    axis: &EuvChartAxis,
    series: &[EuvChartSeries],
    visible: &[bool],
    dataset: Option<&EuvChartDataset>,
    zoom: EuvChartZoomSlice,
) -> (f32, f32, bool) {
    if axis.get_axis_type() == EuvChartAxisType::Category {
        return (
            0.0,
            axis.get_categories().len().saturating_sub(1) as f32,
            false,
        );
    }
    let mut lo: f32 = f32::INFINITY;
    let mut hi: f32 = f32::NEG_INFINITY;
    let mut anchor_zero: bool = false;
    let mut stack_pos: HashMap<(String, usize), f32> = HashMap::new();
    let mut stack_neg: HashMap<(String, usize), f32> = HashMap::new();
    for (s_i, s) in series.iter().enumerate() {
        if !visible.get(s_i).copied().unwrap_or(true) {
            continue;
        }
        let (xi, yi): (usize, usize) = series_axis_indices(s.get_data());
        let bound_idx: usize = if is_x { xi } else { yi };
        if bound_idx != axis_idx {
            continue;
        }
        match s.get_data() {
            EuvChartSeriesData::Line(l)
                if l.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                if !l.get_xy_points().is_empty() {
                    for &(xv, yv) in l.get_xy_points().iter() {
                        let v: f32 = if is_x { xv } else { yv };
                        if v.is_finite() {
                            lo = lo.min(v);
                            hi = hi.max(v);
                        }
                    }
                } else if !is_x {
                    if l.get_area_opacity() > 0.01 {
                        anchor_zero = true;
                    }
                    let vals: Vec<Option<f32>> = resolve_line_values(l, dataset);
                    let s_i0: usize = zoom.get_start_index().min(vals.len());
                    let e_i0: usize = zoom.get_end_index().clamp(s_i0, vals.len());
                    let slice: &[Option<f32>] = if s_i0 < e_i0 {
                        &vals[s_i0..e_i0]
                    } else {
                        &vals
                    };
                    for (local_i, opt_v) in slice.iter().enumerate() {
                        if let Some(v) = *opt_v
                            && v.is_finite()
                        {
                            if let Some(st) = l.stack_opt() {
                                let acc: &mut f32 =
                                    stack_pos.entry((st.to_string(), local_i)).or_insert(0.0);
                                *acc += v;
                                lo = lo.min(*acc);
                                hi = hi.max(*acc);
                            } else {
                                lo = lo.min(v);
                                hi = hi.max(v);
                            }
                        }
                    }
                }
            }
            EuvChartSeriesData::Bar(b)
                if b.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                anchor_zero = true;
                let vals: Vec<Option<f32>> = resolve_bar_values(b, dataset);
                let s_i0: usize = zoom.get_start_index().min(vals.len());
                let e_i0: usize = zoom.get_end_index().clamp(s_i0, vals.len());
                let slice: &[Option<f32>] = if s_i0 < e_i0 {
                    &vals[s_i0..e_i0]
                } else {
                    &vals
                };
                for (local_i, opt_v) in slice.iter().enumerate() {
                    if let Some(v) = *opt_v
                        && v.is_finite()
                    {
                        if let Some(st) = b.stack_opt() {
                            if v >= 0.0 {
                                let acc: &mut f32 =
                                    stack_pos.entry((st.to_string(), local_i)).or_insert(0.0);
                                *acc += v;
                                hi = hi.max(*acc);
                                lo = lo.min(0.0);
                            } else {
                                let acc: &mut f32 =
                                    stack_neg.entry((st.to_string(), local_i)).or_insert(0.0);
                                *acc += v;
                                lo = lo.min(*acc);
                                hi = hi.max(0.0);
                            }
                        } else {
                            lo = lo.min(v);
                            hi = hi.max(v);
                        }
                    }
                }
            }
            EuvChartSeriesData::Scatter(sc) | EuvChartSeriesData::EffectScatter(sc)
                if sc.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                let pts: Vec<EuvChartScatterPoint> = resolve_scatter_points(sc, dataset);
                for pt in pts.iter() {
                    let v: f32 = if is_x { pt.get_x() } else { pt.get_y() };
                    if v.is_finite() {
                        lo = lo.min(v);
                        hi = hi.max(v);
                    }
                }
            }
            EuvChartSeriesData::Candlestick(cs) if !is_x => {
                let all: &[EuvChartOhlcPoint] = cs.get_candles().as_slice();
                let s_i0: usize = zoom.get_start_index().min(all.len());
                let e_i0: usize = zoom.get_end_index().clamp(s_i0, all.len());
                let slice: &[EuvChartOhlcPoint] = if s_i0 < e_i0 { &all[s_i0..e_i0] } else { all };
                for c in slice.iter() {
                    lo = lo.min(c.get_low());
                    hi = hi.max(c.get_high());
                }
            }
            EuvChartSeriesData::Boxplot(bx) if !is_x => {
                for item in bx.get_items().iter() {
                    lo = lo.min(item.get_min());
                    hi = hi.max(item.get_max());
                    for &ov in item.get_outliers().iter() {
                        lo = lo.min(ov);
                        hi = hi.max(ov);
                    }
                }
            }
            EuvChartSeriesData::PictorialBar(pb) if !is_x => {
                anchor_zero = true;
                for &v in pb.get_values().iter() {
                    lo = lo.min(v);
                    hi = hi.max(v);
                }
            }
            EuvChartSeriesData::Custom(cs) if !is_x => {
                for &v in cs.get_values().iter() {
                    lo = lo.min(v);
                    hi = hi.max(v);
                }
            }
            EuvChartSeriesData::Lines(ln)
                if ln.get_coordinate_system() == EuvChartCoordinateSystem::Cartesian2d =>
            {
                for arc in ln.get_arcs().iter() {
                    if is_x {
                        lo = lo.min(arc.get_from_x()).min(arc.get_to_x());
                        hi = hi.max(arc.get_from_x()).max(arc.get_to_x());
                    } else {
                        lo = lo.min(arc.get_from_y()).min(arc.get_to_y());
                        hi = hi.max(arc.get_from_y()).max(arc.get_to_y());
                    }
                }
            }
            _ => {}
        }
    }
    if !lo.is_finite() || !hi.is_finite() {
        (0.0, 100.0, anchor_zero)
    } else {
        (lo, hi, anchor_zero)
    }
}

/// Extracts scalar preview values from the first visible Cartesian series for the `DataZoom::Slider` sparkline.
///
/// # Arguments
///
/// - `&[EuvChartSeries]` - Effective chart series.
/// - `&[bool]` - Visibility flags.
/// - `Option<&EuvChartDataset>` - Optional shared dataset.
///
/// # Returns
///
/// - `Vec<f32>` - Ordered preview values.
#[must_use]
fn extract_primary_preview_values(
    series: &[EuvChartSeries],
    visible: &[bool],
    dataset: Option<&EuvChartDataset>,
) -> Vec<f32> {
    for (i, s) in series.iter().enumerate() {
        if !visible.get(i).copied().unwrap_or(true) {
            continue;
        }
        match s.get_data() {
            EuvChartSeriesData::Line(l) => {
                let vals: Vec<f32> = resolve_line_values(l, dataset)
                    .into_iter()
                    .flatten()
                    .collect();
                if !vals.is_empty() {
                    return vals;
                }
            }
            EuvChartSeriesData::Bar(b) => {
                let vals: Vec<f32> = resolve_bar_values(b, dataset)
                    .into_iter()
                    .flatten()
                    .collect();
                if !vals.is_empty() {
                    return vals;
                }
            }
            EuvChartSeriesData::Candlestick(c) => {
                let vals: Vec<f32> = c
                    .get_candles()
                    .iter()
                    .map(EuvChartOhlcPoint::get_close)
                    .collect();
                if !vals.is_empty() {
                    return vals;
                }
            }
            _ => {}
        }
    }
    Vec::new()
}

/// Renders the bottom interactive `DataZoom::Slider` track, sparkline preview, active selection range, and handles.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Scrubber bounding area.
/// - `&EuvChartState` - Active zoom percentages.
/// - `&[f32]` - Mini sparkline preview values.
/// - `ThemeTokens` - Active UI theme tokens.
///
/// # Returns
///
/// - `EuvChartSliderRects` - Interactive hit rects for left handle, right handle, and window drag.
#[must_use]
fn render_data_zoom_slider(
    dl: &mut DisplayList,
    area: Rect,
    state: &EuvChartState,
    preview: &[f32],
    tokens: ThemeTokens,
) -> EuvChartSliderRects {
    let p: ThemePalette = tokens.get_palette();
    let track: Rect = Rect::new(
        area.get_x(),
        area.get_y() + 4.0,
        area.get_width(),
        area.get_height() - 8.0,
    );
    dl.push(DrawCommand::FillRoundedRect {
        rect: track,
        color: p.get_muted(),
        radius: CornerRadius::all(4.0),
        border: Border::new(1.0, p.get_border()),
    });
    if preview.len() >= 2 {
        let min_v: f32 = preview.iter().copied().fold(f32::INFINITY, f32::min);
        let max_v: f32 = preview.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let span: f32 = (max_v - min_v).max(1e-5);
        let mut pts: Vec<Point> = Vec::with_capacity(preview.len());
        for (i, &v) in preview.iter().enumerate() {
            let px: f32 =
                track.get_x() + track.get_width() * ((i as f32) / ((preview.len() - 1) as f32));
            let frac: f32 = ((v - min_v) / span).clamp(0.0, 1.0);
            let py: f32 =
                track.get_y() + track.get_height() - 2.0 - frac * (track.get_height() - 4.0);
            pts.push(Point::new(px, py));
        }
        draw_polyline(
            dl,
            &pts,
            p.get_muted_foreground().multiply_alpha(0.5),
            1.2,
            false,
        );
    }
    let sx: f32 = track.get_x() + track.get_width() * (state.get_zoom_start_pct() * 0.01);
    let ex: f32 = track.get_x() + track.get_width() * (state.get_zoom_end_pct() * 0.01);
    let win_rect: Rect = Rect::new(sx, track.get_y(), (ex - sx).max(8.0), track.get_height());
    dl.push(DrawCommand::FillRoundedRect {
        rect: win_rect,
        color: p.get_primary().multiply_alpha(0.22),
        radius: CornerRadius::all(3.0),
        border: Border::new(1.2, p.get_primary()),
    });
    let hw: f32 = 10.0;
    let left_h: Rect = Rect::new(
        sx - hw * 0.5,
        area.get_y() + 1.0,
        hw,
        area.get_height() - 2.0,
    );
    let right_h: Rect = Rect::new(
        ex - hw * 0.5,
        area.get_y() + 1.0,
        hw,
        area.get_height() - 2.0,
    );
    for h_rect in [left_h, right_h].into_iter() {
        dl.push(DrawCommand::FillRoundedRect {
            rect: h_rect,
            color: p.get_card(),
            radius: CornerRadius::all(3.0),
            border: Border::new(1.5, p.get_primary()),
        });
        let gx: f32 = h_rect.get_x() + h_rect.get_width() * 0.5;
        dl.push(DrawCommand::Line {
            from: Point::new(gx, h_rect.get_y() + 6.0),
            to: Point::new(gx, h_rect.get_y() + h_rect.get_height() - 6.0),
            color: p.get_primary(),
            width: 1.2,
        });
    }
    EuvChartSliderRects {
        track_rect: track,
        window_rect: win_rect,
        left_handle: left_h,
        right_handle: right_h,
    }
}

/// Renders the hover `AxisPointer` (`Line`, `Shadow`, or `Cross`) across the Cartesian plot area.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `Point` - Hover pointer coordinates.
/// - `&[EuvChartDatumHit]` - Snapped datum hits.
/// - `EuvChartAxisPointerType` - Crosshair guide style.
/// - `ThemeTokens` - Active UI theme tokens.
fn render_axis_pointer(
    dl: &mut DisplayList,
    plot: Rect,
    hover: Point,
    matched: &[EuvChartDatumHit],
    pointer_type: EuvChartAxisPointerType,
    tokens: ThemeTokens,
) {
    let Some(first) = matched.first() else {
        return;
    };
    let p: ThemePalette = tokens.get_palette();
    let guide_col: Color = p.get_muted_foreground().multiply_alpha(0.65);
    let snap_x: f32 = first
        .get_anchor()
        .get_x()
        .clamp(plot.get_x(), plot.get_x() + plot.get_width());
    match pointer_type {
        EuvChartAxisPointerType::None => {}
        EuvChartAxisPointerType::Line => {
            draw_dashed_line(
                dl,
                Point::new(snap_x, plot.get_y()),
                Point::new(snap_x, plot.get_y() + plot.get_height()),
                guide_col,
                1.0,
                4.0,
                3.0,
            );
        }
        EuvChartAxisPointerType::Shadow => {
            let band_w: f32 = first
                .hit_rect_opt()
                .map(|r: Rect| (r.get_width() * 1.45).clamp(16.0, 64.0))
                .unwrap_or(28.0);
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(
                    (snap_x - band_w * 0.5).max(plot.get_x()),
                    plot.get_y(),
                    band_w.min(plot.get_width()),
                    plot.get_height(),
                ),
                color: p.get_primary().multiply_alpha(0.10),
                radius: CornerRadius::all(2.0),
                border: Border::default(),
            });
        }
        EuvChartAxisPointerType::Cross => {
            let snap_y: f32 = hover
                .get_y()
                .clamp(plot.get_y(), plot.get_y() + plot.get_height());
            draw_dashed_line(
                dl,
                Point::new(snap_x, plot.get_y()),
                Point::new(snap_x, plot.get_y() + plot.get_height()),
                guide_col,
                1.0,
                4.0,
                3.0,
            );
            draw_dashed_line(
                dl,
                Point::new(plot.get_x(), snap_y),
                Point::new(plot.get_x() + plot.get_width(), snap_y),
                guide_col,
                1.0,
                4.0,
                3.0,
            );
        }
    }
}

/// Renders an auto-clamped floating multi-series hover tooltip card inside `bounds`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Outer chart canvas bounds for clamping.
/// - `Point` - Active hover coordinate.
/// - `&[EuvChartDatumHit]` - Matched series datums.
/// - `ThemeTokens` - Active UI theme tokens.
fn render_tooltip_card(
    dl: &mut DisplayList,
    bounds: Rect,
    hover: Point,
    matched: &[EuvChartDatumHit],
    tokens: ThemeTokens,
) {
    if matched.is_empty() {
        return;
    }
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let header: &str = matched[0].get_category_label().as_str();
    let max_rows: usize = matched.len().min(6);
    let mut est_chars: usize = header.chars().count();
    for hit in matched[..max_rows].iter() {
        let len: usize =
            hit.get_series_name().chars().count() + hit.get_detail().chars().count() + 4;
        est_chars = est_chars.max(len);
    }
    let card_w: f32 = ((est_chars as f32) * 6.4 + 28.0)
        .clamp(112.0, ENGINE_TOOLTIP_MAX_W)
        .min((bounds.get_width() - 12.0).max(80.0));
    let header_h: f32 = if header.is_empty() { 0.0 } else { 17.0 };
    let card_h: f32 = header_h + (max_rows as f32) * 16.0 + 14.0;
    let raw_x: f32 = if hover.get_x() + 14.0 + card_w <= bounds.get_x() + bounds.get_width() - 6.0 {
        hover.get_x() + 14.0
    } else {
        hover.get_x() - card_w - 14.0
    };
    let card_x: f32 = raw_x.clamp(
        bounds.get_x() + 6.0,
        (bounds.get_x() + bounds.get_width() - card_w - 6.0).max(bounds.get_x() + 6.0),
    );
    let card_y: f32 = (hover.get_y() - card_h * 0.5).clamp(
        bounds.get_y() + 6.0,
        (bounds.get_y() + bounds.get_height() - card_h - 6.0).max(bounds.get_y() + 6.0),
    );
    let card_rect: Rect = Rect::new(card_x, card_y, card_w, card_h);
    let rad: CornerRadius = CornerRadius::all(g.get_radius_md());
    dl.push(DrawCommand::BoxShadow {
        rect: card_rect,
        radius: rad,
        shadow: g.get_shadow_md(),
    });
    dl.push(DrawCommand::FillRoundedRect {
        rect: card_rect,
        color: p.get_card().multiply_alpha(0.96),
        radius: rad,
        border: Border::new(1.0, p.get_border()),
    });
    let mut cy: f32 = card_y + 7.0;
    if !header.is_empty() {
        draw_label(
            dl,
            Rect::new(card_x + 9.0, cy, card_w - 18.0, 14.0),
            header.to_string(),
            10.5,
            FontWeight::SemiBold,
            TextAlign::Left,
            p.get_foreground(),
        );
        cy += header_h;
    }
    for hit in matched[..max_rows].iter() {
        dl.push(DrawCommand::Circle {
            center: Point::new(card_x + 13.0, cy + 6.5),
            radius: 3.5,
            color: hit.get_color(),
        });
        let name_w: f32 = ((card_w - 26.0) * 0.56).max(32.0);
        let val_w: f32 = (card_w - 26.0 - name_w).max(28.0);
        draw_label(
            dl,
            Rect::new(card_x + 21.0, cy, name_w, 14.0),
            hit.get_series_name().clone(),
            10.0,
            FontWeight::Normal,
            TextAlign::Left,
            p.get_muted_foreground(),
        );
        draw_label(
            dl,
            Rect::new(card_x + 21.0 + name_w, cy, val_w, 14.0),
            hit.get_detail().clone(),
            10.0,
            FontWeight::SemiBold,
            TextAlign::Right,
            p.get_foreground(),
        );
        cy += 16.0;
    }
}
