use super::*;

/// Computes the nearest-rank percentile from a sorted non-empty slice of microsecond samples.
///
/// # Arguments
///
/// - `&[f64]` - Ascending-sorted sample slice in microseconds.
/// - `f64` - Target quantile in `[0.0, 1.0]`.
///
/// # Returns
///
/// - `f64` - Nearest-rank sample value in microseconds (`0.0` if empty).
#[must_use]
fn nearest_rank_percentile(sorted: &[f64], quantile: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let clamped: f64 = quantile.clamp(0.0, 1.0);
    let rank: usize = ((sorted.len() as f64) * clamped).ceil() as usize;
    let idx: usize = rank.saturating_sub(1).min(sorted.len() - 1);
    sorted[idx]
}

/// Aggregates raw per-iteration microsecond timings and damage telemetry into a `LatencyDistribution`.
///
/// # Arguments
///
/// - `&[f64]` - Raw per-iteration durations in microseconds.
/// - `&[f32]` - Per-frame `dirty_area_ratio` readings (`empty` when not applicable).
/// - `usize` - Count of measured frames that promoted to full redraw.
///
/// # Returns
///
/// - `LatencyDistribution` - Computed percentile, budget-miss, and damage statistics.
#[must_use]
fn summarize_latency(
    samples_us: &[f64],
    dirty_ratios: &[f32],
    full_redraw_count: usize,
) -> LatencyDistribution {
    let iterations: usize = samples_us.len();
    if iterations == 0 {
        return LatencyDistribution {
            iterations: 0,
            p50_us: 0.0,
            p95_us: 0.0,
            p99_us: 0.0,
            max_us: 0.0,
            mean_us: 0.0,
            over_6_94ms_count: 0,
            over_16_67ms_count: 0,
            mean_dirty_area_ratio: 0.0,
            full_redraw_count,
        };
    }
    let mut sorted: Vec<f64> = samples_us.to_vec();
    sorted.sort_by(f64::total_cmp);
    let p50_us: f64 = nearest_rank_percentile(&sorted, 0.50);
    let p95_us: f64 = nearest_rank_percentile(&sorted, 0.95);
    let p99_us: f64 = nearest_rank_percentile(&sorted, 0.99);
    let max_us: f64 = *sorted.last().unwrap_or(&0.0);
    let mut sum_us: f64 = 0.0;
    let mut over_144: usize = 0;
    let mut over_60: usize = 0;
    for &sample in samples_us.iter() {
        sum_us += sample;
        if sample > BUDGET_144HZ_US {
            over_144 += 1;
        }
        if sample > BUDGET_60HZ_US {
            over_60 += 1;
        }
    }
    let mean_us: f64 = sum_us / (iterations as f64);
    let mean_dirty_area_ratio: f64 = if dirty_ratios.is_empty() {
        0.0
    } else {
        let mut ratio_sum: f64 = 0.0;
        for &ratio in dirty_ratios.iter() {
            ratio_sum += ratio as f64;
        }
        ratio_sum / (dirty_ratios.len() as f64)
    };
    LatencyDistribution {
        iterations,
        p50_us,
        p95_us,
        p99_us,
        max_us,
        mean_us,
        over_6_94ms_count: over_144,
        over_16_67ms_count: over_60,
        mean_dirty_area_ratio,
        full_redraw_count,
    }
}

/// Computes percentage reduction from `baseline` to `candidate` (`> 0.0` when `candidate < baseline`).
///
/// # Arguments
///
/// - `f64` - Baseline measurement.
/// - `f64` - Candidate measurement.
///
/// # Returns
///
/// - `f64` - Percentage reduction in `(-inf, 100.0]`.
#[must_use]
fn reduction_pct(baseline: f64, candidate: f64) -> f64 {
    if baseline <= 0.0 {
        return 0.0;
    }
    ((baseline - candidate) / baseline) * 100.0
}

