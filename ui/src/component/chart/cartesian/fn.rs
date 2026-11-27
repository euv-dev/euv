use super::*;

/// Renders Cartesian grid split areas, split lines, baselines, ticks, and labels for X and Y axes.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `&CartesianAxesBundle<'_>` - Configured axes and computed scales.
/// - `ThemeTokens` - Active UI theme tokens.
pub(crate) fn render_cartesian_axes(
    dl: &mut DisplayList,
    plot: Rect,
    bundle: &CartesianAxesBundle<'_>,
    tokens: ThemeTokens,
) {
    let p: ThemePalette = tokens.get_palette();
    let grid_col: Color = p.get_border().multiply_alpha(0.55);
    let area_col: Color = p.get_muted().multiply_alpha(0.35);
    let axis_col: Color = p.get_border();
    let label_col: Color = p.get_muted_foreground();
    let x0: f32 = plot.get_x();
    let y0: f32 = plot.get_y();
    let x1: f32 = x0 + plot.get_width();
    let y1: f32 = y0 + plot.get_height();
    for (idx, (y_axis, y_scale)) in bundle.y_axes.iter().zip(bundle.y_scales.iter()).enumerate() {
        let side: EuvChartPosition = y_axis.position_opt().unwrap_or(if idx == 0 {
            EuvChartPosition::Left
        } else {
            EuvChartPosition::Right
        });
        let ticks: &[f32] = y_scale.get_ticks().as_slice();
        let labels: &[String] = y_scale.get_labels().as_slice();
        if idx == 0 && y_axis.get_show_split_area() && ticks.len() >= 2 {
            for t_i in 0..(ticks.len() - 1) {
                if t_i % 2 == 1 {
                    let ya: f32 = y_scale.map_y(ticks[t_i], plot);
                    let yb: f32 = y_scale.map_y(ticks[t_i + 1], plot);
                    let top_y: f32 = ya.min(yb);
                    let h: f32 = (ya - yb).abs();
                    dl.push(DrawCommand::FillRoundedRect {
                        rect: Rect::new(x0, top_y, plot.get_width(), h),
                        color: area_col,
                        radius: CornerRadius::all(0.0),
                        border: Border::default(),
                    });
                }
            }
        }
        let axis_x: f32 = if side == EuvChartPosition::Right {
            x1
        } else {
            x0
        };
        if y_axis.get_show_axis_line() {
            dl.push(DrawCommand::Line {
                from: Point::new(axis_x, y0),
                to: Point::new(axis_x, y1),
                color: axis_col,
                width: 1.0,
            });
        }
        for (t_i, &tv) in ticks.iter().enumerate() {
            let py: f32 = y_scale.map_y(tv, plot);
            if idx == 0 && y_axis.get_show_split_line() {
                draw_dashed_line(
                    dl,
                    Point::new(x0, py),
                    Point::new(x1, py),
                    grid_col,
                    1.0,
                    3.0,
                    3.0,
                );
            }
            if y_axis.get_show_axis_tick() {
                let tick_dx: f32 = if side == EuvChartPosition::Right {
                    CARTESIAN_TICK_LEN
                } else {
                    -CARTESIAN_TICK_LEN
                };
                dl.push(DrawCommand::Line {
                    from: Point::new(axis_x, py),
                    to: Point::new(axis_x + tick_dx, py),
                    color: axis_col,
                    width: 1.0,
                });
            }
            if y_axis.get_show_axis_label()
                && let Some(lbl) = labels.get(t_i)
            {
                let (lx, align): (f32, TextAlign) = if side == EuvChartPosition::Right {
                    (axis_x + 6.0, TextAlign::Left)
                } else {
                    ((axis_x - 46.0).max(2.0), TextAlign::Right)
                };
                draw_label(
                    dl,
                    Rect::new(lx, py - 6.0, 40.0, 12.0),
                    lbl.clone(),
                    9.5,
                    FontWeight::Normal,
                    align,
                    label_col,
                );
            }
        }
        if !y_axis.get_name().is_empty() {
            let (nx, align): (f32, TextAlign) = if side == EuvChartPosition::Right {
                (x1 - 54.0, TextAlign::Right)
            } else {
                (x0, TextAlign::Left)
            };
            draw_label(
                dl,
                Rect::new(nx, (y0 - 15.0).max(2.0), 64.0, 12.0),
                y_axis.get_name().clone(),
                9.5,
                FontWeight::SemiBold,
                align,
                label_col,
            );
        }
    }
    for (idx, (x_axis, x_scale)) in bundle.x_axes.iter().zip(bundle.x_scales.iter()).enumerate() {
        let side: EuvChartPosition = x_axis.position_opt().unwrap_or(if idx == 0 {
            EuvChartPosition::Bottom
        } else {
            EuvChartPosition::Top
        });
        let axis_y: f32 = if side == EuvChartPosition::Top {
            y0
        } else {
            y1
        };
        if x_axis.get_show_axis_line() {
            dl.push(DrawCommand::Line {
                from: Point::new(x0, axis_y),
                to: Point::new(x1, axis_y),
                color: axis_col,
                width: 1.0,
            });
        }
        let ticks: &[f32] = x_scale.get_ticks().as_slice();
        let labels: &[String] = x_scale.get_labels().as_slice();
        let max_labels: usize =
            ((plot.get_width() / CARTESIAN_MIN_X_LABEL_STEP).floor() as usize).max(2);
        let label_stride: usize = if ticks.len() > max_labels {
            ticks.len().div_ceil(max_labels)
        } else {
            1
        };
        for (t_i, &tv) in ticks.iter().enumerate() {
            let px: f32 = x_scale.map_x(tv, plot);
            if idx == 0 && x_axis.get_show_split_line() {
                draw_dashed_line(
                    dl,
                    Point::new(px, y0),
                    Point::new(px, y1),
                    grid_col,
                    1.0,
                    3.0,
                    3.0,
                );
            }
            if x_axis.get_show_axis_tick() {
                let tick_dy: f32 = if side == EuvChartPosition::Top {
                    -CARTESIAN_TICK_LEN
                } else {
                    CARTESIAN_TICK_LEN
                };
                dl.push(DrawCommand::Line {
                    from: Point::new(px, axis_y),
                    to: Point::new(px, axis_y + tick_dy),
                    color: axis_col,
                    width: 1.0,
                });
            }
            if x_axis.get_show_axis_label()
                && (t_i % label_stride == 0 || t_i + 1 == ticks.len())
                && let Some(lbl) = labels.get(t_i)
            {
                let ly: f32 = if side == EuvChartPosition::Top {
                    axis_y - 16.0
                } else {
                    axis_y + 6.0
                };
                let slot_w: f32 =
                    (plot.get_width() / (ticks.len().max(1) as f32)).clamp(36.0, 84.0);
                draw_label(
                    dl,
                    Rect::new(px - slot_w * 0.5, ly, slot_w, 13.0),
                    lbl.clone(),
                    9.5,
                    FontWeight::Normal,
                    TextAlign::Center,
                    label_col,
                );
            }
        }
        if !x_axis.get_name().is_empty() {
            draw_label(
                dl,
                Rect::new(x1 - 56.0, axis_y + 16.0, 56.0, 12.0),
                x_axis.get_name().clone(),
                9.5,
                FontWeight::SemiBold,
                TextAlign::Right,
                label_col,
            );
        }
    }
    if bundle.show_border {
        dl.push(DrawCommand::FillRoundedRect {
            rect: plot,
            color: COLOR_TRANSPARENT,
            radius: CornerRadius::all(0.0),
            border: Border::new(1.0, axis_col),
        });
    }
}

/// Resolves nullable category-aligned numeric values for a line series from inline data or `Dataset`.
///
/// # Arguments
///
/// - `&EuvChartLineSeries` - Line series configuration.
/// - `Option<&EuvChartDataset>` - Optional shared dataset.
///
/// # Returns
///
/// - `Vec<Option<f32>>` - Resolved nullable values.
#[must_use]
pub(crate) fn resolve_line_values(
    line: &EuvChartLineSeries,
    dataset: Option<&EuvChartDataset>,
) -> Vec<Option<f32>> {
    if !line.get_values().is_empty() {
        return line.get_values().clone();
    }
    if let Some(ds) = dataset
        && let Some(enc) = line.encode_opt()
        && let Some(dim) = enc.y_opt().or_else(|| enc.value_opt())
    {
        return dataset_extract_numbers(ds, dim);
    }
    Vec::new()
}

/// Resolves nullable numeric values for a bar series from inline data or `Dataset`.
///
/// # Arguments
///
/// - `&EuvChartBarSeries` - Bar series configuration.
/// - `Option<&EuvChartDataset>` - Optional shared dataset.
///
/// # Returns
///
/// - `Vec<Option<f32>>` - Resolved nullable values.
#[must_use]
pub(crate) fn resolve_bar_values(
    bar: &EuvChartBarSeries,
    dataset: Option<&EuvChartDataset>,
) -> Vec<Option<f32>> {
    if !bar.get_values().is_empty() {
        return bar.get_values().clone();
    }
    if let Some(ds) = dataset
        && let Some(enc) = bar.encode_opt()
        && let Some(dim) = enc.y_opt().or_else(|| enc.value_opt())
    {
        return dataset_extract_numbers(ds, dim);
    }
    Vec::new()
}

