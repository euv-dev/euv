use super::*;

/// A global handle to the Tauri `AppHandle`, available only in debug builds.
///
/// Used by the `euv_log!` macro to emit debug log events to the frontend debug panel.
#[cfg(debug_assertions)]
pub(crate) static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

/// A lazily-initialized global HTTP client that reuses connection pools across all fetch operations.
///
/// Avoids the overhead of creating a new `Client` instance per request, which would
/// re-establish TLS sessions and DNS resolutions each time.
pub(crate) static HTTP_CLIENT: OnceLock<Result<Client, String>> = OnceLock::new();

/// Cached `scraper::Selector` instances used by `extract_resources_with_scraper`.
///
/// Compiled CSS selectors are created lazily via `Selector::parse`. Because the
/// selector strings are compile-time constants, parse errors cannot occur at
/// runtime — the cache stores a `Result` purely so the initializer stays free of
/// `unwrap` / `expect` (per rust-standards §R11.4) and can surface the (theoretical)
/// error to callers without panicking.
pub(crate) static RESOURCE_SELECTORS: OnceLock<Result<[Selector; 4], String>> = OnceLock::new();
