use super::*;

/// Decides whether a viewport is laid out edge-to-edge on its physical screen.
///
/// `env(safe-area-inset-*)` is only meaningful when the page really does reach
/// under the system status bar and gesture bar. An Android browser that
/// letterboxes the page below the status bar still reports a non-zero bottom
/// inset, so consuming `env()` unconditionally renders that inset as a blank
/// band. A large `screen.height - innerHeight` gap is the runtime signature of
/// such a letterbox; an immersive WebView keeps the viewport flush with the
/// screen and reports a small gap.
///
/// An explicit immersive declaration always wins, because a host that knows it
/// is edge-to-edge stays trustworthy even when the measurement is unavailable.
///
/// # Arguments
///
/// - `bool` - Whether the host declares edge-to-edge (immersive) mode.
/// - `Option<f64>` - `screen.height - window.innerHeight` in CSS pixels,
///   or `None` when the measurement is unavailable.
///
/// # Returns
///
/// - `bool` - `true` when `env()` insets may be trusted for this viewport.
pub fn is_edge_to_edge_viewport(declared_immersive: bool, viewport_gap: Option<f64>) -> bool {
    if declared_immersive {
        return true;
    }
    let Some(gap) = viewport_gap else {
        return false;
    };
    gap < SAFE_AREA_LETTERBOX_TOLERANCE_PX
}

/// Picks the pixel value one side's contract variable carries for a given host.
///
/// Every side falls back to `0px`, so an untrusted or unmeasured inset can
/// never widen a shell edge: only a measured inset from a genuinely
/// edge-to-edge viewport is written back.
///
/// # Arguments
///
/// - `bool` - Whether the host is a genuinely edge-to-edge (immersive) viewport.
/// - `&str` - The measured `env(safe-area-inset-*)` pixel value, which may be
///   empty or malformed.
///
/// # Returns
///
/// - `String` - The measured pixel value when it is a usable length, and `0px`
///   otherwise.
pub fn safe_area_contract_value(is_trusted: bool, measured: &str) -> String {
    if !is_trusted {
        return String::from(SAFE_AREA_ZERO_VALUE);
    }
    let trimmed: &str = measured.trim();
    if !trimmed.ends_with(SAFE_AREA_PIXEL_UNIT) || trimmed == SAFE_AREA_ZERO_VALUE {
        return String::from(SAFE_AREA_ZERO_VALUE);
    }
    trimmed.to_string()
}

/// Measures `screen.height - window.innerHeight` for the current window.
///
/// # Returns
///
/// - `Option<f64>` - The vertical gap between the physical screen and the layout
///   viewport in CSS pixels, or `None` when either side cannot be read.
pub(crate) fn measure_viewport_screen_gap() -> Option<f64> {
    let window_value: Window = window()?;
    let inner_height: f64 = window_value
        .inner_height()
        .ok()
        .and_then(|height: JsValue| height.as_f64())?;
    let screen: Screen = window_value.screen().ok()?;
    let screen_height: f64 = f64::from(screen.height().ok()?);
    Some(screen_height - inner_height)
}
