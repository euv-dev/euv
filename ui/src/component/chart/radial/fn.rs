use super::*;

/// Renders a `Pie`, Donut, or Nightingale Rose series with external leader lines, percentage labels, and clicked slice pop-out.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartPieSeries` - Pie series configuration.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_pie_series(
    dl: &mut DisplayList,
    plot: Rect,
    meta: SeriesMeta<'_>,
    pie: &EuvChartPieSeries,
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let slices: &[EuvChartPieSlice] = pie.get_slices().as_slice();
    if slices.is_empty() {
        return;
    }
    let total: f32 = slices
        .iter()
        .map(|s: &EuvChartPieSlice| s.get_value().max(0.0))
        .sum();
    if total <= 1e-6 {
        return;
    }
    let max_val: f32 = slices
        .iter()
        .map(|s: &EuvChartPieSlice| s.get_value().max(0.0))
        .fold(1e-6_f32, f32::max);
    let min_dim: f32 = plot.get_width().min(plot.get_height()).max(40.0);
    let outer_r: f32 = (min_dim * 0.5 * pie.get_outer_radius_ratio().clamp(0.25, 0.92)).max(16.0);
    let inner_r: f32 = outer_r * pie.get_inner_radius_ratio().clamp(0.0, 0.85);
    let cx: f32 = plot.get_x() + plot.get_width() * pie.get_center_x_ratio();
    let cy: f32 = plot.get_y() + plot.get_height() * pie.get_center_y_ratio();
    let pad_rad: f32 = pie.get_pad_angle_deg().to_radians() * 0.5;
    let mut cursor_rad: f32 = -pie.get_start_angle_deg().to_radians();
    let label_col: Color = frame.tokens.get_palette().get_foreground();
    let line_col: Color = frame.tokens.get_palette().get_muted_foreground();
    let count_f: f32 = slices.len() as f32;
    let pal_len: usize = frame.palette.len().max(1);
    for (idx, sl) in slices.iter().enumerate() {
        let v: f32 = sl.get_value().max(0.0);
        let pct: f32 = (v / total) * 100.0;
        let sweep: f32 = match pie.get_rose_type() {
            EuvChartRoseType::Area => TAU / count_f,
            EuvChartRoseType::None | EuvChartRoseType::Radius => (v / total) * TAU,
        };
        let a0: f32 = cursor_rad + pad_rad.min(sweep * 0.2);
        let a1: f32 = cursor_rad + sweep - pad_rad.min(sweep * 0.2);
        let mid: f32 = cursor_rad + sweep * 0.5;
        cursor_rad += sweep;
        let slice_outer_r: f32 = match pie.get_rose_type() {
            EuvChartRoseType::None => outer_r,
            EuvChartRoseType::Radius | EuvChartRoseType::Area => {
                let ratio: f32 = (v / max_val).sqrt().clamp(0.25, 1.0);
                inner_r + (outer_r - inner_r) * ratio
            }
        };
        let is_popped: bool = sl.get_selected() || frame.selected_pie_slice == Some(idx);
        let pop: f32 = if is_popped {
            pie.get_selected_offset()
        } else {
            0.0
        };
        let sc: Point = Point::new(cx + pop * mid.cos(), cy + pop * mid.sin());
        let col: Color = sl.color_opt().unwrap_or(frame.palette[idx % pal_len]);
        draw_annular_sector(dl, sc, inner_r, slice_outer_r, a0, a1, col);
        let mid_r: f32 = (inner_r + slice_outer_r) * 0.5;
        let anchor: Point = Point::new(
            sc.get_x() + mid_r * mid.cos(),
            sc.get_y() + mid_r * mid.sin(),
        );
        hits.push(EuvChartDatumHit::point_hit(
            (meta.index, meta.name.to_string(), idx),
            (
                sl.get_name().clone(),
                v,
                format!("{} ({:.1}%)", format_chart_number(v, ""), pct),
            ),
            col,
            anchor,
            ((slice_outer_r - inner_r) * 0.55).max(12.0),
        ));
        if pie.get_show_labels() && pct >= 2.5 {
            let rim: Point = Point::new(
                sc.get_x() + slice_outer_r * mid.cos(),
                sc.get_y() + slice_outer_r * mid.sin(),
            );
            let elbow: Point = Point::new(
                sc.get_x() + (slice_outer_r + RADIAL_PIE_LEADER_ELBOW) * mid.cos(),
                sc.get_y() + (slice_outer_r + RADIAL_PIE_LEADER_ELBOW) * mid.sin(),
            );
            let right_side: bool = mid.cos() >= 0.0;
            let tail_x: f32 = if right_side {
                (elbow.get_x() + RADIAL_PIE_LEADER_TAIL).min(plot.get_x() + plot.get_width() - 48.0)
            } else {
                (elbow.get_x() - RADIAL_PIE_LEADER_TAIL).max(plot.get_x() + 48.0)
            };
            let tail: Point = Point::new(tail_x, elbow.get_y());
            if pie.get_show_leader_lines() {
                dl.push(DrawCommand::Line {
                    from: rim,
                    to: elbow,
                    color: line_col.multiply_alpha(0.7),
                    width: 1.0,
                });
                dl.push(DrawCommand::Line {
                    from: elbow,
                    to: tail,
                    color: line_col.multiply_alpha(0.7),
                    width: 1.0,
                });
            }
            let lbl_text: String = format!("{} {:.0}%", sl.get_name(), pct);
            let (lx, align): (f32, TextAlign) = if right_side {
                (tail.get_x() + 3.0, TextAlign::Left)
            } else {
                ((tail.get_x() - 78.0).max(plot.get_x()), TextAlign::Right)
            };
            draw_label(
                dl,
                Rect::new(lx, tail.get_y() - 6.0, 74.0, 12.0),
                lbl_text,
                9.5,
                FontWeight::Medium,
                align,
                label_col,
            );
        }
    }
}

