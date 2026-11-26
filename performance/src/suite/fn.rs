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
    let ctx: Context = Context::new(scene.clone(), PlatformKind::Headless);
    let container: NodeId = with_context(ctx, || {
        view! {
            column {
                width: Length::Px(800.0)
                height: Length::Px(600.0)
                gap: SPACE_XS
                for idx in 0..120usize {
                    row {
                        width: Length::Percent(100.0)
                        height: Length::Px(24.0)
                        text {
                            { format!("{BENCH_ROW_LABEL} {idx}") }
                        }
                    }
                }
            }
        }
    });
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

/// Benchmarks Unicode + CJK glyph rasterization and GPU RGBA8 atlas shelf-packing.
///
/// # Arguments
///
/// - `usize` - Number of glyph-pack iterations to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured latency and packed glyph count.
#[must_use]
pub fn bench_unicode_glyph_atlas(iterations: usize) -> BenchMetric {
    let count: usize = iterations.max(1);
    let sample_chars: [char; 8] = ['A', 'g', '9', '跨', '端', '原', '生', '框'];
    let provider: NativeSystemFont = NativeSystemFont::deterministic();
    let start: Instant = Instant::now();
    let mut packed_regions: usize = 0;
    for _ in 0..count {
        let mut atlas: GlyphAtlas = GlyphAtlas::default();
        for ch in sample_chars.iter() {
            let key: GlyphKey = GlyphKey::new(*ch, 160, false);
            let _: &RasterizedGlyph = atlas.get_or_rasterize_with(key, &provider);
        }
        let packed: PackedGlyphAtlas = atlas.pack_rgba_atlas();
        packed_regions = packed.get_regions().len();
    }
    build_metric(
        BENCH_GLYPH_ATLAS_NAME,
        count,
        start.elapsed(),
        format!("packed_glyphs={packed_regions}"),
    )
}

/// Benchmarks full forced-redraw 800x600 software SDF tile rasterization frames.
///
/// # Arguments
///
/// - `usize` - Number of full raster frames to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured full-frame raster latency and FPS throughput.
#[must_use]
pub fn bench_software_full_redraw(iterations: usize) -> BenchMetric {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(800, 600).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let Ok(mut app) = App::headless(cfg) else {
        return build_metric(BENCH_SW_FULL_NAME, count, Duration::ZERO, String::new());
    };
    let _: NodeId = app.mount(|| {
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_LG)
                gap: SPACE_MD
                EuvHeader(BENCH_TITLE.to_string(), BENCH_SUBTITLE.to_string(), tokens)
                EuvCard(BENCH_TITLE.to_string(), tokens) {
                    subtitle: BENCH_SUBTITLE
                    EuvBadge(BENCH_BADGE.to_string(), EuvTone::Primary, tokens)
                    EuvProgress(0.72, tokens)
                }
            }
        }
    });
    let wid: WindowId = app.get_window_state().get_id();
    let start: Instant = Instant::now();
    let mut last_hash: u64 = 0;
    let mut last_batches: usize = 0;
    for _ in 0..count {
        app.dispatch_window_event(WindowEvent::RedrawRequested(wid));
        if let Ok(stats) = app.step_frame() {
            last_hash = stats.get_frame_hash();
            last_batches = stats.get_draw_batches();
        }
    }
    build_metric(
        BENCH_SW_FULL_NAME,
        count,
        start.elapsed(),
        format!("batches={last_batches} hash={last_hash:#018x}"),
    )
}

