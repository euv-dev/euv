/// Minimum positive floor clamped for base-10 logarithmic axes.
pub(crate) const SCALE_LOG_MIN_POSITIVE: f32 = 1e-4;

/// Short calendar month labels across 12 months.
pub(crate) const SCALE_MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Short day-of-week labels for calendar matrices.
pub(crate) const SCALE_WEEKDAY_NAMES: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

/// Threshold separating Unix seconds timestamps from smaller relative time units.
pub(crate) const SCALE_UNIX_SECONDS_MIN: f32 = 86_400.0;

/// Suffix appended to thousand-magnitude formatted tick numbers.
pub(crate) const SCALE_SUFFIX_K: &str = "k";

/// Suffix appended to million-magnitude formatted tick numbers.
pub(crate) const SCALE_SUFFIX_M: &str = "M";

/// Suffix appended to billion-magnitude formatted tick numbers.
pub(crate) const SCALE_SUFFIX_B: &str = "B";
