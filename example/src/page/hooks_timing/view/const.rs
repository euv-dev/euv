/// Page title shown in the header of the timing hooks page.
pub(crate) const HOOKS_TIMING_PAGE_TITLE: &str = "Hooks — Timing";

/// Header subtitle naming the three rate-control hooks on the page.
pub(crate) const HOOKS_TIMING_PAGE_SUBTITLE: &str = "DebouncedValue, ThrottledValue, and Previous side-by-side. Each row drives a Signal from a different rate-control policy.";

/// Card title of the debounce row.
pub(crate) const HOOKS_TIMING_ROW_DEBOUNCE_TITLE: &str = "Debounce (quiet period)";

/// Body copy of the debounce row.
pub(crate) const HOOKS_TIMING_ROW_DEBOUNCE_BODY: &str = "Type into the box to seed a pending value; after 300 ms of idle time the debounced signal commits the latest pending value.";

/// Label of the input box shared by the debounce and throttle rows.
pub(crate) const HOOKS_TIMING_INPUT_LABEL: &str = "Live input";

/// Card title of the throttle row.
pub(crate) const HOOKS_TIMING_ROW_THROTTLE_TITLE: &str = "Throttle (max-once-per-window)";

/// Body copy of the throttle row.
pub(crate) const HOOKS_TIMING_ROW_THROTTLE_BODY: &str = "Type into the box to push pending values into the throttler. The committed value updates at most once every 250 ms.";

/// Card title of the previous-value snapshot row.
pub(crate) const HOOKS_TIMING_ROW_PREVIOUS_TITLE: &str = "Previous (snapshot of last render)";

/// Body copy of the previous-value snapshot row.
pub(crate) const HOOKS_TIMING_ROW_PREVIOUS_BODY: &str = "Each render is preceded by `previous_step`, which records the current value and reports the snapshot from the previous render.";

/// Row label of the value that was on screen before the latest render.
pub(crate) const HOOKS_TIMING_CURRENT_PREFIX: &str = "current:";

/// Row label of the value recorded before the latest render.
pub(crate) const HOOKS_TIMING_PREVIOUS_PREFIX: &str = "previous:";
