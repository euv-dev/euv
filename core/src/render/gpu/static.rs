#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::*;

/// Lazily initialized Objective-C selector table eliminating per-frame `CString` and `sel_registerName` overhead on Apple Metal.
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) static METAL_SELECTORS: OnceLock<MetalSelectors> = OnceLock::new();