/// Resolves scatter points from inline points or `Dataset`.
///
/// # Arguments
///
/// - `&EuvChartScatterSeries` - Scatter series configuration.
/// - `Option<&EuvChartDataset>` - Optional shared dataset.
///
/// # Returns
///
/// - `Vec<EuvChartScatterPoint>` - Resolved scatter points.
#[must_use]
pub(crate) fn resolve_scatter_points(
    scatter: &EuvChartScatterSeries,
    dataset: Option<&EuvChartDataset>,
) -> Vec<EuvChartScatterPoint> {
    if !scatter.get_points().is_empty() {
        return scatter.get_points().clone();
    }
    if let Some(ds) = dataset
        && let Some(enc) = scatter.encode_opt()
    {
        return dataset_extract_scatter(ds, enc, scatter.get_default_size());
    }
    Vec::new()
}

/// Renders one Cartesian `Line` series (straight, smooth Catmull-Rom, stepped, stacked, area fill, dashed, symbols, null gaps, and annotations).
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartLineSeries` - Line series descriptor.
/// - `&CartesianRenderCtx<'_>` - Active Cartesian projection context.
/// - `&mut HashMap<(String, usize), f32>` - Cumulative stack heights keyed by `(stack_key, visible_idx)`.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_line_series(
    dl: &mut DisplayList,
    meta: SeriesMeta<'_>,
    line: &EuvChartLineSeries,
    ctx: &CartesianRenderCtx<'_>,
    line_stacks: &mut HashMap<(String, usize), f32>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let halo: Color = ctx.frame.tokens.get_palette().get_card();
    let label_col: Color = ctx.frame.tokens.get_palette().get_foreground();
    let base_y: f32 = ctx.y_scale.baseline_y(ctx.plot_rect);
    let mut runs: Vec<Vec<LineRunPoint>> = Vec::new();
    let mut cur_run: Vec<LineRunPoint> = Vec::new();
    let mut sample_values: Vec<(f32, f32)> = Vec::new();
    if !line.get_xy_points().is_empty() {
        for (idx, &(xv, yv)) in line.get_xy_points().iter().enumerate() {
            if !xv.is_finite() || !yv.is_finite() {
                continue;
            }
            let px: f32 = ctx.x_scale.map_x(xv, ctx.plot_rect);
            let py: f32 = ctx.y_scale.map_y(yv, ctx.plot_rect);
            let pt: Point = Point::new(px, py);
            let bpt: Point = Point::new(px, base_y);
            let cat: String = format_chart_number(xv, "");
            sample_values.push((xv, yv));
            cur_run.push(LineRunPoint {
                top: pt,
                bot: bpt,
                idx,
                cat,
                val: yv,
            });
        }
        if !cur_run.is_empty() {
            runs.push(cur_run);
        }
    } else {
        let all_vals: Vec<Option<f32>> = resolve_line_values(line, ctx.frame.dataset);
        let s_idx: usize = ctx.zoom.get_start_index().min(all_vals.len());
        let e_idx: usize = ctx.zoom.get_end_index().clamp(s_idx, all_vals.len());
        let slice: &[Option<f32>] = if s_idx < e_idx {
            &all_vals[s_idx..e_idx]
        } else {
            all_vals.as_slice()
        };
        for (local_i, opt_v) in slice.iter().enumerate() {
            let Some(raw_v) = *opt_v else {
                if !line.get_connect_nulls() && !cur_run.is_empty() {
                    runs.push(take(&mut cur_run));
                }
                continue;
            };
            let (prev_stack, stacked_v): (f32, f32) = if let Some(st_key) = line.stack_opt() {
                let entry: &mut f32 = line_stacks
                    .entry((st_key.to_string(), local_i))
                    .or_insert(0.0);
                let prev: f32 = *entry;
                *entry = prev + raw_v;
                (prev, *entry)
            } else {
                (0.0, raw_v)
            };
            let px: f32 = ctx.x_scale.map_x(local_i as f32, ctx.plot_rect);
            let py: f32 = ctx.y_scale.map_y(stacked_v, ctx.plot_rect);
            let by: f32 = if line.stack_opt().is_some() {
                ctx.y_scale.map_y(prev_stack, ctx.plot_rect)
            } else {
                base_y
            };
            let cat: String = ctx
                .x_scale
                .get_categories()
                .get(local_i)
                .cloned()
                .unwrap_or_else(|| format!("{}", s_idx + local_i));
            sample_values.push((local_i as f32, stacked_v));
            cur_run.push(LineRunPoint {
                top: Point::new(px, py),
                bot: Point::new(px, by),
                idx: local_i,
                cat,
                val: raw_v,
            });
        }
        if !cur_run.is_empty() {
            runs.push(cur_run);
        }
    }
    dl.push(DrawCommand::PushClip(ctx.plot_rect));
    for run in runs.iter() {
        let raw_top: Vec<Point> = run.iter().map(|r: &LineRunPoint| r.top).collect();
        let raw_bot: Vec<Point> = run.iter().map(|r: &LineRunPoint| r.bot).collect();
        let (curve_top, curve_bot): (Vec<Point>, Vec<Point>) =
            if line.get_step() != EuvChartStepMode::None {
                build_step_polyline(&raw_top, &raw_bot, line.get_step())
            } else if line.get_smooth() && raw_top.len() >= 3 {
                (
                    sample_catmull_rom(&raw_top, DRAW_CURVE_SEGMENTS),
                    sample_catmull_rom(&raw_bot, DRAW_CURVE_SEGMENTS),
                )
            } else {
                (raw_top.clone(), raw_bot.clone())
            };
        if line.get_area_opacity() > 0.01 && curve_top.len() >= 2 {
            let alpha: f32 = line.get_area_opacity();
            if line.get_area_gradient() {
                let bands: usize = CARTESIAN_AREA_GRADIENT_BANDS;
                for b in 0..bands {
                    let f0: f32 = (b as f32) / (bands as f32);
                    let f1: f32 = ((b + 1) as f32) / (bands as f32);
                    let band_alpha: f32 = alpha * (1.0 - (f0 + f1) * 0.45).max(0.05);
                    let mut band_top: Vec<Point> = Vec::with_capacity(curve_top.len());
                    let mut band_bot: Vec<Point> = Vec::with_capacity(curve_top.len());
                    for k in 0..curve_top.len().min(curve_bot.len()) {
                        let t_pt: Point = curve_top[k];
                        let b_pt: Point = curve_bot[k];
                        band_top.push(Point::new(
                            t_pt.get_x(),
                            t_pt.get_y() + (b_pt.get_y() - t_pt.get_y()) * f0,
                        ));
                        band_bot.push(Point::new(
                            t_pt.get_x(),
                            t_pt.get_y() + (b_pt.get_y() - t_pt.get_y()) * f1,
                        ));
                    }
                    draw_ribbon_fill(
                        dl,
                        &band_top,
                        &band_bot,
                        meta.color.multiply_alpha(band_alpha),
                    );
                }
            } else {
                draw_ribbon_fill(dl, &curve_top, &curve_bot, meta.color.multiply_alpha(alpha));
            }
        }
        draw_polyline(
            dl,
            &curve_top,
            meta.color,
            line.get_line_width().max(1.0),
            line.get_dashed(),
        );
        for rp in run.iter() {
            let pt: Point = rp.top;
            if line.get_symbol() != EuvChartSymbol::None {
                draw_symbol(
                    dl,
                    line.get_symbol(),
                    pt,
                    line.get_symbol_size(),
                    meta.color,
                    halo,
                );
            }
            if line.get_show_labels() {
                draw_label(
                    dl,
                    Rect::new(pt.get_x() - 24.0, pt.get_y() - 16.0, 48.0, 12.0),
                    format_chart_number(rp.val, ""),
                    9.5,
                    FontWeight::Medium,
                    TextAlign::Center,
                    label_col,
                );
            }
            hits.push(EuvChartDatumHit::point_hit(
                (meta.index, meta.name.to_string(), rp.idx),
                (rp.cat.clone(), rp.val, format_chart_number(rp.val, "")),
                meta.color,
                pt,
                (line.get_symbol_size() + 8.0).max(11.0),
            ));
        }
    }
    render_cartesian_annotations(dl, line.get_annotations(), &sample_values, meta.color, ctx);
    dl.push(DrawCommand::PopClip);
}

