use super::*;

/// Sanitizes a single safe-area inset scalar, clamping NaN, infinite, or negative values to `0.0`.
///
/// # Arguments
///
/// - `f32` - Candidate inset value in logical pixels.
///
/// # Returns
///
/// - `f32` - Non-negative finite inset value.
#[must_use]
fn sanitize_inset_dp(value: f32) -> f32 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}

/// Selects the highest-refresh `DisplayModeInfo` matching `current`'s physical resolution without imposing a 60Hz cap.
///
/// # Arguments
///
/// - `DisplayModeInfo` - Currently active hardware display mode.
/// - `&[DisplayModeInfo]` - All display modes reported by the OS (`Display.getSupportedModes()`).
///
/// # Returns
///
/// - `DisplayModeInfo` - Preferred mode carrying the highest refresh rate at the native resolution.
#[must_use]
pub fn select_highest_refresh_mode(
    current: DisplayModeInfo,
    supported: &[DisplayModeInfo],
) -> DisplayModeInfo {
    let mut best: DisplayModeInfo = current;
    let target_w: u32 = current.get_physical_width();
    let target_h: u32 = current.get_physical_height();
    let mut best_hz: f32 = sanitize_inset_dp(best.get_refresh_rate_hz());
    for mode in supported.iter().copied() {
        let same_size: bool =
            mode.get_physical_width() == target_w && mode.get_physical_height() == target_h;
        let hz: f32 = sanitize_inset_dp(mode.get_refresh_rate_hz());
        if same_size && hz > best_hz {
            best = mode;
            best_hz = hz;
        }
    }
    if best_hz < MIN_REFRESH_RATE_HZ {
        for mode in supported.iter().copied() {
            let hz: f32 = sanitize_inset_dp(mode.get_refresh_rate_hz());
            if hz > best_hz {
                best = mode;
                best_hz = hz;
            }
        }
    }
    best
}

/// Computes high-refresh VSync pacing intervals (`60`/`90`/`120`/`144`/`240` Hz+) with zero 60Hz throttling.
///
/// # Arguments
///
/// - `f32` - Detected hardware display refresh rate in Hertz.
///
/// # Returns
///
/// - `FramePacingConfig` - Normalized adaptive refresh range and frame duration math.
#[must_use]
pub fn compute_frame_pacing(refresh_rate_hz: f32) -> FramePacingConfig {
    let preferred_hz: f32 = if refresh_rate_hz.is_finite() && refresh_rate_hz >= MIN_REFRESH_RATE_HZ
    {
        refresh_rate_hz.clamp(MIN_REFRESH_RATE_HZ, MAX_REFRESH_RATE_HZ)
    } else {
        FALLBACK_REFRESH_RATE_HZ
    };
    let min_hz: f32 = if preferred_hz >= 120.0 {
        PRO_MOTION_MIN_ACTIVE_HZ
    } else {
        preferred_hz.min(FALLBACK_REFRESH_RATE_HZ)
    };
    let frame_interval_secs: f32 = 1.0 / preferred_hz;
    let frame_interval_nanos: u64 = (1_000_000_000.0 / f64::from(preferred_hz)).round() as u64;
    FramePacingConfig::new(
        min_hz,
        preferred_hz,
        preferred_hz,
        frame_interval_secs,
        frame_interval_nanos,
    )
}

