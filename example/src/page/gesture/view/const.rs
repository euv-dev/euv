/// The emoji icon shown in the gesture page header.
pub(crate) const GESTURE_HEADER_ICON: &str = "👆";

/// The gesture page title.
pub(crate) const GESTURE_HEADER_TITLE: &str = "Gesture";

/// The gesture page subtitle describing what the recognizer reports.
pub(crate) const GESTURE_HEADER_SUBTITLE: &str = "EuvGestureRecognizer turns raw touch points into swipes, taps, long presses, and two-finger pinches. All four handlers mount on one pad, and the readouts below are driven straight off the recognizer signals.";

/// The title of the card holding the touchable demo pad.
pub(crate) const GESTURE_PAD_CARD_TITLE: &str = "Gesture Pad";

/// The primary instruction line rendered inside the pad.
pub(crate) const GESTURE_PAD_PRIMARY_TEXT: &str = "Drag across this pad.";

/// The secondary instruction line rendered inside the pad.
pub(crate) const GESTURE_PAD_SECONDARY_TEXT: &str = "Swipe past the threshold to resolve a direction, hold still for a long press, or use two fingers to pinch.";

/// The inline style that grows the shared touch zone into a large demo pad.
pub(crate) const GESTURE_PAD_STYLE: &str =
    "min-height: 340px; touch-action: none; user-select: none;";

/// The title of the card holding the live recognizer readouts.
pub(crate) const GESTURE_READOUT_CARD_TITLE: &str = "Live Recognizer State";

/// The id of the container wrapping the live recognizer readouts.
pub(crate) const GESTURE_READOUT_CONTAINER_ID: &str = "gesture-readout";

/// The id of the value span reporting the most recently completed gesture.
pub(crate) const GESTURE_LAST_VALUE_ID: &str = "gesture-last-value";

/// The id of the value span reporting the live drag x delta.
pub(crate) const GESTURE_DRAG_DX_VALUE_ID: &str = "gesture-drag-dx-value";

/// The id of the value span reporting the live drag y delta.
pub(crate) const GESTURE_DRAG_DY_VALUE_ID: &str = "gesture-drag-dy-value";

/// The id of the value span reporting the live drag path length.
pub(crate) const GESTURE_DRAG_TRAVEL_VALUE_ID: &str = "gesture-drag-travel-value";

/// The id of the value span reporting the live pinch distance.
pub(crate) const GESTURE_PINCH_DISTANCE_VALUE_ID: &str = "gesture-pinch-distance-value";

/// The id of the value span reporting the live pinch distance delta.
pub(crate) const GESTURE_PINCH_DELTA_VALUE_ID: &str = "gesture-pinch-delta-value";

/// The id of the value span reporting the live pinch scale.
pub(crate) const GESTURE_PINCH_SCALE_VALUE_ID: &str = "gesture-pinch-scale-value";

/// The row label for the most recently completed gesture.
pub(crate) const GESTURE_LABEL_LAST: &str = "Last gesture";

/// The row label for the live drag x delta.
pub(crate) const GESTURE_LABEL_DRAG_DX: &str = "Drag dx";

/// The row label for the live drag y delta.
pub(crate) const GESTURE_LABEL_DRAG_DY: &str = "Drag dy";

/// The row label for the live drag path length.
pub(crate) const GESTURE_LABEL_DRAG_TRAVEL: &str = "Travel";

/// The row label for the live pinch distance.
pub(crate) const GESTURE_LABEL_PINCH_DISTANCE: &str = "Pinch distance";

/// The row label for the live pinch distance delta.
pub(crate) const GESTURE_LABEL_PINCH_DELTA: &str = "Pinch delta";

/// The row label for the live pinch scale.
pub(crate) const GESTURE_LABEL_PINCH_SCALE: &str = "Pinch scale";

/// The placeholder rendered when no gesture has completed yet.
pub(crate) const GESTURE_PLACEHOLDER: &str = "—";

/// The placeholder rendered when no drag or pinch is in flight.
pub(crate) const GESTURE_PLACEHOLDER_NUMBER: &str = "0.0";

/// The title of the card describing the recognizer thresholds.
pub(crate) const GESTURE_CONFIG_CARD_TITLE: &str = "Recognizer Thresholds";

/// The prose paragraph describing how the defaults are tuned.
pub(crate) const GESTURE_CONFIG_DESCRIPTION: &str = "The pad mounts EuvGestureRecognizer::new(), so every reading below comes from the default threshold set. A 48px swipe threshold keeps fingertip jitter out of the directional gestures, a 10px tap slop separates a tap from a drag, a 500ms press turns a stationary touch into a long press, and a 1% pinch change filters out adjacent-move noise.";

/// The row label for the configured swipe threshold.
pub(crate) const GESTURE_LABEL_SWIPE_THRESHOLD: &str = "Swipe";

/// The row label for the configured tap slop.
pub(crate) const GESTURE_LABEL_TAP_SLOP: &str = "Tap slop";

/// The row label for the configured long press duration.
pub(crate) const GESTURE_LABEL_LONG_PRESS: &str = "Long press";

/// The row label for the configured pinch threshold.
pub(crate) const GESTURE_LABEL_PINCH_THRESHOLD: &str = "Pinch";

/// The unit suffix appended to pixel-denominated thresholds.
pub(crate) const GESTURE_UNIT_PIXELS: &str = " px";

/// The unit suffix appended to the millisecond long press threshold.
pub(crate) const GESTURE_UNIT_MILLIS: &str = " ms";

/// The unit suffix appended to the ratio-denominated pinch threshold.
pub(crate) const GESTURE_UNIT_RATIO: &str = " ratio";