/// Renders a multivariate `Radar` web (polygon or circular rings) and all visible `Radar` series polygons.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `&EuvChartRadar` - Radar coordinate configuration.
/// - `&[EuvChartSeries]` - All chart series.
/// - `&[bool]` - Visibility flags aligned with `series`.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_radar_system(
    dl: &mut DisplayList,
    plot: Rect,
    radar: &EuvChartRadar,
    series: &[EuvChartSeries],
    visible: &[bool],
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let inds: &[EuvChartRadarIndicator] = radar.get_indicators().as_slice();
    if inds.len() < 3 {
        return;
    }
    let p: ThemePalette = frame.tokens.get_palette();
    let web_col: Color = p.get_border();
    let label_col: Color = p.get_muted_foreground();
    let halo: Color = p.get_card();
    let cx: f32 = plot.get_x() + plot.get_width() * 0.5;
    let cy: f32 = plot.get_y() + plot.get_height() * 0.52;
    let radius: f32 =
        (plot.get_width().min(plot.get_height()) * 0.5 * radar.get_radius_ratio().clamp(0.3, 0.85))
            .max(24.0);
    let rings: usize = radar.get_split_number().max(2);
    let n: usize = inds.len();
    for r_i in 1..=rings {
        let rr: f32 = radius * ((r_i as f32) / (rings as f32));
        if radar.get_shape() == EuvChartRadarShape::Circle {
            draw_arc_stroke(
                dl,
                Point::new(cx, cy),
                rr,
                0.0,
                TAU,
                web_col.multiply_alpha(0.65),
                1.0,
            );
        } else {
            let mut ring_pts: Vec<Point> = Vec::with_capacity(n);
            for k in 0..n {
                let ang: f32 = -FRAC_PI_2 + TAU * ((k as f32) / (n as f32));
                ring_pts.push(Point::new(cx + rr * ang.cos(), cy + rr * ang.sin()));
            }
            draw_polygon_stroke(dl, &ring_pts, web_col.multiply_alpha(0.65), 1.0);
        }
    }
    for (k, ind) in inds.iter().enumerate() {
        let ang: f32 = -FRAC_PI_2 + TAU * ((k as f32) / (n as f32));
        let tip: Point = Point::new(cx + radius * ang.cos(), cy + radius * ang.sin());
        dl.push(DrawCommand::Line {
            from: Point::new(cx, cy),
            to: tip,
            color: web_col.multiply_alpha(0.55),
            width: 1.0,
        });
        let lx: f32 = cx + (radius + 16.0) * ang.cos();
        let ly: f32 = cy + (radius + 14.0) * ang.sin();
        let align: TextAlign = if ang.cos() > 0.25 {
            TextAlign::Left
        } else if ang.cos() < -0.25 {
            TextAlign::Right
        } else {
            TextAlign::Center
        };
        let box_x: f32 = match align {
            TextAlign::Left => lx - 4.0,
            TextAlign::Right => lx - 66.0,
            TextAlign::Center => lx - 35.0,
        };
        draw_label(
            dl,
            Rect::new(box_x, ly - 6.0, 70.0, 12.0),
            ind.get_name().clone(),
            9.5,
            FontWeight::Medium,
            align,
            label_col,
        );
    }
    let pal_len: usize = frame.palette.len().max(1);
    let mut item_color_cursor: usize = 0;
    for (s_i, s) in series.iter().enumerate() {
        if !visible.get(s_i).copied().unwrap_or(true) {
            continue;
        }
        let EuvChartSeriesData::Radar(rs) = s.get_data() else {
            continue;
        };
        for item in rs.get_items().iter() {
            let col: Color = item
                .color_opt()
                .or_else(|| s.color_opt())
                .unwrap_or(frame.palette[item_color_cursor % pal_len]);
            item_color_cursor += 1;
            let mut poly: Vec<Point> = Vec::with_capacity(n);
            for (k, ind) in inds.iter().enumerate() {
                let raw_v: f32 = item.get_values().get(k).copied().unwrap_or(ind.get_min());
                let span: f32 = (ind.get_max() - ind.get_min()).max(1e-5);
                let frac: f32 = ((raw_v - ind.get_min()) / span).clamp(0.0, 1.0);
                let ang: f32 = -FRAC_PI_2 + TAU * ((k as f32) / (n as f32));
                let pt: Point = Point::new(
                    cx + radius * frac * ang.cos(),
                    cy + radius * frac * ang.sin(),
                );
                poly.push(pt);
                hits.push(EuvChartDatumHit::point_hit(
                    (s_i, item.get_name().clone(), k),
                    (
                        ind.get_name().clone(),
                        raw_v,
                        format_chart_number(raw_v, ""),
                    ),
                    col,
                    pt,
                    10.0,
                ));
            }
            if rs.get_area_opacity() > 0.01 {
                draw_polygon_fill(dl, &poly, col.multiply_alpha(rs.get_area_opacity()));
            }
            draw_polygon_stroke(dl, &poly, col, rs.get_line_width().max(1.2));
            if rs.get_show_vertices() {
                for &vpt in poly.iter() {
                    draw_symbol(dl, EuvChartSymbol::Circle, vpt, 3.2, col, halo);
                }
            }
        }
    }
}

