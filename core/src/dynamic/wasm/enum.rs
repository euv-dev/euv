use super::*;

/// Target execution scope for a dynamically downloaded `.wasm` module.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum WasmDynamicScope {
    /// Scoped partial slot replacing only a single named component subtree without disturbing siblings.
    Partial {
        /// Unique target slot identifier (e.g. `"wasm_promo_slot"`).
        slot_id: String,
    },
    /// Global full-application bundle atomically taking over the root shell and route table with LKG rollback.
    #[default]
    Global,
}

/// Typed lifecycle state for a dynamic WASM download, verification, execution, and mount operation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum WasmLoadState {
    /// No dynamic WASM download has been initiated yet.
    #[default]
    Idle,
    /// Download in progress with byte-level streaming telemetry.
    Downloading {
        /// Bytes received so far.
        received_bytes: u64,
        /// Total expected bytes (`0` if unknown).
        total_bytes: u64,
    },
    /// Verifying binary size ceiling, FNV-1a checksum, and signature integrity.
    Verifying,
    /// Parsing WebAssembly binary sections and validating host ABI exports.
    Compiling,
    /// Executing sandboxed WASM bytecode to generate the native UI tree.
    Executing,
    /// Successfully mounted as native `euv` scene nodes.
    Mounted {
        /// Active module version number.
        version: u64,
        /// Whether bytes were served from verified offline/local cache.
        from_cache: bool,
        /// Exact sandbox instruction fuel consumed during render.
        fuel_used: u64,
    },
    /// Automatically or manually rolled back to a previously verified Last-Known-Good snapshot.
    RolledBack {
        /// Version active after rollback (`0` indicates built-in native fallback).
        active_version: u64,
        /// Human-readable reason triggering the rollback.
        reason: String,
    },
    /// Operation failed cleanly with typed diagnostics; host remains intact.
    Failed(WasmDynamicError),
}

/// Comprehensive typed error taxonomy covering all network, integrity, ABI, and sandbox failure cases.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum WasmDynamicError {
    /// Download URL was empty, malformed, or used an unsupported scheme.
    InvalidUrl(String),
    /// Network transport is offline and no eligible verified cache entry exists.
    NetworkOffline(String),
    /// Remote server returned a non-2xx HTTP status code.
    HttpStatus {
        /// HTTP status code (e.g. `404`, `500`, `503`).
        status: u16,
        /// Requested bundle URL.
        url: String,
    },
    /// Download exceeded configured `timeout_ms`.
    Timeout {
        /// Requested bundle URL.
        url: String,
        /// Timeout threshold in milliseconds.
        timeout_ms: u64,
    },
    /// Request was explicitly cancelled before completion.
    Cancelled {
        /// Cancelled monotonic request identifier.
        request_id: u64,
    },
    /// Response belonged to a superseded request generation and was discarded safely.
    StaleResponse {
        /// Superseded request identifier.
        request_id: u64,
        /// Current active request identifier.
        active_request_id: u64,
    },
    /// Downloaded binary exceeded `max_binary_bytes`.
    BinaryTooLarge {
        /// Actual byte length.
        actual: usize,
        /// Configured ceiling.
        limit: usize,
    },
    /// Binary failed WebAssembly magic/version/section decoding.
    InvalidWasmBinary(String),
    /// Computed FNV-1a checksum did not match `expected_checksum`.
    ChecksumMismatch {
        /// Expected checksum declared by manifest.
        expected: u64,
        /// Actual checksum of downloaded bytes.
        actual: u64,
    },
    /// Cryptographic/HMAC signature tag verification failed.
    SignatureInvalid {
        /// Expected signature string.
        expected: String,
        /// Computed signature string.
        actual: String,
    },
    /// Module reported an incompatible euv host ABI version.
    AbiMismatch {
        /// Host ABI version required by runtime.
        expected: u32,
        /// ABI version returned by module.
        actual: u32,
    },
    /// Required exported function (e.g. `euv_render`) was absent from `.wasm` module.
    MissingExport(String),
    /// Incoming version was lower than or equal to the active version without `allow_downgrade`.
    VersionRegression {
        /// Currently active version.
        current: u64,
        /// Rejected incoming version.
        incoming: u64,
    },
    /// Module requested linear memory exceeding `max_memory_bytes`.
    MemoryLimitExceeded {
        /// Requested linear memory bytes.
        requested: usize,
        /// Configured sandbox ceiling.
        limit: usize,
    },
    /// WASM instruction attempted an out-of-bounds linear memory read or write.
    MemoryOutOfBounds {
        /// Target linear memory address.
        addr: u32,
        /// Requested slice length.
        len: u32,
        /// Active memory size in bytes.
        memory_size: usize,
    },
    /// Sandbox terminated runaway/infinite-loop execution upon hitting `max_fuel`.
    OutOfFuel {
        /// Fuel instruction ceiling exceeded.
        limit: u64,
    },
    /// WASM execution trapped (`unreachable`, divide-by-zero, or stack underflow/overflow).
    ExecutionTrap(String),
    /// WASM emitted an invalid or unparseable native UI specification payload.
    InvalidUiPayload(String),
    /// Rollback requested when no prior verified snapshot exists.
    NoRollbackSnapshot,
}

/// Deterministic fault injection scenarios for verifying every production failure path end-to-end.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum WasmFaultKind {
    /// Normal healthy `.wasm` delivery and execution.
    #[default]
    None,
    /// Simulate offline device network failure (exercises offline cache fallback when available).
    NetworkOffline,
    /// Simulate transient timeout on first attempt that succeeds on automatic retry.
    TimeoutOnceThenSucceed,
    /// Simulate persistent HTTP 500 server failure across all retry attempts.
    Http500,
    /// Deliver bytes with corrupted `.wasm` magic header.
    CorruptedMagic,
    /// Flip a byte inside a valid `.wasm` to trigger checksum rejection.
    TamperedChecksum,
    /// Provide a mismatched signature tag to trigger signature rejection.
    InvalidSignature,
    /// Execute a valid `.wasm` returning an unsupported `euv_abi_version`.
    AbiMismatch,
    /// Execute a valid `.wasm` missing the required `euv_render` export.
    MissingExport,
    /// Execute a valid `.wasm` containing an infinite `loop { br 0 }` to trigger `OutOfFuel`.
    InfiniteLoopOutOfFuel,
    /// Execute a valid `.wasm` executing `unreachable` to trigger `ExecutionTrap`.
    ExecutionTrap,
    /// Deliver a payload exceeding `max_binary_bytes` to trigger `BinaryTooLarge`.
    OversizedBinary,
}
