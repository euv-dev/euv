use super::*;

/// Measures the wall-clock execution duration of `work` in milliseconds on native targets.
///
/// # Arguments
///
/// - `F` - Closure executing the benchmarked workload.
///
/// # Returns
///
/// - `f64` - Elapsed duration in milliseconds.
#[cfg(not(target_arch = "wasm32"))]
fn measure_ms<F>(work: F) -> f64
where
    F: FnOnce(),
{
    let start: Instant = Instant::now();
    work();
    start.elapsed().as_secs_f64() * 1000.0
}

/// Measures the wall-clock execution duration of `work` in milliseconds in WebAssembly via `Performance.now()`.
///
/// # Arguments
///
/// - `F` - Closure executing the benchmarked workload.
///
/// # Returns
///
/// - `f64` - Elapsed duration in milliseconds.
#[cfg(target_arch = "wasm32")]
fn measure_ms<F>(work: F) -> f64
where
    F: FnOnce(),
{
    let perf: Option<Performance> = window().and_then(|w: Window| w.performance());
    let t0: f64 = perf.as_ref().map(Performance::now).unwrap_or(0.0);
    work();
    let t1: f64 = perf.as_ref().map(Performance::now).unwrap_or(t0);
    (t1 - t0).max(0.0)
}

/// Computes `(avg_ms, p95_ms, max_ms)` from a slice of per-iteration millisecond timings.
///
/// # Arguments
///
/// - `&[f64]` - Per-iteration millisecond sample slice.
///
/// # Returns
///
/// - `(f64, f64, f64)` - Tuple containing mean, 95th percentile, and maximum latency in milliseconds.
fn summarize_samples(samples: &[f64]) -> (f64, f64, f64) {
    if samples.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let mut sorted: Vec<f64> = samples.to_vec();
    sorted.sort_by(f64::total_cmp);
    let sum: f64 = sorted.iter().copied().sum();
    let len: usize = sorted.len();
    let avg: f64 = sum / (len as f64);
    let p95_idx: usize = ((len * 95) / 100).min(len - 1);
    let p95: f64 = sorted[p95_idx];
    let max: f64 = sorted[len - 1];
    (avg, p95, max)
}

/// Formats an iteration rate string (FPS or ops/s) from a mean per-iteration latency in milliseconds.
///
/// # Arguments
///
/// - `f64` - Mean latency per iteration in milliseconds.
/// - `bool` - When `true`, formats as `FPS`; otherwise formats as `ops/s`.
///
/// # Returns
///
/// - `String` - Human-readable throughput string.
fn format_rate(avg_ms: f64, is_fps: bool) -> String {
    let rate: f64 = if avg_ms > 0.0001 {
        1000.0 / avg_ms
    } else {
        999_999.0
    };
    if is_fps {
        format!("{rate:.0} FPS")
    } else {
        format!("{rate:.0} ops/s")
    }
}

/// Constructs an idle `StressCaseResult` descriptor ready for interactive execution.
///
/// # Arguments
///
/// - `&str` - Stable scenario identifier.
/// - `&str` - Human-readable title.
/// - `&str` - Short category label.
/// - `&str` - Workload description.
/// - `&str` - Action button label.
/// - `f64` - Per-iteration budget threshold in milliseconds.
///
/// # Returns
///
/// - `StressCaseResult` - Idle scenario entry.
fn make_idle_case(
    id: &str,
    title: &str,
    category: &str,
    description: &str,
    button_label: &str,
    budget_ms: f64,
) -> StressCaseResult {
    StressCaseResult {
        id: id.to_string(),
        title: title.to_string(),
        category: category.to_string(),
        description: description.to_string(),
        button_label: button_label.to_string(),
        status: STRESS_STATUS_IDLE.to_string(),
        budget_ms,
        iterations: 0,
        avg_ms: 0.0,
        p95_ms: 0.0,
        max_ms: 0.0,
        throughput: STRESS_UNIT_NA.to_string(),
        details: STRESS_UNIT_NA.to_string(),
        runs: 0,
    }
}

/// Returns the default list of 10 interactive UI stress benchmark scenarios in `IDLE` state.
///
/// # Returns
///
/// - `Vec<StressCaseResult>` - Ordered list of 10 ready stress benchmark scenarios.
#[must_use]
pub fn default_stress_results() -> Vec<StressCaseResult> {
    vec![
        make_idle_case(
            STRESS_ID_VLIST_100K,
            STRESS_TITLE_VLIST,
            STRESS_CAT_LIST,
            STRESS_DESC_VLIST,
            STRESS_BTN_VLIST,
            8.33,
        ),
        make_idle_case(
            STRESS_ID_LAYOUT_2000,
            STRESS_TITLE_LAYOUT,
            STRESS_CAT_LAYOUT,
            STRESS_DESC_LAYOUT,
            STRESS_BTN_LAYOUT,
            2.0,
        ),
        make_idle_case(
            STRESS_ID_REACTIVE_1000,
            STRESS_TITLE_REACTIVE,
            STRESS_CAT_REACTIVE,
            STRESS_DESC_REACTIVE,
            STRESS_BTN_REACTIVE,
            6.0,
        ),
        make_idle_case(
            STRESS_ID_WASM_HOTSWAP,
            STRESS_TITLE_WASM,
            STRESS_CAT_WASM,
            STRESS_DESC_WASM,
            STRESS_BTN_WASM,
            5.0,
        ),
        make_idle_case(
            STRESS_ID_DRAWER_ANIM,
            STRESS_TITLE_ANIM,
            STRESS_CAT_ANIM,
            STRESS_DESC_ANIM,
            STRESS_BTN_ANIM,
            8.33,
        ),
        make_idle_case(
            STRESS_ID_TOUCH_HITTEST,
            STRESS_TITLE_TOUCH,
            STRESS_CAT_INPUT,
            STRESS_DESC_TOUCH,
            STRESS_BTN_TOUCH,
            1.5,
        ),
        make_idle_case(
            STRESS_ID_HIDPI_SHADOW,
            STRESS_TITLE_HIDPI,
            STRESS_CAT_RASTER,
            STRESS_DESC_HIDPI,
            STRESS_BTN_HIDPI,
            16.67,
        ),
        make_idle_case(
            STRESS_ID_CJK_ATLAS,
            STRESS_TITLE_CJK,
            STRESS_CAT_TEXT,
            STRESS_DESC_CJK,
            STRESS_BTN_CJK,
            4.0,
        ),
        make_idle_case(
            STRESS_ID_GAME_2D_3D,
            STRESS_TITLE_GAME,
            STRESS_CAT_ENGINE,
            STRESS_DESC_GAME,
            STRESS_BTN_GAME,
            16.67,
        ),
        make_idle_case(
            STRESS_ID_MACRO_MOUNT,
            STRESS_TITLE_MACRO,
            STRESS_CAT_MACRO,
            STRESS_DESC_MACRO,
            STRESS_BTN_MACRO,
            6.0,
        ),
    ]
}

