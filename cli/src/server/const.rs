/// The error message returned when the server is not yet ready.
pub(crate) const ERROR_SERVER_NOT_READY: &str = "server not ready";

/// The error message returned when the global state is initialized twice.
pub(crate) const ERROR_GLOBAL_STATE_ALREADY_INITIALIZED: &str = "Global state already initialized";

/// The suffix appended to the JS bridge stem to address the wasm binary.
pub(crate) const WASM_FILE_SUFFIX: &str = "_bg.wasm";

/// The error message for a failed read of a custom `index.html` template.
pub(crate) const ERROR_READ_CUSTOM_INDEX_HTML: &str = "Failed to read custom index.html";

/// The error message for a custom `index.html` that is not valid UTF-8.
pub(crate) const ERROR_CUSTOM_INDEX_HTML_NOT_UTF8: &str = "Custom index.html is not valid UTF-8";

/// The error message for a failed creation of the static serving directory.
pub(crate) const ERROR_CREATE_STATIC_DIRECTORY: &str = "Failed to create static directory";

/// The error message for a failed write of the generated `index.html`.
pub(crate) const ERROR_WRITE_INDEX_HTML: &str = "Failed to write index.html";

/// The route parameter name carrying the requested static-asset path.
pub(crate) const ROUTE_PARAM_PATH: &str = "path";

/// The HTTP scheme prefix.
pub(crate) const HTTP_SCHEME: &str = "http";

/// The Windows UNC path prefix.
pub(crate) const WINDOWS_UNC_PREFIX: &str = r"\\?\";

/// The `Cache-Control` header name.
///
/// http-constant narrowed its own `CACHE_CONTROL` to `pub(crate)` in 21.12.0,
/// so euv-cli spells the header name locally instead of re-exporting a
/// constant that upstream may keep tightening.
pub(crate) const HEADER_CACHE_CONTROL: &str = "cache-control";

/// The `Expires` header name.
///
/// Narrowed to `pub(crate)` in http-constant 21.12.0 alongside
/// `CACHE_CONTROL`; see [`HEADER_CACHE_CONTROL`].
pub(crate) const HEADER_EXPIRES: &str = "expires";

/// The `Pragma` header name.
pub(crate) const HEADER_PRAGMA: &str = "pragma";