/// Benchmarks 256 dynamic 2D rigid bodies stepping through spatial-hash broad-phase and narrow-phase collision resolution.
///
/// # Arguments
///
/// - `usize` - Number of fixed 60Hz physics ticks to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured physics tick latency and collision throughput.
#[must_use]
pub fn bench_game_ecs_physics(iterations: usize) -> BenchMetric {
    let count: usize = iterations.max(1);
    let mut world: GameWorld = GameWorld::with_capacity(256);
    for idx in 0..256usize {
        let col: f32 = (idx % 16) as f32;
        let row: f32 = (idx / 16) as f32;
        let eid: EntityId = world.spawn(
            EntityTag::Enemy,
            Point::new(40.0 + col * 52.0, 40.0 + row * 40.0),
        );
        if let Some(ent) = world.get_entity_mut(eid) {
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
    let start: Instant = Instant::now();
    let mut total_collisions: usize = 0;
    for _ in 0..count {
        let evs: Vec<CollisionEvent> = physics.step(&mut world, 1.0 / 60.0);
        total_collisions += evs.len();
    }
    build_metric(
        BENCH_GAME_PHYSICS_NAME,
        count,
        start.elapsed(),
        format!("entities=256 collisions={total_collisions}"),
    )
}

/// Benchmarks updating 1,024 active zero-allocation pooled particles per frame.
///
/// # Arguments
///
/// - `usize` - Number of particle update steps to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured particle update latency and throughput.
#[must_use]
pub fn bench_game_particles(iterations: usize) -> BenchMetric {
    let count: usize = iterations.max(1);
    let mut system: ParticleSystem = ParticleSystem::with_capacity(1024);
    let mut cfg: ParticleEmitterConfig = ParticleEmitterConfig::default();
    cfg.set_min_life(10.0).set_max_life(20.0);
    system.emit_burst(Point::new(400.0, 300.0), 1024, cfg);
    let start: Instant = Instant::now();
    let mut live: usize = 0;
    for _ in 0..count {
        live = system.update(1.0 / 60.0);
    }
    build_metric(
        BENCH_GAME_PARTICLES_NAME,
        count,
        start.elapsed(),
        format!("active_particles={live}"),
    )
}

/// Benchmarks the AAA 3D open-world PBR + BVH ray-traced shadows and reflections pipeline.
///
/// # Arguments
///
/// - `usize` - Number of 3D ray-traced frames to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured 3D frame latency, triangle throughput, and BVH ray count.
#[must_use]
pub fn bench_game3d_pbr_bvh_raytrace(iterations: usize) -> BenchMetric {
    let count: usize = iterations.max(1);
    let mut world: World3D = World3D::create_open_world_city();
    let mut renderer: Renderer3D = Renderer3D::default();
    let cfg: RenderQualityConfig = RenderQualityConfig::from_preset(QualityPreset::High);
    let start: Instant = Instant::now();
    let mut last_rays: usize = 0;
    let mut last_tris: usize = 0;
    for _ in 0..count {
        let _: ImageBuffer = renderer.render_world(&mut world, 480, 270, &cfg);
        let st: RenderStats3D = renderer.get_last_stats();
        last_rays = st.get_rays_traced();
        last_tris = st.get_triangles_drawn();
    }
    build_metric(
        BENCH_GAME_3D_RT_NAME,
        count,
        start.elapsed(),
        format!("tris={last_tris} rays={last_rays}"),
    )
}

/// Benchmarks maxed-out `Cinematic` + `FullPathTracedGi` + `125%` scale 3D ray tracing at HD resolution.
///
/// # Arguments
///
/// - `usize` - Number of max-settings 3D frames to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured frame latency, FPS throughput, resolution, and ray count.
#[must_use]
pub fn bench_game3d_cinematic_max_rt(iterations: usize) -> BenchMetric {
    let count: usize = iterations.max(1);
    let mut world: World3D = World3D::create_open_world_city();
    let mut renderer: Renderer3D = Renderer3D::default();
    let mut cfg: RenderQualityConfig = RenderQualityConfig::from_preset(QualityPreset::Cinematic);
    cfg.set_ray_tracing(RayTracingMode::FullPathTracedGi)
        .set_resolution_scale(1.25);
    let start: Instant = Instant::now();
    let mut last_rays: usize = 0;
    let mut last_w: u32 = 0;
    let mut last_h: u32 = 0;
    for _ in 0..count {
        let _: ImageBuffer = renderer.render_world(&mut world, 960, 480, &cfg);
        let st: RenderStats3D = renderer.get_last_stats();
        last_rays = st.get_rays_traced();
        last_w = st.get_render_width();
        last_h = st.get_render_height();
    }
    build_metric(
        BENCH_GAME_3D_MAX_RT_NAME,
        count,
        start.elapsed(),
        format!("res={last_w}x{last_h} rays={last_rays}"),
    )
}

/// Stress-benchmarks continuous scrolling of a 100,000-row `EuvVirtualList` through full layout and software rasterization.
///
/// # Arguments
///
/// - `usize` - Number of scrolled frames to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured frame latency, throughput, and final virtual scroll offset.
#[must_use]
pub fn bench_stress_long_list_100k(iterations: usize) -> BenchMetric {
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
        return build_metric(
            BENCH_STRESS_LONG_LIST_NAME,
            count,
            Duration::ZERO,
            String::new(),
        );
    };
    let scroll_y: Signal<f32> = Signal::new(0.0);
    let _: NodeId = app.mount(|| {
        let vlist: EuvVirtualList =
            EuvVirtualList::new(100_000, 36.0, scroll_y, tokens).viewport_height(680.0);
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
                        { format!("{BENCH_ROW_LABEL} #{idx}") }
                    }
                }
            }
        })
    });
    let _: Result<FrameStats, PlatformError> = app.step_frame();
    let start: Instant = Instant::now();
    let mut last_shaded: usize = 0;
    for step in 1..=count {
        let next_y: f32 = ((step as f32) * 144.0) % 3_500_000.0;
        let _: bool = scroll_y.set_if_changed(next_y);
        if let Ok(stats) = app.step_frame() {
            last_shaded = stats.get_pixels_shaded();
        }
    }
    build_metric(
        BENCH_STRESS_LONG_LIST_NAME,
        count,
        start.elapsed(),
        format!(
            "rows=100000 scroll_y={:.0}px shaded={last_shaded}",
            scroll_y.get_untracked()
        ),
    )
}