/// Expands paired top/bottom polylines into stepped orthogonal segments for `Start`, `Middle`, or `End` step mode.
///
/// # Arguments
///
/// - `&[Point]` - Upper polyline vertices.
/// - `&[Point]` - Lower baseline vertices.
/// - `EuvChartStepMode` - Step interpolation rule.
///
/// # Returns
///
/// - `(Vec<Point>, Vec<Point>)` - Stepped `(top, bottom)` vertex lists.
#[must_use]
fn build_step_polyline(
    top: &[Point],
    bot: &[Point],
    mode: EuvChartStepMode,
) -> (Vec<Point>, Vec<Point>) {
    let n: usize = top.len().min(bot.len());
    if n < 2 {
        return (top.to_vec(), bot.to_vec());
    }
    let mut out_t: Vec<Point> = Vec::with_capacity(n * 3);
    let mut out_b: Vec<Point> = Vec::with_capacity(n * 3);
    out_t.push(top[0]);
    out_b.push(bot[0]);
    for i in 0..(n - 1) {
        let t0: Point = top[i];
        let t1: Point = top[i + 1];
        let b0: Point = bot[i];
        let b1: Point = bot[i + 1];
        match mode {
            EuvChartStepMode::Start => {
                out_t.push(Point::new(t0.get_x(), t1.get_y()));
                out_b.push(Point::new(b0.get_x(), b1.get_y()));
            }
            EuvChartStepMode::Middle => {
                let mx: f32 = (t0.get_x() + t1.get_x()) * 0.5;
                out_t.push(Point::new(mx, t0.get_y()));
                out_b.push(Point::new(mx, b0.get_y()));
                out_t.push(Point::new(mx, t1.get_y()));
                out_b.push(Point::new(mx, b1.get_y()));
            }
            EuvChartStepMode::End | EuvChartStepMode::None => {
                out_t.push(Point::new(t1.get_x(), t0.get_y()));
                out_b.push(Point::new(b1.get_x(), b0.get_y()));
            }
        }
        out_t.push(t1);
        out_b.push(b1);
    }
    (out_t, out_b)
}

/// Renders one Cartesian `Bar` series supporting vertical, horizontal, grouped, stacked, rounded caps, background tracks, and annotations.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartBarSeries` - Bar series descriptor.
/// - `&CartesianRenderCtx<'_>` - Active Cartesian projection context.
/// - `&mut BarSlotLayout` - Shared grouped/stacked bar slot tracker.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_bar_series(
    dl: &mut DisplayList,
    meta: SeriesMeta<'_>,
    bar: &EuvChartBarSeries,
    ctx: &CartesianRenderCtx<'_>,
    slots: &mut BarSlotLayout,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let horizontal: bool = ctx.y_scale.get_axis_type() == EuvChartAxisType::Category
        && ctx.x_scale.get_axis_type() != EuvChartAxisType::Category;
    let all_vals: Vec<Option<f32>> = resolve_bar_values(bar, ctx.frame.dataset);
    let s_idx: usize = ctx.zoom.get_start_index().min(all_vals.len());
    let e_idx: usize = ctx.zoom.get_end_index().clamp(s_idx, all_vals.len());
    let slice: &[Option<f32>] = if s_idx < e_idx {
        &all_vals[s_idx..e_idx]
    } else {
        all_vals.as_slice()
    };
    let slot_key: String = bar
        .stack_opt()
        .map(str::to_string)
        .unwrap_or_else(|| format!("s_{}", meta.index));
    let slot_pos: usize = slots
        .slot_keys
        .iter()
        .position(|k: &String| k == &slot_key)
        .unwrap_or(0);
    let total_slots: usize = slots.slot_keys.len().max(1);
    let cat_count: usize = if horizontal {
        ctx.y_scale.get_categories().len().max(slice.len()).max(1)
    } else {
        ctx.x_scale.get_categories().len().max(slice.len()).max(1)
    };
    let bg_track: Color = ctx
        .frame
        .tokens
        .get_palette()
        .get_muted()
        .multiply_alpha(0.55);
    let label_col: Color = ctx.frame.tokens.get_palette().get_foreground();
    let rad: f32 = bar.get_bar_radius();
    let mut sample_values: Vec<(f32, f32)> = Vec::new();
    dl.push(DrawCommand::PushClip(ctx.plot_rect));
    for (local_i, opt_v) in slice.iter().enumerate() {
        let Some(val) = *opt_v else {
            continue;
        };
        if !val.is_finite() {
            continue;
        }
        let fill: Color = ctx
            .frame
            .visual_eval
            .map(|vm: &EuvChartVisualEvaluator| vm.map_color(val))
            .unwrap_or(meta.color);
        let (base_v, top_v): (f32, f32) = if bar.stack_opt().is_some() {
            if val >= 0.0 {
                let entry: &mut f32 = slots
                    .pos_acc
                    .entry((slot_key.clone(), local_i))
                    .or_insert(0.0);
                let prev: f32 = *entry;
                *entry = prev + val;
                (prev, *entry)
            } else {
                let entry: &mut f32 = slots
                    .neg_acc
                    .entry((slot_key.clone(), local_i))
                    .or_insert(0.0);
                let prev: f32 = *entry;
                *entry = prev + val;
                (prev, *entry)
            }
        } else {
            (0.0, val)
        };
        if horizontal {
            let band_h: f32 = ctx.plot_rect.get_height() / (cat_count as f32);
            let group_h: f32 = band_h * bar.get_bar_width_ratio().clamp(0.15, 0.92);
            let single_h: f32 = (group_h / (total_slots as f32) - 2.0).max(3.0);
            let cy: f32 = ctx.y_scale.map_y(local_i as f32, ctx.plot_rect);
            let top_y: f32 =
                cy - group_h * 0.5 + (slot_pos as f32) * (group_h / (total_slots as f32)) + 1.0;
            if bar.get_show_background() {
                dl.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(
                        ctx.plot_rect.get_x(),
                        top_y,
                        ctx.plot_rect.get_width(),
                        single_h,
                    ),
                    color: bg_track,
                    radius: CornerRadius::all(rad.min(single_h * 0.5)),
                    border: Border::default(),
                });
            }
            let x_a: f32 = ctx.x_scale.map_x(base_v, ctx.plot_rect);
            let x_b: f32 = ctx.x_scale.map_x(top_v, ctx.plot_rect);
            let rx: f32 = x_a.min(x_b);
            let rw: f32 = (x_b - x_a).abs().max(2.0);
            let r_cap: f32 = rad.min(single_h * 0.5).min(rw * 0.5);
            let cr: CornerRadius = if val >= 0.0 {
                CornerRadius::new(0.0, r_cap, r_cap, 0.0)
            } else {
                CornerRadius::new(r_cap, 0.0, 0.0, r_cap)
            };
            let rect: Rect = Rect::new(rx, top_y, rw, single_h);
            dl.push(DrawCommand::FillRoundedRect {
                rect,
                color: fill,
                radius: cr,
                border: Border::default(),
            });
            if bar.get_show_labels() {
                draw_label(
                    dl,
                    Rect::new(rx + rw + 4.0, top_y, 44.0, single_h.max(12.0)),
                    format_chart_number(val, ""),
                    9.5,
                    FontWeight::Medium,
                    TextAlign::Left,
                    label_col,
                );
            }
            let cat: String = ctx
                .y_scale
                .get_categories()
                .get(local_i)
                .cloned()
                .unwrap_or_else(|| format!("{}", s_idx + local_i));
            sample_values.push((top_v, local_i as f32));
            hits.push(EuvChartDatumHit::rect_hit(
                (meta.index, meta.name.to_string(), local_i),
                (cat, val, format_chart_number(val, "")),
                fill,
                Point::new(x_b, top_y + single_h * 0.5),
                rect,
            ));
        } else {
            let band_w: f32 = ctx.plot_rect.get_width() / (cat_count as f32);
            let group_w: f32 = band_w * bar.get_bar_width_ratio().clamp(0.15, 0.92);
            let single_w: f32 = (group_w / (total_slots as f32) - 2.0).max(3.0);
            let cx: f32 = ctx.x_scale.map_x(local_i as f32, ctx.plot_rect);
            let left_x: f32 =
                cx - group_w * 0.5 + (slot_pos as f32) * (group_w / (total_slots as f32)) + 1.0;
            if bar.get_show_background() {
                dl.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(
                        left_x,
                        ctx.plot_rect.get_y(),
                        single_w,
                        ctx.plot_rect.get_height(),
                    ),
                    color: bg_track,
                    radius: CornerRadius::all(rad.min(single_w * 0.5)),
                    border: Border::default(),
                });
            }
            let y_a: f32 = ctx.y_scale.map_y(base_v, ctx.plot_rect);
            let y_b: f32 = ctx.y_scale.map_y(top_v, ctx.plot_rect);
            let ry: f32 = y_a.min(y_b);
            let rh: f32 = (y_b - y_a).abs().max(2.0);
            let r_cap: f32 = rad.min(single_w * 0.5).min(rh * 0.5);
            let cr: CornerRadius = if val >= 0.0 {
                CornerRadius::new(r_cap, r_cap, 0.0, 0.0)
            } else {
                CornerRadius::new(0.0, 0.0, r_cap, r_cap)
            };
            let rect: Rect = Rect::new(left_x, ry, single_w, rh);
            dl.push(DrawCommand::FillRoundedRect {
                rect,
                color: fill,
                radius: cr,
                border: Border::default(),
            });
            if bar.get_show_labels() {
                draw_label(
                    dl,
                    Rect::new(
                        left_x - 8.0,
                        (ry - 14.0).max(ctx.plot_rect.get_y()),
                        single_w + 16.0,
                        12.0,
                    ),
                    format_chart_number(val, ""),
                    9.5,
                    FontWeight::Medium,
                    TextAlign::Center,
                    label_col,
                );
            }
            let cat: String = ctx
                .x_scale
                .get_categories()
                .get(local_i)
                .cloned()
                .unwrap_or_else(|| format!("{}", s_idx + local_i));
            sample_values.push((local_i as f32, top_v));
            hits.push(EuvChartDatumHit::rect_hit(
                (meta.index, meta.name.to_string(), local_i),
                (cat, val, format_chart_number(val, "")),
                fill,
                Point::new(left_x + single_w * 0.5, y_b),
                rect,
            ));
        }
    }
    if !horizontal {
        render_cartesian_annotations(dl, bar.get_annotations(), &sample_values, meta.color, ctx);
    }
    dl.push(DrawCommand::PopClip);
}