/// Renders a `Polar` coordinate system and all visible Polar `Bar`, `Line`, `Scatter`, and `EffectScatter` series.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `&EuvChartPolar` - Polar coordinate configuration.
/// - `&[EuvChartSeries]` - All chart series.
/// - `&[bool]` - Visibility flags aligned with `series`.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_polar_system(
    dl: &mut DisplayList,
    plot: Rect,
    polar: &EuvChartPolar,
    series: &[EuvChartSeries],
    visible: &[bool],
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let p: ThemePalette = frame.tokens.get_palette();
    let cx: f32 = plot.get_x() + plot.get_width() * polar.get_center_x_ratio();
    let cy: f32 = plot.get_y() + plot.get_height() * polar.get_center_y_ratio();
    let center: Point = Point::new(cx, cy);
    let max_r: f32 =
        (plot.get_width().min(plot.get_height()) * 0.5 * polar.get_radius_ratio().clamp(0.3, 0.88))
            .max(24.0);
    let mut r_max_data: f32 = 1.0;
    for (s_i, s) in series.iter().enumerate() {
        if !visible.get(s_i).copied().unwrap_or(true) {
            continue;
        }
        match s.get_data() {
            EuvChartSeriesData::Bar(b)
                if b.get_coordinate_system() == EuvChartCoordinateSystem::Polar =>
            {
                for v in b.get_values().iter().flatten() {
                    r_max_data = r_max_data.max(*v);
                }
            }
            EuvChartSeriesData::Line(l)
                if l.get_coordinate_system() == EuvChartCoordinateSystem::Polar =>
            {
                for v in l.get_values().iter().flatten() {
                    r_max_data = r_max_data.max(*v);
                }
                for &(_, y) in l.get_xy_points().iter() {
                    r_max_data = r_max_data.max(y);
                }
            }
            EuvChartSeriesData::Scatter(sc) | EuvChartSeriesData::EffectScatter(sc)
                if sc.get_coordinate_system() == EuvChartCoordinateSystem::Polar =>
            {
                for pt in sc.get_points().iter() {
                    r_max_data = r_max_data.max(pt.get_y());
                }
            }
            _ => {}
        }
    }
    let rad_scale: EuvChartAxisScale =
        build_axis_scale(polar.get_radius_axis(), 0.0, r_max_data, true, None);
    let rings: usize = rad_scale.get_ticks().len().clamp(2, 6);
    for i in 1..rings {
        let rr: f32 = max_r * ((i as f32) / ((rings - 1) as f32));
        draw_arc_stroke(
            dl,
            center,
            rr,
            0.0,
            TAU,
            p.get_border().multiply_alpha(0.6),
            1.0,
        );
    }
    let cats: &[String] = polar.get_angle_axis().get_categories().as_slice();
    let spoke_cnt: usize = if cats.is_empty() { 8 } else { cats.len() };
    for k in 0..spoke_cnt {
        let ang: f32 = -FRAC_PI_2 + TAU * ((k as f32) / (spoke_cnt as f32));
        dl.push(DrawCommand::Line {
            from: center,
            to: Point::new(cx + max_r * ang.cos(), cy + max_r * ang.sin()),
            color: p.get_border().multiply_alpha(0.45),
            width: 1.0,
        });
        let lbl: String = cats
            .get(k)
            .cloned()
            .unwrap_or_else(|| format!("{}°", (k * 360) / spoke_cnt));
        draw_label(
            dl,
            Rect::new(
                cx + (max_r + 14.0) * ang.cos() - 22.0,
                cy + (max_r + 14.0) * ang.sin() - 6.0,
                44.0,
                12.0,
            ),
            lbl,
            9.0,
            FontWeight::Normal,
            TextAlign::Center,
            p.get_muted_foreground(),
        );
    }
    let halo: Color = p.get_card();
    let pal_len: usize = frame.palette.len().max(1);
    for (s_i, s) in series.iter().enumerate() {
        if !visible.get(s_i).copied().unwrap_or(true) {
            continue;
        }
        let col: Color = s.color_opt().unwrap_or(frame.palette[s_i % pal_len]);
        match s.get_data() {
            EuvChartSeriesData::Bar(b)
                if b.get_coordinate_system() == EuvChartCoordinateSystem::Polar =>
            {
                let n: usize = b.get_values().len().max(1);
                let step_ang: f32 = TAU / (n as f32);
                for (i, opt_v) in b.get_values().iter().enumerate() {
                    let Some(v) = *opt_v else {
                        continue;
                    };
                    let rr: f32 = max_r * rad_scale.normalize(v).clamp(0.0, 1.0);
                    let a0: f32 = -FRAC_PI_2 + (i as f32) * step_ang + 0.06;
                    let a1: f32 = -FRAC_PI_2 + ((i + 1) as f32) * step_ang - 0.06;
                    draw_annular_sector(
                        dl,
                        center,
                        6.0,
                        rr.max(8.0),
                        a0,
                        a1,
                        col.multiply_alpha(0.84),
                    );
                    let mid: f32 = (a0 + a1) * 0.5;
                    hits.push(EuvChartDatumHit::point_hit(
                        (s_i, s.get_name().clone(), i),
                        (
                            cats.get(i).cloned().unwrap_or_else(|| format!("{}", i)),
                            v,
                            format_chart_number(v, ""),
                        ),
                        col,
                        Point::new(cx + rr * 0.65 * mid.cos(), cy + rr * 0.65 * mid.sin()),
                        12.0,
                    ));
                }
            }
            EuvChartSeriesData::Line(l)
                if l.get_coordinate_system() == EuvChartCoordinateSystem::Polar =>
            {
                let n: usize = l.get_values().len().max(1);
                let mut pts: Vec<Point> = Vec::with_capacity(n + 1);
                for (i, opt_v) in l.get_values().iter().enumerate() {
                    let Some(v) = *opt_v else {
                        continue;
                    };
                    let ang: f32 = -FRAC_PI_2 + TAU * ((i as f32) / (n as f32));
                    let rr: f32 = max_r * rad_scale.normalize(v).clamp(0.0, 1.0);
                    let pt: Point = Point::new(cx + rr * ang.cos(), cy + rr * ang.sin());
                    pts.push(pt);
                    hits.push(EuvChartDatumHit::point_hit(
                        (s_i, s.get_name().clone(), i),
                        (
                            cats.get(i).cloned().unwrap_or_else(|| format!("{}", i)),
                            v,
                            format_chart_number(v, ""),
                        ),
                        col,
                        pt,
                        10.0,
                    ));
                }
                if let Some(&first) = pts.first()
                    && pts.len() >= 3
                {
                    pts.push(first);
                }
                draw_polyline(dl, &pts, col, l.get_line_width().max(1.5), l.get_dashed());
                for &pt in pts.iter() {
                    draw_symbol(dl, l.get_symbol(), pt, l.get_symbol_size(), col, halo);
                }
            }
            EuvChartSeriesData::Scatter(sc) | EuvChartSeriesData::EffectScatter(sc)
                if sc.get_coordinate_system() == EuvChartCoordinateSystem::Polar =>
            {
                for (i, pt) in sc.get_points().iter().enumerate() {
                    let ang: f32 = -FRAC_PI_2 + pt.get_x().to_radians();
                    let rr: f32 = max_r * rad_scale.normalize(pt.get_y()).clamp(0.0, 1.0);
                    let pos: Point = Point::new(cx + rr * ang.cos(), cy + rr * ang.sin());
                    let sz: f32 = if pt.get_size() > 0.5 {
                        pt.get_size()
                    } else {
                        sc.get_default_size()
                    };
                    draw_symbol(dl, sc.get_symbol(), pos, sz, col, halo);
                    hits.push(EuvChartDatumHit::point_hit(
                        (s_i, s.get_name().clone(), i),
                        (
                            format!("{}°", format_chart_number(pt.get_x(), "")),
                            pt.get_y(),
                            format_chart_number(pt.get_y(), ""),
                        ),
                        col,
                        pos,
                        (sz + 6.0).max(10.0),
                    ));
                }
            }
            _ => {}
        }
    }
}

