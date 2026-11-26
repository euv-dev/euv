/// Server-Sent Events endpoint streaming live reload and hot-update notifications.
pub const HMR_ENDPOINT: &str = "/__euv_hmr";
/// JSON manifest endpoint advertising the latest Dynamic WASM OTA hot-update build.
pub const OTA_MANIFEST_ENDPOINT: &str = "/__euv_ota/manifest.json";
/// Binary endpoint serving the latest signed Dynamic WASM `.wasm` module over HTTP.
pub const OTA_MODULE_ENDPOINT: &str = "/__euv_ota/module.wasm";
/// JSON health and revision status endpoint for local tooling and CI probes.
pub const DEV_STATUS_ENDPOINT: &str = "/__euv_status";
/// Shared signing key used for local dev OTA hot-update verification.
pub const OTA_DEV_SIGNING_KEY: &str = "euv-dev-ota-key-2026";
/// Default component slot targeted by local dev OTA hot updates.
pub const OTA_SLOT_DEV: &str = "euv-dev-hot-slot";
/// Default local bind host for `euv dev` and `euv docs dev`.
pub const DEFAULT_DEV_HOST: &str = "127.0.0.1";
/// Default static documentation output directory.
pub const DEFAULT_DOCS_OUT_DIR: &str = "dist/docs";

pub(crate) const WATCH_IGNORED_DIRS: [&str; 12] = [
    "target",
    ".git",
    "node_modules",
    "dist",
    "www",
    ".cargo",
    ".idea",
    ".vscode",
    ".next",
    "pkg",
    "out",
    "build",
];
pub(crate) const WATCH_EXTS_DOCS: [&str; 8] =
    ["md", "yaml", "yml", "png", "jpg", "jpeg", "svg", "webp"];
pub(crate) const WATCH_EXTS_CODE: [&str; 8] =
    ["rs", "toml", "md", "html", "css", "wgsl", "glsl", "wasm"];
pub(crate) const MODE_LABEL_DOCS: &str = "docs";
pub(crate) const MODE_LABEL_WEB: &str = "web";
pub(crate) const MODE_LABEL_NATIVE: &str = "native";
pub(crate) const HMR_EVENT_RELOAD: &str = "reload";
pub(crate) const HMR_EVENT_OTA: &str = "ota";
pub(crate) const HMR_EVENT_ERROR: &str = "error";
pub(crate) const HTTP_METHOD_HEAD: &str = "HEAD";
pub(crate) const HTTP_METHOD_OPTIONS: &str = "OPTIONS";
pub(crate) const HTTP_200_PREFIX: &str = "HTTP/1.1 200 OK\r\nContent-Type: ";
pub(crate) const HTTP_204_OPTIONS_RESP: &str = "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, HEAD, OPTIONS\r\nAccess-Control-Allow-Headers: *\r\nConnection: close\r\n\r\n";
pub(crate) const HTTP_404_RESP: &str = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: 9\r\nConnection: close\r\n\r\nNot Found";
pub(crate) const HTTP_SSE_HEADERS: &str = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\nAccess-Control-Allow-Origin: *\r\n\r\nretry: 1000\n\n: connected\n\n";
pub(crate) const HTTP_CORS_HEADERS: &str =
    "\r\nCache-Control: no-cache\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: ";
pub(crate) const HTTP_CONN_CLOSE_SEP: &str = "\r\nConnection: close\r\n\r\n";
pub(crate) const MIME_HTML: &str = "text/html; charset=utf-8";
pub(crate) const MIME_JSON: &str = "application/json; charset=utf-8";
pub(crate) const MIME_XML: &str = "application/xml; charset=utf-8";
pub(crate) const MIME_CSS: &str = "text/css; charset=utf-8";
pub(crate) const MIME_WASM: &str = "application/wasm";
pub(crate) const MIME_SVG: &str = "image/svg+xml";
pub(crate) const MIME_PNG: &str = "image/png";
pub(crate) const MIME_JPEG: &str = "image/jpeg";
pub(crate) const MIME_WEBP: &str = "image/webp";
pub(crate) const MIME_OCTET: &str = "application/octet-stream";
pub(crate) const EXT_HTML: &str = "html";
pub(crate) const EXT_JSON: &str = "json";
pub(crate) const EXT_XML: &str = "xml";
pub(crate) const EXT_CSS: &str = "css";
pub(crate) const EXT_WASM: &str = "wasm";
pub(crate) const EXT_SVG: &str = "svg";
pub(crate) const EXT_PNG: &str = "png";
pub(crate) const EXT_JPG: &str = "jpg";
pub(crate) const EXT_JPEG: &str = "jpeg";
pub(crate) const EXT_WEBP: &str = "webp";
pub(crate) const ENV_DEV_URL: &str = "EUV_DEV_URL";
pub(crate) const ENV_HMR_URL: &str = "EUV_HMR_URL";
pub(crate) const ENV_OTA_MANIFEST_URL: &str = "EUV_OTA_MANIFEST_URL";
pub(crate) const ENV_OTA_MODULE_URL: &str = "EUV_OTA_MODULE_URL";
pub(crate) const HTML_BODY_CLOSE_TAG: &str = "</body>";
pub(crate) const DEV_HMR_SCRIPT_PREFIX: &str = "<script>(function(){try{var es=new EventSource('";
pub(crate) const DEV_HMR_SCRIPT_SUFFIX: &str = "');es.addEventListener('reload',function(){location.reload();});es.addEventListener('ota',function(){location.reload();});}catch(e){}})();</script>";
pub(crate) const MSG_INITIAL_BUILD_READY: &str = "Initial build ready";
pub(crate) const MSG_DOCS_REBUILT: &str = "Docs hot-rebuilt";
pub(crate) const MSG_WEB_REBUILT: &str = "Web WASM hot-rebuilt";
pub(crate) const MSG_NATIVE_RESTARTED: &str = "Native binary rebuilt and hot-restarted";
pub(crate) const OTA_INITIAL_TITLE: &str = "Euv Dev OTA Initial Module";
pub(crate) const SSE_PING_LINE: &str = ": ping\n\n";
pub(crate) const EMPTY_JSON_OBJECT: &str = "{}";
