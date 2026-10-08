use super::*;

/// Returns the current time in milliseconds.
///
/// Reads `Date.now()` through `js_sys` rather than the DOM high-resolution
/// clock, because the web-sys `Performance` feature is not enabled for this
/// crate. Millisecond resolution is sufficient here: the only threshold it
/// feeds is `long_press_millis`, whose default is two orders of magnitude
/// larger than the clock's granularity.
///
/// Off-wasm the JS runtime is absent, so the monotonic host clock is used
/// instead. That keeps the state machine testable on the host, where a real
/// `touchstart` can never occur anyway.
///
/// # Returns
///
/// - `f64` - Milliseconds since the Unix epoch, or `0.0` if the clock is
///   unavailable.
pub(crate) fn now_millis() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0.0, |elapsed: Duration| elapsed.as_secs_f64() * 1000.0)
    }
}