/// Builds a deterministic 1280x720 dense dashboard app for the partial-layout-damage A/B benchmark.
///
/// # Arguments
///
/// - `bool` - Whether `App::set_partial_layout_damage` is enabled.
///
/// # Returns
///
/// - `Option<(App, Vec<(NodeId, NodeId)>)>` - Initialized app and `(badge_box, value_text)` handles for each row.
#[must_use]
fn build_partial_row_ab_app(partial_enabled: bool) -> Option<(App, Vec<(NodeId, NodeId)>)> {
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(1280, 720).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).ok()?;
    app.set_partial_layout_damage(partial_enabled);
    app.set_telemetry_enabled(true);
    let mut row_handles: Vec<(NodeId, NodeId)> = Vec::with_capacity(24);
    let _: NodeId = app.mount(|| {
        let mut ctx: Context = current_context();
        let root_style: Style = Style::default()
            .column()
            .width(Length::Px(1280.0))
            .height(Length::Px(720.0))
            .padding(Edges::all(SPACE_MD))
            .gap(SPACE_SM)
            .background(p.get_background());
        let root: NodeId = ctx.view(root_style);
        let header_style: Style = Style::default()
            .row()
            .width(Length::Percent(100.0))
            .height(Length::Px(56.0))
            .padding(Edges::symmetric(SPACE_XS, SPACE_MD))
            .align(AlignItems::Center)
            .justify(JustifyContent::SpaceBetween)
            .background(p.get_card())
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_md());
        let header: NodeId = ctx.view(header_style);
        let title_node: NodeId = ctx.text(
            format!("{BENCH_TITLE} — Partial Layout A/B Matrix"),
            Style::default()
                .font_size(16.0)
                .font_weight(FontWeight::Bold)
                .color(p.get_foreground()),
        );
        let subtitle_node: NodeId = ctx.text(
            BENCH_SUBTITLE.to_string(),
            Style::default()
                .font_size(12.0)
                .color(p.get_muted_foreground()),
        );
        ctx.append(header, title_node);
        ctx.append(header, subtitle_node);
        ctx.append(root, header);
        let grid_style: Style = Style::default()
            .grid(2)
            .width(Length::Percent(100.0))
            .gap(SPACE_SM);
        let grid: NodeId = ctx.view(grid_style);
        for idx in 0..24usize {
            let row_card_style: Style = Style::default()
                .row()
                .width(Length::Px(612.0))
                .height(Length::Px(46.0))
                .padding(Edges::symmetric(SPACE_XS, SPACE_MD))
                .align(AlignItems::Center)
                .justify(JustifyContent::SpaceBetween)
                .overflow(Overflow::Hidden)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_sm());
            let card: NodeId = ctx.view(row_card_style);
            let label_node: NodeId = ctx.text(
                format!("Worker Lane #{idx:02} — Telemetry Stream"),
                Style::default().font_size(13.0).color(p.get_foreground()),
            );
            let right_cluster: NodeId = ctx.view(
                Style::default()
                    .row()
                    .width(Length::Px(220.0))
                    .height(Length::Px(30.0))
                    .align(AlignItems::Center)
                    .justify(JustifyContent::End)
                    .gap(SPACE_SM),
            );
            let badge_box: NodeId = ctx.view(
                Style::default()
                    .row()
                    .width(Length::Px(72.0))
                    .height(Length::Px(24.0))
                    .align(AlignItems::Center)
                    .justify(JustifyContent::Center)
                    .background(p.get_primary())
                    .radius(g.get_radius_sm()),
            );
            let badge_txt: NodeId = ctx.text(
                BENCH_BADGE.to_string(),
                Style::default()
                    .font_size(11.0)
                    .font_weight(FontWeight::Bold)
                    .color(p.get_primary_foreground()),
            );
            ctx.append(badge_box, badge_txt);
            let value_node: NodeId = ctx.text(
                format!("v=0000.{idx:02}"),
                Style::default().font_size(13.0).color(p.get_foreground()),
            );
            ctx.append(right_cluster, badge_box);
            ctx.append(right_cluster, value_node);
            ctx.append(card, label_node);
            ctx.append(card, right_cluster);
            ctx.append(grid, card);
            row_handles.push((badge_box, value_node));
        }
        ctx.append(root, grid);
        root
    });
    let _: FrameStats = app.step_frame().ok()?;
    Some((app, row_handles))
}

