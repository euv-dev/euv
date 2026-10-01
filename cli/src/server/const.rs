/// The error message returned when the server is not yet ready.
pub const ERROR_SERVER_NOT_READY: &str = "server not ready";

/// The error message returned when the global state is initialized twice.
pub const ERROR_GLOBAL_STATE_ALREADY_INITIALIZED: &str = "Global state already initialized";

/// The suffix appended to the JS bridge stem to address the wasm binary.
pub const WASM_FILE_SUFFIX: &str = "_bg.wasm";

/// The error message for a failed read of a custom `index.html` template.
pub const ERROR_READ_CUSTOM_INDEX_HTML: &str = "Failed to read custom index.html";

/// The error message for a custom `index.html` that is not valid UTF-8.
pub const ERROR_CUSTOM_INDEX_HTML_NOT_UTF8: &str = "Custom index.html is not valid UTF-8";

/// The error message for a failed creation of the static serving directory.
pub const ERROR_CREATE_STATIC_DIRECTORY: &str = "Failed to create static directory";

/// The error message for a failed write of the generated `index.html`.
pub const ERROR_WRITE_INDEX_HTML: &str = "Failed to write index.html";

/// The route parameter name carrying the requested static-asset path.
pub const ROUTE_PARAM_PATH: &str = "path";

/// The HTTP scheme prefix.
pub const HTTP_SCHEME: &str = "http";

/// The Windows UNC path prefix.
pub const WINDOWS_UNC_PREFIX: &str = r"\\?\";
