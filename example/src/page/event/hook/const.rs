/// The minimum dimension (width and height in pixels) for the rendered QR code SVG.
pub(crate) const QR_CODE_MIN_DIMENSION: u32 = 200;

/// The prefix for an SVG data URL used as an image source.
pub(crate) const SVG_DATA_URL_PREFIX: &str = "data:image/svg+xml,";

/// The characters that must be percent-encoded when an SVG payload is
/// inlined into a `data:image/svg+xml,` URL.
pub(crate) const SVG_ESCAPED_CHARS: &[char] = &['%', '#', '"', '\'', '<', '>', '&', '{', '}'];

/// The placeholder shown before any event has been seen.
pub(crate) const EVENT_STATUS_NONE: &str = "None";

/// The placeholder shown before the mouse has moved.
pub(crate) const EVENT_MOUSE_POSITION_NONE: &str = "(0, 0)";

/// The placeholder shown before a clip has loaded.
pub(crate) const EVENT_MEDIA_STATUS_NOT_LOADED: &str = "Not loaded";

/// The placeholder shown before a media time is known.
pub(crate) const EVENT_TIME_NONE: &str = "0.00";
