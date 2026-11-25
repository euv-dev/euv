/// Shared library name for Linux X11 client runtime.
pub(crate) const LINUX_LIB_X11: &str = "libX11.so.6\0";

/// Shared library name for Linux Wayland client runtime.
pub(crate) const LINUX_LIB_WAYLAND: &str = "libwayland-client.so.0\0";

/// Dynamic symbol for `XOpenDisplay`.
pub(crate) const SYM_X_OPEN_DISPLAY: &str = "XOpenDisplay\0";

/// Dynamic symbol for `XDefaultScreen`.
pub(crate) const SYM_X_DEFAULT_SCREEN: &str = "XDefaultScreen\0";

/// Dynamic symbol for `XRootWindow`.
pub(crate) const SYM_X_ROOT_WINDOW: &str = "XRootWindow\0";

/// Dynamic symbol for `XCreateSimpleWindow`.
pub(crate) const SYM_X_CREATE_SIMPLE_WINDOW: &str = "XCreateSimpleWindow\0";

/// Dynamic symbol for `XStoreName`.
pub(crate) const SYM_X_STORE_NAME: &str = "XStoreName\0";

/// Dynamic symbol for `XMapWindow`.
pub(crate) const SYM_X_MAP_WINDOW: &str = "XMapWindow\0";

/// Dynamic symbol for `XResizeWindow`.
pub(crate) const SYM_X_RESIZE_WINDOW: &str = "XResizeWindow\0";

/// Dynamic symbol for `XDestroyWindow`.
pub(crate) const SYM_X_DESTROY_WINDOW: &str = "XDestroyWindow\0";

/// Dynamic symbol for `XFlush`.
pub(crate) const SYM_X_FLUSH: &str = "XFlush\0";

/// Dynamic symbol for `wl_display_connect`.
pub(crate) const SYM_WL_DISPLAY_CONNECT: &str = "wl_display_connect\0";

/// POSIX `RTLD_LAZY` flag for `dlopen`.
pub(crate) const RTLD_LAZY: i32 = 1;

/// Error message when neither Wayland nor X11 display server is reachable.
pub(crate) const ERR_LINUX_NO_DISPLAY: &str = "neither X11 nor Wayland display server available";
