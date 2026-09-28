/// The minimum dimension (width and height in pixels) for the rendered QR code SVG.
pub(crate) const QR_CODE_MIN_DIMENSION: u32 = 200;

/// The prefix for an SVG data URL used as an image source.
pub(crate) const SVG_DATA_URL_PREFIX: &str = "data:image/svg+xml,";

/// The characters that must be percent-encoded when an SVG payload is
/// inlined into a `data:image/svg+xml,` URL.
pub(crate) const SVG_ESCAPED_CHARS: &[char] = &['%', '#', '"', '\'', '<', '>', '&', '{', '}'];
