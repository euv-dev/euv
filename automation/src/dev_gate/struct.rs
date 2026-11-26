#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
use super::*;

/// Dev-only live AI Schema v1 loopback inspector bridge; compiles to a zero-sized no-op in release builds.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
#[derive(CustomDebug)]
pub struct DevAutomationBridge {
    /// Non-blocking loopback TCP listener serving JSON-RPC AI Schema requests in dev mode only.
    #[debug(skip)]
    pub(crate) listener: Option<TcpListener>,
    /// Bound socket address string.
    pub(crate) bound_addr: String,
    /// Total AI schema requests served during this dev session.
    pub(crate) requests_served: u64,
    /// Optional host-application telemetry and control hooks.
    #[debug(skip)]
    pub(crate) hooks: Option<AiAppHooks>,
}

/// Zero-sized release stub for `DevAutomationBridge` guaranteeing zero memory or CPU overhead in release builds.
#[cfg(not(all(debug_assertions, not(target_arch = "wasm32"))))]
#[derive(Clone, Copy, Debug, Default)]
pub struct DevAutomationBridge;
