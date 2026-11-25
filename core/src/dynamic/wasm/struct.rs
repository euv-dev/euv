use super::*;

/// Sandbox safety, memory, fuel, timeout, retry, and caching configuration for dynamic WASM execution.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, PartialEq)]
pub struct WasmSandboxConfig {
    /// Maximum allowed `.wasm` payload size in bytes.
    #[get(type(copy))]
    pub(crate) max_binary_bytes: usize,
    /// Maximum allowed linear memory allocation in bytes.
    #[get(type(copy))]
    pub(crate) max_memory_bytes: usize,
    /// Maximum instructions executed per WASM call before terminating with `OutOfFuel`.
    #[get(type(copy))]
    pub(crate) max_fuel: u64,
    /// Download timeout ceiling in milliseconds.
    #[get(type(copy))]
    pub(crate) timeout_ms: u64,
    /// Automatic retry count for transient transport failures (`Timeout` / `HttpStatus >= 500`).
    #[get(type(copy))]
    pub(crate) max_retries: u32,
    /// Whether verified cached `.wasm` entries may be served when the network is offline.
    #[get(type(copy))]
    pub(crate) allow_offline_cache: bool,
    /// Whether non-empty `expected_signature` verification is mandatory.
    #[get(type(copy))]
    pub(crate) require_signature: bool,
}

/// Declarative request descriptor for downloading and executing a dynamic `.wasm` module.
#[derive(Clone, Data, Debug, Eq, Hash, New, PartialEq)]
pub struct WasmDownloadRequest {
    /// Source URL (`https://`, `http://`, `euv-cdn://`, or `file://`).
    pub(crate) url: String,
    /// Execution scope (`Partial { slot_id }` or `Global`).
    pub(crate) scope: WasmDynamicScope,
    /// Monotonic release version (`>= 1`).
    #[get(type(copy))]
    pub(crate) version: u64,
    /// Optional expected FNV-1a checksum (`0` disables checksum comparison).
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) expected_checksum: u64,
    /// Optional expected signature tag (verified with `compute_wasm_signature`).
    #[new(skip)]
    pub(crate) expected_signature: String,
    /// Whether this request may install a version `<=` the currently mounted version.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) allow_downgrade: bool,
    /// Optional cache key override (defaults to `url`).
    #[new(skip)]
    pub(crate) cache_key: String,
}

/// Raw downloaded `.wasm` payload returned by a `WasmFetcher` implementation.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct WasmFetchedPayload {
    /// Raw WebAssembly binary bytes.
    pub(crate) bytes: Vec<u8>,
    /// Optional ETag / content revision identifier.
    #[new(skip)]
    pub(crate) etag: String,
    /// Whether this payload originated from cache.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) from_cache: bool,
}

/// Parsed WebAssembly function signature `(param_count, result_count)`.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, PartialEq)]
pub struct WasmFuncSignature {
    /// Number of `i32` parameters.
    #[get(type(copy))]
    pub(crate) param_count: usize,
    /// Number of `i32` results (`0` or `1`).
    #[get(type(copy))]
    pub(crate) result_count: usize,
}

/// Parsed WebAssembly host import declaration.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct WasmImportEntry {
    /// Imported module namespace (e.g. `"env"`).
    pub(crate) module: String,
    /// Imported function name (e.g. `"euv_emit_ui"`).
    pub(crate) name: String,
    /// Type index inside the module's type table.
    #[get(type(copy))]
    pub(crate) type_index: u32,
}

/// Parsed WebAssembly active linear memory data segment.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct WasmDataSegment {
    /// Linear memory byte offset where `bytes` are initialized.
    #[get(type(copy))]
    pub(crate) offset: u32,
    /// Raw data segment payload bytes.
    pub(crate) bytes: Vec<u8>,
}

/// Parsed WebAssembly function body ready for sandbox interpretation.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct WasmParsedFunction {
    /// Type index inside the module's type table.
    #[get(type(copy))]
    pub(crate) type_index: u32,
    /// Count of additional `i32` local variables declared in the body header.
    #[get(type(copy))]
    pub(crate) extra_locals: usize,
    /// Raw WebAssembly bytecode instructions ending with `0x0B`.
    pub(crate) body: Vec<u8>,
}

/// Validated in-memory representation of a decoded `.wasm` binary module.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct WasmParsedModule {
    /// Declared function signatures from the Type section.
    pub(crate) types: Vec<WasmFuncSignature>,
    /// Imported host functions from the Import section.
    pub(crate) imports: Vec<WasmImportEntry>,
    /// Defined function bodies paired with type indices.
    pub(crate) functions: Vec<WasmParsedFunction>,
    /// Exported function name -> absolute function index map.
    pub(crate) exports: HashMap<String, u32>,
    /// Minimum linear memory pages declared (`1 page = 64 KiB`).
    #[get(type(copy))]
    pub(crate) memory_min_pages: u32,
    /// Initialized linear memory data segments.
    pub(crate) data_segments: Vec<WasmDataSegment>,
    /// Custom sections keyed by section name.
    pub(crate) custom_sections: HashMap<String, Vec<u8>>,
}

