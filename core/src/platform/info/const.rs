/// Canonical name string for Web platform.
pub const PLATFORM_NAME_WEB: &str = "web";

/// Canonical name string for Android native platform.
pub const PLATFORM_NAME_ANDROID: &str = "android";

/// Canonical name string for iOS native platform.
pub const PLATFORM_NAME_IOS: &str = "ios";

/// Canonical name string for iPadOS native platform.
pub const PLATFORM_NAME_IPADOS: &str = "ipados";

/// Canonical name string for macOS native platform.
pub const PLATFORM_NAME_MACOS: &str = "macos";

/// Canonical name string for Windows native platform.
pub const PLATFORM_NAME_WINDOWS: &str = "windows";

/// Canonical name string for Linux native platform.
pub const PLATFORM_NAME_LINUX: &str = "linux";

/// Canonical name string for Headless offscreen platform.
pub const PLATFORM_NAME_HEADLESS: &str = "headless";

/// Minimum supported display refresh rate in Hertz.
pub const MIN_REFRESH_RATE_HZ: f32 = 24.0;

/// Maximum supported display refresh rate in Hertz.
pub const MAX_REFRESH_RATE_HZ: f32 = 480.0;

/// Fallback display refresh rate in Hertz when the OS reports unknown.
pub const FALLBACK_REFRESH_RATE_HZ: f32 = 60.0;

/// Minimum preferred ProMotion active refresh floor in Hertz on 120Hz+ displays.
pub const PRO_MOTION_MIN_ACTIVE_HZ: f32 = 80.0;

/// Android `MotionEvent.ACTION_DOWN` masked action constant.
pub const ANDROID_ACTION_DOWN: i32 = 0;

/// Android `MotionEvent.ACTION_UP` masked action constant.
pub const ANDROID_ACTION_UP: i32 = 1;

/// Android `MotionEvent.ACTION_MOVE` masked action constant.
pub const ANDROID_ACTION_MOVE: i32 = 2;

/// Android `MotionEvent.ACTION_CANCEL` masked action constant.
pub const ANDROID_ACTION_CANCEL: i32 = 3;

/// Android `MotionEvent.ACTION_POINTER_DOWN` masked action constant.
pub const ANDROID_ACTION_POINTER_DOWN: i32 = 5;

/// Android `MotionEvent.ACTION_POINTER_UP` masked action constant.
pub const ANDROID_ACTION_POINTER_UP: i32 = 6;

/// Native action argument enabling OS immersive fullscreen mode.
pub const FULLSCREEN_ACTION_ON: &str = "on";

/// Native action argument exiting OS immersive fullscreen mode.
pub const FULLSCREEN_ACTION_OFF: &str = "off";