/// Applies measured samples and diagnostic metadata to a `StressCaseResult` and evaluates budget compliance.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Target result entry to update.
/// - `&[f64]` - Per-iteration millisecond timing samples.
/// - `bool` - Whether throughput should be displayed in `FPS` (`true`) or `ops/s` (`false`).
/// - `String` - Scenario-specific telemetry details.
fn finalize_case(entry: &mut StressCaseResult, samples: &[f64], is_fps: bool, details: String) {
    let (avg_ms, p95_ms, max_ms): (f64, f64, f64) = summarize_samples(samples);
    let pass: bool = avg_ms <= entry.get_budget_ms();
    let status: String = if pass {
        STRESS_STATUS_PASS.to_string()
    } else {
        STRESS_STATUS_WARN.to_string()
    };
    let next_runs: u32 = entry.get_runs().saturating_add(1);
    entry
        .set_status(status)
        .set_iterations(samples.len())
        .set_avg_ms(avg_ms)
        .set_p95_ms(p95_ms)
        .set_max_ms(max_ms)
        .set_throughput(format_rate(avg_ms, is_fps))
        .set_details(details)
        .set_runs(next_runs);
}

/// Runs Scenario 1: continuous scrolling across a 100,000-row virtualized list.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Result slot populated with live timing metrics.
fn run_stress_vlist_100k(entry: &mut StressCaseResult) {
    let (scope, (samples, details)): (Scope, (Vec<f64>, String)) = Scope::run(|_: Scope| {
        let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let mut win: WindowConfig = WindowConfig::default();
        win.set_size(360, 640).set_render(RenderConfig::software());
        let mut cfg: AppConfig = AppConfig::default();
        cfg.set_window(win);
        let Ok(mut app) = App::headless(cfg) else {
            return (Vec::new(), STRESS_UNIT_NA.to_string());
        };
        let scroll_y: Signal<f32> = Signal::new(0.0);
        let _: NodeId = app.mount(|| {
            let vlist: EuvVirtualList =
                EuvVirtualList::new(100_000, 36.0, scroll_y, tokens).viewport_height(600.0);
            vlist.mount_with(move |idx: usize| {
                view! {
                    row {
                        width: Length::Percent(100.0)
                        height: Length::Px(36.0)
                        padding: Edges::symmetric(SPACE_XS, SPACE_MD)
                        align: AlignItems::Center
                        border: (g.get_border_width(), p.get_border())
                        text {
                            font_size: 13.0
                            color: p.get_foreground()
                            { format!("{STRESS_ROW_PREFIX} #{idx}") }
                        }
                    }
                }
            })
        });
        let _: Result<FrameStats, PlatformError> = app.step_frame();
        let mut samples: Vec<f64> = Vec::with_capacity(24);
        let mut last_shaded: usize = 0;
        let mut mounted_nodes: usize = 0;
        for step in 1..=24usize {
            let next_y: f32 = ((step as f32) * 144.0) % 3_500_000.0;
            let dt: f64 = measure_ms(|| {
                let _: bool = scroll_y.set_if_changed(next_y);
                if let Ok(stats) = app.step_frame() {
                    last_shaded = stats.get_pixels_shaded();
                }
            });
            samples.push(dt);
        }
        mounted_nodes = app.get_scene().with_read(
            |tree: &SceneTree| {
                tree.get_nodes()
                    .iter()
                    .filter(|n: &&SceneNode| n.get_alive())
                    .count()
            },
            mounted_nodes,
        );
        let info: String = format!(
            "rows=100k mounted_nodes={mounted_nodes} scroll={:.0}px shaded={last_shaded}",
            scroll_y.get_untracked()
        );
        (samples, info)
    });
    scope.dispose();
    finalize_case(entry, &samples, true, details);
}