/// Renders Cartesian `Scatter` or `EffectScatter` (with concentric translucent ripple rings).
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartScatterSeries` - Scatter descriptor.
/// - `bool` - When `true`, draws concentric ripple rings around each marker (`EffectScatter`).
/// - `&CartesianRenderCtx<'_>` - Active Cartesian projection context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_scatter_series(
    dl: &mut DisplayList,
    meta: SeriesMeta<'_>,
    scatter: &EuvChartScatterSeries,
    is_effect: bool,
    ctx: &CartesianRenderCtx<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let points: Vec<EuvChartScatterPoint> = resolve_scatter_points(scatter, ctx.frame.dataset);
    let halo: Color = ctx.frame.tokens.get_palette().get_card();
    let label_col: Color = ctx.frame.tokens.get_palette().get_foreground();
    let mut sample_values: Vec<(f32, f32)> = Vec::new();
    dl.push(DrawCommand::PushClip(ctx.plot_rect));
    for (idx, pt) in points.iter().enumerate() {
        let px: f32 = ctx.x_scale.map_x(pt.get_x(), ctx.plot_rect);
        let py: f32 = ctx.y_scale.map_y(pt.get_y(), ctx.plot_rect);
        let center: Point = Point::new(px, py);
        let r: f32 = if pt.get_size() > 0.5 {
            pt.get_size().clamp(2.5, 34.0)
        } else {
            scatter.get_default_size().clamp(2.5, 24.0)
        };
        let fill: Color = match (ctx.frame.visual_eval, pt.value_opt()) {
            (Some(vm), Some(mv)) => vm.map_color(mv),
            (Some(vm), None) => vm.map_color(pt.get_y()),
            _ => meta.color,
        };
        if is_effect {
            let rings: usize = scatter.get_ripple_rings().clamp(1, 5);
            let max_scale: f32 = scatter.get_ripple_scale().clamp(1.4, 4.0);
            for ring in (1..=rings).rev() {
                let frac: f32 = (ring as f32) / (rings as f32);
                let rr: f32 = r * (1.0 + (max_scale - 1.0) * frac);
                let alpha: f32 = (0.34 * (1.0 - frac * 0.72)).max(0.06);
                draw_arc_stroke(dl, center, rr, 0.0, TAU, fill.multiply_alpha(alpha), 1.6);
            }
        }
        draw_symbol(
            dl,
            scatter.get_symbol(),
            center,
            r,
            fill.multiply_alpha(0.86),
            halo,
        );
        if scatter.get_show_labels() && !pt.get_label().is_empty() {
            draw_label(
                dl,
                Rect::new(px - 34.0, py - r - 14.0, 68.0, 12.0),
                pt.get_label().clone(),
                9.5,
                FontWeight::Medium,
                TextAlign::Center,
                label_col,
            );
        }
        let cat: String = if pt.get_label().is_empty() {
            format!(
                "({}, {})",
                format_chart_number(pt.get_x(), ""),
                format_chart_number(pt.get_y(), "")
            )
        } else {
            pt.get_label().clone()
        };
        sample_values.push((pt.get_x(), pt.get_y()));
        hits.push(EuvChartDatumHit::point_hit(
            (meta.index, meta.name.to_string(), idx),
            (
                cat,
                pt.value_opt().unwrap_or(pt.get_y()),
                format!(
                    "x: {}  y: {}",
                    format_chart_number(pt.get_x(), ""),
                    format_chart_number(pt.get_y(), "")
                ),
            ),
            fill,
            center,
            (r + 6.0).max(10.0),
        ));
    }
    render_cartesian_annotations(
        dl,
        scatter.get_annotations(),
        &sample_values,
        meta.color,
        ctx,
    );
    dl.push(DrawCommand::PopClip);
}

/// Renders an OHLC financial `Candlestick` (K-Line) series composable with moving-average line series.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartCandlestickSeries` - Candlestick descriptor.
/// - `&CartesianRenderCtx<'_>` - Active Cartesian projection context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_candlestick_series(
    dl: &mut DisplayList,
    meta: SeriesMeta<'_>,
    candle_s: &EuvChartCandlestickSeries,
    ctx: &CartesianRenderCtx<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let all: &[EuvChartOhlcPoint] = candle_s.get_candles().as_slice();
    let s_idx: usize = ctx.zoom.get_start_index().min(all.len());
    let e_idx: usize = ctx.zoom.get_end_index().clamp(s_idx, all.len());
    let slice: &[EuvChartOhlcPoint] = if s_idx < e_idx {
        &all[s_idx..e_idx]
    } else {
        all
    };
    if slice.is_empty() {
        return;
    }
    let bull: Color = candle_s.resolved_bull_color();
    let bear: Color = candle_s.resolved_bear_color();
    let slot_w: f32 = ctx.plot_rect.get_width() / (slice.len().max(1) as f32);
    let body_w: f32 = (slot_w * 0.58).clamp(3.0, 24.0);
    let mut sample_values: Vec<(f32, f32)> = Vec::with_capacity(slice.len());
    dl.push(DrawCommand::PushClip(ctx.plot_rect));
    for (local_i, c) in slice.iter().enumerate() {
        let cx: f32 = ctx.x_scale.map_x(local_i as f32, ctx.plot_rect);
        let y_open: f32 = ctx.y_scale.map_y(c.get_open(), ctx.plot_rect);
        let y_close: f32 = ctx.y_scale.map_y(c.get_close(), ctx.plot_rect);
        let y_high: f32 = ctx.y_scale.map_y(c.get_high(), ctx.plot_rect);
        let y_low: f32 = ctx.y_scale.map_y(c.get_low(), ctx.plot_rect);
        let col: Color = if c.get_close() >= c.get_open() {
            bull
        } else {
            bear
        };
        dl.push(DrawCommand::Line {
            from: Point::new(cx, y_high),
            to: Point::new(cx, y_low),
            color: col,
            width: 1.4,
        });
        let top_y: f32 = y_open.min(y_close);
        let body_h: f32 = (y_open - y_close).abs().max(CARTESIAN_CANDLE_MIN_BODY);
        let rect: Rect = Rect::new(cx - body_w * 0.5, top_y, body_w, body_h);
        dl.push(DrawCommand::FillRoundedRect {
            rect,
            color: col,
            radius: CornerRadius::all(1.0),
            border: Border::default(),
        });
        let cat: String = ctx
            .x_scale
            .get_categories()
            .get(local_i)
            .cloned()
            .unwrap_or_else(|| format!("{}", s_idx + local_i));
        let detail: String = format!(
            "O:{} C:{} L:{} H:{}",
            format_chart_number(c.get_open(), ""),
            format_chart_number(c.get_close(), ""),
            format_chart_number(c.get_low(), ""),
            format_chart_number(c.get_high(), ""),
        );
        sample_values.push((local_i as f32, c.get_close()));
        hits.push(EuvChartDatumHit::rect_hit(
            (meta.index, meta.name.to_string(), local_i),
            (cat, c.get_close(), detail),
            col,
            Point::new(cx, y_close),
            Rect::new(
                cx - body_w * 0.6,
                y_high.min(y_low),
                body_w * 1.2,
                (y_low - y_high).abs().max(8.0),
            ),
        ));
    }
    render_cartesian_annotations(dl, candle_s.get_annotations(), &sample_values, bull, ctx);
    dl.push(DrawCommand::PopClip);
}

