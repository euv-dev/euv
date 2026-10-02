#[cfg(target_arch = "wasm32")]
use super::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn device_pixel_ratio_falls_back_to_one_without_a_window() {
    let ratio: f64 = CanvasRenderer::detect_dpr();
    assert_eq!(
        ratio, 1.0,
        "a host with no window must yield the documented default, not a panic"
    );
    assert!(ratio.is_finite() && ratio >= 1.0, "the default itself is clamped");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn the_device_pixel_ratio_fallback_is_stable_across_calls() {
    let first: f64 = CanvasRenderer::detect_dpr();
    let second: f64 = CanvasRenderer::detect_dpr();
    assert_eq!(
        first, second,
        "the fallback must not depend on a partially initialised cache"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn the_current_time_falls_back_to_zero_without_a_window() {
    let time: f64 = SchedulerState::current_time();
    assert_eq!(
        time, 0.0,
        "performance.now is unreachable, so the documented 0.0 stands in"
    );
    assert!(time.is_finite());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn the_webgl2_capability_probe_reports_unavailable_without_a_window() {
    assert!(
        !WebGl2Backend::is_available(),
        "node exposes no window, so the probe must say no rather than panic"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn initialising_a_webgl2_backend_without_a_window_reports_the_selector_it_could_not_find() {
    let config: RenderConfig = RenderConfig::webgl("#stage", 800.0, 600.0);
    let result: Result<WebGl2Backend, WebGl2InitError> = WebGl2Backend::init(&config);
    assert_eq!(
        result.err(),
        Some(WebGl2InitError::CanvasNotFound("#stage".to_string())),
        "the selector must be carried into the error so the caller can diagnose it"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn the_webgpu_capability_probe_reports_unavailable_without_a_window() {
    assert!(
        !WebGpuRenderer::is_available(),
        "navigator.gpu is unreachable, so the probe must say no"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn initialising_a_webgpu_renderer_without_a_window_reports_a_missing_navigator_gpu() {
    let config: RenderConfig = RenderConfig::webgpu("#stage", 800.0, 600.0);
    let result: Result<WebGpuRenderer, WebGpuInitError> = WebGpuRenderer::init(&config).await;
    assert!(
        matches!(result, Err(WebGpuInitError::NavigatorGpuMissing)),
        "the error type carries a JsValue, so it is matched by shape rather than compared"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn building_a_canvas_renderer_from_a_selector_finds_nothing_without_a_document() {
    let renderer: Option<CanvasRenderer> = CanvasRenderer::from_selector("#stage", 800.0, 600.0);
    assert!(
        renderer.is_none(),
        "a host with no document cannot satisfy a selector lookup"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn building_a_multisampled_canvas_from_a_selector_finds_nothing_without_a_document() {
    let canvas: Option<SsaaCanvas> =
        SsaaCanvas::from_selector_with_scale("#stage", 800.0, 600.0, 2.0);
    assert!(canvas.is_none());
    let bare: Option<SsaaCanvas> = SsaaCanvas::from_selector("#stage", 800.0, 600.0);
    assert!(
        bare.is_none(),
        "the unscaled constructor shares the same lookup, so it must fail the same way"
    );
}