/// Renders a circular instrument `Gauge` with multi-segment color ring, progress arc, major/minor ticks, pointer needle, and readout.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartGaugeSeries` - Gauge descriptor.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_gauge_series(
    dl: &mut DisplayList,
    plot: Rect,
    meta: SeriesMeta<'_>,
    gauge: &EuvChartGaugeSeries,
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let p: ThemePalette = frame.tokens.get_palette();
    let cx: f32 = plot.get_x() + plot.get_width() * 0.5;
    let cy: f32 = plot.get_y() + plot.get_height() * 0.56;
    let center: Point = Point::new(cx, cy);
    let outer_r: f32 = (plot.get_width().min(plot.get_height()) * 0.44).max(32.0);
    let ring_w: f32 = RADIAL_GAUGE_RING_WIDTH.min(outer_r * 0.22);
    let inner_r: f32 = (outer_r - ring_w).max(6.0);
    let start_rad: f32 = -gauge.get_start_angle_deg().to_radians();
    let end_rad: f32 = -gauge.get_end_angle_deg().to_radians();
    let total_sweep: f32 = end_rad - start_rad;
    let span: f32 = (gauge.get_max() - gauge.get_min()).max(1e-5);
    let frac: f32 = ((gauge.get_value() - gauge.get_min()) / span).clamp(0.0, 1.0);
    if gauge.get_segments().is_empty() {
        draw_annular_sector(
            dl,
            center,
            inner_r,
            outer_r,
            start_rad,
            end_rad,
            p.get_muted(),
        );
    } else {
        let mut prev_ratio: f32 = 0.0;
        for seg in gauge.get_segments().iter() {
            let next_ratio: f32 = seg.get_ratio().clamp(prev_ratio, 1.0);
            let a0: f32 = start_rad + total_sweep * prev_ratio;
            let a1: f32 = start_rad + total_sweep * next_ratio;
            draw_annular_sector(
                dl,
                center,
                inner_r,
                outer_r,
                a0,
                a1,
                seg.get_color().multiply_alpha(0.32),
            );
            prev_ratio = next_ratio;
        }
    }
    if gauge.get_show_progress() && frac > 0.005 {
        let val_rad: f32 = start_rad + total_sweep * frac;
        draw_annular_sector(dl, center, inner_r, outer_r, start_rad, val_rad, meta.color);
    }
    let splits: usize = gauge.get_split_number().max(2);
    let total_minor: usize = splits * RADIAL_GAUGE_MINOR_SUBDIVS;
    for t in 0..=total_minor {
        let tf: f32 = (t as f32) / (total_minor as f32);
        let ang: f32 = start_rad + total_sweep * tf;
        let is_major: bool = t % RADIAL_GAUGE_MINOR_SUBDIVS == 0;
        let tick_outer: f32 = inner_r - 2.0;
        let tick_inner: f32 = if is_major {
            inner_r - 10.0
        } else {
            inner_r - 6.0
        };
        dl.push(DrawCommand::Line {
            from: Point::new(cx + tick_inner * ang.cos(), cy + tick_inner * ang.sin()),
            to: Point::new(cx + tick_outer * ang.cos(), cy + tick_outer * ang.sin()),
            color: p
                .get_muted_foreground()
                .multiply_alpha(if is_major { 0.9 } else { 0.45 }),
            width: if is_major { 1.6 } else { 1.0 },
        });
        if is_major && outer_r >= 58.0 {
            let tv: f32 = gauge.get_min() + span * tf;
            let lr: f32 = inner_r - 22.0;
            draw_label(
                dl,
                Rect::new(
                    cx + lr * ang.cos() - 18.0,
                    cy + lr * ang.sin() - 6.0,
                    36.0,
                    12.0,
                ),
                format_chart_number(tv, ""),
                9.0,
                FontWeight::Normal,
                TextAlign::Center,
                p.get_muted_foreground(),
            );
        }
    }
    if gauge.get_show_pointer() {
        let val_rad: f32 = start_rad + total_sweep * frac;
        let tip: Point = Point::new(
            cx + (inner_r - 8.0).max(10.0) * val_rad.cos(),
            cy + (inner_r - 8.0).max(10.0) * val_rad.sin(),
        );
        let perp: f32 = val_rad + FRAC_PI_2;
        let base_l: Point = Point::new(cx + 4.0 * perp.cos(), cy + 4.0 * perp.sin());
        let base_r: Point = Point::new(cx - 4.0 * perp.cos(), cy - 4.0 * perp.sin());
        dl.push(DrawCommand::Triangle {
            p0: tip,
            p1: base_l,
            p2: base_r,
            color: p.get_foreground(),
        });
        dl.push(DrawCommand::Circle {
            center,
            radius: 6.0,
            color: p.get_foreground(),
        });
        dl.push(DrawCommand::Circle {
            center,
            radius: 2.6,
            color: p.get_card(),
        });
    }
    let readout: String = format!(
        "{}{}",
        format_chart_number(gauge.get_value(), ""),
        gauge.get_unit()
    );
    draw_label(
        dl,
        Rect::new(cx - 60.0, cy + outer_r * 0.24, 120.0, 22.0),
        readout.clone(),
        18.0,
        FontWeight::Bold,
        TextAlign::Center,
        p.get_foreground(),
    );
    if !gauge.get_title().is_empty() {
        draw_label(
            dl,
            Rect::new(cx - 72.0, cy + outer_r * 0.52, 144.0, 14.0),
            gauge.get_title().clone(),
            10.5,
            FontWeight::Medium,
            TextAlign::Center,
            p.get_muted_foreground(),
        );
    }
    hits.push(EuvChartDatumHit::point_hit(
        (meta.index, meta.name.to_string(), 0),
        (
            if gauge.get_title().is_empty() {
                meta.name.to_string()
            } else {
                gauge.get_title().clone()
            },
            gauge.get_value(),
            readout,
        ),
        meta.color,
        center,
        outer_r,
    ));
}