/// Renders a statistical five-number `Boxplot` series with whiskers, IQR boxes, median bars, and outlier dots.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartBoxplotSeries` - Boxplot descriptor.
/// - `&CartesianRenderCtx<'_>` - Active Cartesian projection context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_boxplot_series(
    dl: &mut DisplayList,
    meta: SeriesMeta<'_>,
    box_s: &EuvChartBoxplotSeries,
    ctx: &CartesianRenderCtx<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let items: &[EuvChartBoxplotItem] = box_s.get_items().as_slice();
    if items.is_empty() {
        return;
    }
    let slot_w: f32 = ctx.plot_rect.get_width() / (items.len().max(1) as f32);
    let box_w: f32 = (slot_w * 0.44).clamp(10.0, 42.0);
    let cap_w: f32 = box_w * 0.56;
    let fill_col: Color = meta.color.multiply_alpha(0.22);
    let mut sample_values: Vec<(f32, f32)> = Vec::with_capacity(items.len());
    dl.push(DrawCommand::PushClip(ctx.plot_rect));
    for (idx, item) in items.iter().enumerate() {
        let cx: f32 = ctx.x_scale.map_x(idx as f32, ctx.plot_rect);
        let y_min: f32 = ctx.y_scale.map_y(item.get_min(), ctx.plot_rect);
        let y_q1: f32 = ctx.y_scale.map_y(item.get_q1(), ctx.plot_rect);
        let y_med: f32 = ctx.y_scale.map_y(item.get_median(), ctx.plot_rect);
        let y_q3: f32 = ctx.y_scale.map_y(item.get_q3(), ctx.plot_rect);
        let y_max: f32 = ctx.y_scale.map_y(item.get_max(), ctx.plot_rect);
        dl.push(DrawCommand::Line {
            from: Point::new(cx, y_max),
            to: Point::new(cx, y_q3),
            color: meta.color,
            width: 1.5,
        });
        dl.push(DrawCommand::Line {
            from: Point::new(cx, y_q1),
            to: Point::new(cx, y_min),
            color: meta.color,
            width: 1.5,
        });
        dl.push(DrawCommand::Line {
            from: Point::new(cx - cap_w * 0.5, y_max),
            to: Point::new(cx + cap_w * 0.5, y_max),
            color: meta.color,
            width: 1.6,
        });
        dl.push(DrawCommand::Line {
            from: Point::new(cx - cap_w * 0.5, y_min),
            to: Point::new(cx + cap_w * 0.5, y_min),
            color: meta.color,
            width: 1.6,
        });
        let top_y: f32 = y_q3.min(y_q1);
        let box_h: f32 = (y_q1 - y_q3).abs().max(2.0);
        let rect: Rect = Rect::new(cx - box_w * 0.5, top_y, box_w, box_h);
        dl.push(DrawCommand::FillRoundedRect {
            rect,
            color: fill_col,
            radius: CornerRadius::all(2.0),
            border: Border::new(1.6, meta.color),
        });
        dl.push(DrawCommand::Line {
            from: Point::new(cx - box_w * 0.5, y_med),
            to: Point::new(cx + box_w * 0.5, y_med),
            color: meta.color,
            width: 2.4,
        });
        for &out_v in item.get_outliers().iter() {
            let oy: f32 = ctx.y_scale.map_y(out_v, ctx.plot_rect);
            dl.push(DrawCommand::Circle {
                center: Point::new(cx, oy),
                radius: 3.2,
                color: meta.color,
            });
        }
        let cat: String = ctx
            .x_scale
            .get_categories()
            .get(idx)
            .cloned()
            .unwrap_or_else(|| format!("{}", idx));
        let detail: String = format!(
            "Min:{} Q1:{} Med:{} Q3:{} Max:{}",
            format_chart_number(item.get_min(), ""),
            format_chart_number(item.get_q1(), ""),
            format_chart_number(item.get_median(), ""),
            format_chart_number(item.get_q3(), ""),
            format_chart_number(item.get_max(), ""),
        );
        sample_values.push((idx as f32, item.get_median()));
        hits.push(EuvChartDatumHit::rect_hit(
            (meta.index, meta.name.to_string(), idx),
            (cat, item.get_median(), detail),
            meta.color,
            Point::new(cx, y_med),
            rect,
        ));
    }
    render_cartesian_annotations(dl, box_s.get_annotations(), &sample_values, meta.color, ctx);
    dl.push(DrawCommand::PopClip);
}

/// Renders a Cartesian 2D matrix `Heatmap` colored through `VisualMap`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartHeatmapSeries` - Heatmap descriptor.
/// - `&CartesianRenderCtx<'_>` - Active Cartesian projection context.
/// - `&EuvChartVisualEvaluator` - Active color mapper.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_cartesian_heatmap(
    dl: &mut DisplayList,
    meta: SeriesMeta<'_>,
    heat: &EuvChartHeatmapSeries,
    ctx: &CartesianRenderCtx<'_>,
    visual_eval: &EuvChartVisualEvaluator,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let max_x: usize = heat
        .get_cells()
        .iter()
        .map(EuvChartHeatmapCell::get_x_index)
        .max()
        .map(|m: usize| m + 1)
        .unwrap_or(1)
        .max(ctx.x_scale.get_categories().len());
    let max_y: usize = heat
        .get_cells()
        .iter()
        .map(EuvChartHeatmapCell::get_y_index)
        .max()
        .map(|m: usize| m + 1)
        .unwrap_or(1)
        .max(ctx.y_scale.get_categories().len());
    let cell_w: f32 = ctx.plot_rect.get_width() / (max_x as f32);
    let cell_h: f32 = ctx.plot_rect.get_height() / (max_y as f32);
    let gap: f32 = heat.get_cell_gap().clamp(0.0, cell_w.min(cell_h) * 0.25);
    dl.push(DrawCommand::PushClip(ctx.plot_rect));
    for (idx, cell) in heat.get_cells().iter().enumerate() {
        let rx: f32 = ctx.plot_rect.get_x() + (cell.get_x_index() as f32) * cell_w + gap * 0.5;
        let ry: f32 = ctx.plot_rect.get_y()
            + ((max_y.saturating_sub(1).saturating_sub(cell.get_y_index())) as f32) * cell_h
            + gap * 0.5;
        let rw: f32 = (cell_w - gap).max(2.0);
        let rh: f32 = (cell_h - gap).max(2.0);
        let fill: Color = visual_eval.map_color(cell.get_value());
        let rect: Rect = Rect::new(rx, ry, rw, rh);
        dl.push(DrawCommand::FillRoundedRect {
            rect,
            color: fill,
            radius: CornerRadius::all(2.0),
            border: Border::default(),
        });
        if heat.get_show_labels() && rw >= 22.0 && rh >= 14.0 {
            let txt: String = if cell.get_label().is_empty() {
                format_chart_number(cell.get_value(), "")
            } else {
                cell.get_label().clone()
            };
            draw_label(
                dl,
                Rect::new(rx + 1.0, ry + (rh - 12.0) * 0.5, rw - 2.0, 12.0),
                txt,
                9.5,
                FontWeight::SemiBold,
                TextAlign::Center,
                contrast_ink_for(fill),
            );
        }
        let x_lbl: String = ctx
            .x_scale
            .get_categories()
            .get(cell.get_x_index())
            .cloned()
            .unwrap_or_else(|| format!("{}", cell.get_x_index()));
        let y_lbl: String = ctx
            .y_scale
            .get_categories()
            .get(cell.get_y_index())
            .cloned()
            .unwrap_or_else(|| format!("{}", cell.get_y_index()));
        hits.push(EuvChartDatumHit::rect_hit(
            (meta.index, meta.name.to_string(), idx),
            (
                format!("{} / {}", x_lbl, y_lbl),
                cell.get_value(),
                format_chart_number(cell.get_value(), ""),
            ),
            fill,
            Point::new(rx + rw * 0.5, ry + rh * 0.5),
            rect,
        ));
    }
    dl.push(DrawCommand::PopClip);
}