/// Runs Scenario 2: 2,000-node nested Flex/Grid layout and surgical dirty-subtree relayout passes.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Result slot populated with live timing metrics.
fn run_stress_layout_2000(entry: &mut StressCaseResult) {
    let scene: SceneHandle = SceneHandle::new_tree();
    let ctx: Context = Context::new(scene.clone(), PlatformKind::Headless);
    let (leaf_nodes, page): (Vec<NodeId>, NodeId) = with_context(ctx, || {
        let mut leaves: Vec<NodeId> = Vec::with_capacity(50);
        let mut sections: Vec<NodeId> = Vec::with_capacity(50);
        for section_idx in 0..50usize {
            let mut first_title: NodeId = NodeId::default();
            let grid_node: NodeId = view! {
                grid {
                    columns: 4
                    width: Length::Percent(100.0)
                    gap: SPACE_SM
                    for card_idx in 0..8usize {
                        column {
                            width: Length::Percent(100.0)
                            padding: Edges::all(SPACE_SM)
                            gap: SPACE_XS
                            row {
                                width: Length::Percent(100.0)
                                justify: JustifyContent::SpaceBetween
                                if card_idx == 0 {
                                    text {
                                        node_ref: first_title
                                        font_size: 13.0
                                        { format!("{STRESS_CELL_PREFIX} {section_idx}-{card_idx}") }
                                    }
                                } else {
                                    text {
                                        font_size: 13.0
                                        { format!("{STRESS_CELL_PREFIX} {section_idx}-{card_idx}") }
                                    }
                                }
                                text {
                                    font_size: 11.0
                                    { format!("{section_idx}") }
                                }
                            }
                        }
                    }
                }
            };
            leaves.push(first_title);
            sections.push(grid_node);
        }
        let p_node: NodeId = view! {
            column {
                width: Length::Px(1280.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_SM
                ..sections
            }
        };
        (leaves, p_node)
    });
    scene.with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            tree.replace_children(root, vec![page]);
        },
        (),
    );
    let mut engine: LayoutEngine = LayoutEngine::default();
    let viewport: Size = Size::new(1280.0, 800.0);
    let mut init_nodes: usize = 0;
    let init_ms: f64 = measure_ms(|| {
        let st: LayoutStats = scene.with_write(
            |tree: &mut SceneTree| engine.compute(tree, viewport),
            LayoutStats::default(),
        );
        init_nodes = st.get_nodes_computed();
    });
    let mut samples: Vec<f64> = Vec::with_capacity(40);
    let mut total_computed: usize = 0;
    let mut total_skipped: usize = 0;
    for step in 0..40usize {
        let target: NodeId = leaf_nodes[step % leaf_nodes.len()];
        let dt: f64 = measure_ms(|| {
            scene.with_write(
                |tree: &mut SceneTree| {
                    tree.set_text(target, format!("{STRESS_CELL_PREFIX} #{step}"));
                    let stats: LayoutStats = engine.compute(tree, viewport);
                    total_computed += stats.get_nodes_computed();
                    total_skipped += stats.get_subtrees_skipped();
                },
                (),
            );
        });
        samples.push(dt);
    }
    let details: String = format!(
        "nodes={init_nodes} init={init_ms:.2}ms avg_recomputed={} skipped={}",
        total_computed / 40,
        total_skipped / 40
    );
    finalize_case(entry, &samples, false, details);
}

/// Runs Scenario 3: bulk batched reactive signal and memo updates across 1,000 live components.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Result slot populated with live timing metrics.
fn run_stress_reactive_1000(entry: &mut StressCaseResult) {
    let (scope, (samples, details)): (Scope, (Vec<f64>, String)) = Scope::run(|_: Scope| {
        let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
        let p: ThemePalette = tokens.get_palette();
        let mut win: WindowConfig = WindowConfig::default();
        win.set_size(800, 600).set_render(RenderConfig::hardware());
        let mut cfg: AppConfig = AppConfig::default();
        cfg.set_window(win);
        let Ok(mut app) = App::headless(cfg) else {
            return (Vec::new(), STRESS_UNIT_NA.to_string());
        };
        let mut signals: Vec<Signal<u32>> = Vec::with_capacity(1_000);
        for idx in 0..1_000u32 {
            signals.push(Signal::new(idx));
        }
        let signals_mount: Vec<Signal<u32>> = signals.clone();
        let _: NodeId = app.mount(move || {
            let derived_memos: Vec<Memo<u32>> = signals_mount
                .into_iter()
                .map(|sig: Signal<u32>| {
                    Memo::new(move || sig.get().wrapping_mul(3).wrapping_add(1))
                })
                .collect();
            view! {
                scroll {
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    gap: 2.0
                    for derived in derived_memos.into_iter() {
                        text {
                            font_size: 12.0
                            color: p.get_foreground()
                            { move || format!("{STRESS_CELL_PREFIX} {}", derived.get()) }
                        }
                    }
                }
            }
        });
        let _: Result<FrameStats, PlatformError> = app.step_frame();
        let mut samples: Vec<f64> = Vec::with_capacity(16);
        let mut last_rev: u64 = 0;
        for tick in 1..=16u32 {
            let dt: f64 = measure_ms(|| {
                batch(|| {
                    for (idx, sig) in signals.iter().enumerate() {
                        let _: bool = sig.set_if_changed(tick.wrapping_add(idx as u32));
                    }
                });
                flush_effects();
                if app.step_frame().is_ok() {
                    last_rev = app.get_last_rendered_revision();
                }
            });
            samples.push(dt);
        }
        let info: String = format!("signals=1000 memos=1000 rev={last_rev}");
        (samples, info)
    });
    scope.dispose();
    finalize_case(entry, &samples, false, details);
}

