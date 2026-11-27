/// Shared library name for Linux X11 client runtime.
pub(crate) const LINUX_LIB_X11: &str = "libX11.so.6\0";

/// Shared library name for Linux Wayland client runtime.
pub(crate) const LINUX_LIB_WAYLAND: &str = "libwayland-client.so.0\0";

/// Shared library name for Linux XRandR extension runtime.
pub(crate) const LINUX_LIB_XRANDR: &str = "libXrandr.so.2\0";

/// Dynamic symbol for `XRRGetScreenInfo`.
pub(crate) const SYM_XRR_GET_SCREEN_INFO: &str = "XRRGetScreenInfo\0";

/// Dynamic symbol for `XRRConfigCurrentRate`.
pub(crate) const SYM_XRR_CONFIG_CURRENT_RATE: &str = "XRRConfigCurrentRate\0";

/// Dynamic symbol for `XRRFreeScreenConfigInfo`.
pub(crate) const SYM_XRR_FREE_SCREEN_CONFIG_INFO: &str = "XRRFreeScreenConfigInfo\0";

/// Environment variable overriding detected display refresh rate in Hertz on Linux.
pub(crate) const LINUX_REFRESH_ENV_VAR: &str = "EUV_REFRESH_RATE_HZ";

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

/// Dynamic symbol for `XDefaultGC`.
pub(crate) const SYM_X_DEFAULT_GC: &str = "XDefaultGC\0";

/// Dynamic symbol for `XDefaultVisual`.
pub(crate) const SYM_X_DEFAULT_VISUAL: &str = "XDefaultVisual\0";

/// Dynamic symbol for `XDefaultDepth`.
pub(crate) const SYM_X_DEFAULT_DEPTH: &str = "XDefaultDepth\0";

/// Dynamic symbol for `XCreateImage`.
pub(crate) const SYM_X_CREATE_IMAGE: &str = "XCreateImage\0";

/// Dynamic symbol for `XPutImage`.
pub(crate) const SYM_X_PUT_IMAGE: &str = "XPutImage\0";

/// Dynamic symbol for `XFree`.
pub(crate) const SYM_X_FREE: &str = "XFree\0";

/// Dynamic symbol for `XSelectInput`.
pub(crate) const SYM_X_SELECT_INPUT: &str = "XSelectInput\0";

/// Dynamic symbol for `XInternAtom`.
pub(crate) const SYM_X_INTERN_ATOM: &str = "XInternAtom\0";

/// Dynamic symbol for `XSetWMProtocols`.
pub(crate) const SYM_X_SET_WM_PROTOCOLS: &str = "XSetWMProtocols\0";

/// Dynamic symbol for `XPending`.
pub(crate) const SYM_X_PENDING: &str = "XPending\0";

/// Dynamic symbol for `XNextEvent`.
pub(crate) const SYM_X_NEXT_EVENT: &str = "XNextEvent\0";

/// Dynamic symbol for `XLookupString`.
pub(crate) const SYM_X_LOOKUP_STRING: &str = "XLookupString\0";

/// Dynamic symbol for `XResourceManagerString`.
pub(crate) const SYM_X_RESOURCE_MANAGER_STRING: &str = "XResourceManagerString\0";

/// Dynamic symbol for `wl_display_connect`.
pub(crate) const SYM_WL_DISPLAY_CONNECT: &str = "wl_display_connect\0";

/// ICCCM atom name for window manager close requests.
pub(crate) const ATOM_WM_DELETE_WINDOW: &str = "WM_DELETE_WINDOW\0";

/// Environment variable names inspected for HiDPI scale overrides on Linux.
pub(crate) const LINUX_DPI_ENV_VARS: [&str; 3] =
    ["EUV_SCALE_FACTOR", "GDK_SCALE", "QT_SCALE_FACTOR"];

/// Xrdb resource key specifying logical DPI on Linux X11 desktops.
pub(crate) const XFT_DPI_PREFIX: &str = "Xft.dpi:";

/// Standard 96 DPI baseline for X11 scale conversion.
pub(crate) const X11_DEFAULT_DPI: f32 = 96.0;

/// POSIX `RTLD_LAZY` flag for `dlopen`.
pub(crate) const RTLD_LAZY: i32 = 1;

/// X11 `ZPixmap` image format identifier.
pub(crate) const X11_Z_PIXMAP: i32 = 2;

/// Combined X11 input mask (`KeyPress | KeyRelease | ButtonPress | ButtonRelease | PointerMotion | Exposure | StructureNotify | FocusChange`).
pub(crate) const X11_EVENT_MASK: isize = 0x0022_804F;

/// X11 `KeyPress` event type.
pub(crate) const X11_KEY_PRESS: i32 = 2;

/// X11 `KeyRelease` event type.
pub(crate) const X11_KEY_RELEASE: i32 = 3;

/// X11 `ButtonPress` event type.
pub(crate) const X11_BUTTON_PRESS: i32 = 4;

/// X11 `ButtonRelease` event type.
pub(crate) const X11_BUTTON_RELEASE: i32 = 5;

/// X11 `MotionNotify` event type.
pub(crate) const X11_MOTION_NOTIFY: i32 = 6;

/// X11 `FocusIn` event type.
pub(crate) const X11_FOCUS_IN: i32 = 9;

/// X11 `FocusOut` event type.
pub(crate) const X11_FOCUS_OUT: i32 = 10;

/// X11 `Expose` event type.
pub(crate) const X11_EXPOSE: i32 = 12;

/// X11 `ConfigureNotify` event type.
pub(crate) const X11_CONFIGURE_NOTIFY: i32 = 22;

/// X11 `ClientMessage` event type.
pub(crate) const X11_CLIENT_MESSAGE: i32 = 33;

/// X11 modifier mask for Shift.
pub(crate) const X11_SHIFT_MASK: u32 = 1;

/// X11 modifier mask for Control.
pub(crate) const X11_CTRL_MASK: u32 = 4;

/// X11 modifier mask for Alt (`Mod1`).
pub(crate) const X11_ALT_MASK: u32 = 8;

/// X11 modifier mask for Super/Meta (`Mod4`).
pub(crate) const X11_META_MASK: u32 = 64;

/// Error message when neither Wayland nor X11 display server is reachable.
pub(crate) const ERR_LINUX_NO_DISPLAY: &str = "neither X11 nor Wayland display server available";
