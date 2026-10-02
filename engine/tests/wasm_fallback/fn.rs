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