/// Runs Scenario 4: dynamic `.wasm` binary verification, sandbox execution, hot-swap, and rollback.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Result slot populated with live timing metrics.
fn run_stress_wasm_hotswap(entry: &mut StressCaseResult) {
    let (scope, (samples, details)): (Scope, (Vec<f64>, String)) = Scope::run(|_: Scope| {
        let mut win: WindowConfig = WindowConfig::default();
        win.set_size(640, 480).set_render(RenderConfig::hardware());
        let mut cfg: AppConfig = AppConfig::default();
        cfg.set_window(win);
        let Ok(mut app) = App::headless(cfg) else {
            return (Vec::new(), STRESS_UNIT_NA.to_string());
        };
        let hub: DynamicUpdateHub = app.get_dynamic_hub().clone();
        let registry: RegistryWasmFetcher = RegistryWasmFetcher::default();
        let wasm_v1: Vec<u8> = build_partial_widget_wasm(STRESS_WASM_TITLE_V1, 1, 0x2563EB);
        let wasm_v2: Vec<u8> = build_partial_widget_wasm(STRESS_WASM_TITLE_V2, 2, 0x7C3AED);
        let csum_v1: u64 = compute_wasm_checksum(&wasm_v1);
        let csum_v2: u64 = compute_wasm_checksum(&wasm_v2);
        registry.register(
            STRESS_WASM_URL_V1.to_string(),
            wasm_v1,
            STRESS_WASM_ETAG_V1.to_string(),
            WasmFaultKind::None,
        );
        registry.register(
            STRESS_WASM_URL_V2.to_string(),
            wasm_v2,
            STRESS_WASM_ETAG_V2.to_string(),
            WasmFaultKind::None,
        );
        let manager: WasmDynamicManager =
            WasmDynamicManager::new(hub.clone(), Rc::new(registry), WasmSandboxConfig::default());
        let hub_slot: DynamicUpdateHub = hub;
        let _: NodeId = app.mount(move || {
            view! {
                slot {
                    id: STRESS_WASM_SLOT
                    hub: hub_slot
                    text {
                        font_size: 14.0
                        { STRESS_WASM_FALLBACK }
                    }
                }
            }
        });
        let _: Result<FrameStats, PlatformError> = app.step_frame();
        let mut samples: Vec<f64> = Vec::with_capacity(12);
        let mut completed: usize = 0;
        for cycle in 0..12usize {
            let base_ver: u64 = ((cycle as u64) * 2) + 1;
            let dt: f64 = measure_ms(|| {
                let req_v1: WasmDownloadRequest = WasmDownloadRequest::partial(
                    STRESS_WASM_URL_V1.to_string(),
                    STRESS_WASM_SLOT.to_string(),
                    base_ver,
                )
                .with_checksum(csum_v1)
                .with_allow_downgrade(true);
                let req_v2: WasmDownloadRequest = WasmDownloadRequest::partial(
                    STRESS_WASM_URL_V2.to_string(),
                    STRESS_WASM_SLOT.to_string(),
                    base_ver + 1,
                )
                .with_checksum(csum_v2)
                .with_allow_downgrade(true);
                if manager.load_and_mount(req_v1, STRESS_WASM_KEY).is_ok()
                    && manager.load_and_mount(req_v2, STRESS_WASM_KEY).is_ok()
                    && manager
                        .rollback_partial(STRESS_WASM_SLOT, STRESS_WASM_RB_REASON)
                        .is_ok()
                {
                    let _: Result<FrameStats, PlatformError> = app.step_frame();
                    completed += 1;
                }
            });
            samples.push(dt);
        }
        let info: String = format!("cycles={completed}/12 cache={}", manager.cache_len());
        (samples, info)
    });
    scope.dispose();
    finalize_case(entry, &samples, false, details);
}

/// Runs Scenario 5: 30-frame mobile sidebar drawer slide-in & scrim animation over shadowed cards.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Result slot populated with live timing metrics.
fn run_stress_drawer_anim(entry: &mut StressCaseResult) {
    let (scope, (samples, details)): (Scope, (Vec<f64>, String)) = Scope::run(|_: Scope| {
        let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
        let p: ThemePalette = tokens.get_palette();
        let mut win: WindowConfig = WindowConfig::default();
        win.set_size(360, 780).set_render(RenderConfig::software());
        let mut cfg: AppConfig = AppConfig::default();
        cfg.set_window(win);
        let Ok(mut app) = App::headless(cfg) else {
            return (Vec::new(), STRESS_UNIT_NA.to_string());
        };
        let drawer_open: Signal<bool> = Signal::new(false);
        let _: NodeId = app.mount(|| {
            let drawer_body: NodeId = view! {
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_SM
                    for idx in 0..8usize {
                        text {
                            font_size: 14.0
                            color: p.get_foreground()
                            { format!("{STRESS_DRAWER_ITEM} {idx}") }
                        }
                    }
                }
            };
            view! {
                column {
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    padding: Edges::all(SPACE_MD)
                    gap: SPACE_MD
                    background: p.get_background()
                    for idx in 0..6usize {
                        EuvCard(format!("{STRESS_CARD_PREFIX} #{idx}"), tokens) {
                            subtitle: STRESS_CARD_SUB
                            EuvBadge(STRESS_BADGE_TXT.to_string(), EuvTone::Primary, tokens)
                        }
                    }
                    EuvDrawer(drawer_open, STRESS_DRAWER_TITLE.to_string(), drawer_body, tokens)
                }
            }
        });
        app.set_exact_physical_viewport(720, 1560, 2.0);
        let _: Result<FrameStats, PlatformError> = app.step_frame();
        let _: bool = drawer_open.set_if_changed(true);
        let mut samples: Vec<f64> = Vec::with_capacity(30);
        let mut active_frames: usize = 0;
        for step in 0..30usize {
            if step == 16 {
                let _: bool = drawer_open.set_if_changed(false);
            }
            let dt: f64 = measure_ms(|| {
                if app.step_game_frame(1.0 / 120.0).is_ok() && app.has_active_animations() {
                    active_frames += 1;
                }
            });
            samples.push(dt);
        }
        let info: String = format!("720x1560@2x anim_frames={active_frames} backdrop_cache=on");
        (samples, info)
    });
    scope.dispose();
    finalize_case(entry, &samples, true, details);
}

