use super::*;

/// Filesystem modification stamp used for deterministic cross-platform change detection.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Getter, New, PartialEq, Serialize)]
pub struct FileStamp {
    /// Nanoseconds since UNIX epoch of last file modification.
    #[get(type(copy))]
    pub(crate) modified_nanos: u128,
    /// File byte length.
    #[get(type(copy))]
    pub(crate) size_bytes: u64,
}

/// Recursive file tree snapshot capturing stamps for all watched source files.
#[derive(Clone, Debug, Default, Eq, Getter, PartialEq)]
pub struct WatchSnapshot {
    /// Ordered map from watched file paths to their modification stamps.
    pub(crate) entries: BTreeMap<PathBuf, FileStamp>,
}

/// Categorized file changes detected between two consecutive `WatchSnapshot` scans.
#[derive(Clone, Debug, Default, Deserialize, Eq, Getter, New, PartialEq, Serialize)]
pub struct FileChangeSet {
    /// Newly created file paths.
    pub(crate) added: Vec<String>,
    /// Modified file paths.
    pub(crate) modified: Vec<String>,
    /// Deleted file paths.
    pub(crate) removed: Vec<String>,
}

/// Signed Dynamic WASM OTA manifest served at `/__euv_ota/manifest.json`.
#[derive(Clone, Debug, Deserialize, Eq, Getter, New, PartialEq, Serialize)]
pub struct OtaManifest {
    /// Monotonically increasing hot-update version number.
    #[get(type(copy))]
    pub(crate) version: u64,
    /// Target dynamic component slot identifier.
    pub(crate) slot_id: String,
    /// HTTP URL path serving the executable `.wasm` binary.
    pub(crate) module_url: String,
    /// FNV-1a integrity checksum of the `.wasm` bytes.
    #[get(type(copy))]
    pub(crate) checksum: u64,
    /// Keyed integrity signature matching `OTA_DEV_SIGNING_KEY`.
    pub(crate) signature: String,
    /// Byte size of the compiled `.wasm` binary.
    #[get(type(copy))]
    pub(crate) size_bytes: usize,
    /// Source files that triggered this hot-update build.
    pub(crate) changed_files: Vec<String>,
}

/// In-memory pair holding the active OTA manifest and its executable `.wasm` bytes.
#[derive(Clone, Debug, Eq, Getter, New, PartialEq)]
pub struct OtaBundleState {
    /// Public manifest metadata.
    pub(crate) manifest: OtaManifest,
    /// Raw executable `.wasm` binary bytes.
    pub(crate) wasm_bytes: Vec<u8>,
}

/// Structured live-reload notification broadcast over SSE `/__euv_hmr`.
#[derive(Clone, Debug, Deserialize, Eq, Getter, New, PartialEq, Serialize)]
pub struct HmrEvent {
    /// Monotonic reload revision number.
    #[get(type(copy))]
    pub(crate) revision: u64,
    /// Event kind (`Reload`, `OtaUpdate`, or `BuildError`).
    #[get(type(copy))]
    pub(crate) kind: HmrEventKind,
    /// Human-readable status summary.
    pub(crate) message: String,
    /// Paths of changed files that triggered this event.
    pub(crate) changed_files: Vec<String>,
}

