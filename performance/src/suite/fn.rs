use super::*;

/// Benchmarks fine-grained `Signal` updates propagating through a diamond `Memo` graph.
///
/// # Arguments
///
/// - `usize` - Number of signal updates to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured latency and throughput telemetry.
#[must_use]
pub fn bench_reactive_signals_and_memos(iterations: usize) -> BenchMetric {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let source: Signal<i64> = Signal::new(0);
    let left: Memo<i64> = Memo::new(move || source.get() + 1);
    let right: Memo<i64> = Memo::new(move || source.get() * 2);
    let sum: Memo<i64> = Memo::new(move || left.get() + right.get());
    let start: Instant = Instant::now();
    let mut checksum: i64 = 0;
    for i in 1..=(count as i64) {
        let _: bool = source.set(i);
        checksum = checksum.wrapping_add(sum.get());
    }
    build_metric(
        BENCH_REACTIVE_NAME,
        count,
        start.elapsed(),
        format!("checksum={checksum}"),
    )
}

/// Benchmarks O(1) `Selector<usize>` active-index switching across 1,000 tracked rows.
///
/// # Arguments
///
/// - `usize` - Number of active-row switches to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured latency and throughput telemetry.
#[must_use]
pub fn bench_selector_o1(iterations: usize) -> BenchMetric {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let selected: Signal<usize> = Signal::new(0);
    let selector: Selector<usize> = Selector::new(move || selected.get());
    let start: Instant = Instant::now();
    let mut hits: usize = 0;
    for i in 0..count {
        let key: usize = i % 1_000;
        let _: bool = selected.set(key);
        if selector.is_selected(key) {
            hits += 1;
        }
    }
    build_metric(
        BENCH_SELECTOR_NAME,
        count,
        start.elapsed(),
        format!("hits={hits}"),
    )
}

/// Benchmarks retained layout engine clean-subtree caching across a 120-node UI tree.
///
/// # Arguments
///
/// - `usize` - Number of layout passes to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured latency and cache-hit telemetry.
#[must_use]
pub fn bench_layout_cache(iterations: usize) -> BenchMetric {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let scene: SceneHandle = SceneHandle::new_tree();
    let mut ctx: BuildContext = BuildContext::create(scene.clone(), PlatformKind::Headless);
    let container: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Px(800.0))
            .height(Length::Px(600.0))
            .gap(SPACE_XS),
    );
    for idx in 0..120usize {
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .height(Length::Px(24.0)),
        );
        let txt: NodeId = ctx.text(format!("{BENCH_ROW_LABEL} {idx}"), Style::empty());
        ctx.append(row, txt);
        ctx.append(container, row);
    }
    scene.with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            tree.replace_children(root, vec![container]);
        },
        (),
    );
    let mut engine: LayoutEngine = LayoutEngine::default();
    let viewport: Size = Size::new(800.0, 600.0);
    let _: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| engine.compute(tree, viewport),
        LayoutStats::default(),
    );
    let start: Instant = Instant::now();
    let mut cache_hits: usize = 0;
    for _ in 0..count {
        let stats: LayoutStats = scene.with_write(
            |tree: &mut SceneTree| engine.compute(tree, viewport),
            LayoutStats::default(),
        );
        cache_hits += stats.get_subtrees_skipped();
    }
    build_metric(
        BENCH_LAYOUT_CACHE_NAME,
        count,
        start.elapsed(),
        format!("cache_hits={cache_hits}"),
    )
}

/// Benchmarks full 800x600 software SDF tile rasterization frames.
///
/// # Arguments
///
/// - `usize` - Number of rendered frames to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured frame latency, FPS throughput, and pixel hash.
#[must_use]
pub fn bench_software_rasterizer(iterations: usize) -> BenchMetric {
    bench_render_pipeline(BENCH_SW_RASTER_NAME, RenderConfig::software(), iterations)
}

/// Benchmarks hardware GPU instanced draw-batch compilation frames.
///
/// # Arguments
///
/// - `usize` - Number of GPU frame submissions to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured frame latency, FPS throughput, and batch count.
#[must_use]
pub fn bench_hardware_gpu_batching(iterations: usize) -> BenchMetric {
    bench_render_pipeline(BENCH_HW_BATCH_NAME, RenderConfig::hardware(), iterations)
}

