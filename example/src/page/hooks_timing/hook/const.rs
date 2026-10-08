/// Placeholder text shared by the live-input boxes.
pub(crate) const TIMING_INPUT_PLACEHOLDER: &str = "Type here…";

/// Quiet period for the debounce row (ms).
pub(crate) const TIMING_DEBOUNCE_MS: u32 = 300;

/// Throttle window for the throttle row (ms).
pub(crate) const TIMING_THROTTLE_MS: u32 = 250;

/// Interval at which the ticks the `App::use_interval` driver
/// pushes the throttle / debounce state machine forward.
pub(crate) const TIMING_TICK_MS: i32 = 50;

/// DOM id for the debounce row's input.
pub(crate) const TIMING_DEBOUNCE_INPUT_ID: &str = "timing-debounce-input";

/// DOM id for the throttle row's input.
pub(crate) const TIMING_THROTTLE_INPUT_ID: &str = "timing-throttle-input";
