use super::*;

#[test]
fn canvas_2d_selects_the_canvas_backend() {
    let config: RenderConfig = RenderConfig::canvas2d("#stage", 320.0, 240.0);
    assert_eq!(
        config.get_backend(),
        RenderBackendType::Canvas2D,
        "the canvas2d constructor must select the Canvas2D backend"
    );
}

#[test]
fn webgpu_selects_the_webgpu_backend() {
    let config: RenderConfig = RenderConfig::webgpu("#stage", 320.0, 240.0);
    assert_eq!(
        config.get_backend(),
        RenderBackendType::WebGpu,
        "the webgpu constructor must select the WebGpu backend"
    );
}

#[test]
fn webgl_selects_the_webgl_backend() {
    let config: RenderConfig = RenderConfig::webgl("#stage", 320.0, 240.0);
    assert_eq!(
        config.get_backend(),
        RenderBackendType::WebGl,
        "the webgl constructor must select the WebGl backend"
    );
}

#[test]
fn render_config_keeps_the_selector_and_viewport_it_was_given() {
    let config: RenderConfig = RenderConfig::canvas2d("#game-root", 1024.0, 768.0);
    assert_eq!(
        config.get_canvas_selector(),
        "#game-root",
        "the selector is stored verbatim"
    );
    assert_eq!(config.get_width(), 1024.0, "the width is stored verbatim");
    assert_eq!(config.get_height(), 768.0, "the height is stored verbatim");
}

#[test]
fn render_config_accepts_an_owned_string_selector() {
    let selector: String = "#owned".to_string();
    let config: RenderConfig = RenderConfig::canvas2d(selector, 1.0, 1.0);
    assert_eq!(
        config.get_canvas_selector(),
        "#owned",
        "the generic selector parameter accepts an owned String"
    );
}

#[test]
fn render_config_falls_back_to_the_documented_defaults_for_the_other_fields() {
    let config: RenderConfig = RenderConfig::canvas2d("#stage", 320.0, 240.0);
    assert_eq!(
        config.get_ssaa_scale_factor(),
        2.0,
        "the SSAA scale falls back to the default supersampling factor"
    );
    assert!(config.get_antialias(), "anti-aliasing defaults to enabled");
}

#[test]
fn render_backend_default_is_canvas2d() {
    let observed: RenderBackendType = RenderBackendType::default();
    assert_eq!(
        observed,
        RenderBackendType::Canvas2D,
        "Canvas2D is the universally supported default"
    );
}

#[test]
fn engine_config_create_keeps_the_render_config_it_was_given() {
    let render: RenderConfig = RenderConfig::webgpu("#gpu", 800.0, 600.0);
    let config: EngineConfig = EngineConfig::create(render.clone());
    assert_eq!(
        config.get_render().get_backend(),
        RenderBackendType::WebGpu,
        "create must not rewrite the render backend"
    );
    assert_eq!(
        config.get_render().get_width(),
        render.get_width(),
        "create must not rewrite the viewport"
    );
}

#[test]
fn engine_config_create_installs_the_default_scheduler() {
    let render: RenderConfig = RenderConfig::canvas2d("#stage", 320.0, 240.0);
    let config: EngineConfig = EngineConfig::create(render);
    assert_eq!(
        config.get_scheduler(),
        SchedulerConfig::default(),
        "create installs the default scheduler"
    );
}

#[test]
fn with_scheduler_replaces_the_default_scheduler() {
    let render: RenderConfig = RenderConfig::canvas2d("#stage", 320.0, 240.0);
    let base: EngineConfig = EngineConfig::create(render);
    let replacement: SchedulerConfig = SchedulerConfig::new(1.0 / 120.0, 0.25);
    let updated: EngineConfig = base.with_scheduler(replacement);
    assert_eq!(
        updated.get_scheduler(),
        replacement,
        "with_scheduler must install the supplied scheduler"
    );
}

#[test]
fn with_scheduler_leaves_the_render_config_untouched() {
    let render: RenderConfig = RenderConfig::webgl("#gl", 640.0, 480.0);
    let base: EngineConfig = EngineConfig::create(render);
    let updated: EngineConfig = base.with_scheduler(SchedulerConfig::new(0.5, 1.0));
    assert_eq!(
        updated.get_render().get_backend(),
        RenderBackendType::WebGl,
        "the render config survives a scheduler swap"
    );
    assert_eq!(
        updated.get_render().get_canvas_selector(),
        "#gl",
        "the canvas selector survives a scheduler swap"
    );
}

#[test]
fn gpu_power_preference_default_is_low_power() {
    let observed: GpuPowerPreference = GpuPowerPreference::default();
    assert_eq!(
        observed,
        GpuPowerPreference::LowPower,
        "the derive default is the battery-friendly adapter hint"
    );
}

#[test]
fn low_power_preference_renders_the_web_spec_string() {
    let observed: &str = GpuPowerPreference::LowPower.to_web_sys_string();
    assert_eq!(
        observed, "low-power",
        "the WebGPU spec spells the low-power hint with a hyphen"
    );
}

#[test]
fn high_performance_preference_renders_the_web_spec_string() {
    let observed: &str = GpuPowerPreference::HighPerformance.to_web_sys_string();
    assert_eq!(
        observed, "high-performance",
        "the WebGPU spec spells the high-performance hint with a hyphen"
    );
}

#[test]
fn the_two_power_preferences_render_different_strings() {
    let low: &str = GpuPowerPreference::LowPower.to_web_sys_string();
    let high: &str = GpuPowerPreference::HighPerformance.to_web_sys_string();
    assert_ne!(
        low, high,
        "the two adapter hints must not collapse onto one string"
    );
}
