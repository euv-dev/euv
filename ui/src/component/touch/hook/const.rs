/// Wire names for [`EuvGesture`](crate::EuvGesture), indexed to match the variant order.
///
/// Kept as a table so `EuvGesture::name` is a lookup rather than a match
/// arm, and so the spellings are greppable in one place.
pub(crate) const GESTURE_NAMES: [&str; 6] = ["left", "right", "up", "down", "tap", "long-press"];
