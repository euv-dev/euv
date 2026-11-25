/// CLI flag to emit JSON benchmark report output.
pub const BENCH_FLAG_JSON: &str = "--json";

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
pub(crate) const BENCH_ROW_LABEL: &str = "Benchmark Item";
pub(crate) const BENCH_TITLE: &str = "Benchmark Surface";
pub(crate) const BENCH_SUBTITLE: &str = "Multi-mode rendering telemetry";
pub(crate) const BENCH_BADGE: &str = "PERF";
pub(crate) const REPORT_HEADER: &str = "euv 0.38 Cross-Platform Native Benchmark Suite\n-------------------------------------------------------------------------------\n";