/// Stress-benchmarks a deeply nested 2,000-node Flex/Grid layout tree under repeated full and dirty-subtree layout passes.
///
/// # Arguments
///
/// - `usize` - Number of layout passes to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured per-pass layout latency and node count.
#[must_use]
pub fn bench_stress_complex_layout_2000(iterations: usize) -> BenchMetric {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
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
                                        { format!("{BENCH_CELL_LABEL} {section_idx}-{card_idx}") }
                                    }
                                } else {
                                    text {
                                        font_size: 13.0
                                        { format!("{BENCH_CELL_LABEL} {section_idx}-{card_idx}") }
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
    let init_stats: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| engine.compute(tree, viewport),
        LayoutStats::default(),
    );
    let start: Instant = Instant::now();
    let mut total_computed: usize = 0;
    let mut total_skipped: usize = 0;
    for step in 0..count {
        let target: NodeId = leaf_nodes[step % leaf_nodes.len()];
        scene.with_write(
            |tree: &mut SceneTree| {
                tree.set_text(target, format!("{BENCH_CELL_LABEL} #{step}"));
                let stats: LayoutStats = engine.compute(tree, viewport);
                total_computed += stats.get_nodes_computed();
                total_skipped += stats.get_subtrees_skipped();
            },
            (),
        );
    }
    build_metric(
        BENCH_STRESS_COMPLEX_LAYOUT_NAME,
        count,
        start.elapsed(),
        format!(
            "initial_nodes={} avg_computed={} avg_skipped={}",
            init_stats.get_nodes_computed(),
            total_computed / count,
            total_skipped / count
        ),
    )
}

/// Stress-benchmarks bulk batched updates across 1,000 live reactive UI components with subscribed signals and memos.
///
/// # Arguments
///
/// - `usize` - Number of full 1,000-component batch update ticks to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured batch update + reactive flush + scene patch latency.
#[must_use]
pub fn bench_stress_bulk_reactive_1000(iterations: usize) -> BenchMetric {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let p: ThemePalette = tokens.get_palette();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(800, 600).set_render(RenderConfig::hardware());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let Ok(mut app) = App::headless(cfg) else {
        return build_metric(
            BENCH_STRESS_BULK_REACTIVE_NAME,
            count,
            Duration::ZERO,
            String::new(),
        );
    };
    let mut signals: Vec<Signal<u32>> = Vec::with_capacity(1_000);
    for idx in 0..1_000u32 {
        signals.push(Signal::new(idx));
    }
    let signals_mount: Vec<Signal<u32>> = signals.clone();
    let _: NodeId = app.mount(move || {
        let derived_memos: Vec<Memo<u32>> = signals_mount
            .into_iter()
            .map(|sig: Signal<u32>| Memo::new(move || sig.get().wrapping_mul(3).wrapping_add(1)))
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
                        { move || format!("{BENCH_CELL_LABEL} {}", derived.get()) }
                    }
                }
            }
        }
    });
    let _: Result<FrameStats, PlatformError> = app.step_frame();
    let start: Instant = Instant::now();
    let mut last_rev: u64 = 0;
    for tick in 1..=(count as u32) {
        batch(|| {
            for (idx, sig) in signals.iter().enumerate() {
                let _: bool = sig.set_if_changed(tick.wrapping_add(idx as u32));
            }
        });
        flush_effects();
        if app.step_frame().is_ok() {
            last_rev = app.get_last_rendered_revision();
        }
    }
    build_metric(
        BENCH_STRESS_BULK_REACTIVE_NAME,
        count,
        start.elapsed(),
        format!("components=1000 final_rev={last_rev}"),
    )
}