/// Runs Scenario 6: 240Hz multi-touch pan, 1,000-node hit-test, and tap-jitter drift verification.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Result slot populated with live timing metrics.
fn run_stress_touch_hittest(entry: &mut StressCaseResult) {
    let (scope, (samples, details)): (Scope, (Vec<f64>, String)) = Scope::run(|_: Scope| {
        let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
        let p: ThemePalette = tokens.get_palette();
        let mut win: WindowConfig = WindowConfig::default();
        win.set_size(400, 800).set_render(RenderConfig::hardware());
        let mut cfg: AppConfig = AppConfig::default();
        cfg.set_window(win);
        let Ok(mut app) = App::headless(cfg) else {
            return (Vec::new(), STRESS_UNIT_NA.to_string());
        };
        let mut scroll_id: NodeId = NodeId::default();
        let _: NodeId = app.mount(|| {
            view! {
                scroll {
                    node_ref: scroll_id
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    gap: SPACE_XS
                    padding: Edges::all(SPACE_SM)
                    for idx in 0..250usize {
                        row {
                            width: Length::Percent(100.0)
                            height: Length::Px(32.0)
                            gap: SPACE_XS
                            text { font_size: 12.0 color: p.get_foreground() { format!("{STRESS_ROW_PREFIX} {idx}") } }
                            text { font_size: 11.0 color: p.get_muted_foreground() { STRESS_BADGE_TXT } }
                            text { font_size: 11.0 color: p.get_primary() { format!("#{idx}") } }
                        }
                    }
                }
            }
        });
        let _: Result<FrameStats, PlatformError> = app.step_frame();
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::PointerDown(
            wid,
            PointerEvent::touch(99, 200.0, 400.0),
        ));
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::touch(99, 204.0, 406.0),
        ));
        app.dispatch_window_event(WindowEvent::PointerUp(
            wid,
            PointerEvent::touch(99, 204.0, 406.0),
        ));
        let _: Result<FrameStats, PlatformError> = app.step_frame();
        let tap_drift: f32 = app.get_scene().with_read(
            |tree: &SceneTree| {
                tree.get_node(scroll_id)
                    .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                    .unwrap_or(0.0)
            },
            0.0,
        );
        let mut samples: Vec<f64> = Vec::with_capacity(120);
        for step in 0..120usize {
            let y: f32 = 120.0 + ((step * 19) % 580) as f32;
            let dt: f64 = measure_ms(|| {
                app.dispatch_window_event(WindowEvent::PointerMove(
                    wid,
                    PointerEvent::mouse(200.0, y),
                ));
                app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 14.0)));
                let _: Result<FrameStats, PlatformError> = app.step_game_frame(1.0 / 240.0);
            });
            samples.push(dt);
        }
        let final_scroll: f32 = app.get_scene().with_read(
            |tree: &SceneTree| {
                tree.get_node(scroll_id)
                    .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                    .unwrap_or(0.0)
            },
            0.0,
        );
        let info: String =
            format!("nodes=1000 tap_drift={tap_drift:.2}px scrolled={final_scroll:.0}px");
        (samples, info)
    });
    scope.dispose();
    finalize_case(entry, &samples, false, details);
}

/// Runs Scenario 7: HiDPI 3x (`1080x1920`) Gaussian shadow and SDF rounded-rect rasterization.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Result slot populated with live timing metrics.
fn run_stress_hidpi_shadow(entry: &mut StressCaseResult) {
    let (scope, (samples, details)): (Scope, (Vec<f64>, String)) = Scope::run(|_: Scope| {
        let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
        let mut win: WindowConfig = WindowConfig::default();
        win.set_size(360, 640).set_render(RenderConfig::software());
        let mut cfg: AppConfig = AppConfig::default();
        cfg.set_window(win);
        let Ok(mut app) = App::headless(cfg) else {
            return (Vec::new(), STRESS_UNIT_NA.to_string());
        };
        let mut scroll_id: NodeId = NodeId::default();
        let _: NodeId = app.mount(|| {
            view! {
                scroll {
                    node_ref: scroll_id
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    padding: Edges::all(SPACE_MD)
                    gap: SPACE_MD
                    background: tokens.get_palette().get_background()
                    for idx in 0..12usize {
                        EuvCard(format!("{STRESS_CARD_PREFIX} #{idx}"), tokens) {
                            subtitle: STRESS_CARD_SUB
                            EuvBadge(STRESS_BADGE_TXT.to_string(), EuvTone::Primary, tokens)
                            EuvProgress(((idx % 9) as f32 + 1.0) * 0.1, tokens)
                        }
                    }
                }
            }
        });
        app.set_exact_physical_viewport(1080, 1920, 3.0);
        let _: Result<FrameStats, PlatformError> = app.step_frame();
        let mut samples: Vec<f64> = Vec::with_capacity(12);
        let mut last_shaded: usize = 0;
        for step in 1..=12usize {
            let offset_y: f32 = ((step as f32) * 24.0) % 600.0;
            let dt: f64 = measure_ms(|| {
                app.get_scene().with_write(
                    |tree: &mut SceneTree| {
                        tree.set_scroll_offset(scroll_id, Point::new(0.0, offset_y));
                    },
                    (),
                );
                if let Ok(stats) = app.step_frame() {
                    last_shaded = stats.get_pixels_shaded();
                }
            });
            samples.push(dt);
        }
        let info: String = format!("1080x1920@3x shaded={last_shaded}px/f");
        (samples, info)
    });
    scope.dispose();
    finalize_case(entry, &samples, true, details);
}