/// Renders a Calendar week-by-day matrix heatmap with month and weekday captions.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Available plot rectangle.
/// - `&EuvChartCalendar` - Calendar coordinate configuration.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartHeatmapSeries` - Calendar heatmap cell values.
/// - `&EuvChartVisualEvaluator` - Active visual-map color evaluator.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_calendar_heatmap(
    dl: &mut DisplayList,
    plot: Rect,
    calendar: &EuvChartCalendar,
    meta: SeriesMeta<'_>,
    heat: &EuvChartHeatmapSeries,
    visual_eval: &EuvChartVisualEvaluator,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let weeks: usize = calendar.get_weeks().max(4);
    let left_pad: f32 = 34.0;
    let top_pad: f32 = 22.0;
    let grid_w: f32 = (plot.get_width() - left_pad - 8.0).max(40.0);
    let grid_h: f32 = (plot.get_height() - top_pad - 8.0).max(40.0);
    let cell_w: f32 = (grid_w / (weeks as f32)).min(grid_h / 7.0 * 1.35);
    let cell_h: f32 = (grid_h / 7.0).min(cell_w);
    let gap: f32 = calendar.get_cell_gap().clamp(1.0, cell_h * 0.25);
    let origin_x: f32 = plot.get_x() + left_pad;
    let origin_y: f32 = plot.get_y() + top_pad;
    let label_col: Color = meta.color.multiply_alpha(0.75);
    let empty_col: Color = meta.color.multiply_alpha(0.12);
    draw_label(
        dl,
        Rect::new(plot.get_x(), plot.get_y() + 2.0, left_pad, 14.0),
        format!("{}", calendar.get_year()),
        10.0,
        FontWeight::Bold,
        TextAlign::Left,
        meta.color,
    );
    let months: Vec<String> = if calendar.get_month_labels().is_empty() {
        let count: usize = (weeks / 4).clamp(1, 12);
        SCALE_MONTH_NAMES[..count]
            .iter()
            .map(|m: &&str| (*m).to_string())
            .collect()
    } else {
        calendar.get_month_labels().clone()
    };
    let month_span: f32 = (weeks as f32) * cell_w / (months.len().max(1) as f32);
    for (m_i, m_lbl) in months.iter().enumerate() {
        draw_label(
            dl,
            Rect::new(
                origin_x + (m_i as f32) * month_span,
                plot.get_y() + 3.0,
                34.0,
                12.0,
            ),
            m_lbl.clone(),
            9.5,
            FontWeight::Medium,
            TextAlign::Left,
            label_col,
        );
    }
    for (d_i, day_name) in SCALE_WEEKDAY_NAMES.iter().enumerate() {
        if d_i % 2 == 0 {
            draw_label(
                dl,
                Rect::new(
                    plot.get_x(),
                    origin_y + (d_i as f32) * cell_h,
                    28.0,
                    cell_h.max(11.0),
                ),
                (*day_name).to_string(),
                9.0,
                FontWeight::Normal,
                TextAlign::Left,
                label_col,
            );
        }
    }
    let mut cell_map: HashMap<(usize, usize), &EuvChartHeatmapCell> = HashMap::new();
    for cell in heat.get_cells().iter() {
        let _: Option<&EuvChartHeatmapCell> =
            cell_map.insert((cell.get_x_index(), cell.get_y_index() % 7), cell);
    }
    for w in 0..weeks {
        for (d, day_name) in SCALE_WEEKDAY_NAMES.iter().enumerate() {
            let rx: f32 = origin_x + (w as f32) * cell_w + gap * 0.5;
            let ry: f32 = origin_y + (d as f32) * cell_h + gap * 0.5;
            let rw: f32 = (cell_w - gap).max(2.0);
            let rh: f32 = (cell_h - gap).max(2.0);
            let rect: Rect = Rect::new(rx, ry, rw, rh);
            if let Some(cell) = cell_map.get(&(w, d)) {
                let fill: Color = visual_eval.map_color(cell.get_value());
                dl.push(DrawCommand::FillRoundedRect {
                    rect,
                    color: fill,
                    radius: CornerRadius::all(2.5),
                    border: Border::default(),
                });
                hits.push(EuvChartDatumHit::rect_hit(
                    (meta.index, meta.name.to_string(), w * 7 + d),
                    (
                        format!("W{} {}", w + 1, day_name),
                        cell.get_value(),
                        format_chart_number(cell.get_value(), ""),
                    ),
                    fill,
                    Point::new(rx + rw * 0.5, ry + rh * 0.5),
                    rect,
                ));
            } else {
                dl.push(DrawCommand::FillRoundedRect {
                    rect,
                    color: empty_col,
                    radius: CornerRadius::all(2.5),
                    border: Border::default(),
                });
            }
        }
    }
}

/// Renders a stacked symbol `PictorialBar` series.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartPictorialBarSeries` - Pictorial bar descriptor.
/// - `&CartesianRenderCtx<'_>` - Active Cartesian projection context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_pictorial_bar_series(
    dl: &mut DisplayList,
    meta: SeriesMeta<'_>,
    pic: &EuvChartPictorialBarSeries,
    ctx: &CartesianRenderCtx<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let vals: &[f32] = pic.get_values().as_slice();
    if vals.is_empty() {
        return;
    }
    let base_y: f32 = ctx.y_scale.baseline_y(ctx.plot_rect);
    let halo: Color = ctx.frame.tokens.get_palette().get_card();
    let sym_r: f32 = (pic.get_symbol_size() * 0.5).clamp(3.0, 14.0);
    let step_y: f32 = sym_r * 2.25;
    dl.push(DrawCommand::PushClip(ctx.plot_rect));
    for (idx, &val) in vals.iter().enumerate() {
        let cx: f32 = ctx.x_scale.map_x(idx as f32, ctx.plot_rect);
        let top_y: f32 = ctx.y_scale.map_y(val, ctx.plot_rect);
        let span_h: f32 = (base_y - top_y).abs().max(2.0);
        let fill: Color = ctx
            .frame
            .visual_eval
            .map(|vm: &EuvChartVisualEvaluator| vm.map_color(val))
            .unwrap_or(meta.color);
        if pic.get_symbol_repeat() {
            let count: usize = ((span_h / step_y).round() as usize).clamp(1, 32);
            let dir: f32 = if top_y <= base_y { -1.0 } else { 1.0 };
            for s in 0..count {
                let cy: f32 = base_y + dir * (sym_r + (s as f32) * step_y);
                draw_symbol(dl, pic.get_symbol(), Point::new(cx, cy), sym_r, fill, halo);
            }
        } else {
            let half_w: f32 =
                (ctx.plot_rect.get_width() / (vals.len() as f32) * 0.28).clamp(6.0, 26.0);
            dl.push(DrawCommand::Triangle {
                p0: Point::new(cx, top_y),
                p1: Point::new(cx + half_w, base_y),
                p2: Point::new(cx - half_w, base_y),
                color: fill,
            });
        }
        let cat: String = ctx
            .x_scale
            .get_categories()
            .get(idx)
            .cloned()
            .unwrap_or_else(|| format!("{}", idx));
        hits.push(EuvChartDatumHit::rect_hit(
            (meta.index, meta.name.to_string(), idx),
            (cat, val, format_chart_number(val, "")),
            fill,
            Point::new(cx, top_y),
            Rect::new(cx - sym_r * 1.4, top_y.min(base_y), sym_r * 2.8, span_h),
        ));
    }
    dl.push(DrawCommand::PopClip);
}

/// Invokes a caller-provided `EuvChartCustomSeries` vector painter hook safely inside the clipped plot region.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `&EuvChartCustomSeries` - Custom painter series descriptor.
/// - `&CartesianRenderCtx<'_>` - Active Cartesian projection context.
pub(crate) fn render_custom_series(
    dl: &mut DisplayList,
    custom: &EuvChartCustomSeries,
    ctx: &CartesianRenderCtx<'_>,
) {
    let Some(painter) = custom.painter_opt() else {
        return;
    };
    let draw_ctx: EuvChartCustomDrawContext = EuvChartCustomDrawContext {
        plot_rect: ctx.plot_rect,
        x_min: ctx.x_scale.get_min(),
        x_max: ctx.x_scale.get_max(),
        y_min: ctx.y_scale.get_min(),
        y_max: ctx.y_scale.get_max(),
        palette: ctx.frame.palette.to_vec(),
        tokens: ctx.frame.tokens,
        hover_pos: ctx.frame.hover_pos,
    };
    dl.push(DrawCommand::PushClip(ctx.plot_rect));
    painter(ctx.plot_rect, &draw_ctx, dl);
    dl.push(DrawCommand::PopClip);
}

/// Renders directional quadratic curved `Lines` arcs over Cartesian axes.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartLinesSeries` - Lines series descriptor.
/// - `&CartesianRenderCtx<'_>` - Active Cartesian projection context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_cartesian_lines_series(
    dl: &mut DisplayList,
    meta: SeriesMeta<'_>,
    lines: &EuvChartLinesSeries,
    ctx: &CartesianRenderCtx<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let halo: Color = ctx.frame.tokens.get_palette().get_card();
    dl.push(DrawCommand::PushClip(ctx.plot_rect));
    for (idx, arc) in lines.get_arcs().iter().enumerate() {
        let p0: Point = Point::new(
            ctx.x_scale.map_x(arc.get_from_x(), ctx.plot_rect),
            ctx.y_scale.map_y(arc.get_from_y(), ctx.plot_rect),
        );
        let p2: Point = Point::new(
            ctx.x_scale.map_x(arc.get_to_x(), ctx.plot_rect),
            ctx.y_scale.map_y(arc.get_to_y(), ctx.plot_rect),
        );
        let dx: f32 = p2.get_x() - p0.get_x();
        let dy: f32 = p2.get_y() - p0.get_y();
        let ctrl: Point = Point::new(
            (p0.get_x() + p2.get_x()) * 0.5 - dy * lines.get_curveness(),
            (p0.get_y() + p2.get_y()) * 0.5 + dx * lines.get_curveness(),
        );
        let stroke: Color = arc.color_opt().unwrap_or(meta.color);
        let sampled: Vec<Point> = sample_quad_bezier(p0, ctrl, p2, DRAW_BEZIER_SEGMENTS);
        draw_polyline(dl, &sampled, stroke, lines.get_line_width().max(1.2), false);
        draw_symbol(dl, EuvChartSymbol::Circle, p0, 3.0, stroke, halo);
        if lines.get_show_arrows() && sampled.len() >= 2 {
            let approach: Point = sampled[sampled.len() - 2];
            draw_arrow_head(dl, approach, p2, 8.0, stroke);
        }
        let mid: Point = sampled[sampled.len() / 2];
        hits.push(EuvChartDatumHit::point_hit(
            (meta.index, meta.name.to_string(), idx),
            (
                format!("{} -> {}", arc.get_from_name(), arc.get_to_name()),
                arc.get_value(),
                format_chart_number(arc.get_value(), ""),
            ),
            stroke,
            mid,
            12.0,
        ));
    }
    dl.push(DrawCommand::PopClip);
}