/// Stress-benchmarks end-to-end dynamic `.wasm` binary verification, sandbox execution, hot-swap, and rollback cycles.
///
/// # Arguments
///
/// - `usize` - Number of v1 -> v2 -> rollback hot-swap cycles to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured latency per full dynamic WASM hot-swap cycle.
#[must_use]
pub fn bench_stress_wasm_dynamic_hotswap(iterations: usize) -> BenchMetric {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(640, 480).set_render(RenderConfig::hardware());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let Ok(mut app) = App::headless(cfg) else {
        return build_metric(
            BENCH_STRESS_WASM_LOAD_NAME,
            count,
            Duration::ZERO,
            String::new(),
        );
    };
    let hub: DynamicUpdateHub = app.get_dynamic_hub().clone();
    let registry: RegistryWasmFetcher = RegistryWasmFetcher::default();
    let wasm_v1: Vec<u8> = build_partial_widget_wasm(BENCH_WASM_TITLE_V1, 1, 0x2563EB);
    let wasm_v2: Vec<u8> = build_partial_widget_wasm(BENCH_WASM_TITLE_V2, 2, 0x7C3AED);
    let csum_v1: u64 = compute_wasm_checksum(&wasm_v1);
    let csum_v2: u64 = compute_wasm_checksum(&wasm_v2);
    registry.register(
        BENCH_WASM_URL_V1.to_string(),
        wasm_v1,
        BENCH_WASM_ETAG_V1.to_string(),
        WasmFaultKind::None,
    );
    registry.register(
        BENCH_WASM_URL_V2.to_string(),
        wasm_v2,
        BENCH_WASM_ETAG_V2.to_string(),
        WasmFaultKind::None,
    );
    let manager: WasmDynamicManager =
        WasmDynamicManager::new(hub.clone(), Rc::new(registry), WasmSandboxConfig::default());
    let hub_slot: DynamicUpdateHub = hub;
    let _: NodeId = app.mount(move || {
        view! {
            slot {
                id: BENCH_WASM_SLOT
                hub: hub_slot
                text {
                    font_size: 14.0
                    { BENCH_TITLE }
                }
            }
        }
    });
    let _: Result<FrameStats, PlatformError> = app.step_frame();
    let start: Instant = Instant::now();
    let mut completed: usize = 0;
    for cycle in 0..count {
        let base_ver: u64 = ((cycle as u64) * 2) + 1;
        let req_v1: WasmDownloadRequest = WasmDownloadRequest::partial(
            BENCH_WASM_URL_V1.to_string(),
            BENCH_WASM_SLOT.to_string(),
            base_ver,
        )
        .with_checksum(csum_v1)
        .with_allow_downgrade(true);
        let req_v2: WasmDownloadRequest = WasmDownloadRequest::partial(
            BENCH_WASM_URL_V2.to_string(),
            BENCH_WASM_SLOT.to_string(),
            base_ver + 1,
        )
        .with_checksum(csum_v2)
        .with_allow_downgrade(true);
        if manager.load_and_mount(req_v1, BENCH_WASM_SIGN_KEY).is_ok()
            && manager.load_and_mount(req_v2, BENCH_WASM_SIGN_KEY).is_ok()
            && manager
                .rollback_partial(BENCH_WASM_SLOT, BENCH_WASM_ROLLBACK_REASON)
                .is_ok()
        {
            let _: Result<FrameStats, PlatformError> = app.step_frame();
            completed += 1;
        }
    }
    build_metric(
        BENCH_STRESS_WASM_LOAD_NAME,
        count,
        start.elapsed(),
        format!("cycles={completed} cache={}", manager.cache_len()),
    )
}

/// Stress-benchmarks flagship mobile `1440x3168 @ 4.0x` multi-card shadowed UI scrolling in the multi-core software rasterizer.
///
/// # Arguments
///
/// - `usize` - Number of full scrolled 4x-DPI frames to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured frame latency and FPS throughput at `1440x3168`.
#[must_use]
pub fn bench_stress_hidpi_4x_1440x3168(iterations: usize) -> BenchMetric {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 792).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let Ok(mut app) = App::headless(cfg) else {
        return build_metric(
            BENCH_STRESS_HIDPI_4X_NAME,
            count,
            Duration::ZERO,
            String::new(),
        );
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
                for idx in 0..16usize {
                    EuvCard(format!("{BENCH_TITLE} #{idx}"), tokens) {
                        subtitle: BENCH_CARD_CAPTION
                        EuvBadge(BENCH_BADGE.to_string(), EuvTone::Primary, tokens)
                        EuvProgress(((idx % 9) as f32 + 1.0) * 0.1, tokens)
                    }
                }
            }
        }
    });
    app.set_exact_physical_viewport(1440, 3168, 4.0);
    let _: Result<FrameStats, PlatformError> = app.step_frame();
    let start: Instant = Instant::now();
    let mut last_shaded: usize = 0;
    for step in 1..=count {
        let offset_y: f32 = ((step as f32) * 18.5) % 800.0;
        app.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.set_scroll_offset(scroll_id, Point::new(0.0, offset_y));
            },
            (),
        );
        if let Ok(stats) = app.step_frame() {
            last_shaded = stats.get_pixels_shaded();
        }
    }
    build_metric(
        BENCH_STRESS_HIDPI_4X_NAME,
        count,
        start.elapsed(),
        format!("res=1440x3168@4x shaded={last_shaded}"),
    )
}