/// Applies one deterministic single-row leaf mutation (badge resize + value text update) and measures `step_frame`.
///
/// # Arguments
///
/// - `&mut App` - Target headless application instance.
/// - `(NodeId, NodeId)` - `(badge_box, value_text)` handles of the mutated leaf row.
/// - `usize` - Monotonic iteration index.
///
/// # Returns
///
/// - `(f64, u64, FrameTelemetry)` - `(elapsed_us, frame_hash, telemetry)` for this step.
fn step_single_row_mutation(
    app: &mut App,
    handles: (NodeId, NodeId),
    step: usize,
) -> (f64, u64, FrameTelemetry) {
    let (badge_box, value_node): (NodeId, NodeId) = handles;
    let next_badge_w: f32 = 64.0 + ((step % 5) as f32) * 8.0;
    let next_val: String = format!("v={:04}.{:02}", step + 1, (step * 7) % 100);
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            let _: bool = tree.mutate_style(badge_box, |st: &mut Style| {
                let _: &mut LayoutStyle = st.get_mut_layout().set_width(Length::Px(next_badge_w));
            });
            tree.set_text(value_node, next_val);
        },
        (),
    );
    let started: Instant = Instant::now();
    let hash: u64 = match app.step_frame() {
        Ok(stats) => stats.get_frame_hash(),
        Err(_) => 0,
    };
    let elapsed_us: f64 = (started.elapsed().as_nanos() as f64) / 1_000.0;
    let telemetry: FrameTelemetry = app.last_frame_telemetry();
    (elapsed_us, hash, telemetry)
}