/// Renders a multi-ring concentric hierarchical `Sunburst` chart.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartSunburstSeries` - Sunburst descriptor.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_sunburst_series(
    dl: &mut DisplayList,
    plot: Rect,
    meta: SeriesMeta<'_>,
    sun: &EuvChartSunburstSeries,
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let roots: &[EuvChartHierarchyNode] = sun.get_nodes().as_slice();
    if roots.is_empty() {
        return;
    }
    let max_depth: usize = roots
        .iter()
        .map(EuvChartHierarchyNode::max_depth)
        .max()
        .unwrap_or(1)
        .max(1);
    let total: f32 = roots.iter().map(EuvChartHierarchyNode::total_value).sum();
    if total <= 1e-6 {
        return;
    }
    let cx: f32 = plot.get_x() + plot.get_width() * 0.5;
    let cy: f32 = plot.get_y() + plot.get_height() * 0.5;
    let max_r: f32 = (plot.get_width().min(plot.get_height())
        * 0.5
        * sun.get_outer_radius_ratio().clamp(0.4, 0.94))
    .max(24.0);
    let inner_r: f32 = max_r * sun.get_inner_radius_ratio().clamp(0.0, 0.45);
    let ring_step: f32 = (max_r - inner_r) / (max_depth as f32);
    let mut walk: SunburstWalk<'_> = SunburstWalk {
        center: Point::new(cx, cy),
        inner_r,
        ring_step,
        pad_rad: sun.get_pad_angle_deg().to_radians() * 0.5,
        show_labels: sun.get_show_labels(),
        meta,
        hit_idx: 0,
        hits,
    };
    let pal_len: usize = frame.palette.len().max(1);
    let mut cursor: f32 = -FRAC_PI_2;
    for (r_i, node) in roots.iter().enumerate() {
        let w: f32 = node.total_value();
        let sweep: f32 = (w / total) * TAU;
        let col: Color = node.color_opt().unwrap_or(frame.palette[r_i % pal_len]);
        render_sunburst_branch(dl, &mut walk, node, (0, cursor, cursor + sweep), col);
        cursor += sweep;
    }
}

