/// Weekday header labels rendered across the 7-column `EuvCalendar` grid.
pub(crate) const CALENDAR_WEEKDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// Label displayed on the collapsible toggle badge when content is expanded.
pub(crate) const COLLAPSIBLE_HIDE_LABEL: &str = "Hide";

/// Label displayed on the collapsible toggle badge when content is collapsed.
pub(crate) const COLLAPSIBLE_SHOW_LABEL: &str = "Show";

/// Placeholder displayed inside an empty OTP digit slot.
pub(crate) const OTP_EMPTY_SLOT: &str = "-";

/// Label shown in `EuvCommand` when no command matches the active search query.
pub(crate) const COMMAND_EMPTY_LABEL: &str = "No results found.";

/// Label shown in `EuvCarousel` when no slides are supplied.
pub(crate) const CAROUSEL_EMPTY_LABEL: &str = "No slides";

/// Default cancel button label in `EuvAlertDialog`.
pub(crate) const ALERT_DEFAULT_CANCEL_LABEL: &str = "Cancel";

/// Default confirm button label in `EuvAlertDialog`.
pub(crate) const ALERT_DEFAULT_CONFIRM_LABEL: &str = "Continue";

/// Default search prompt placeholder in `EuvCommand`.
pub(crate) const COMMAND_DEFAULT_PLACEHOLDER: &str = "Type a command or search...";

/// Overlay stack identifier for `EuvAlertDialog`.
pub(crate) const ALERT_DIALOG_OVERLAY_NAME: &str = "EuvAlertDialog";

/// Slide counter string when `EuvCarousel` contains zero slides.
pub(crate) const CAROUSEL_ZERO_COUNTER: &str = "0 / 0";
