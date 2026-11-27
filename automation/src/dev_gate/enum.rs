use super::*;

/// Decoded HTTP request routed off-thread into the bounded UI command queue.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
#[derive(Debug)]
pub(crate) enum BridgeWorkerRoute {
    /// Immediate response produced entirely on the background worker thread (e.g. OPTIONS, health, or 400/429).
    Immediate {
        /// Raw pre-formatted HTTP/1.1 response payload.
        raw_http: String,
        /// Whether to increment `requests_served` upon completion.
        count_served: bool,
    },
    /// Typed AI Schema request queued to run on the UI thread within its frame budget.
    UiRequest(Box<AiSchemaRequest>),
}
