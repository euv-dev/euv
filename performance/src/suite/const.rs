/// CLI flag to emit JSON benchmark report output.
pub const BENCH_FLAG_JSON: &str = "--json";
/// CLI flag to filter benchmarks by substring match.
pub const BENCH_FLAG_FILTER: &str = "--filter";

pub(crate) const BENCH_REACTIVE_NAME: &str = "reactive_signal_memo_diamond";
pub(crate) const BENCH_SELECTOR_NAME: &str = "reactive_selector_o1_switch";
pub(crate) const BENCH_LAYOUT_CACHE_NAME: &str = "layout_clean_subtree_cache";
pub(crate) const BENCH_SW_RASTER_NAME: &str = "render_software_sdf_800x600";
pub(crate) const BENCH_SW_FULL_NAME: &str = "render_software_full_redraw";
pub(crate) const BENCH_HW_BATCH_NAME: &str = "render_hardware_gpu_batching";
pub(crate) const BENCH_GLYPH_ATLAS_NAME: &str = "font_unicode_cjk_atlas_pack";
pub(crate) const BENCH_GAME_PHYSICS_NAME: &str = "game_ecs_physics_256_bodies";
pub(crate) const BENCH_GAME_PARTICLES_NAME: &str = "game_particles_1024_pool";
pub(crate) const BENCH_GAME_3D_RT_NAME: &str = "game3d_pbr_bvh_raytrace";
pub(crate) const BENCH_GAME_3D_MAX_RT_NAME: &str = "game3d_cinematic_max_rt_125pct";
pub(crate) const BENCH_STRESS_LONG_LIST_NAME: &str = "stress_long_list_virtual_100k";
pub(crate) const BENCH_STRESS_COMPLEX_LAYOUT_NAME: &str = "stress_complex_layout_2000_nodes";
pub(crate) const BENCH_STRESS_BULK_REACTIVE_NAME: &str = "stress_bulk_reactive_1000_nodes";
pub(crate) const BENCH_STRESS_WASM_LOAD_NAME: &str = "stress_wasm_dynamic_load_hotswap";
pub(crate) const BENCH_STRESS_HIDPI_4X_NAME: &str = "stress_hidpi_4x_1440x3168_scroll";
pub(crate) const BENCH_STRESS_INPUT_HITTEST_NAME: &str = "stress_240hz_scroll_and_hittest";
pub(crate) const BENCH_SW_ROUNDED_CLIP_NAME: &str = "render_software_rounded_clip_cards";
pub(crate) const BENCH_SW_PRIMITIVES_NAME: &str = "render_software_primitives_image_vec";
pub(crate) const BENCH_SW_PORTAL_ANIM_NAME: &str = "render_software_portal_overlay_anim";
pub(crate) const BENCH_HW_ACTIVE_BATCH_NAME: &str = "render_hardware_active_batching";
pub(crate) const BENCH_FONT_WRAP_CACHE_NAME: &str = "font_text_wrap_measure_cache";
pub(crate) const BENCH_APP_SMOOTH_SCROLL_NAME: &str = "app_smooth_scroll_fractional";
pub(crate) const BENCH_UI_MARKDOWN_MOUNT_NAME: &str = "ui_markdown_codeblock_mount";
pub(crate) const BENCH_UI_VIEW_MOUNT_500_NAME: &str = "ui_view_macro_mount_500_nodes";
pub(crate) const BENCH_UI_ROUTER_FREEZE_NAME: &str = "ui_router_freeze_switch";
pub(crate) const BENCH_UI_DATATABLE_NAME: &str = "ui_datatable_filter_select";
pub(crate) const BENCH_ROW_LABEL: &str = "Benchmark Item";
pub(crate) const BENCH_TITLE: &str = "Benchmark Surface";
pub(crate) const BENCH_SUBTITLE: &str = "Multi-mode rendering telemetry";
pub(crate) const BENCH_BADGE: &str = "PERF";
pub(crate) const BENCH_WASM_SLOT: &str = "bench_wasm_slot";
pub(crate) const BENCH_WASM_URL_V1: &str = "euv-cdn://bench/widget-v1.wasm";
pub(crate) const BENCH_WASM_URL_V2: &str = "euv-cdn://bench/widget-v2.wasm";
pub(crate) const BENCH_WASM_ETAG_V1: &str = "bench-v1";
pub(crate) const BENCH_WASM_ETAG_V2: &str = "bench-v2";
pub(crate) const BENCH_WASM_TITLE_V1: &str = "Bench Dynamic WASM Widget v1";
pub(crate) const BENCH_WASM_TITLE_V2: &str = "Bench Dynamic WASM Widget v2";
pub(crate) const BENCH_WASM_SIGN_KEY: &str = "bench-signing-key";
pub(crate) const BENCH_WASM_ROLLBACK_REASON: &str = "bench-cycle";
pub(crate) const BENCH_CELL_LABEL: &str = "Cell";
pub(crate) const BENCH_CARD_CAPTION: &str = "Multi-DPI lossless shadow card";
pub(crate) const BENCH_CARD_WRAP_COPY: &str = "Rounded clipped surface validating 4x4 RGSS corner masks, crisp border strokes, shadow falloff, and deterministic multi-line wrapping.";
pub(crate) const BENCH_DRAWER_TITLE: &str = "Overlay Telemetry Drawer";
pub(crate) const BENCH_DRAWER_BODY: &str = "Portal slide animation exercising cached backdrop restoration and dirty-band software raster dispatch.";
pub(crate) const BENCH_PARA_MIXED_A: &str = "Cross-platform native Rust rendering engine 跨端原生渲染引擎 with deterministic subpixel layout and zero WebView overhead across desktop and mobile viewports.";
pub(crate) const BENCH_PARA_MIXED_B: &str = "Reactive arena scheduling 响应式内存池调度 with O(1) clean-subtree skipping, BVH ray-traced lighting, and Eva Bold syntax highlighting.";
pub(crate) const BENCH_PARA_SINGLE: &str = "Single-line fast-path header telemetry 240Hz";
pub(crate) const BENCH_BTN_LABEL: &str = "Inspect";
pub(crate) const BENCH_TAG_LABEL: &str = "Native";
pub(crate) const BENCH_STAT_LABEL: &str = "Throughput";
pub(crate) const BENCH_STAT_CAPTION: &str = "Zero-VDOM retained pipeline";
pub(crate) const BENCH_FALLBACK_LABEL: &str = "Not Found";
pub(crate) const BENCH_ROUTE_PATHS: [&str; 8] = [
    "/overview",
    "/render",
    "/layout",
    "/reactive",
    "/widgets",
    "/markdown",
    "/media",
    "/settings",
];
pub(crate) const BENCH_ROUTE_LABELS: [&str; 8] = [
    "Overview", "Renderer", "Layout", "Reactive", "Widgets", "Markdown", "Media", "Settings",
];
pub(crate) const BENCH_TABLE_HEADERS: [&str; 5] = ["ID", "Service", "Team", "Latency", "Status"];
pub(crate) const BENCH_TABLE_FILTERS: [&str; 4] = ["", "Alpha", "Beta", "04"];
pub(crate) const BENCH_TABLE_TEAMS: [&str; 4] = ["Alpha", "Beta", "Gamma", "Delta"];
pub(crate) const BENCH_TABLE_STATUSES: [&str; 4] = ["Healthy", "Active", "Cached", "Optimal"];
pub(crate) const BENCH_MARKDOWN_DOC: &str = r##"# Euv Performance Benchmark Markdown Document

