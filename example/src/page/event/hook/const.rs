/// The minimum dimension (width and height in pixels) for the rendered QR code SVG.
pub(crate) const QR_CODE_MIN_DIMENSION: u32 = 200;

/// The prefix for an SVG data URL used as an image source.
pub(crate) const SVG_DATA_URL_PREFIX: &str = "data:image/svg+xml,";

/// The characters that must be percent-encoded when an SVG payload is
/// inlined into a `data:image/svg+xml,` URL.
pub(crate) const SVG_ESCAPED_CHARS: &[char] = &['%', '#', '"', '\'', '<', '>', '&', '{', '}'];

/// The initial display text for event-state labels before any event fires.
pub(crate) const EVENT_LABEL_INITIAL_NONE: &str = "None";

/// The initial display text for coordinate pair readouts before any pointer event fires.
pub(crate) const EVENT_COORDINATES_INITIAL: &str = "(0, 0)";

/// The initial status text for media elements before the resource loads.
pub(crate) const EVENT_MEDIA_STATUS_NOT_LOADED: &str = "Not loaded";

/// The initial display text for media time readouts before metadata loads.
pub(crate) const EVENT_MEDIA_TIME_INITIAL: &str = "0.00";