/// Output produced by executing an exported function inside the WASM sandbox.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct WasmExecutionOutput {
    /// Optional `i32` return value from the invoked WASM function.
    #[get(type(copy))]
    pub(crate) return_value: i32,
    /// Total instructions metered during execution.
    #[get(type(copy))]
    pub(crate) fuel_used: u64,
    /// Native UI subtree emitted via `env::euv_emit_ui` (if any).
    #[get(skip)]
    pub(crate) emitted_spec: Option<DynamicNodeSpec>,
    /// Native global app bundle emitted via `env::euv_emit_bundle` (if any).
    #[get(skip)]
    pub(crate) emitted_bundle: Option<DynamicUpdateBundle>,
    /// Diagnostic messages emitted via `env::euv_log`.
    pub(crate) logs: Vec<String>,
}

/// Verified `.wasm` binary cached locally for instant reload and offline fallback.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct WasmCachedEntry {
    /// Source URL.
    pub(crate) url: String,
    /// Verified release version.
    #[get(type(copy))]
    pub(crate) version: u64,
    /// Verified FNV-1a checksum.
    #[get(type(copy))]
    pub(crate) checksum: u64,
    /// Verified raw `.wasm` binary bytes.
    pub(crate) bytes: Vec<u8>,
    /// Server ETag / content identifier.
    pub(crate) etag: String,
}

/// Verified mounted WASM revision snapshot stored on a partial slot or global rollback stack.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct WasmMountedSnapshot {
    /// Active release version.
    #[get(type(copy))]
    pub(crate) version: u64,
    /// Source URL.
    pub(crate) url: String,
    /// Verified checksum.
    #[get(type(copy))]
    pub(crate) checksum: u64,
    /// Parsed validated `.wasm` module kept ready for live `euv_on_event` callbacks.
    pub(crate) module: WasmParsedModule,
    /// Current integer state value passed into `euv_render(state)` / updated by `euv_on_event`.
    #[get(type(copy))]
    pub(crate) state_value: i32,
    /// Optional emitted partial spec snapshot.
    #[get(skip)]
    pub(crate) spec: Option<DynamicNodeSpec>,
    /// Optional emitted global bundle snapshot.
    #[get(skip)]
    pub(crate) bundle: Option<DynamicUpdateBundle>,
}

/// Cumulative reactive telemetry counters for dynamic WASM loading, caching, sandboxing, and rollbacks.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, PartialEq)]
pub struct WasmDynamicTelemetry {
    /// Total download attempts initiated.
    #[get(type(copy))]
    pub(crate) downloads_started: u64,
    /// Total successful downloads + mounts.
    #[get(type(copy))]
    pub(crate) downloads_succeeded: u64,
    /// Total verified offline/local cache hits.
    #[get(type(copy))]
    pub(crate) cache_hits: u64,
    /// Total automatic transport retries executed.
    #[get(type(copy))]
    pub(crate) retries_performed: u64,
    /// Total automatic or manual rollbacks executed.
    #[get(type(copy))]
    pub(crate) rollbacks_executed: u64,
    /// Total WASM runtime traps isolated without harming host UI.
    #[get(type(copy))]
    pub(crate) sandbox_traps_caught: u64,
    /// Total infinite-loop fuel exhaustions terminated cleanly.
    #[get(type(copy))]
    pub(crate) fuel_exhaustions_caught: u64,
    /// Total checksum, signature, binary, or ABI rejections.
    #[get(type(copy))]
    pub(crate) integrity_rejections: u64,
    /// Total interactive events dispatched into mounted WASM modules.
    #[get(type(copy))]
    pub(crate) events_handled: u64,
}

/// Registered entry inside `RegistryWasmFetcher` for deterministic cross-platform CDN and fault simulation.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct RegistryWasmRecord {
    /// Raw `.wasm` bytes served for this URL.
    pub(crate) bytes: Vec<u8>,
    /// ETag label returned with the payload.
    pub(crate) etag: String,
    /// Optional injected fault scenario.
    #[get(type(copy))]
    pub(crate) fault: WasmFaultKind,
}