/// Recursively renders one sunburst ring wedge and its proportional children in outer rings.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `&mut SunburstWalk<'_>` - Mutable sunburst traversal context.
/// - `&EuvChartHierarchyNode` - Current hierarchy node.
/// - `(usize, f32, f32)` - Tuple `(ring_depth, start_rad, end_rad)`.
/// - `Color` - Inherited branch base color.
fn render_sunburst_branch(
    dl: &mut DisplayList,
    walk: &mut SunburstWalk<'_>,
    node: &EuvChartHierarchyNode,
    arc: (usize, f32, f32),
    base_color: Color,
) {
    let (depth, start_rad, end_rad): (usize, f32, f32) = arc;
    let sweep: f32 = end_rad - start_rad;
    if sweep <= 1e-4 {
        return;
    }
    let r0: f32 = walk.inner_r + (depth as f32) * walk.ring_step + 1.0;
    let r1: f32 = (walk.inner_r + ((depth + 1) as f32) * walk.ring_step - 1.0).max(r0 + 2.0);
    let pad: f32 = walk.pad_rad.min(sweep * 0.18);
    let lighten: f32 = ((depth as f32) * 0.12).clamp(0.0, 0.36);
    let fill: Color = node
        .color_opt()
        .unwrap_or_else(|| base_color.lerp(Color::hex(0xFFFFFF), lighten));
    draw_annular_sector(
        dl,
        walk.center,
        r0,
        r1,
        start_rad + pad,
        end_rad - pad,
        fill,
    );
    let mid_ang: f32 = (start_rad + end_rad) * 0.5;
    let mid_r: f32 = (r0 + r1) * 0.5;
    let anchor: Point = Point::new(
        walk.center.get_x() + mid_r * mid_ang.cos(),
        walk.center.get_y() + mid_r * mid_ang.sin(),
    );
    let val: f32 = node.total_value();
    let idx: usize = walk.hit_idx;
    walk.hit_idx += 1;
    walk.hits.push(EuvChartDatumHit::point_hit(
        (walk.meta.index, walk.meta.name.to_string(), idx),
        (node.get_name().clone(), val, format_chart_number(val, "")),
        fill,
        anchor,
        ((r1 - r0) * 0.5).max(8.0),
    ));
    if walk.show_labels && sweep >= RADIAL_SUNBURST_LABEL_MIN_RAD && (r1 - r0) >= 14.0 {
        draw_label(
            dl,
            Rect::new(anchor.get_x() - 26.0, anchor.get_y() - 6.0, 52.0, 12.0),
            node.get_name().clone(),
            9.0,
            FontWeight::SemiBold,
            TextAlign::Center,
            contrast_ink_for(fill),
        );
    }
    let children: &[EuvChartHierarchyNode] = node.get_children().as_slice();
    if !children.is_empty() {
        let child_total: f32 = children
            .iter()
            .map(EuvChartHierarchyNode::total_value)
            .sum();
        if child_total > 1e-6 {
            let mut cur: f32 = start_rad;
            for ch in children.iter() {
                let cw: f32 = ch.total_value();
                let cs: f32 = (cw / child_total) * sweep;
                render_sunburst_branch(dl, walk, ch, (depth + 1, cur, cur + cs), fill);
                cur += cs;
            }
        }
    }
}

