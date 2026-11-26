/// Android `AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM` pixel format constant (`1`).
pub(crate) const ANDROID_WINDOW_FORMAT_RGBA_8888: i32 = 1;

/// Default Android status-bar top safe-area inset in logical dp.
pub(crate) const ANDROID_DEFAULT_SAFE_TOP: f32 = 28.0;

/// Default Android gesture-navigation bottom safe-area inset in logical dp.
pub(crate) const ANDROID_DEFAULT_SAFE_BOTTOM: f32 = 24.0;

/// Error message when `ANativeWindow_lock` fails to lock the hardware surface buffer.
pub(crate) const ERR_ANDROID_LOCK_SURFACE: &str = "ANativeWindow_lock returned non-zero status";