/// Cross-platform in-memory WASM CDN registry and deterministic fault-injection fetcher.
#[derive(Clone, Debug, Default)]
pub struct RegistryWasmFetcher {
    /// URL -> `.wasm` record registry.
    pub(crate) records: Rc<RefCell<HashMap<String, RegistryWasmRecord>>>,
    /// Global offline toggle simulating airplane mode across all URLs.
    pub(crate) offline: Rc<RefCell<bool>>,
}

/// Native pure-std HTTP (`http://`) and local filesystem (`file://`) WASM fetcher with timeout and progress streaming.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, PartialEq)]
pub struct HttpWasmFetcher {
    /// Socket connect/read timeout in milliseconds.
    #[get(type(copy))]
    pub(crate) timeout_ms: u64,
}

/// Internal interior-mutable state owned by `WasmDynamicManager`.
#[derive(CustomDebug)]
pub(crate) struct WasmDynamicManagerState {
    /// Active `DynamicUpdateHub` receiving compiled native scene updates.
    pub(crate) hub: DynamicUpdateHub,
    /// Pluggable transport fetcher (`HttpWasmFetcher`, `RegistryWasmFetcher`, or custom CDN).
    #[debug(skip)]
    pub(crate) fetcher: Rc<dyn WasmFetcher>,
    /// Sandbox fuel, memory, binary size, timeout, retry, and caching limits.
    pub(crate) config: WasmSandboxConfig,
    /// Monotonic request generation counter for cancellation and stale-response protection.
    pub(crate) next_request_id: u64,
    /// Active latest request generation per scope key.
    pub(crate) active_request_ids: HashMap<String, u64>,
    /// Verified `.wasm` binary cache keyed by cache key / URL.
    pub(crate) cache: HashMap<String, WasmCachedEntry>,
    /// Mounted revision stacks per partial slot ID.
    pub(crate) partial_stacks: HashMap<String, Vec<WasmMountedSnapshot>>,
    /// Mounted revision stack for global full-app WASM takeovers.
    pub(crate) global_stack: Vec<WasmMountedSnapshot>,
    /// Per-scope reactive `WasmLoadState` signals.
    pub(crate) state_signals: HashMap<String, Signal<WasmLoadState>>,
    /// Reactive global active version signal (`0` = built-in native shell).
    pub(crate) global_version_signal: Signal<u64>,
    /// Reactive telemetry snapshot signal.
    pub(crate) telemetry_signal: Signal<WasmDynamicTelemetry>,
    /// Last diagnostic log entries emitted by executed WASM modules.
    pub(crate) last_logs: Signal<Vec<String>>,
}

/// Shared cross-platform manager orchestrating partial and global dynamic `.wasm` download, verification, execution, and native rendering.
#[derive(Clone, CustomDebug)]
pub struct WasmDynamicManager {
    /// Interior-mutable manager state shared across slots, pages, and host shell.
    pub(crate) inner: Rc<RefCell<WasmDynamicManagerState>>,
}

/// Spec-compliant `.wasm` binary encoder producing genuine executable WebAssembly modules for euv native rendering.
#[derive(Clone, Data, Debug, Default)]
pub struct WasmModuleBuilder {
    /// ABI version returned by `euv_abi_version`.
    #[get(type(copy))]
    pub(crate) abi_version: u32,
    /// Whether to export `euv_render`.
    #[get(type(copy))]
    pub(crate) export_render: bool,
    /// Whether `euv_render` emits a global `DynamicUpdateBundle` instead of a partial `DynamicNodeSpec`.
    #[get(type(copy))]
    pub(crate) is_global_bundle: bool,
    /// UTF-8 JSON UI payload placed in linear memory and emitted via host import.
    pub(crate) ui_payload_json: String,
    /// Optional second variant JSON payload emitted when interactive state is odd (`state & 1 != 0`).
    pub(crate) alt_ui_payload_json: String,
    /// Optional diagnostic log line emitted during `euv_render`.
    pub(crate) log_message: String,
    /// Optional fault behavior compiled directly into `euv_render` bytecode (`InfiniteLoopOutOfFuel` / `ExecutionTrap`).
    #[get(type(copy))]
    pub(crate) fault: WasmFaultKind,
    /// Optional custom metadata section bytes.
    pub(crate) custom_meta: Vec<u8>,
}

/// Mutable execution context threaded through sandboxed WASM interpreter invocations.
pub(crate) struct WasmExecContext<'a> {
    /// Active WASM linear memory byte slice.
    pub(crate) memory: &'a mut [u8],
    /// Accumulated host import outputs and logs.
    pub(crate) out: &'a mut WasmExecutionOutput,
    /// Running instruction fuel counter.
    pub(crate) fuel_used: u64,
    /// Hard instruction fuel limit.
    pub(crate) max_fuel: u64,
    /// Current recursion call depth.
    pub(crate) depth: usize,
}
