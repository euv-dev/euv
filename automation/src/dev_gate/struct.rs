#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
use super::*;

/// Structured health check payload emitted off-thread by `GET /ai/health`.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
#[derive(Clone, Debug, New, Serialize)]
pub(crate) struct DevBridgeHealthPayload {
    /// Health status string (`"ok"`).
    pub(crate) status: String,
    /// Total requests served since bridge startup.
    pub(crate) requests_served: u64,
    /// Total requests rejected due to bounded queue saturation.
    pub(crate) backpressure_rejected: u64,
    /// Current approximate pending UI command queue depth.
    pub(crate) queue_depth: usize,
    /// Default AI Schema protocol identifier.
    pub(crate) protocol: String,
    /// All supported AI Schema protocol identifiers.
    pub(crate) supported_protocols: Vec<String>,
}

/// Bounded UI-thread command dispatched from the resident background HTTP worker thread.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
#[derive(Debug)]
pub(crate) struct BridgeUiCommand {
    /// Decoded AI Schema request to execute on the UI thread.
    pub(crate) request: AiSchemaRequest,
    /// One-shot bounded reply channel back to the background HTTP worker.
    pub(crate) reply_tx: SyncSender<AiSchemaResponse>,
}

/// Non-blocking incoming HTTP client socket currently accumulating headers and body bytes on the worker thread.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
#[derive(Debug)]
pub(crate) struct PendingClientRead {
    /// Connected non-blocking client TCP socket.
    pub(crate) stream: TcpStream,
    /// Accumulated raw HTTP request bytes.
    pub(crate) raw_bytes: Vec<u8>,
    /// Byte index immediately after `\r\n\r\n` once headers are complete.
    pub(crate) header_end: Option<usize>,
    /// Parsed `Content-Length` byte count.
    pub(crate) content_length: usize,
    /// Absolute deadline to abandon incomplete reads.
    pub(crate) deadline: Instant,
}

/// Accepted HTTP client socket waiting asynchronously for the UI thread to finish its command.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
#[derive(Debug)]
pub(crate) struct WaitingClientReply {
    /// Connected client TCP socket awaiting HTTP response bytes.
    pub(crate) stream: TcpStream,
    /// One-shot reply receiver fulfilled by `DevAutomationBridge::poll_budgeted`.
    pub(crate) reply_rx: Receiver<AiSchemaResponse>,
    /// Absolute deadline to emit `504 Gateway Timeout` if UI stops polling.
    pub(crate) deadline: Instant,
}

/// Resident background worker state isolating TCP accept, HTTP parsing, and JSON serialization off the UI thread.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
#[derive(Debug)]
pub(crate) struct BridgeWorkerState {
    /// Non-blocking loopback TCP listener owned exclusively by the worker thread.
    pub(crate) listener: TcpListener,
    /// Bounded control channel sender pushing decoded commands to the UI thread without blocking.
    pub(crate) ui_tx: SyncSender<BridgeUiCommand>,
    /// Cooperative shutdown signal set by `DevAutomationBridge::drop`.
    pub(crate) shutdown: Arc<AtomicBool>,
    /// Atomic count of completed requests.
    pub(crate) requests_served: Arc<AtomicU64>,
    /// Atomic count of requests rejected with HTTP `429` / `BACKPRESSURE`.
    pub(crate) backpressure_rejected: Arc<AtomicU64>,
    /// Approximate depth of pending commands waiting in the bounded UI channel.
    pub(crate) pending_count: Arc<AtomicUsize>,
    /// Non-blocking client sockets currently reading HTTP request payloads.
    pub(crate) reading: Vec<PendingClientRead>,
    /// Non-blocking client sockets awaiting UI-thread responses.
    pub(crate) waiting: Vec<WaitingClientReply>,
}

/// Dev-only live non-blocking AI Schema v1/v2 loopback inspector bridge; compiles to a zero-sized no-op in release builds.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
#[derive(CustomDebug)]
pub struct DevAutomationBridge {
    /// Bound socket address string.
    pub(crate) bound_addr: String,
    /// Fixed bounded control queue capacity.
    pub(crate) queue_capacity: usize,
    /// Whether the resident background TCP worker is active.
    pub(crate) active: bool,
    /// Shared atomic count of total served requests.
    pub(crate) requests_served: Arc<AtomicU64>,
    /// Shared atomic count of requests rejected due to bounded queue saturation.
    pub(crate) backpressure_rejected: Arc<AtomicU64>,
    /// Shared approximate pending queue depth.
    pub(crate) pending_count: Arc<AtomicUsize>,
    /// Cooperative shutdown flag stopping the background worker cleanly on drop.
    pub(crate) shutdown: Arc<AtomicBool>,
    /// Non-blocking UI-thread command receiver drained strictly within frame budgets.
    #[debug(skip)]
    pub(crate) ui_rx: Option<Receiver<BridgeUiCommand>>,
    /// Resident background network/JSON worker thread handle.
    #[debug(skip)]
    pub(crate) worker: Option<JoinHandle<()>>,
    /// Optional host-application telemetry and control hooks.
    #[debug(skip)]
    pub(crate) hooks: Option<AiAppHooks>,
}

/// Zero-sized release stub for `DevAutomationBridge` guaranteeing zero memory or CPU overhead in release builds.
#[cfg(not(all(debug_assertions, not(target_arch = "wasm32"))))]
#[derive(Clone, Copy, Debug, Default)]
pub struct DevAutomationBridge;