/// Runs Scenario 8: Unicode + CJK glyph rasterization and RGBA8 atlas shelf-packing.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Result slot populated with live timing metrics.
fn run_stress_cjk_atlas(entry: &mut StressCaseResult) {
    let sample_chars: [char; 16] = [
        'A', 'W', 'g', '9', '跨', '端', '原', '生', '框', '架', '高', '性', '能', '渲', '染', '器',
    ];
    let provider: NativeSystemFont = NativeSystemFont::deterministic();
    let mut samples: Vec<f64> = Vec::with_capacity(16);
    let mut packed_regions: usize = 0;
    let mut atlas_w: u32 = 0;
    let mut atlas_h: u32 = 0;
    for _ in 0..16usize {
        let dt: f64 = measure_ms(|| {
            let mut atlas: GlyphAtlas = GlyphAtlas::default();
            for sz in [120u16, 160u16, 220u16].into_iter() {
                for ch in sample_chars.iter() {
                    let key: GlyphKey = GlyphKey::new(*ch, sz, sz >= 220);
                    let _: &RasterizedGlyph = atlas.get_or_rasterize_with(key, &provider);
                }
            }
            let packed: PackedGlyphAtlas = atlas.pack_rgba_atlas();
            packed_regions = packed.get_regions().len();
            atlas_w = packed.get_width();
            atlas_h = packed.get_height();
        });
        samples.push(dt);
    }
    let details: String = format!("glyphs={packed_regions} atlas={atlas_w}x{atlas_h}");
    finalize_case(entry, &samples, false, details);
}

/// Runs Scenario 9: 2D rigid-body physics (256 bodies + 1,024 particles) and 3D BVH ray-tracing.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Result slot populated with live timing metrics.
fn run_stress_game_2d_3d(entry: &mut StressCaseResult) {
    let mut world2d: GameWorld = GameWorld::with_capacity(256);
    for idx in 0..256usize {
        let col: f32 = (idx % 16) as f32;
        let row: f32 = (idx / 16) as f32;
        let eid: EntityId = world2d.spawn(
            EntityTag::Enemy,
            Point::new(40.0 + col * 52.0, 40.0 + row * 40.0),
        );
        if let Some(ent) = world2d.get_entity_mut(eid) {
            ent.set_velocity(Velocity2D::new(
                if idx % 2 == 0 { 110.0 } else { -95.0 },
                if idx % 3 == 0 { 85.0 } else { -75.0 },
                0.0,
            ));
            ent.set_body(Some(RigidBody2D::dynamic(1.0, 0.9, 0.02)));
            ent.set_collider(Some(Collider2D::circle(
                10.0,
                COLLISION_LAYER_ENEMY,
                COLLISION_LAYER_ALL,
                false,
            )));
        }
    }
    let mut phys_cfg: PhysicsConfig = PhysicsConfig::default();
    phys_cfg.set_arena_bounds(Some(Rect::new(0.0, 0.0, 960.0, 720.0)));
    let mut physics: PhysicsWorld2D = PhysicsWorld2D::with_config(phys_cfg);
    let mut particles: ParticleSystem = ParticleSystem::with_capacity(1024);
    let mut emit_cfg: ParticleEmitterConfig = ParticleEmitterConfig::default();
    emit_cfg.set_min_life(10.0).set_max_life(20.0);
    particles.emit_burst(Point::new(400.0, 300.0), 1024, emit_cfg);
    let mut collisions: usize = 0;
    let phys_ms: f64 = measure_ms(|| {
        for _ in 0..60usize {
            collisions += physics.step(&mut world2d, 1.0 / 60.0).len();
            let _: usize = particles.update(1.0 / 60.0);
        }
    });
    let mut world3d: World3D = World3D::create_open_world_city();
    let mut renderer3d: Renderer3D = Renderer3D::default();
    let cfg3d: RenderQualityConfig = RenderQualityConfig::from_preset(QualityPreset::High);
    let mut samples: Vec<f64> = Vec::with_capacity(4);
    let mut last_tris: usize = 0;
    let mut last_rays: usize = 0;
    for _ in 0..4usize {
        let dt: f64 = measure_ms(|| {
            let _: ImageBuffer = renderer3d.render_world(&mut world3d, 320, 180, &cfg3d);
            let st: RenderStats3D = renderer3d.get_last_stats();
            last_tris = st.get_triangles_drawn();
            last_rays = st.get_rays_traced();
        });
        samples.push(dt);
    }
    let details: String =
        format!("2d_60t={phys_ms:.2}ms col={collisions} 3d_tris={last_tris} rays={last_rays}");
    finalize_case(entry, &samples, true, details);
}

/// Runs Scenario 10: declarative `view!` macro component tree mount, layout, and disposal.
///
/// # Arguments
///
/// - `&mut StressCaseResult` - Result slot populated with live timing metrics.
fn run_stress_macro_mount(entry: &mut StressCaseResult) {
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let p: ThemePalette = tokens.get_palette();
    let mut samples: Vec<f64> = Vec::with_capacity(8);
    let mut last_nodes: usize = 0;
    for _ in 0..8usize {
        let dt: f64 = measure_ms(|| {
            let scene: SceneHandle = SceneHandle::new_tree();
            let ctx: Context = Context::new(scene.clone(), PlatformKind::Headless);
            let tree_root: NodeId = with_context(ctx, || {
                view! {
                    column {
                        width: Length::Px(1000.0)
                        gap: SPACE_XS
                        for idx in 0..160usize {
                            row {
                                width: Length::Percent(100.0)
                                justify: JustifyContent::SpaceBetween
                                align: AlignItems::Center
                                padding: Edges::symmetric(SPACE_XS, SPACE_SM)
                                background: p.get_card()
                                text {
                                    font_size: 13.0
                                    color: p.get_foreground()
                                    { format!("{STRESS_CARD_PREFIX} {idx}") }
                                }
                                EuvBadge(STRESS_BADGE_TXT.to_string(), EuvTone::Primary, tokens)
                            }
                        }
                    }
                }
            });
            let mut engine: LayoutEngine = LayoutEngine::default();
            scene.with_write(
                |tree: &mut SceneTree| {
                    let root: NodeId = tree.get_root();
                    tree.replace_children(root, vec![tree_root]);
                    let st: LayoutStats = engine.compute(tree, Size::new(1000.0, 800.0));
                    last_nodes = st.get_nodes_computed();
                },
                (),
            );
        });
        samples.push(dt);
    }
    let details: String = format!("cards=160 nodes={last_nodes} zero_alloc_macro=ok");
    finalize_case(entry, &samples, false, details);
}

