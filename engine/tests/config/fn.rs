use super::*;

fn close(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}

#[test]
fn each_backend_constructor_picks_its_backend_and_keeps_the_canvas_settings() {
    for (config, expected) in [
        (
            RenderConfig::canvas2d("#app", 800.0, 600.0),
            RenderBackendType::Canvas2D,
        ),
        (
            RenderConfig::webgpu("#app", 800.0, 600.0),
            RenderBackendType::WebGpu,
        ),
        (
            RenderConfig::webgl("#app", 800.0, 600.0),
            RenderBackendType::WebGl,
        ),
    ] {
        assert_eq!(
            config.get_backend(),
            expected,
            "the backend must match the constructor"
        );
        assert_eq!(
            config.get_canvas_selector(),
            "#app",
            "the selector round-trips"
        );
        assert!(close(config.get_width(), 800.0), "the width round-trips");
        assert!(close(config.get_height(), 600.0), "the height round-trips");
    }
}

#[test]
fn a_render_config_uses_the_documented_quality_defaults() {
    let config: RenderConfig = RenderConfig::webgpu("#app", 100.0, 100.0);
    assert_eq!(
        config.get_quality(),
        RenderQuality::High,
        "the default quality is high"
    );
    assert!(config.get_antialias(), "antialiasing defaults on");
    assert_eq!(
        config.get_power_preference(),
        GpuPowerPreference::LowPower,
        "the default power preference is LowPower, not HighPerformance"
    );
}

#[test]
fn the_gpu_power_preference_converts_to_a_web_sys_string() {
    assert_eq!(
        GpuPowerPreference::HighPerformance.to_web_sys_string(),
        "high-performance",
        "the camelCase GpuPowerPreference maps to the kebab-case GPU enum"
    );
    assert_eq!(
        GpuPowerPreference::LowPower.to_web_sys_string(),
        "low-power",
        "and so does the low-power variant"
    );
}

#[test]
fn engine_config_wraps_a_render_config_and_accepts_a_scheduler_override() {
    let render: RenderConfig = RenderConfig::canvas2d("#root", 320.0, 240.0);
    let base: EngineConfig = EngineConfig::create(render);
    assert_eq!(
        base.get_render().get_canvas_selector(),
        "#root",
        "the render config is carried through"
    );
    let custom: SchedulerConfig = SchedulerConfig::new(0.125, 0.25);
    let tuned: EngineConfig = base.with_scheduler(custom);
    assert!(
        close(tuned.get_scheduler().get_fixed_timestep(), 0.125),
        "the timestep override stuck, got {}",
        tuned.get_scheduler().get_fixed_timestep()
    );
    assert!(
        close(tuned.get_scheduler().get_max_frame_time(), 0.25),
        "so did the frame clamp"
    );
    assert!(
        tuned.get_render().get_canvas_selector() == "#root",
        "and the render config is untouched by the scheduler override"
    );
}

#[test]
fn the_default_engine_config_is_the_canvas_backend_at_sixty_hertz() {
    let config: EngineConfig = EngineConfig::default();
    assert_eq!(
        config.get_render().get_backend(),
        RenderBackendType::Canvas2D,
        "Canvas2D is the default backend"
    );
    assert!(
        close(config.get_scheduler().get_fixed_timestep(), 1.0 / 60.0),
        "the default timestep is 60 Hz, got {}",
        config.get_scheduler().get_fixed_timestep()
    );
}

#[test]
fn the_scheduler_config_reports_its_two_knobs_independently() {
    let config: SchedulerConfig = SchedulerConfig::new(0.02, 0.1);
    assert!(
        close(config.get_fixed_timestep(), 0.02),
        "the timestep round-trips"
    );
    assert!(
        close(config.get_max_frame_time(), 0.1),
        "the clamp round-trips"
    );
    let defaults: SchedulerConfig = SchedulerConfig::default();
    assert!(
        defaults.get_fixed_timestep() > 0.0,
        "a default timestep must be positive"
    );
    assert!(
        defaults.get_max_frame_time() >= defaults.get_fixed_timestep(),
        "the frame clamp must be at least one step, or frames would never advance"
    );
}

#[test]
fn current_time_is_zero_off_the_browser() {
    let now: f64 = SchedulerState::current_time();
    assert!(
        close(now, 0.0),
        "there is no performance.now() outside a browser, so the clock reports zero, got {now}"
    );
}

#[test]
fn a_fresh_scheduler_state_is_uninitialised_and_stopped() {
    let state: SchedulerState = SchedulerState::default();
    assert!(!state.get_running(), "a fresh scheduler is not running");
    assert!(
        close(state.get_accumulator(), 0.0),
        "no time has accumulated yet"
    );
    assert!(
        close(state.get_update_count() as f64, 0.0),
        "no updates have run"
    );
    assert!(
        close(state.get_frame_count() as f64, 0.0),
        "no frames have run"
    );
    assert!(
        state.get_raf_id().is_none(),
        "no animation frame is pending"
    );
}

#[test]
fn a_fresh_task_registry_is_empty() {
    let mut registry: TaskRegistry = TaskRegistry::default();
    assert!(registry.is_empty(), "a fresh registry holds no tasks");
    assert!(
        close(registry.len() as f64, 0.0),
        "and reports a length of zero"
    );
    registry.update_all(0.1);
    assert!(
        registry.is_empty(),
        "advancing an empty registry keeps it empty"
    );
}

#[test]
fn a_fresh_asset_cache_reports_nothing_loaded() {
    let cache: AssetCache = AssetCache::default();
    assert!(
        cache.get_state("missing.png").is_none(),
        "an unknown url has no state"
    );
    assert!(
        cache.get_image("missing.png").is_none(),
        "and no image handle"
    );
    assert!(close(cache.loaded_count() as f64, 0.0), "nothing is loaded");
    assert!(
        cache.is_all_loaded(),
        "an empty cache is vacuously all loaded"
    );
    assert!(
        cache.is_all_loaded(),
        "and stays vacuously complete with nothing in flight"
    );
}

#[test]
fn the_default_asset_state_is_loading() {
    assert_eq!(
        AssetState::default(),
        AssetState::Loading,
        "an entry starts out loading until a callback settles it"
    );
    let loading: AssetState = AssetState::Loading;
    let loaded: AssetState = AssetState::Loaded;
    let errored: AssetState = AssetState::Error;
    assert!(loading != loaded, "the three states are distinct");
    assert!(loaded != errored, "including loaded versus errored");
}
