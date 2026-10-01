/// The brand name displayed in the navigation header.
pub(crate) const BRAND_NAME: &str = "Euv";

/// The GitHub repository URL for the project.
pub(crate) const GITHUB_URL: &str = "https://github.com/euv-dev/euv";

/// The URL for checking the latest euv crate documentation status.
pub(crate) const DOCS_STATUS_URL: &str = "https://docs.rs/crate/euv/latest/status.json";

/// The `target` value that opens a link in a new browser tab.
///
/// Used for every outbound GitHub link in the desktop sidebar, the mobile
/// header, and the mobile drawer footer.
pub(crate) const LINK_TARGET_BLANK: &str = "_blank";

/// The sidebar section label that introduces the page list.
pub(crate) const NAV_SECTION_LABEL_PAGES: &str = "Pages";

/// The leading text of the sidebar footer credit, before the brand span.
pub(crate) const NAV_FOOTER_CREDIT_PREFIX: &str = "Built with ";

/// The brand name shown in the sidebar footer credit span.
pub(crate) const NAV_FOOTER_CREDIT_BRAND: &str = "Euv & Wasm";

/// Message reported when the docs status fetch exhausted every retry.
pub(crate) const MESSAGE_FETCH_DOCS_STATUS_FAILED: &str = "failed to fetch docs status";

/// Message reported when docs.rs offers no version newer than the running one.
pub(crate) const MESSAGE_ALREADY_LATEST_VERSION: &str = "already on the latest version";

/// Message reported when the native bridge is missing, so no update is possible.
pub(crate) const MESSAGE_BRIDGE_UNAVAILABLE: &str = "native bridge is not available";

/// The invoke command name for updating the local cache via bridge.
pub(crate) const INVOKE_UPDATE_CACHE: &str = "update_cache";

/// The fallback string used when a `JsValue` error cannot be converted via `as_string()`.
pub(crate) const ERROR_NULL_TEXT: &str = "null";

/// The maximum number of retry attempts for fetching version status.
pub(crate) const VERSION_FETCH_MAX_RETRY_COUNT: u32 = 8;

/// The delay duration between retry attempts in milliseconds.
pub(crate) const VERSION_FETCH_RETRY_DELAY_MS: u32 = 1000;

/// The maximum number of retry attempts when re-notifying the native side to
/// pull the latest cache after a failed bridge call.
///
/// When `bridge.core.invoke("update_cache", ...)` either rejects on the JS
/// side or returns a `success: false` payload from native, the webview retries
/// the invocation this many times before giving up.
pub(crate) const VIEW_UPDATE_RETRY_COUNT: u32 = 8;

/// The delay duration in milliseconds between retry attempts when
/// re-notifying the native side (see `VIEW_UPDATE_RETRY_COUNT`).
pub(crate) const VIEW_UPDATE_RETRY_DELAY_MS: u32 = 1000;