/// Renders a centered stacked `ThemeRiver` streamgraph over sorted timestamps.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartThemeRiverSeries` - ThemeRiver descriptor.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_themeriver_series(
    dl: &mut DisplayList,
    plot: Rect,
    meta: SeriesMeta<'_>,
    river: &EuvChartThemeRiverSeries,
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let pts: &[EuvChartThemeRiverPoint] = river.get_points().as_slice();
    if pts.is_empty() {
        return;
    }
    let mut times: Vec<f32> = Vec::new();
    let mut themes: Vec<String> = Vec::new();
    for pt in pts.iter() {
        if !times
            .iter()
            .any(|&t: &f32| (t - pt.get_time()).abs() < 1e-4)
        {
            times.push(pt.get_time());
        }
        if !themes.iter().any(|th: &String| th == pt.get_theme()) {
            themes.push(pt.get_theme().clone());
        }
    }
    times.sort_by(|a: &f32, b: &f32| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    if times.len() < 2 || themes.is_empty() {
        return;
    }
    let mut grid: Vec<Vec<f32>> = vec![vec![0.0; times.len()]; themes.len()];
    for pt in pts.iter() {
        if let Some(ti) = times
            .iter()
            .position(|&t: &f32| (t - pt.get_time()).abs() < 1e-4)
            && let Some(thi) = themes.iter().position(|th: &String| th == pt.get_theme())
        {
            grid[thi][ti] += pt.get_value().max(0.0);
        }
    }
    let mut col_totals: Vec<f32> = vec![0.0; times.len()];
    let mut max_total: f32 = 1e-4;
    for t_i in 0..times.len() {
        let mut sum: f32 = 0.0;
        for row in grid.iter() {
            sum += row[t_i];
        }
        col_totals[t_i] = sum;
        max_total = max_total.max(sum);
    }
    let x_min: f32 = times[0];
    let x_max: f32 = times[times.len() - 1];
    let x_span: f32 = (x_max - x_min).max(1e-4);
    let cy: f32 = plot.get_y() + plot.get_height() * 0.48;
    let usable_h: f32 = plot.get_height() * 0.78;
    let mut running: Vec<f32> = col_totals.iter().map(|&tot: &f32| -tot * 0.5).collect();
    let pal_len: usize = frame.palette.len().max(1);
    dl.push(DrawCommand::PushClip(plot));
    for (th_i, theme_name) in themes.iter().enumerate() {
        let col: Color = frame.palette[th_i % pal_len];
        let mut bot_pts: Vec<Point> = Vec::with_capacity(times.len());
        let mut top_pts: Vec<Point> = Vec::with_capacity(times.len());
        for (t_i, &tv) in times.iter().enumerate() {
            let px: f32 = plot.get_x() + ((tv - x_min) / x_span) * plot.get_width();
            let lo: f32 = running[t_i];
            let hi: f32 = lo + grid[th_i][t_i];
            running[t_i] = hi;
            let y_lo: f32 = cy - (lo / max_total) * usable_h;
            let y_hi: f32 = cy - (hi / max_total) * usable_h;
            bot_pts.push(Point::new(px, y_lo));
            top_pts.push(Point::new(px, y_hi));
            hits.push(EuvChartDatumHit::point_hit(
                (meta.index, meta.name.to_string(), th_i * times.len() + t_i),
                (
                    format!("{} @ {}", theme_name, format_chart_number(tv, "")),
                    grid[th_i][t_i],
                    format_chart_number(grid[th_i][t_i], ""),
                ),
                col,
                Point::new(px, (y_lo + y_hi) * 0.5),
                12.0,
            ));
        }
        let (draw_top, draw_bot): (Vec<Point>, Vec<Point>) =
            if river.get_smooth() && times.len() >= 3 {
                (
                    sample_catmull_rom(&top_pts, DRAW_CURVE_SEGMENTS),
                    sample_catmull_rom(&bot_pts, DRAW_CURVE_SEGMENTS),
                )
            } else {
                (top_pts.clone(), bot_pts.clone())
            };
        draw_ribbon_fill(dl, &draw_top, &draw_bot, col.multiply_alpha(0.84));
        if river.get_show_labels() && !top_pts.is_empty() {
            let mid_idx: usize = times.len() / 2;
            let mid_y: f32 = (top_pts[mid_idx].get_y() + bot_pts[mid_idx].get_y()) * 0.5;
            let band_h: f32 = (bot_pts[mid_idx].get_y() - top_pts[mid_idx].get_y()).abs();
            if band_h >= 14.0 {
                draw_label(
                    dl,
                    Rect::new(top_pts[mid_idx].get_x() - 36.0, mid_y - 6.0, 72.0, 12.0),
                    theme_name.clone(),
                    9.5,
                    FontWeight::SemiBold,
                    TextAlign::Center,
                    contrast_ink_for(col),
                );
            }
        }
    }
    let axis_y: f32 = plot.get_y() + plot.get_height() - 16.0;
    let label_col: Color = frame.tokens.get_palette().get_muted_foreground();
    dl.push(DrawCommand::Line {
        from: Point::new(plot.get_x(), axis_y),
        to: Point::new(plot.get_x() + plot.get_width(), axis_y),
        color: frame.tokens.get_palette().get_border(),
        width: 1.0,
    });
    let stride: usize = (times.len() / 6).max(1);
    for (t_i, &tv) in times.iter().enumerate() {
        if t_i % stride == 0 || t_i + 1 == times.len() {
            let px: f32 = plot.get_x() + ((tv - x_min) / x_span) * plot.get_width();
            draw_label(
                dl,
                Rect::new(px - 24.0, axis_y + 2.0, 48.0, 12.0),
                format_chart_number(tv, ""),
                9.0,
                FontWeight::Normal,
                TextAlign::Center,
                label_col,
            );
        }
    }
    dl.push(DrawCommand::PopClip);
}

/// Renders a 1D `SingleAxis` track and any `Scatter` / `EffectScatter` series bound to `SingleAxis`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `&EuvChartSingleAxis` - Single-axis coordinate configuration.
/// - `&[EuvChartSeries]` - All chart series.
/// - `&[bool]` - Visibility flags aligned with `series`.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_single_axis_system(
    dl: &mut DisplayList,
    plot: Rect,
    single: &EuvChartSingleAxis,
    series: &[EuvChartSeries],
    visible: &[bool],
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let p: ThemePalette = frame.tokens.get_palette();
    let cy: f32 = plot.get_y() + plot.get_height() * single.get_center_y_ratio().clamp(0.2, 0.85);
    let mut x_min: f32 = f32::INFINITY;
    let mut x_max: f32 = f32::NEG_INFINITY;
    for (s_i, s) in series.iter().enumerate() {
        if !visible.get(s_i).copied().unwrap_or(true) {
            continue;
        }
        if let EuvChartSeriesData::Scatter(sc) | EuvChartSeriesData::EffectScatter(sc) =
            s.get_data()
            && sc.get_coordinate_system() == EuvChartCoordinateSystem::SingleAxis
        {
            for pt in sc.get_points().iter() {
                x_min = x_min.min(pt.get_x());
                x_max = x_max.max(pt.get_x());
            }
        }
    }
    if !x_min.is_finite() {
        x_min = 0.0;
        x_max = 10.0;
    }
    let x_scale: EuvChartAxisScale = build_axis_scale(single.get_axis(), x_min, x_max, false, None);
    dl.push(DrawCommand::Line {
        from: Point::new(plot.get_x(), cy),
        to: Point::new(plot.get_x() + plot.get_width(), cy),
        color: p.get_border(),
        width: 1.2,
    });
    for (i, &tv) in x_scale.get_ticks().iter().enumerate() {
        let px: f32 = x_scale.map_x(tv, plot);
        dl.push(DrawCommand::Line {
            from: Point::new(px, cy - 4.0),
            to: Point::new(px, cy + 4.0),
            color: p.get_border(),
            width: 1.0,
        });
        if let Some(lbl) = x_scale.get_labels().get(i) {
            draw_label(
                dl,
                Rect::new(px - 26.0, cy + 8.0, 52.0, 12.0),
                lbl.clone(),
                9.5,
                FontWeight::Normal,
                TextAlign::Center,
                p.get_muted_foreground(),
            );
        }
    }
    let halo: Color = p.get_card();
    let pal_len: usize = frame.palette.len().max(1);
    for (s_i, s) in series.iter().enumerate() {
        if !visible.get(s_i).copied().unwrap_or(true) {
            continue;
        }
        let col: Color = s.color_opt().unwrap_or(frame.palette[s_i % pal_len]);
        if let EuvChartSeriesData::Scatter(sc) | EuvChartSeriesData::EffectScatter(sc) =
            s.get_data()
            && sc.get_coordinate_system() == EuvChartCoordinateSystem::SingleAxis
        {
            let is_effect: bool = matches!(s.get_data(), EuvChartSeriesData::EffectScatter(_));
            for (idx, pt) in sc.get_points().iter().enumerate() {
                let px: f32 = x_scale.map_x(pt.get_x(), plot);
                let r: f32 = if pt.get_size() > 0.5 {
                    pt.get_size().clamp(3.0, 26.0)
                } else {
                    sc.get_default_size().clamp(3.0, 20.0)
                };
                let fill: Color = match (frame.visual_eval, pt.value_opt()) {
                    (Some(vm), Some(mv)) => vm.map_color(mv),
                    _ => col,
                };
                let center: Point = Point::new(px, cy);
                if is_effect {
                    draw_arc_stroke(
                        dl,
                        center,
                        r * 1.65,
                        0.0,
                        TAU,
                        fill.multiply_alpha(0.32),
                        1.5,
                    );
                }
                draw_symbol(
                    dl,
                    sc.get_symbol(),
                    center,
                    r,
                    fill.multiply_alpha(0.86),
                    halo,
                );
                hits.push(EuvChartDatumHit::point_hit(
                    (s_i, s.get_name().clone(), idx),
                    (
                        if pt.get_label().is_empty() {
                            format_chart_number(pt.get_x(), "")
                        } else {
                            pt.get_label().clone()
                        },
                        pt.value_opt().unwrap_or(pt.get_y()),
                        format_chart_number(pt.value_opt().unwrap_or(pt.get_y()), ""),
                    ),
                    fill,
                    center,
                    (r + 6.0).max(10.0),
                ));
            }
        }
    }
}

