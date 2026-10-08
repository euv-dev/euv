/// Body shown when a Debug readout is constructed without a formatter.
pub(crate) const DEBUG_MISSING_FORMATTER_PLACEHOLDER: &str = "<no formatter>";

/// State marker written to the `data-euv-debug` / `data-euv-debug-value`
/// attributes when the readout is expanded into a multi-line block.
pub(crate) const DEBUG_STATE_EXPANDED: &str = "expanded";

/// State marker written to the `data-euv-debug` / `data-euv-debug-value`
/// attributes when the readout stays on a single line.
pub(crate) const DEBUG_STATE_INLINE: &str = "inline";