/// Stress-benchmarks 240Hz touch-pan scroll physics, thumb-lift anti-reversal filtering, and 1,000-node scene hit-testing.
///
/// # Arguments
///
/// - `usize` - Number of 240Hz input/hit-test ticks to measure.
///
/// # Returns
///
/// - `BenchMetric` - Measured input + hit-test latency and throughput.
#[must_use]
pub fn bench_stress_scroll_physics_and_hittest(iterations: usize) -> BenchMetric {
    reset_reactive_runtime();
    let count: usize = iterations.max(1);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 792).set_render(RenderConfig::hardware());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let Ok(mut app) = App::headless(cfg) else {
        return build_metric(
            BENCH_STRESS_INPUT_HITTEST_NAME,
            count,
            Duration::ZERO,
            String::new(),
        );
    };
    let _: NodeId = app.mount(|| {
        view! {
            scroll {
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                gap: 4.0
                for idx in 0..1_000usize {
                    row {
                        width: Length::Percent(100.0)
                        height: Length::Px(32.0)
                        cursor: CursorIcon::Pointer
                        text {
                            { format!("{BENCH_ROW_LABEL} {idx}") }
                        }
                    }
                }
            }
        }
    });
    let wid: WindowId = app.get_window_state().get_id();
    let _: Result<FrameStats, PlatformError> = app.step_game_frame(1.0 / 120.0);
    let start: Instant = Instant::now();
    let mut hits: usize = 0;
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(1, 180.0, 620.0),
    ));
    for step in 0..count {
        let y: f32 = 620.0 - ((step % 48) as f32) * 8.0;
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::touch(1, 180.0, y),
        ));
        let found: bool = app.get_scene().with_read(
            |tree: &SceneTree| hit_test(tree, Point::new(180.0, 320.0)).is_some(),
            false,
        );
        if found {
            hits += 1;
        }
        if step % 2 == 0 {
            let _: Result<FrameStats, PlatformError> = app.step_game_frame(1.0 / 240.0);
        }
    }
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(1, 180.0, 240.0),
    ));
    build_metric(
        BENCH_STRESS_INPUT_HITTEST_NAME,
        count,
        start.elapsed(),
        format!("nodes=1000 hits={hits}"),
    )
}

/// Runs the complete reproducible benchmark and stress-test suite across reactivity, layout, virtual lists, WASM OTA, high-DPI rendering, input, and game physics.
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
        bench_unicode_glyph_atlas(100),
        bench_software_full_redraw(20),
        bench_software_rasterizer(20),
        bench_hardware_gpu_batching(200),
        bench_game_ecs_physics(200),
        bench_game_particles(1_000),
        bench_game3d_pbr_bvh_raytrace(10),
        bench_game3d_cinematic_max_rt(10),
        bench_stress_long_list_100k(30),
        bench_stress_complex_layout_2000(40),
        bench_stress_bulk_reactive_1000(20),
        bench_stress_wasm_dynamic_hotswap(30),
        bench_stress_hidpi_4x_1440x3168(15),
        bench_stress_scroll_physics_and_hittest(240),
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
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(800, 600).set_render(render);
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let Ok(mut app) = App::headless(cfg) else {
        return build_metric(name, count, Duration::ZERO, String::new());
    };
    let _: NodeId = app.mount(|| {
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_LG)
                gap: SPACE_MD
                EuvHeader(BENCH_TITLE.to_string(), BENCH_SUBTITLE.to_string(), tokens)
                EuvCard(BENCH_TITLE.to_string(), tokens) {
                    subtitle: BENCH_SUBTITLE
                    EuvBadge(BENCH_BADGE.to_string(), EuvTone::Primary, tokens)
                    EuvProgress(0.72, tokens)
                }
            }
        }
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
