/// Android `AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM` pixel format constant (`1`).
pub(crate) const ANDROID_WINDOW_FORMAT_RGBA_8888: i32 = 1;

/// Android `ANATIVEWINDOW_FRAME_RATE_COMPATIBILITY_FIXED_SOURCE` constant (`1`).
pub(crate) const ANDROID_FRAME_RATE_COMPAT_FIXED_SOURCE: i8 = 1;

/// Android `ANATIVEWINDOW_CHANGE_FRAME_RATE_ONLY_IF_SEAMLESS` constant (`0`).
pub(crate) const ANDROID_CHANGE_FRAME_RATE_SEAMLESS: i8 = 0;

/// POSIX `RTLD_LAZY` flag for dynamically resolving Android API 30+/31+ NDK symbols.
pub(crate) const ANDROID_RTLD_LAZY: i32 = 1;

/// Null-terminated shared library name for Android NDK `libandroid.so`.
pub(crate) const ANDROID_LIB_NAME: &str = "libandroid.so\0";

/// Null-terminated Android API 31+ symbol `ANativeWindow_setFrameRateWithChangeStrategy`.
pub(crate) const SYM_ANDROID_SET_FRAME_RATE_STRATEGY: &str =
    "ANativeWindow_setFrameRateWithChangeStrategy\0";

/// Null-terminated Android API 30+ symbol `ANativeWindow_setFrameRate`.
pub(crate) const SYM_ANDROID_SET_FRAME_RATE: &str = "ANativeWindow_setFrameRate\0";

/// Default Android status-bar top safe-area inset in logical dp.
pub(crate) const ANDROID_DEFAULT_SAFE_TOP: f32 = 28.0;

/// Default Android gesture-navigation bottom safe-area inset in logical dp.
pub(crate) const ANDROID_DEFAULT_SAFE_BOTTOM: f32 = 24.0;

/// Error message when `ANativeWindow_lock` fails to lock the hardware surface buffer.
pub(crate) const ERR_ANDROID_LOCK_SURFACE: &str = "ANativeWindow_lock returned non-zero status";
