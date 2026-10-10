use super::*;

/// Decides whether this host is one where `env(safe-area-inset-*)` describes a
/// real screen feature worth reserving space for.
///
/// The inset only means something where the OS draws a notch, a rounded corner
/// or a home indicator over the page. iOS is the platform that always has one
/// (and reports it in every browser, Safari or not). Desktop Safari has one on
/// notched Macs, and is the only desktop engine that reports the variable at
/// all. Every other platform — Android, Windows, Linux desktop — either has no
/// inset to report or lies about it, which is precisely the blank band the
/// contract variables were introduced to avoid.
///
/// Detection reads `navigator.userAgent` rather than `navigator.platform`,
/// because `platform` reports `MacIntel` for iPadOS and cannot separate it
/// from a real Mac, and because the value is available synchronously where a
/// `userAgentData` query would not be.
///
/// # Arguments
///
/// - `&str` - The `navigator.userAgent` string of the current host.
///
/// # Returns
///
/// - `bool` - `true` when the host is iOS-family or desktop Safari.
pub fn needs_safe_area_insets(user_agent: &str) -> bool {
    let haystack: String = user_agent.to_lowercase();
    if haystack.contains("iphone") || haystack.contains("ipad") || haystack.contains("ipod") {
        return true;
    }
    if !haystack.contains(USER_AGENT_SAFARI_DESKTOP_TOKEN) {
        return false;
    }
    !has_any_token(&haystack, USER_AGENT_NON_SAFARI_TOKENS)
}

/// Returns whether any of the comma-separated tokens in `tokens` appears in
/// `haystack`, which the caller has already lowercased.
///
/// The tokens are a comma-separated list rather than one regex-style string
/// because `str::contains` matches substrings: a `"chrome|firefox"` literal
/// would never be found in a user agent.
///
/// # Arguments
///
/// - `&str` - The lowercased user agent to search.
/// - `&str` - The comma-separated lowercase tokens to look for.
///
/// # Returns
///
/// - `bool` - `true` when at least one token occurs in the haystack.
fn has_any_token(haystack: &str, tokens: &str) -> bool {
    tokens
        .split(',')
        .any(|token: &str| haystack.contains(token))
}

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