/// Configuration for starting a live hot-reload and Dynamic WASM OTA development server.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DevServerConfig {
    /// Development target (`Docs`, `Web`, or `Native`).
    #[get(type(copy))]
    pub(crate) target: DevTargetArg,
    /// Render mode forwarded to native or web runs.
    #[get(type(copy))]
    pub(crate) render: RenderArg,
    /// Whether release optimization profile is enabled for Web or Native rebuilds.
    #[get(type(copy))]
    pub(crate) release: bool,
    /// Source root directory watched for live changes.
    pub(crate) src_dir: PathBuf,
    /// Output directory served over HTTP.
    pub(crate) out_dir: PathBuf,
    /// Optional site title (used in Docs mode).
    pub(crate) title: Option<String>,
    /// Optional base URL prefix (used in Docs mode).
    pub(crate) base_url: String,
    /// Optional Cargo package name (used in Web/Native modes).
    pub(crate) package: Option<String>,
    /// Bind host address (default `127.0.0.1`).
    pub(crate) host: String,
    /// Bind TCP port (`0` selects an available ephemeral port).
    #[get(type(copy))]
    pub(crate) port: u16,
    /// File watcher polling interval in milliseconds.
    #[get(type(copy))]
    pub(crate) poll_ms: u64,
    /// Whether to expose Dynamic WASM OTA manifest and `.wasm` binary endpoints.
    #[get(type(copy))]
    pub(crate) enable_ota: bool,
    /// When `true`, performs initial build and server check then exits immediately.
    #[get(type(copy))]
    pub(crate) once: bool,
}

/// Machine-readable report returned when starting or probing the dev/docs server.
#[derive(Clone, Debug, Deserialize, Eq, Getter, New, PartialEq, Serialize)]
pub struct DevServerReport {
    /// Active dev mode (`docs`, `web`, or `native`).
    pub(crate) mode: String,
    /// Base HTTP URL of the live dev server.
    pub(crate) url: String,
    /// SSE hot-reload stream URL.
    pub(crate) hmr_url: String,
    /// Dynamic WASM OTA manifest URL.
    pub(crate) ota_manifest_url: String,
    /// Output bundle directory served by the dev server.
    pub(crate) out_dir: String,
    /// Initial build revision number.
    #[get(type(copy))]
    pub(crate) revision: u64,
    /// Total generated documentation pages (for Docs mode, `0` otherwise).
    #[get(type(copy))]
    pub(crate) page_count: usize,
}

/// Shared concurrent state coordinating SSE reload notifications, OTA bundles, and server shutdown.
#[derive(Debug)]
pub struct HmrHub {
    /// Monotonically incremented rebuild revision counter.
    pub(crate) revision: AtomicU64,
    /// Latest broadcast `HmrEvent`.
    pub(crate) last_event: Mutex<Option<HmrEvent>>,
    /// Latest signed Dynamic WASM OTA manifest and binary bytes.
    pub(crate) ota_state: Mutex<OtaBundleState>,
    /// Flag indicating whether background HTTP and watcher loops should keep running.
    pub(crate) running: AtomicBool,
}

/// Running hot-reload dev server handle supporting clean programmatic shutdown and inspection.
#[derive(Debug)]
pub struct DevServerHandle {
    /// Bound socket address (`ip:port`).
    pub(crate) address: SocketAddr,
    /// Served output directory path.
    pub(crate) out_dir: PathBuf,
    /// Active mode label (`docs`, `web`, `native`).
    pub(crate) mode: String,
    /// Initial page count produced during startup build.
    pub(crate) page_count: usize,
    /// Shared HMR and OTA state hub.
    pub(crate) hub: Arc<HmrHub>,
    /// Background HTTP/SSE/OTA server thread.
    pub(crate) http_thread: Option<JoinHandle<()>>,
    /// Background recursive file watcher and rebuild thread.
    pub(crate) watch_thread: Option<JoinHandle<()>>,
}

/// Internal HTTP serving parameters shared across accepted connections.
#[derive(Clone, Debug, Getter)]
pub(crate) struct HttpServeContext {
    /// Output root directory served over HTTP.
    #[get(pub(crate))]
    pub(crate) out_dir: PathBuf,
    /// Optional base URL prefix stripped from incoming static requests.
    #[get(pub(crate))]
    pub(crate) base_url: String,
    /// Whether Dynamic WASM OTA endpoints are exposed.
    #[get(pub(crate), type(copy))]
    pub(crate) enable_ota: bool,
    /// Shared HMR and OTA state hub.
    #[get(pub(crate))]
    pub(crate) hub: Arc<HmrHub>,
}