/// Resolves safe-area insets in logical dp, guaranteeing cold-start and Atomic Island / Dynamic Island clearance when not fullscreen.
///
/// # Arguments
///
/// - `SafeAreaInsets` - Live visible window insets reported by the OS.
/// - `SafeAreaInsets` - Visibility-ignoring system bar and cutout insets (`getInsetsIgnoringVisibility`).
/// - `f32` - Maximum top cutout / island bounding-rect bottom edge in logical dp.
/// - `f32` - Cold-start fallback status-bar height in logical dp (`status_bar_height` dimen or `statusBarFrame`).
/// - `bool` - Whether true edge-to-edge immersive fullscreen mode is currently active.
///
/// # Returns
///
/// - `SafeAreaInsets` - Sanitized safe-area insets (`0` on all edges in fullscreen; cleared top/cutout otherwise).
#[must_use]
pub fn resolve_mobile_safe_area(
    visible: SafeAreaInsets,
    ignoring_visibility: SafeAreaInsets,
    cutout_bounding_top_dp: f32,
    fallback_status_bar_dp: f32,
    immersive_fullscreen: bool,
) -> SafeAreaInsets {
    if immersive_fullscreen {
        return SafeAreaInsets::default();
    }
    let top: f32 = sanitize_inset_dp(visible.get_top())
        .max(sanitize_inset_dp(ignoring_visibility.get_top()))
        .max(sanitize_inset_dp(cutout_bounding_top_dp))
        .max(sanitize_inset_dp(fallback_status_bar_dp));
    let right: f32 = sanitize_inset_dp(visible.get_right())
        .max(sanitize_inset_dp(ignoring_visibility.get_right()));
    let bottom: f32 = sanitize_inset_dp(visible.get_bottom())
        .max(sanitize_inset_dp(ignoring_visibility.get_bottom()));
    let left: f32 = sanitize_inset_dp(visible.get_left())
        .max(sanitize_inset_dp(ignoring_visibility.get_left()));
    SafeAreaInsets::new(top, right, bottom, left)
}

/// Converts a non-negative physical pixel count to logical dp using display density `scale`.
///
/// # Arguments
///
/// - `i32` - Physical pixel measurement.
/// - `f32` - Sanitized display density scale (`>= 0.75`).
///
/// # Returns
///
/// - `f32` - Logical dp value.
#[must_use]
fn px_to_dp(px: i32, scale: f32) -> f32 {
    (px.max(0) as f32) / scale
}

/// Converts raw Android physical-pixel window insets, cutout rects, and status-bar fallback into logical `SafeAreaInsets`.
///
/// # Arguments
///
/// - `[i32; 4]` - Visible insets `[top, right, bottom, left]` in physical pixels.
/// - `[i32; 4]` - Ignoring-visibility insets `[top, right, bottom, left]` in physical pixels.
/// - `i32` - Maximum top display-cutout / Atomic Island bounding-rect bottom edge in physical pixels.
/// - `i32` - Fallback Android `status_bar_height` system dimen in physical pixels.
/// - `f32` - Active display density scale (`displayMetrics.density`).
/// - `bool` - Whether immersive fullscreen mode is active.
///
/// # Returns
///
/// - `SafeAreaInsets` - Resolved logical-dp safe-area insets.
#[must_use]
pub fn resolve_safe_area_from_px(
    visible_px: [i32; 4],
    ignoring_visibility_px: [i32; 4],
    cutout_max_bottom_px: i32,
    fallback_status_bar_px: i32,
    density: f32,
    immersive_fullscreen: bool,
) -> SafeAreaInsets {
    let scale: f32 = if density.is_finite() && density >= 0.75 {
        density
    } else {
        1.0
    };
    let visible: SafeAreaInsets = SafeAreaInsets::new(
        px_to_dp(visible_px[0], scale),
        px_to_dp(visible_px[1], scale),
        px_to_dp(visible_px[2], scale),
        px_to_dp(visible_px[3], scale),
    );
    let ignoring: SafeAreaInsets = SafeAreaInsets::new(
        px_to_dp(ignoring_visibility_px[0], scale),
        px_to_dp(ignoring_visibility_px[1], scale),
        px_to_dp(ignoring_visibility_px[2], scale),
        px_to_dp(ignoring_visibility_px[3], scale),
    );
    resolve_mobile_safe_area(
        visible,
        ignoring,
        px_to_dp(cutout_max_bottom_px, scale),
        px_to_dp(fallback_status_bar_px, scale),
        immersive_fullscreen,
    )
}
