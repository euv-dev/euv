/// The message an error boundary reports when the panic payload carries no text.
///
/// A `catch_unwind` payload can only be recovered as a `String` or a
/// `&'static str`; any other thrown value is surfaced as this
/// placeholder so the boundary always has a message to display.
pub(crate) const PANIC_PAYLOAD_FALLBACK_MESSAGE: &str = "<unknown panic payload>";