/// Runs interleaved A/B single-row leaf updates comparing `partial_layout_damage = false` vs `true` with bit-exact RGBA parity verification.
///
/// # Arguments
///
/// - `usize` - Number of interleaved single-row update frames per arm.
///
/// # Returns
///
/// - `(StressMatrixEntry, PartialDamageAbSummary)` - Matrix entry (candidate partial-damage latency) and structured A/B summary.
#[must_use]
pub fn stress_partial_single_row_update_ab(
    iterations: usize,
) -> (StressMatrixEntry, PartialDamageAbSummary) {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let Some((mut app_full, rows_full)) = build_partial_row_ab_app(false) else {
        let empty: LatencyDistribution = summarize_latency(&[], &[], 0);
        let summary: PartialDamageAbSummary = PartialDamageAbSummary {
            name: STRESS_PARTIAL_ROW_AB_NAME.to_string(),
            baseline_full_redraw: empty,
            candidate_partial_damage: empty,
            p50_speedup_pct: 0.0,
            p95_speedup_pct: 0.0,
            p99_speedup_pct: 0.0,
            dirty_area_ratio_drop_pct: 0.0,
            bit_exact_parity: false,
            verified_steps: 0,
            detail: String::from(STRESS_INIT_FAILED),
        };
        let entry: StressMatrixEntry = StressMatrixEntry::new(
            STRESS_PARTIAL_ROW_AB_NAME.to_string(),
            empty,
            String::from(STRESS_INIT_FAILED),
        );
        return (entry, summary);
    };
    let Some((mut app_partial, rows_partial)) = build_partial_row_ab_app(true) else {
        let empty: LatencyDistribution = summarize_latency(&[], &[], 0);
        let summary: PartialDamageAbSummary = PartialDamageAbSummary {
            name: STRESS_PARTIAL_ROW_AB_NAME.to_string(),
            baseline_full_redraw: empty,
            candidate_partial_damage: empty,
            p50_speedup_pct: 0.0,
            p95_speedup_pct: 0.0,
            p99_speedup_pct: 0.0,
            dirty_area_ratio_drop_pct: 0.0,
            bit_exact_parity: false,
            verified_steps: 0,
            detail: String::from(STRESS_INIT_FAILED),
        };
        let entry: StressMatrixEntry = StressMatrixEntry::new(
            STRESS_PARTIAL_ROW_AB_NAME.to_string(),
            empty,
            String::from(STRESS_INIT_FAILED),
        );
        return (entry, summary);
    };
    let mut full_samples: Vec<f64> = Vec::with_capacity(count);
    let mut partial_samples: Vec<f64> = Vec::with_capacity(count);
    let mut full_ratios: Vec<f32> = Vec::with_capacity(count);
    let mut partial_ratios: Vec<f32> = Vec::with_capacity(count);
    let mut full_redraws_a: usize = 0;
    let mut full_redraws_b: usize = 0;
    let mut parity_ok: bool = true;
    for step in 0..count {
        let row_idx: usize = step % rows_full.len();
        let (full_res, partial_res): ((f64, u64, FrameTelemetry), (f64, u64, FrameTelemetry)) =
            if step % 2 == 0 {
                let res_a: (f64, u64, FrameTelemetry) =
                    step_single_row_mutation(&mut app_full, rows_full[row_idx], step);
                let res_b: (f64, u64, FrameTelemetry) =
                    step_single_row_mutation(&mut app_partial, rows_partial[row_idx], step);
                (res_a, res_b)
            } else {
                let res_b: (f64, u64, FrameTelemetry) =
                    step_single_row_mutation(&mut app_partial, rows_partial[row_idx], step);
                let res_a: (f64, u64, FrameTelemetry) =
                    step_single_row_mutation(&mut app_full, rows_full[row_idx], step);
                (res_a, res_b)
            };
        full_samples.push(full_res.0);
        partial_samples.push(partial_res.0);
        full_ratios.push(full_res.2.get_dirty_area_ratio());
        partial_ratios.push(partial_res.2.get_dirty_area_ratio());
        if full_res.2.get_full_redraw_reason() != FullRedrawReason::None {
            full_redraws_a += 1;
        }
        if partial_res.2.get_full_redraw_reason() != FullRedrawReason::None {
            full_redraws_b += 1;
        }
        if full_res.1 == 0 || full_res.1 != partial_res.1 {
            parity_ok = false;
        }
    }
    if let (Some(fb_a), Some(fb_b)) = (
        app_full.get_renderer().framebuffer(),
        app_partial.get_renderer().framebuffer(),
    ) && (fb_a.get_width() != fb_b.get_width()
        || fb_a.get_height() != fb_b.get_height()
        || fb_a.get_pixels() != fb_b.get_pixels())
    {
        parity_ok = false;
    }
    let baseline: LatencyDistribution =
        summarize_latency(&full_samples, &full_ratios, full_redraws_a);
    let candidate: LatencyDistribution =
        summarize_latency(&partial_samples, &partial_ratios, full_redraws_b);
    let p50_pct: f64 = reduction_pct(baseline.get_p50_us(), candidate.get_p50_us());
    let p95_pct: f64 = reduction_pct(baseline.get_p95_us(), candidate.get_p95_us());
    let p99_pct: f64 = reduction_pct(baseline.get_p99_us(), candidate.get_p99_us());
    let dirty_drop_pct: f64 = reduction_pct(
        baseline.get_mean_dirty_area_ratio(),
        candidate.get_mean_dirty_area_ratio(),
    );
    let detail: String = format!(
        "1280x720 parity={parity_ok} p50={:.1}us->{:.1}us ({:+.1}%) p95={:+.1}% p99={:+.1}% dirty={:.3}->{:.4} ({:+.1}% drop) full_redraws={}->{}",
        baseline.get_p50_us(),
        candidate.get_p50_us(),
        p50_pct,
        p95_pct,
        p99_pct,
        baseline.get_mean_dirty_area_ratio(),
        candidate.get_mean_dirty_area_ratio(),
        dirty_drop_pct,
        baseline.get_full_redraw_count(),
        candidate.get_full_redraw_count(),
    );
    let summary: PartialDamageAbSummary = PartialDamageAbSummary {
        name: STRESS_PARTIAL_ROW_AB_NAME.to_string(),
        baseline_full_redraw: baseline,
        candidate_partial_damage: candidate,
        p50_speedup_pct: p50_pct,
        p95_speedup_pct: p95_pct,
        p99_speedup_pct: p99_pct,
        dirty_area_ratio_drop_pct: dirty_drop_pct,
        bit_exact_parity: parity_ok,
        verified_steps: count,
        detail: detail.clone(),
    };
    let entry: StressMatrixEntry =
        StressMatrixEntry::new(STRESS_PARTIAL_ROW_AB_NAME.to_string(), candidate, detail);
    (entry, summary)
}