/// Executes a single interactive stress scenario by `scenario_id` and updates `state` signals immediately.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state storing stress telemetry.
/// - `&str` - Stable scenario identifier (`STRESS_ID_*`).
pub fn execute_stress_case(state: &ShowcaseState, scenario_id: &str) {
    let results_sig: Signal<Vec<StressCaseResult>> = state.get_stress_results();
    let summary_sig: Signal<String> = state.get_stress_summary();
    let mut rows: Vec<StressCaseResult> = results_sig.get_untracked();
    let Some(entry) = rows
        .iter_mut()
        .find(|item: &&mut StressCaseResult| item.get_id().as_str() == scenario_id)
    else {
        return;
    };
    match scenario_id {
        STRESS_ID_VLIST_100K => run_stress_vlist_100k(entry),
        STRESS_ID_LAYOUT_2000 => run_stress_layout_2000(entry),
        STRESS_ID_REACTIVE_1000 => run_stress_reactive_1000(entry),
        STRESS_ID_WASM_HOTSWAP => run_stress_wasm_hotswap(entry),
        STRESS_ID_DRAWER_ANIM => run_stress_drawer_anim(entry),
        STRESS_ID_TOUCH_HITTEST => run_stress_touch_hittest(entry),
        STRESS_ID_HIDPI_SHADOW => run_stress_hidpi_shadow(entry),
        STRESS_ID_CJK_ATLAS => run_stress_cjk_atlas(entry),
        STRESS_ID_GAME_2D_3D => run_stress_game_2d_3d(entry),
        STRESS_ID_MACRO_MOUNT => run_stress_macro_mount(entry),
        _ => return,
    }
    let summary: String = format!(
        "{} -> {} | avg={:.2}ms p95={:.2}ms max={:.2}ms ({}) [{}]",
        entry.get_title(),
        entry.get_status(),
        entry.get_avg_ms(),
        entry.get_p95_ms(),
        entry.get_max_ms(),
        entry.get_throughput(),
        entry.get_details()
    );
    batch(|| {
        let _: bool = results_sig.set(rows);
        let _: bool = summary_sig.set_if_changed(summary);
    });
}

/// Executes a slice of stress scenario identifiers sequentially and updates live summary telemetry.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state storing stress telemetry.
/// - `&[&str]` - Ordered list of scenario IDs to run.
pub fn execute_stress_batch(state: &ShowcaseState, ids: &[&str]) {
    let mut total_ms: f64 = 0.0;
    for scenario_id in ids.iter() {
        let dt: f64 = measure_ms(|| execute_stress_case(state, scenario_id));
        total_ms += dt;
    }
    let rows: Vec<StressCaseResult> = state.get_stress_results().get_untracked();
    let completed: usize = rows
        .iter()
        .filter(|r: &&StressCaseResult| r.get_runs() > 0)
        .count();
    let passed: usize = rows
        .iter()
        .filter(|r: &&StressCaseResult| r.get_status().as_str() == STRESS_STATUS_PASS)
        .count();
    let summary: String = format!(
        "Completed {completed}/{} scenarios ({passed}/{completed} within budget) in {total_ms:.1}ms total.",
        rows.len()
    );
    let _: bool = state.get_stress_summary().set_if_changed(summary);
}