This document exercises **GFM parsing**, *inline emphasis*, ~~legacy pipelines~~, `inline_code_tokens`, and [Native Documentation](https://euv.ltpp.vip/docs/guide).

## Subsystem Matrix

| Subsystem | Strategy | Complexity | Target |
| :--- | :--- | :---: | ---: |
| Reactive Arena | Generation-indexed slots | O(1) | 240Hz |
| Layout Solver | Clean-subtree cache | O(dirty) | 120Hz |
| Software Raster | Multi-band SDF tiles | O(pixels) | 4x DPI |
| Hardware Batch | Instanced GPU quads | O(batches) | Metal |
| Glyph Atlas | Shelf-packed RGBA8 | O(1) hit | CJK |
| Router Outlet | Freeze retention scope | O(1) swap | Zero-GC |

> [!NOTE]
> Retained scene graphs skip layout on paint-only mutations and restore cached portal backdrops across overlay animations.

### Verification Checklist

- [x] Generation-indexed reactive signals and memos
- [x] Clean-subtree flexbox and grid caching
- [x] Zero-overshoot monotonic drawer easing
- [ ] Complete 100-round optimization ledger

## Reference Rust Implementation

```rust
pub struct RenderTelemetry {
    pub frame_index: u64,
    pub draw_batches: usize,
    pub pixels_shaded: usize,
    pub elapsed_micros: u64,
}

impl RenderTelemetry {
    pub fn new(frame_index: u64) -> Self {
        Self {
            frame_index,
            draw_batches: 0,
            pixels_shaded: 0,
            elapsed_micros: 0,
        }
    }

    pub fn record_pass(&mut self, batches: usize, shaded: usize, micros: u64) {
        let Self {
            draw_batches,
            pixels_shaded,
            elapsed_micros,
            ..
        } = self;
        *draw_batches = draw_batches.saturating_add(batches);
        *pixels_shaded = pixels_shaded.saturating_add(shaded);
        *elapsed_micros = elapsed_micros.saturating_add(micros);
    }

    pub fn pixels_per_micro(&self) -> f64 {
        let Self {
            pixels_shaded,
            elapsed_micros,
            ..
        } = *self;
        let denom: f64 = elapsed_micros.max(1) as f64;
        (pixels_shaded as f64) / denom
    }
}

pub fn evaluate_budget(frames: &[RenderTelemetry], budget_us: u64) -> usize {
    let mut within_budget: usize = 0;
    let mut peak_micros: u64 = 0;
    let mut total_batches: usize = 0;
    for sample in frames.iter() {
        let cur: u64 = sample.elapsed_micros;
        if cur <= budget_us {
            within_budget += 1;
        }
        if cur > peak_micros {
            peak_micros = cur;
        }
        total_batches += sample.draw_batches;
    }
    let avg_batches: usize = if frames.is_empty() {
        0
    } else {
        total_batches / frames.len()
    };
    if peak_micros == 0 && avg_batches == 0 {
        0
    } else {
        within_budget
    }
}

pub fn synthesize_samples(count: usize) -> Vec<RenderTelemetry> {
    let mut out: Vec<RenderTelemetry> = Vec::with_capacity(count);
    for idx in 0..count {
        let mut item: RenderTelemetry = RenderTelemetry::new(idx as u64);
        item.record_pass((idx % 12) + 4, 480_000, 1_850 + ((idx as u64) % 120));
        out.push(item);
    }
    out
}
```
"##;
pub(crate) const REPORT_HEADER: &str = "euv Cross-Platform Native Benchmark & Stress Suite\n-------------------------------------------------------------------------------\n";