/// Stress-tests 100,000-row `EuvVirtualList` high-frequency scrolling and records tail latency distribution.
///
/// # Arguments
///
/// - `usize` - Number of high-frequency scroll frames to measure.
///
/// # Returns
///
/// - `StressMatrixEntry` - Per-frame tail latencies, over-budget counts, and damage ratio.
#[must_use]
pub fn stress_virtual_list_scroll_tail(iterations: usize) -> StressMatrixEntry {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(400, 720).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let Ok(mut app) = App::headless(cfg) else {
        return StressMatrixEntry::new(
            STRESS_VIRTUAL_LIST_TAIL_NAME.to_string(),
            summarize_latency(&[], &[], 0),
            String::from(STRESS_INIT_FAILED),
        );
    };
    app.set_telemetry_enabled(true);
    let scroll_y: Signal<f32> = Signal::new(0.0);
    let _: NodeId = app.mount(|| {
        let vlist: EuvVirtualList =
            EuvVirtualList::new(100_000, 36.0, scroll_y, tokens).viewport_height(680.0);
        vlist.mount_with(move |idx: usize| {
            let row_label: String = format!("{BENCH_ROW_LABEL} #{idx:06}");
            view! {
                row {
                    width: Length::Percent(100.0)
                    height: Length::Px(36.0)
                    padding: Edges::symmetric(SPACE_XS, SPACE_MD)
                    align: AlignItems::Center
                    justify: JustifyContent::SpaceBetween
                    border: (g.get_border_width(), p.get_border())
                    text {
                        font_size: 13.0
                        color: p.get_foreground()
                        { row_label }
                    }
                }
            }
        })
    });
    let _: Result<FrameStats, PlatformError> = app.step_frame();
    let mut samples_us: Vec<f64> = Vec::with_capacity(count);
    let mut dirty_ratios: Vec<f32> = Vec::with_capacity(count);
    let mut full_redraw_count: usize = 0;
    let mut last_shaded: usize = 0;
    for step in 1..=count {
        let delta: f32 = if step % 20 == 0 { 1_440.0 } else { 72.0 };
        let next_y: f32 = (scroll_y.get_untracked() + delta) % 3_500_000.0;
        let _: bool = scroll_y.set_if_changed(next_y);
        let started: Instant = Instant::now();
        if let Ok(stats) = app.step_frame() {
            last_shaded = stats.get_pixels_shaded();
        }
        samples_us.push((started.elapsed().as_nanos() as f64) / 1_000.0);
        let tel: FrameTelemetry = app.last_frame_telemetry();
        dirty_ratios.push(tel.get_dirty_area_ratio());
        if tel.get_full_redraw_reason() != FullRedrawReason::None {
            full_redraw_count += 1;
        }
    }
    let latency: LatencyDistribution =
        summarize_latency(&samples_us, &dirty_ratios, full_redraw_count);
    let detail: String = format!(
        "rows=100000 scroll_y={:.0}px shaded={last_shaded} dirty={:.3} full_redraws={full_redraw_count}",
        scroll_y.get_untracked(),
        latency.get_mean_dirty_area_ratio(),
    );
    StressMatrixEntry::new(STRESS_VIRTUAL_LIST_TAIL_NAME.to_string(), latency, detail)
}