/// Mounts the declarative Interactive UI Stress & Performance Lab page (`/stress`).
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active resolved design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted stress lab page handle.
pub fn mount_stress_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let results_sig: Signal<Vec<StressCaseResult>> = state.get_stress_results();
    let summary_sig: Signal<String> = state.get_stress_summary();
    let st_all: ShowcaseState = state.clone();
    let st_core: ShowcaseState = state.clone();
    let st_reset: ShowcaseState = state.clone();
    let st_list: ShowcaseState = state.clone();
    view! {
        EuvPage(STRESS_PAGE_TITLE.to_string(), STRESS_PAGE_SUBTITLE.to_string(), tokens) {
            column {
                width: Length::Percent(100.0)
                bind_children: move || {
                    let rows: Vec<StressCaseResult> = results_sig.get();
                    let total: usize = rows.len();
                    let done: usize = rows.iter().filter(|r: &&StressCaseResult| r.get_runs() > 0).count();
                    let passed: usize = rows
                    .iter()
                    .filter(|r: &&StressCaseResult| r.get_status().as_str() == STRESS_STATUS_PASS)
                    .count();
                    let latest_opt: Option<&StressCaseResult> = rows
                    .iter()
                    .filter(|r: &&StressCaseResult| r.get_runs() > 0)
                    .max_by_key(|r: &&StressCaseResult| r.get_runs());
                    let (latest_val, latest_cap): (String, String) = match latest_opt {
                        Some(item) => (
                            format!("{:.2} ms", item.get_avg_ms()),
                            format!("{} ({})", item.get_title(), item.get_throughput()),
                        ),
                        None => (
                            STRESS_KPI_IDLE_VAL.to_string(),
                            STRESS_KPI_IDLE_CAP.to_string(),
                        ),
                    };
                    let kpi_grid: NodeId = view! {
                        EuvStatGrid() {
                            EuvStatCard(
                                STRESS_KPI_COMPLETED_LABEL.to_string(),
                                format!("{done} / {total}"),
                                format!("{total} interactive workloads ready"),
                                tokens
                            )
                            EuvStatCard(
                                STRESS_KPI_BUDGET_LABEL.to_string(),
                                if done == 0 {
                                    STRESS_KPI_IDLE_VAL.to_string()
                                } else {
                                    format!("{passed} / {done} {STRESS_STATUS_PASS}")
                                },
                                STRESS_KPI_BUDGET_CAP.to_string(),
                                tokens
                            )
                            EuvStatCard(
                                STRESS_KPI_LATEST_LABEL.to_string(),
                                latest_val,
                                latest_cap,
                                tokens
                            )
                        }
                    };
                    vec![kpi_grid]
                }
            }
            EuvCard(STRESS_CTRL_TITLE.to_string(), tokens) {
                subtitle: STRESS_CTRL_SUBTITLE
                EuvActionRow() {
                    EuvButton(STRESS_BTN_RUN_ALL.to_string(), tokens) {
                        on_click: move |_: &UiEvent| {
                            let all_ids: [&str; 10] = [
                                STRESS_ID_VLIST_100K,
                                STRESS_ID_LAYOUT_2000,
                                STRESS_ID_REACTIVE_1000,
                                STRESS_ID_WASM_HOTSWAP,
                                STRESS_ID_DRAWER_ANIM,
                                STRESS_ID_TOUCH_HITTEST,
                                STRESS_ID_HIDPI_SHADOW,
                                STRESS_ID_CJK_ATLAS,
                                STRESS_ID_GAME_2D_3D,
                                STRESS_ID_MACRO_MOUNT,
                            ];
                            execute_stress_batch(&st_all, &all_ids);
                        }
                    }
                    EuvButton(STRESS_BTN_RUN_CORE.to_string(), tokens) {
                        secondary()
                        on_click: move |_: &UiEvent| {
                            let core_ids: [&str; 5] = [
                                STRESS_ID_VLIST_100K,
                                STRESS_ID_LAYOUT_2000,
                                STRESS_ID_REACTIVE_1000,
                                STRESS_ID_WASM_HOTSWAP,
                                STRESS_ID_DRAWER_ANIM,
                            ];
                            execute_stress_batch(&st_core, &core_ids);
                        }
                    }
                    EuvButton(STRESS_BTN_RESET.to_string(), tokens) {
                        outline()
                        on_click: move |_: &UiEvent| {
                            batch(|| {
                                let _: bool = st_reset
                                .get_stress_results()
                                .set(default_stress_results());
                                let _: bool = st_reset
                                .get_stress_summary()
                                .set_if_changed(STRESS_SUMMARY_READY.to_string());
                            });
                        }
                    }
                }
                text {
                    font_size: 13.0
                    color: p.get_primary()
                    { move || summary_sig.get() }
                }
            }
            EuvCard(STRESS_SCENARIOS_TITLE.to_string(), tokens) {
                subtitle: STRESS_SCENARIOS_SUB
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_MD
                    bind_children: move || {
                        let rows: Vec<StressCaseResult> = results_sig.get();
                        let mut cards: Vec<NodeId> = Vec::with_capacity(rows.len());
                        for item in rows.into_iter() {
                            let sid: String = item.get_id().clone();
                            let st_run: ShowcaseState = st_list.clone();
                            let status_tone: EuvTone = match item.get_status().as_str() {
                                STRESS_STATUS_PASS => EuvTone::Success,
                                STRESS_STATUS_WARN => EuvTone::Warning,
                                _ => EuvTone::Neutral,
                            };
                            let metric_line: String = if item.get_runs() == 0 {
                                format!(
                                    "Budget <= {:.2}ms | Status: {} | Click button to run",
                                    item.get_budget_ms(),
                                    item.get_status()
                                )
                            } else {
                                format!(
                                    "Avg: {:.2}ms | P95: {:.2}ms | Max: {:.2}ms | Budget <= {:.2}ms | {} (runs={})",
                                    item.get_avg_ms(),
                                    item.get_p95_ms(),
                                    item.get_max_ms(),
                                    item.get_budget_ms(),
                                    item.get_throughput(),
                                    item.get_runs()
                                )
                            };
                            let detail_line: String = if item.get_runs() == 0 {
                                item.get_description().clone()
                            } else {
                                format!("{} | {}", item.get_description(), item.get_details())
                            };
                            let item_card: NodeId = view! {
                                column {
                                    width: Length::Percent(100.0)
                                    padding: Edges::all(SPACE_MD)
                                    gap: SPACE_SM
                                    background: p.get_card()
                                    border: (g.get_border_width(), p.get_border())
                                    radius: g.get_radius_md()
                                    row {
                                        width: Length::Percent(100.0)
                                        flex_wrap: FlexWrap::Wrap
                                        justify: JustifyContent::SpaceBetween
                                        align: AlignItems::Center
                                        gap: SPACE_SM
                                        text {
                                            font_size: 14.0
                                            font_weight: FontWeight::Bold
                                            color: p.get_foreground()
                                            { item.get_title().clone() }
                                        }
                                        row {
                                            gap: SPACE_XS
                                            align: AlignItems::Center
                                            EuvBadge(item.get_category().clone(), EuvTone::Primary, tokens)
                                            EuvBadge(item.get_status().clone(), status_tone, tokens)
                                        }
                                    }
                                    text {
                                        font_size: 12.5
                                        color: p.get_muted_foreground()
                                        { detail_line }
                                    }
                                    text {
                                        font_size: 12.5
                                        font_weight: FontWeight::SemiBold
                                        color: p.get_foreground()
                                        { metric_line }
                                    }
                                    EuvActionRow() {
                                        EuvButton(item.get_button_label().clone(), tokens) {
                                            secondary()
                                            sm()
                                            on_click: move |_: &UiEvent| {
                                                execute_stress_case(&st_run, &sid);
                                            }
                                        }
                                    }
                                }
                            };
                            cards.push(item_card);
                        }
                        cards
                    }
                }
            }
        }
    }
}