/// Renders Cartesian `MarkArea`, `MarkLine`, and `MarkPoint` overlays for one series.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `&EuvChartAnnotations` - Series mark annotations.
/// - `&[(f32, f32)]` - Observed `(x, y)` series coordinates for `Min`/`Max`/`Average` evaluation.
/// - `Color` - Default series color.
/// - `&CartesianRenderCtx<'_>` - Active Cartesian projection context.
pub(crate) fn render_cartesian_annotations(
    dl: &mut DisplayList,
    ann: &EuvChartAnnotations,
    samples: &[(f32, f32)],
    series_color: Color,
    ctx: &CartesianRenderCtx<'_>,
) {
    let plot: Rect = ctx.plot_rect;
    for area in ann.get_mark_areas().iter() {
        let (sx, ex, sy, ey): (Option<f32>, Option<f32>, Option<f32>, Option<f32>) =
            area.bounds_opt();
        let x0: f32 = sx
            .map(|v: f32| ctx.x_scale.map_x(v, plot))
            .unwrap_or(plot.get_x());
        let x1: f32 = ex
            .map(|v: f32| ctx.x_scale.map_x(v, plot))
            .unwrap_or(plot.get_x() + plot.get_width());
        let y0: f32 = sy
            .map(|v: f32| ctx.y_scale.map_y(v, plot))
            .unwrap_or(plot.get_y());
        let y1: f32 = ey
            .map(|v: f32| ctx.y_scale.map_y(v, plot))
            .unwrap_or(plot.get_y() + plot.get_height());
        let rx: f32 = x0.min(x1);
        let ry: f32 = y0.min(y1);
        let rw: f32 = (x1 - x0).abs().max(2.0);
        let rh: f32 = (y1 - y0).abs().max(2.0);
        let tint: Color = area.color_opt().unwrap_or(series_color);
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(rx, ry, rw, rh),
            color: tint.multiply_alpha(0.14),
            radius: CornerRadius::all(2.0),
            border: Border::default(),
        });
        if !area.get_name().is_empty() {
            draw_label(
                dl,
                Rect::new(rx + 4.0, ry + 3.0, (rw - 8.0).max(24.0), 12.0),
                area.get_name().clone(),
                9.5,
                FontWeight::SemiBold,
                TextAlign::Center,
                tint,
            );
        }
    }
    let stats: Option<SampleExtrema> = compute_sample_extrema(samples);
    for mline in ann.get_mark_lines().iter() {
        let col: Color = mline.color_opt().unwrap_or(series_color);
        match mline.get_kind() {
            EuvChartMarkLineKind::XAxis(xv) => {
                let px: f32 = ctx.x_scale.map_x(xv, plot);
                if mline.get_dashed() {
                    draw_dashed_line(
                        dl,
                        Point::new(px, plot.get_y()),
                        Point::new(px, plot.get_y() + plot.get_height()),
                        col,
                        1.4,
                        DRAW_DASH_LEN,
                        DRAW_GAP_LEN,
                    );
                } else {
                    dl.push(DrawCommand::Line {
                        from: Point::new(px, plot.get_y()),
                        to: Point::new(px, plot.get_y() + plot.get_height()),
                        color: col,
                        width: 1.4,
                    });
                }
                let txt: String = if mline.get_name().is_empty() {
                    format_chart_number(xv, "")
                } else {
                    mline.get_name().clone()
                };
                draw_pill_badge(dl, Point::new(px, plot.get_y() + 10.0), txt.as_str(), col);
            }
            other => {
                let yv_opt: Option<f32> = match other {
                    EuvChartMarkLineKind::Average => stats.map(|s: SampleExtrema| s.mean_y),
                    EuvChartMarkLineKind::Min => stats.map(|s: SampleExtrema| s.min_pt.1),
                    EuvChartMarkLineKind::Max => stats.map(|s: SampleExtrema| s.max_pt.1),
                    EuvChartMarkLineKind::YAxis(v) => Some(v),
                    EuvChartMarkLineKind::XAxis(_) => None,
                };
                if let Some(yv) = yv_opt {
                    let py: f32 = ctx.y_scale.map_y(yv, plot);
                    if mline.get_dashed() {
                        draw_dashed_line(
                            dl,
                            Point::new(plot.get_x(), py),
                            Point::new(plot.get_x() + plot.get_width(), py),
                            col,
                            1.4,
                            DRAW_DASH_LEN,
                            DRAW_GAP_LEN,
                        );
                    } else {
                        dl.push(DrawCommand::Line {
                            from: Point::new(plot.get_x(), py),
                            to: Point::new(plot.get_x() + plot.get_width(), py),
                            color: col,
                            width: 1.4,
                        });
                    }
                    let txt: String = if mline.get_name().is_empty() {
                        format!("{}: {}", CARTESIAN_MARK_AVG, format_chart_number(yv, ""))
                    } else {
                        format!("{}: {}", mline.get_name(), format_chart_number(yv, ""))
                    };
                    draw_pill_badge(
                        dl,
                        Point::new(plot.get_x() + plot.get_width() - 36.0, py - 9.0),
                        txt.as_str(),
                        col,
                    );
                }
            }
        }
    }
    let halo: Color = ctx.frame.tokens.get_palette().get_card();
    for mpt in ann.get_mark_points().iter() {
        let target: Option<(f32, f32)> = match mpt.get_kind() {
            EuvChartMarkPointKind::Min => stats.map(|s: SampleExtrema| s.min_pt),
            EuvChartMarkPointKind::Max => stats.map(|s: SampleExtrema| s.max_pt),
            EuvChartMarkPointKind::Average => {
                if let Some(s) = stats
                    && !samples.is_empty()
                {
                    let mid_x: f32 = samples[samples.len() / 2].0;
                    Some((mid_x, s.mean_y))
                } else {
                    None
                }
            }
            EuvChartMarkPointKind::Coord(cx, cy) => Some((cx, cy)),
        };
        if let Some((dx, dy)) = target {
            let col: Color = mpt.color_opt().unwrap_or(series_color);
            let anchor: Point =
                Point::new(ctx.x_scale.map_x(dx, plot), ctx.y_scale.map_y(dy, plot));
            draw_symbol(dl, EuvChartSymbol::Pin, anchor, 6.5, col, halo);
            let txt: String = if mpt.get_name().is_empty() {
                format_chart_number(dy, "")
            } else {
                format!("{} {}", mpt.get_name(), format_chart_number(dy, ""))
            };
            draw_pill_badge(
                dl,
                Point::new(
                    anchor.get_x(),
                    (anchor.get_y() - 22.0).max(plot.get_y() + 10.0),
                ),
                txt.as_str(),
                col,
            );
        }
    }
}

/// Computes `(min_xy, max_xy, mean_y)` across a slice of Cartesian sample points.
///
/// # Arguments
///
/// - `&[(f32, f32)]` - Sample points.
///
/// # Returns
///
/// - `Option<SampleExtrema>` - Extrema and arithmetic mean when non-empty.
#[must_use]
fn compute_sample_extrema(samples: &[(f32, f32)]) -> Option<SampleExtrema> {
    let first: (f32, f32) = *samples.first()?;
    let mut min_pt: (f32, f32) = first;
    let mut max_pt: (f32, f32) = first;
    let mut sum_y: f32 = 0.0;
    for &(x, y) in samples.iter() {
        if y < min_pt.1 {
            min_pt = (x, y);
        }
        if y > max_pt.1 {
            max_pt = (x, y);
        }
        sum_y += y;
    }
    Some(SampleExtrema {
        min_pt,
        max_pt,
        mean_y: sum_y / (samples.len() as f32),
    })
}