/// Stress-tests continuous window resize reflow bursts across mobile, tablet, and desktop viewports while asserting zero stale bounds.
///
/// # Arguments
///
/// - `usize` - Number of continuous resize-reflow frames to measure.
///
/// # Returns
///
/// - `StressMatrixEntry` - Per-frame reflow latencies, stale-bounds count, and telemetry.
#[must_use]
pub fn stress_window_resize_reflow(iterations: usize) -> StressMatrixEntry {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(800, 600).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let Ok(mut app) = App::headless(cfg) else {
        return StressMatrixEntry::new(
            STRESS_WINDOW_RESIZE_NAME.to_string(),
            summarize_latency(&[], &[], 0),
            String::from(STRESS_INIT_FAILED),
        );
    };
    app.set_telemetry_enabled(true);
    let shell_node: NodeId = app.mount(|| {
        view! {
            column {
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_SM
                overflow: Overflow::Hidden
                background: p.get_background()
                EuvHeader {
                    title: BENCH_TITLE.to_string()
                    subtitle: BENCH_SUBTITLE.to_string()
                    tokens: tokens
                }
                grid {
                    columns: 3
                    width: Length::Percent(100.0)
                    gap: SPACE_SM
                    for idx in 0..12usize {
                        column {
                            width: Length::Percent(100.0)
                            padding: Edges::all(SPACE_SM)
                            gap: SPACE_XS
                            overflow: Overflow::Hidden
                            background: p.get_card()
                            border: (g.get_border_width(), p.get_border())
                            radius: g.get_radius_md()
                            text {
                                width: Length::Percent(100.0)
                                font_size: 13.0
                                color: p.get_foreground()
                                { format!("{BENCH_CELL_LABEL} #{idx:02} — Responsive Card") }
                            }
                            text {
                                width: Length::Percent(100.0)
                                font_size: 11.0
                                color: p.get_muted_foreground()
                                { BENCH_CARD_WRAP_COPY }
                            }
                        }
                    }
                }
            }
        }
    });
    let wid: WindowId = app.get_window_state().get_id();
    let _: Result<FrameStats, PlatformError> = app.step_frame();
    let presets: [(u32, u32); 6] = [
        (390, 780),
        (540, 720),
        (768, 900),
        (1024, 720),
        (1280, 720),
        (960, 640),
    ];
    let mut samples_us: Vec<f64> = Vec::with_capacity(count);
    let mut dirty_ratios: Vec<f32> = Vec::with_capacity(count);
    let mut full_redraw_count: usize = 0;
    let mut stale_bounds_count: usize = 0;
    for step in 0..count {
        let (base_w, base_h): (u32, u32) = presets[step % presets.len()];
        let jitter: u32 = ((step / presets.len()) as u32) * 12;
        let next_w: u32 = base_w + (jitter % 96);
        let next_h: u32 = base_h + (jitter % 64);
        let started: Instant = Instant::now();
        app.dispatch_window_event(WindowEvent::Resized(wid, next_w, next_h, 1.0));
        let _: Result<FrameStats, PlatformError> = app.step_frame();
        samples_us.push((started.elapsed().as_nanos() as f64) / 1_000.0);
        let tel: FrameTelemetry = app.last_frame_telemetry();
        dirty_ratios.push(tel.get_dirty_area_ratio());
        if tel.get_full_redraw_reason() != FullRedrawReason::None {
            full_redraw_count += 1;
        }
        let bounds: Rect = app.get_scene().with_read(
            |tree: &SceneTree| {
                tree.get_node(shell_node)
                    .map(|node: &SceneNode| node.get_layout_box().get_bounds())
                    .unwrap_or_default()
            },
            Rect::default(),
        );
        if (bounds.get_width() - (next_w as f32)).abs() > 1.0
            || (bounds.get_height() - (next_h as f32)).abs() > 1.0
        {
            stale_bounds_count += 1;
        }
    }
    let latency: LatencyDistribution =
        summarize_latency(&samples_us, &dirty_ratios, full_redraw_count);
    let detail: String = format!(
        "stale_bounds={stale_bounds_count} dirty={:.3} full_redraws={full_redraw_count}",
        latency.get_mean_dirty_area_ratio(),
    );
    StressMatrixEntry::new(STRESS_WINDOW_RESIZE_NAME.to_string(), latency, detail)
}

