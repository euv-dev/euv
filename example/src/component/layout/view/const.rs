/// The URL for checking the latest euv crate documentation status.
pub(crate) const DOCS_STATUS_URL: &str = "https://docs.rs/crate/euv/latest/status.json";

/// The `target` value that opens a link in a new browser tab.
///
/// Used for every outbound GitHub link in the desktop sidebar, the mobile
/// header, and the mobile drawer footer.
pub(crate) const LINK_TARGET_BLANK: &str = "_blank";

/// The sidebar section label that introduces the page list.
pub(crate) const NAV_SECTION_LABEL_PAGES: &str = "Pages";

/// The GitHub repository URL for the project.
pub(crate) const GITHUB_URL: &str = "https://github.com/euv-dev/euv";

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

pub(crate) const THEME_DARK: &str = "dark";

/// The selector value for the build with no `theme-*` feature enabled.
pub(crate) const THEME_FEATURE_DEFAULT: &str = "default";

/// Prefix shared by the separately compiled theme features.
pub(crate) const THEME_FEATURE_PREFIX: &str = "theme-";

/// The caller-styled build, which takes precedence over built-in component styles.
pub(crate) const THEME_FEATURE_CUSTOM: &str = "theme-custom";

/// The explicit monochrome feature, distinct from the no-feature build.
pub(crate) const THEME_FEATURE_MONOCHROME: &str = "theme-monochrome";

/// The visible label for the build with no `theme-*` feature enabled.
pub(crate) const THEME_FEATURE_DEFAULT_LABEL: &str = "default (no theme-*)";

/// The accessible and visible label for the sidebar theme feature selector.
pub(crate) const THEME_FEATURE_SELECTOR_LABEL: &str = "Theme feature";

/// The stable id assigned to the sidebar theme feature selector.
pub(crate) const THEME_FEATURE_SELECTOR_ID: &str = "theme-feature-selector";

/// The directory containing the independently built theme variants.
pub(crate) const THEME_FEATURES_DIRECTORY: &str = "themes";

/// The path separator identifying a theme variant directory in a URL.
pub(crate) const THEME_FEATURES_PATH_SEPARATOR: &str = "/themes/";

/// The entry HTML filename written for every independently built variant.
pub(crate) const THEME_FEATURE_INDEX_FILE: &str = "index.html";

/// Every selectable example theme build, including the no-feature build.
pub(crate) const THEME_FEATURE_OPTIONS: [(&str, &str); 19] = [
    (THEME_FEATURE_DEFAULT, THEME_FEATURE_DEFAULT_LABEL),
    ("theme-antdesign", "theme-antdesign"),
    ("theme-bootstrap", "theme-bootstrap"),
    ("theme-cupertino", "theme-cupertino"),
    ("theme-custom", "theme-custom"),
    ("theme-daisyui", "theme-daisyui"),
    ("theme-fluent", "theme-fluent"),
    ("theme-geist", "theme-geist"),
    ("theme-heroui", "theme-heroui"),
    ("theme-linear", "theme-linear"),
    ("theme-mantine", "theme-mantine"),
    ("theme-material", "theme-material"),
    ("theme-monochrome", "theme-monochrome"),
    ("theme-notion", "theme-notion"),
    ("theme-park", "theme-park"),
    ("theme-radix", "theme-radix"),
    ("theme-shadcn", "theme-shadcn"),
    ("theme-stripe", "theme-stripe"),
    ("theme-supabase", "theme-supabase"),
];