/// Runs the complete reproducible benchmark suite across reactivity, layout, and rendering.
///
/// # Returns
///
/// - `BenchReport` - Aggregated benchmark results.
#[must_use]
pub fn run_full_benchmark_suite() -> BenchReport {
    let metrics: Vec<BenchMetric> = vec![
        bench_reactive_signals_and_memos(10_000),
        bench_selector_o1(10_000),
        bench_layout_cache(1_000),
        bench_software_rasterizer(20),
        bench_hardware_gpu_batching(200),
    ];
    BenchReport::new(metrics)
}

/// Formats a `BenchReport` into an aligned terminal summary table.
///
/// # Arguments
///
/// - `&BenchReport` - Completed benchmark report.
///
/// # Returns
///
/// - `String` - Human-readable report table.
#[must_use]
pub fn format_bench_report(report: &BenchReport) -> String {
    let mut out: String = String::from(REPORT_HEADER);
    for m in report.get_metrics().iter() {
        out.push_str(&format!(
            "{:32} iters={:6} avg={:10.1} ns/op  throughput={:10.0} ops/s  {}\n",
            m.get_name(),
            m.get_iterations(),
            m.get_ns_per_op(),
            m.get_ops_per_sec(),
            m.get_detail()
        ));
    }
    out
}

/// Serializes a `BenchReport` to formatted JSON.
///
/// # Arguments
///
/// - `&BenchReport` - Completed benchmark report.
///
/// # Returns
///
/// - `String` - Pretty-printed JSON string.
#[must_use]
pub fn report_to_json(report: &BenchReport) -> String {
    to_string_pretty(report).unwrap_or_default()
}

/// Shared rendering benchmark harness supporting both Software and Hardware modes.
///
/// # Arguments
///
/// - `&str` - Benchmark name.
/// - `RenderConfig` - Renderer pipeline configuration.
/// - `usize` - Number of frames to step.
///
/// # Returns
///
/// - `BenchMetric` - Measured frame telemetry.
fn bench_render_pipeline(name: &str, render: RenderConfig, iterations: usize) -> BenchMetric {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let cfg: AppConfig = AppConfig::default().with_window(
        WindowConfig::default()
            .with_size(800, 600)
            .with_render(render),
    );
    let Ok(mut app) = App::headless(cfg) else {
        return build_metric(name, count, Duration::ZERO, String::new());
    };
    let _: NodeId = app.mount(|ctx: &mut BuildContext| {
        let badge: NodeId =
            EuvBadge::create(BENCH_BADGE.to_string(), EuvTone::Primary, tokens).mount(ctx);
        let prog: NodeId = EuvProgress::create(0.72, tokens).mount(ctx);
        let header: NodeId =
            EuvHeader::create(BENCH_TITLE.to_string(), BENCH_SUBTITLE.to_string(), tokens)
                .mount(ctx);
        let card: NodeId = EuvCard::create(BENCH_TITLE.to_string(), tokens)
            .subtitle(BENCH_SUBTITLE.to_string())
            .child(badge)
            .child(prog)
            .mount(ctx);
        let root: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_LG))
                .gap(SPACE_MD),
        );
        ctx.append(root, header);
        ctx.append(root, card);
        root
    });
    let start: Instant = Instant::now();
    let mut last_hash: u64 = 0;
    let mut last_batches: usize = 0;
    for _ in 0..count {
        if let Ok(stats) = app.step_frame() {
            last_hash = stats.get_frame_hash();
            last_batches = stats.get_draw_batches();
        }
    }
    build_metric(
        name,
        count,
        start.elapsed(),
        format!("batches={last_batches} hash={last_hash:#018x}"),
    )
}

/// Constructs a `BenchMetric` from elapsed wall time and iteration count.
///
/// # Arguments
///
/// - `&str` - Metric name.
/// - `usize` - Completed iterations.
/// - `Duration` - Total elapsed time.
/// - `String` - Extra metadata string.
///
/// # Returns
///
/// - `BenchMetric` - Computed timing metric.
fn build_metric(name: &str, iterations: usize, elapsed: Duration, detail: String) -> BenchMetric {
    let total_nanos: f64 = elapsed.as_nanos().max(1) as f64;
    let ns_per_op: f64 = total_nanos / (iterations.max(1) as f64);
    let ops_per_sec: f64 = 1_000_000_000.0 / ns_per_op.max(1.0);
    BenchMetric::new(
        name.to_string(),
        iterations,
        elapsed.as_micros() as u64,
        ns_per_op,
        ops_per_sec,
        detail,
    )
}