/// Stress-tests §06 `TerminalRenderer` full-sync, 1-cell incremental ANSI diff bytes, and 0-byte idle flush throughput.
///
/// # Arguments
///
/// - `usize` - Number of incremental 1-cell diff frames and idle flush checks to measure.
///
/// # Returns
///
/// - `StressMatrixEntry` - Incremental diff latency distribution plus full-sync and idle-flush telemetry.
#[must_use]
pub fn stress_terminal_tui_diff(iterations: usize) -> StressMatrixEntry {
    let count: usize = iterations.max(1);
    let mut tree: SceneTree = SceneTree::default();
    let root: NodeId = tree.get_root();
    let container: NodeId = tree.create_view(Style::default().column());
    let header: NodeId = tree.create_text(
        String::from(STRESS_TUI_HEADER),
        Style::default()
            .font_weight(FontWeight::Bold)
            .color(Color::rgb(96, 165, 250)),
    );
    tree.append_child(container, header);
    for idx in 0..24usize {
        let row_txt: NodeId = tree.create_text(
            format!("Service #{idx:02}  status=OK  latency=120us  queue=0"),
            Style::default().color(Color::rgb(226, 232, 240)),
        );
        tree.append_child(container, row_txt);
    }
    let counter_node: NodeId = tree.create_text(
        String::from(STRESS_TUI_TICK_ZERO),
        Style::default().color(Color::rgb(52, 211, 153)),
    );
    tree.append_child(container, counter_node);
    tree.append_child(root, container);
    let mut renderer: TerminalRenderer = TerminalRenderer::default();
    let full_started: Instant = Instant::now();
    let (_full_payload, full_stats): (String, TerminalDiffStats) =
        renderer.render_tree(&tree, 120, 40, TerminalRenderMode::InteractiveTui);
    let full_sync_us: f64 = (full_started.elapsed().as_nanos() as f64) / 1_000.0;
    let mut diff_samples_us: Vec<f64> = Vec::with_capacity(count);
    let mut diff_bytes_total: usize = 0;
    let mut max_changed_cells: usize = 0;
    for step in 0..count {
        let digit: usize = (step + 1) % 10;
        tree.set_text(counter_node, format!("tick={digit}"));
        let started: Instant = Instant::now();
        let (_diff_payload, stats): (String, TerminalDiffStats) =
            renderer.render_tree(&tree, 120, 40, TerminalRenderMode::InteractiveTui);
        diff_samples_us.push((started.elapsed().as_nanos() as f64) / 1_000.0);
        diff_bytes_total += stats.get_output_bytes();
        max_changed_cells = max_changed_cells.max(stats.get_changed_cells());
    }
    let mut idle_samples_us: Vec<f64> = Vec::with_capacity(count);
    let mut idle_bytes_total: usize = 0;
    let mut idle_frames_ok: usize = 0;
    for _ in 0..count {
        let started: Instant = Instant::now();
        let (idle_payload, idle_stats): (String, TerminalDiffStats) =
            renderer.render_tree(&tree, 120, 40, TerminalRenderMode::InteractiveTui);
        idle_samples_us.push((started.elapsed().as_nanos() as f64) / 1_000.0);
        idle_bytes_total += idle_payload.len();
        if idle_stats.get_is_idle() && idle_stats.get_output_bytes() == 0 {
            idle_frames_ok += 1;
        }
    }
    let diff_latency: LatencyDistribution = summarize_latency(&diff_samples_us, &[], 0);
    let idle_latency: LatencyDistribution = summarize_latency(&idle_samples_us, &[], 0);
    let avg_diff_bytes: f64 = (diff_bytes_total as f64) / (count as f64);
    let detail: String = format!(
        "120x40 full_sync={}B ({:.1}us) diff_avg={:.1}B max_cells={max_changed_cells} idle_p50={:.1}us idle_bytes={idle_bytes_total} idle_ok={idle_frames_ok}/{count}",
        full_stats.get_output_bytes(),
        full_sync_us,
        avg_diff_bytes,
        idle_latency.get_p50_us(),
    );
    StressMatrixEntry::new(
        STRESS_TERMINAL_TUI_DIFF_NAME.to_string(),
        diff_latency,
        detail,
    )
}

/// Runs all 4 §03/§08 stress-test verification matrix scenarios with default production iteration counts.
///
/// # Returns
///
/// - `StressMatrixReport` - Complete tail-latency and A/B parity verification report.
#[must_use]
pub fn run_stress_matrix() -> StressMatrixReport {
    run_filtered_stress_matrix(None)
}

/// Returns `true` when `name` matches optional substring `filter`.
///
/// # Arguments
///
/// - `&str` - Canonical scenario identifier.
/// - `Option<&str>` - Optional substring filter.
///
/// # Returns
///
/// - `bool` - Whether the scenario should execute.
#[must_use]
fn matches_stress_filter(name: &str, filter: Option<&str>) -> bool {
    filter
        .map(|substr: &str| name.contains(substr))
        .unwrap_or(true)
}

