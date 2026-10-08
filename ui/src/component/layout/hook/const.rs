/// The window event that re-runs the debounced mobile-viewport measurement.
pub(crate) const WINDOW_EVENT_RESIZE: &str = "resize";

/// The window event fired when entering or leaving a native (browser) fullscreen.
pub(crate) const WINDOW_EVENT_FULLSCREEN_CHANGE: &str = "fullscreenchange";

/// The WebKit-prefixed fullscreen event still emitted by older iOS Safari.
pub(crate) const WINDOW_EVENT_WEBKIT_FULLSCREEN_CHANGE: &str = "webkitfullscreenchange";

/// The CSS custom property carrying the real top safe-area inset for the immersive shell.
pub(crate) const IMMERSIVE_SAFE_TOP_PROPERTY: &str = "--euv-mobile-safe-top";

/// The `window` property a host sets to declare edge-to-edge (immersive) mode.
pub(crate) const IMMERSIVE_WINDOW_FLAG_PROPERTY: &str = "__EUV_IMMERSIVE__";

/// The selector matching the meta tag a host may ship to declare immersive mode.
pub(crate) const IMMERSIVE_META_SELECTOR: &str = r#"meta[name="euv-immersive"]"#;

/// The attribute of the immersive meta tag carrying the declared value.
pub(crate) const IMMERSIVE_META_CONTENT_ATTRIBUTE: &str = "content";

/// The meta tag `content` value that enables immersive mode.
pub(crate) const IMMERSIVE_META_CONTENT_ENABLED: &str = "true";

/// The `position` style property set on the measuring sentinel element.
pub(crate) const SENTINEL_STYLE_POSITION_PROPERTY: &str = "position";

/// The `position` value taking the sentinel out of the normal document flow.
pub(crate) const SENTINEL_STYLE_POSITION_ABSOLUTE: &str = "absolute";

/// The `visibility` style property hiding the sentinel from the user.
pub(crate) const SENTINEL_STYLE_VISIBILITY_PROPERTY: &str = "visibility";

/// The `visibility` value keeping the sentinel invisible.
pub(crate) const SENTINEL_STYLE_VISIBILITY_HIDDEN: &str = "hidden";

/// The `pointer-events` style property keeping the sentinel out of hit-testing.
pub(crate) const SENTINEL_STYLE_POINTER_EVENTS_PROPERTY: &str = "pointer-events";

/// The `pointer-events` value making the sentinel transparent to input.
pub(crate) const SENTINEL_STYLE_POINTER_EVENTS_NONE: &str = "none";

/// The `padding-top` style property probed for the top safe-area inset.
pub(crate) const SAFE_AREA_PADDING_TOP_PROPERTY: &str = "padding-top";

/// The `padding-right` style property probed for the right safe-area inset.
pub(crate) const SAFE_AREA_PADDING_RIGHT_PROPERTY: &str = "padding-right";

/// The `padding-bottom` style property probed for the bottom safe-area inset.
pub(crate) const SAFE_AREA_PADDING_BOTTOM_PROPERTY: &str = "padding-bottom";

/// The `padding-left` style property probed for the left safe-area inset.
pub(crate) const SAFE_AREA_PADDING_LEFT_PROPERTY: &str = "padding-left";

/// The `padding-top` value resolving the top safe-area inset with a zero fallback.
pub(crate) const SAFE_AREA_PADDING_TOP_VALUE: &str = "env(safe-area-inset-top, 0px)";

/// The `padding-right` value resolving the right safe-area inset with a zero fallback.
pub(crate) const SAFE_AREA_PADDING_RIGHT_VALUE: &str = "env(safe-area-inset-right, 0px)";

/// The `padding-bottom` value resolving the bottom safe-area inset with a zero fallback.
pub(crate) const SAFE_AREA_PADDING_BOTTOM_VALUE: &str = "env(safe-area-inset-bottom, 0px)";

/// The `padding-left` value resolving the left safe-area inset with a zero fallback.
pub(crate) const SAFE_AREA_PADDING_LEFT_VALUE: &str = "env(safe-area-inset-left, 0px)";

/// The CSS custom property overridden with the cached top safe-area pixel value.
pub(crate) const SAFE_AREA_INSET_TOP_PROPERTY: &str = "--safe-area-inset-top";

/// The CSS custom property overridden with the cached right safe-area pixel value.
pub(crate) const SAFE_AREA_INSET_RIGHT_PROPERTY: &str = "--safe-area-inset-right";

/// The CSS custom property overridden with the cached bottom safe-area pixel value.
pub(crate) const SAFE_AREA_INSET_BOTTOM_PROPERTY: &str = "--safe-area-inset-bottom";

/// The CSS custom property overridden with the cached left safe-area pixel value.
pub(crate) const SAFE_AREA_INSET_LEFT_PROPERTY: &str = "--safe-area-inset-left";

/// The selector for the mobile app root receiving the cached inset overrides.
pub(crate) const APP_ROOT_MOBILE_SELECTOR: &str = ".c_mobile_app_root";

/// The selector for the desktop app root receiving the cached inset overrides.
pub(crate) const APP_ROOT_DESKTOP_SELECTOR: &str = ".c_app_root";

/// The selector for the fullscreen canvas container patched outside the app root.
pub(crate) const FULLSCREEN_CANVAS_CONTAINER_SELECTOR: &str = ".c_canvas_container_fullscreen";

/// The selector for the fullscreen game container patched outside the app root.
pub(crate) const FULLSCREEN_GAME_CONTAINER_SELECTOR: &str = ".c_game_container_fullscreen";