/// Renders an ascending or descending conversion `Funnel` trapezoid series.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartFunnelSeries` - Funnel descriptor.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_funnel_series(
    dl: &mut DisplayList,
    plot: Rect,
    meta: SeriesMeta<'_>,
    funnel: &EuvChartFunnelSeries,
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    if funnel.get_stages().is_empty() {
        return;
    }
    let mut ordered: Vec<(usize, &EuvChartFunnelStage)> =
        funnel.get_stages().iter().enumerate().collect();
    match funnel.get_sort() {
        EuvChartFunnelSort::Descending => {
            ordered.sort_by(
                |a: &(usize, &EuvChartFunnelStage), b: &(usize, &EuvChartFunnelStage)| {
                    b.1.get_value()
                        .partial_cmp(&a.1.get_value())
                        .unwrap_or(Ordering::Equal)
                },
            );
        }
        EuvChartFunnelSort::Ascending => {
            ordered.sort_by(
                |a: &(usize, &EuvChartFunnelStage), b: &(usize, &EuvChartFunnelStage)| {
                    a.1.get_value()
                        .partial_cmp(&b.1.get_value())
                        .unwrap_or(Ordering::Equal)
                },
            );
        }
        EuvChartFunnelSort::None => {}
    }
    let max_val: f32 = ordered
        .iter()
        .map(|(_, st): &(usize, &EuvChartFunnelStage)| st.get_value().max(0.0))
        .fold(1e-6_f32, f32::max);
    let n: usize = ordered.len();
    let gap: f32 = funnel.get_gap().clamp(0.0, 12.0);
    let total_gap: f32 = gap * ((n.saturating_sub(1)) as f32);
    let row_h: f32 = ((plot.get_height() - total_gap) / (n as f32)).max(12.0);
    let max_w: f32 = (plot.get_width() * 0.82).max(40.0);
    let cx: f32 = plot.get_x() + plot.get_width() * 0.5;
    let pal_len: usize = frame.palette.len().max(1);
    for (row_i, &(orig_i, stage)) in ordered.iter().enumerate() {
        let cur_frac: f32 = (stage.get_value().max(0.0) / max_val).clamp(0.08, 1.0);
        let next_frac: f32 = if row_i + 1 < n {
            (ordered[row_i + 1].1.get_value().max(0.0) / max_val).clamp(0.08, 1.0)
        } else if funnel.get_sort() == EuvChartFunnelSort::Ascending {
            cur_frac
        } else {
            (cur_frac * 0.55).max(0.06)
        };
        let top_w: f32 = max_w * cur_frac;
        let bot_w: f32 = max_w * next_frac;
        let y0: f32 = plot.get_y() + (row_i as f32) * (row_h + gap);
        let y1: f32 = y0 + row_h;
        let poly: [Point; 4] = [
            Point::new(cx - top_w * 0.5, y0),
            Point::new(cx + top_w * 0.5, y0),
            Point::new(cx + bot_w * 0.5, y1),
            Point::new(cx - bot_w * 0.5, y1),
        ];
        let col: Color = stage.color_opt().unwrap_or(frame.palette[orig_i % pal_len]);
        draw_polygon_fill(dl, &poly, col);
        let pct: f32 = (stage.get_value().max(0.0) / max_val) * 100.0;
        let detail: String = if funnel.get_show_percent() {
            format!(
                "{} ({:.0}%)",
                format_chart_number(stage.get_value(), ""),
                pct
            )
        } else {
            format_chart_number(stage.get_value(), "")
        };
        if funnel.get_show_labels() {
            let lbl: String = if funnel.get_show_percent() {
                format!("{} {:.0}%", stage.get_name(), pct)
            } else {
                stage.get_name().clone()
            };
            draw_label(
                dl,
                Rect::new(
                    cx - max_w * 0.48,
                    y0 + (row_h - 13.0) * 0.5,
                    max_w * 0.96,
                    13.0,
                ),
                lbl,
                10.0,
                FontWeight::SemiBold,
                TextAlign::Center,
                contrast_ink_for(col),
            );
        }
        let hit_w: f32 = top_w.max(bot_w);
        hits.push(EuvChartDatumHit::rect_hit(
            (meta.index, meta.name.to_string(), orig_i),
            (stage.get_name().clone(), stage.get_value(), detail),
            col,
            Point::new(cx, (y0 + y1) * 0.5),
            Rect::new(cx - hit_w * 0.5, y0, hit_w, row_h),
        ));
    }
}