/// Runs §03/§08 stress-test verification matrix scenarios whose canonical names contain `filter` (or all 4 when `None`).
///
/// # Arguments
///
/// - `Option<&str>` - Optional substring filter matching scenario names.
///
/// # Returns
///
/// - `StressMatrixReport` - Filtered tail-latency and A/B parity verification report.
#[must_use]
pub fn run_filtered_stress_matrix(filter: Option<&str>) -> StressMatrixReport {
    let mut entries: Vec<StressMatrixEntry> = Vec::with_capacity(4);
    let mut partial_damage_ab: Option<PartialDamageAbSummary> = None;
    if matches_stress_filter(STRESS_PARTIAL_ROW_AB_NAME, filter) {
        let (entry, summary): (StressMatrixEntry, PartialDamageAbSummary) =
            stress_partial_single_row_update_ab(STRESS_PARTIAL_DEFAULT_ITERS);
        entries.push(entry);
        partial_damage_ab = Some(summary);
    }
    if matches_stress_filter(STRESS_VIRTUAL_LIST_TAIL_NAME, filter) {
        entries.push(stress_virtual_list_scroll_tail(
            STRESS_VLIST_TAIL_DEFAULT_ITERS,
        ));
    }
    if matches_stress_filter(STRESS_WINDOW_RESIZE_NAME, filter) {
        entries.push(stress_window_resize_reflow(STRESS_RESIZE_DEFAULT_ITERS));
    }
    if matches_stress_filter(STRESS_TERMINAL_TUI_DIFF_NAME, filter) {
        entries.push(stress_terminal_tui_diff(STRESS_TERMINAL_DEFAULT_ITERS));
    }
    StressMatrixReport::new(entries, partial_damage_ab)
}

/// Formats a `StressMatrixReport` into a human-readable terminal table showing tail percentiles and A/B parity.
///
/// # Arguments
///
/// - `&StressMatrixReport` - Completed stress-matrix report.
///
/// # Returns
///
/// - `String` - Aligned human-readable summary table.
#[must_use]
pub fn format_stress_matrix_report(report: &StressMatrixReport) -> String {
    let mut out: String = String::from(STRESS_REPORT_HEADER);
    for entry in report.get_entries().iter() {
        let lat: LatencyDistribution = entry.get_latency();
        out.push_str(&format!(
            "{:36} n={:4} p50={:8.1}us p95={:8.1}us p99={:8.1}us max={:8.1}us >6.94ms={:3} >16.67ms={:3}  {}\n",
            entry.get_name(),
            lat.get_iterations(),
            lat.get_p50_us(),
            lat.get_p95_us(),
            lat.get_p99_us(),
            lat.get_max_us(),
            lat.get_over_6_94ms_count(),
            lat.get_over_16_67ms_count(),
            entry.get_detail(),
        ));
    }
    if let Some(ab) = report.partial_damage_ab() {
        let base: LatencyDistribution = ab.get_baseline_full_redraw();
        let cand: LatencyDistribution = ab.get_candidate_partial_damage();
        out.push_str(&format!(
            "\nA/B Partial-Layout Damage Parity ({})\n  full_redraw:    p50={:8.1}us p95={:8.1}us p99={:8.1}us dirty={:.4} full_redraws={}\n  partial_damage: p50={:8.1}us p95={:8.1}us p99={:8.1}us dirty={:.4} full_redraws={}\n  delta:          p50={:+.1}% p95={:+.1}% p99={:+.1}% dirty_drop={:+.1}% bit_exact_parity={} steps={}\n",
            ab.get_name(),
            base.get_p50_us(),
            base.get_p95_us(),
            base.get_p99_us(),
            base.get_mean_dirty_area_ratio(),
            base.get_full_redraw_count(),
            cand.get_p50_us(),
            cand.get_p95_us(),
            cand.get_p99_us(),
            cand.get_mean_dirty_area_ratio(),
            cand.get_full_redraw_count(),
            ab.get_p50_speedup_pct(),
            ab.get_p95_speedup_pct(),
            ab.get_p99_speedup_pct(),
            ab.get_dirty_area_ratio_drop_pct(),
            ab.get_bit_exact_parity(),
            ab.get_verified_steps(),
        ));
    }
    out
}

/// Serializes a `StressMatrixReport` to pretty-printed JSON.
///
/// # Arguments
///
/// - `&StressMatrixReport` - Completed stress-matrix report.
///
/// # Returns
///
/// - `String` - Pretty-printed JSON payload.
#[must_use]
pub fn stress_matrix_to_json(report: &StressMatrixReport) -> String {
    to_string_pretty(report).unwrap_or_default()
}
